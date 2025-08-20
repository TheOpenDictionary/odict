use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs,
    num::NonZero,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use clap::{command, Args, ValueEnum};
use console::style;
use env_logger::Env;
use lru::LruCache;
use odict::{DictionaryFile, DictionaryLoader};

use crate::CLIContext;

mod lookup;
mod search;
mod tokenize;

#[derive(Debug, Clone, ValueEnum)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct ServeArgs {
    #[arg(short, default_value_t = 5005, help = "Port to listen on")]
    port: u16,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Maximum number of dictionaries to keep in memory"
    )]
    capacity: usize,

    // Sets the default log level
    #[arg(short, long)]
    level: Option<LogLevel>,

    // List of dictionary paths or aliases to serve
    #[arg()]
    dictionaries: Vec<String>,
}

pub(self) fn get_dictionary_map(
    dictionaries: &Vec<String>,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let mut dictionary_map = HashMap::<String, PathBuf>::new();

    for path in dictionaries {
        let path_buf = PathBuf::from(path);

        if path_buf.is_dir() {
            for entry in fs::read_dir(path_buf)? {
                let p = entry?.path();
                let name = p.file_stem().unwrap().to_string_lossy().to_string();

                if p.is_file() && p.extension().map_or(false, |ext| ext == "odict") {
                    dictionary_map.insert(name.clone(), p.clone());
                }
            }
        } else {
            let name = path_buf.file_stem().unwrap().to_string_lossy().to_string();
            dictionary_map.insert(name.clone(), path_buf.clone());
        }
    }

    Ok(dictionary_map)
}

struct DictionaryCache {
    cache: RwLock<LruCache<String, Arc<DictionaryFile>>>,
    dictionaries: HashMap<String, PathBuf>,
    loader: DictionaryLoader,
}

impl DictionaryCache {
    fn new(
        size: NonZero<usize>,
        dictionaries: HashMap<String, PathBuf>,
        loader: DictionaryLoader,
    ) -> Self {
        DictionaryCache {
            cache: RwLock::new(LruCache::new(size)),
            dictionaries,
            loader,
        }
    }

    pub async fn get(&self, key: &str) -> anyhow::Result<Option<Arc<DictionaryFile>>> {
        {
            let cache = self.cache.read().unwrap();
            if let Some(file) = cache.peek(key) {
                // Return a clone of the Arc (cheap operation)
                return Ok(Some(Arc::clone(file)));
            }
        }

        // Not in cache, need to load it
        if let Some(path) = self.dictionaries.get(key) {
            let dict = self
                .loader
                .load(path.to_string_lossy().as_ref())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?;

            // Now get a write lock to update the cache
            let mut cache = self.cache.write().unwrap();

            // Wrap in Arc before putting in cache
            let arc_dict = Arc::new(dict);
            cache.put(String::from(key), Arc::clone(&arc_dict));

            return Ok(Some(arc_dict));
        }

        Ok(None)
    }
}

pub async fn serve<'a>(ctx: &mut CLIContext<'a>, args: &ServeArgs) -> anyhow::Result<()> {
    let ServeArgs {
        port,
        dictionaries,
        level,
        capacity,
    } = args;

    let CLIContext { loader, .. } = ctx;

    let dictionary_map = get_dictionary_map(&dictionaries)?;
    let log_level = format!("{}", level.as_ref().unwrap_or(&LogLevel::Info));

    let dictionary_cache = DictionaryCache::new(
        NonZero::new(*capacity).unwrap(),
        dictionary_map.to_owned(),
        loader.to_owned(),
    );

    if dictionary_map.is_empty() {
        ctx.println(format!(
            "\n⚠️  No dictionaries found to serve. Please provide valid dictionary files or directories containing .odict files."
        ));
        return Ok(());
    }

    ctx.println(format!(
        "\n🟢  Serving the following dictionaries on port {} with log level \"{}\":\n",
        port, log_level
    ));

    for (name, dict) in &dictionary_map {
        ctx.println(format!(
            "   • {} {}",
            style(name).bold(),
            style(format!("({})", dict.to_string_lossy())).dim()
        ));
    }

    ctx.println("");

    env_logger::init_from_env(Env::new().default_filter_or(log_level));

    let data = Data::new(dictionary_cache);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&data))
            .service(lookup::handle_lookup)
            .service(search::handle_search)
            .service(tokenize::handle_tokenize)
    })
    .bind(("127.0.0.1", *port))?
    .run()
    .await?;

    Ok(())
}
