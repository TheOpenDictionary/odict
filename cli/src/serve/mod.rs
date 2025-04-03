use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs,
    num::NonZero,
    path::PathBuf,
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use clap::{command, Args, ValueEnum};
use console::style;
use env_logger::Env;
use lru::LruCache;
use odict::{config::AliasManager, DictionaryFile, DictionaryReader};

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

    // Sets the default log level
    #[arg(short, long)]
    level: Option<LogLevel>,

    // List of dictionary paths or aliases to serve
    #[arg()]
    dictionaries: Vec<String>,
}

pub(self) fn get_dictionary_map(
    dictionaries: &Vec<String>,
) -> odict::Result<HashMap<String, PathBuf>> {
    let mut dictionary_map = HashMap::<String, PathBuf>::new();

    for path in dictionaries {
        let path_buf = PathBuf::from(path);
        let name = path_buf.file_stem().unwrap().to_string_lossy().to_string();

        if path_buf.is_dir() {
            for entry in fs::read_dir(path_buf)? {
                let p = entry?.path();

                if p.is_file() && p.extension().map_or(false, |ext| ext == "odict") {
                    dictionary_map.insert(name.clone(), p.clone());
                }
            }
        } else {
            dictionary_map.insert(name.clone(), path_buf.clone());
        }
    }

    Ok(dictionary_map)
}

struct DictionaryCache {
    cache: LruCache<String, DictionaryFile>,
    dictionaries: HashMap<String, PathBuf>,
    reader: DictionaryReader,
    alias_manager: AliasManager,
}

impl DictionaryCache {
    fn new(
        size: NonZero<usize>,
        dictionaries: HashMap<String, PathBuf>,
        reader: DictionaryReader,
        alias_manager: AliasManager,
    ) -> Self {
        DictionaryCache {
            cache: LruCache::new(size),
            dictionaries,
            reader,
            alias_manager,
        }
    }

    pub fn get(&mut self, key: &str) -> odict::Result<Option<&DictionaryFile>> {
        if !self.cache.contains(key) {
            if let Some(path) = self.dictionaries.get(key) {
                let dict = self.reader.read_from_path_or_alias_with_manager(
                    path.to_string_lossy().as_ref(),
                    &self.alias_manager,
                )?;

                self.cache.put(String::from(key), dict);
            } else {
                return Ok(None);
            }
        }

        return Ok(self.cache.get(key));
    }
}

#[actix_web::main]
pub async fn serve(ctx: &mut CLIContext, args: &ServeArgs) -> anyhow::Result<()> {
    let ServeArgs {
        port,
        dictionaries,
        level,
    } = args;

    let CLIContext {
        alias_manager,
        reader,
        ..
    } = ctx;

    let dictionary_map = get_dictionary_map(&dictionaries)?;
    let log_level = format!("{}", level.as_ref().unwrap_or(&LogLevel::Info));

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

    let dictionary_cache = DictionaryCache::new(
        NonZero::new(100).unwrap(),
        dictionary_map.to_owned(),
        reader.to_owned(),
        alias_manager.to_owned(),
    );

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
