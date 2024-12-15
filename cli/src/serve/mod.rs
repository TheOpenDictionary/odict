use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use clap::{command, Args, ValueEnum};
use console::style;
use env_logger::Env;
use odict::{config::AliasManager, DictionaryFile, DictionaryReader};

use crate::CLIContext;

mod lookup;
mod search;

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
    reader: &DictionaryReader,
    alias_manager: &AliasManager,
    dictionaries: &Vec<String>,
) -> anyhow::Result<HashMap<String, DictionaryFile>> {
    let mut dictionary_map = HashMap::<String, DictionaryFile>::new();

    for dictionary in dictionaries {
        let dict = reader.read_from_path_or_alias_with_manager(&dictionary, &alias_manager)?;

        dictionary_map.insert(
            PathBuf::from(dictionary)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            dict,
        );
    }

    Ok(dictionary_map)
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

    let dictionary_map = get_dictionary_map(reader, alias_manager, &dictionaries)?;
    let log_level = format!("{}", level.as_ref().unwrap_or(&LogLevel::Info));

    ctx.println(format!(
        "\n🟢  Serving the following dictionaries on port {} with log level \"{}\":\n",
        port, log_level
    ));

    for (name, dict) in &dictionary_map {
        ctx.println(format!(
            "   • {} {}",
            style(name).bold(),
            style(format!(
                "({})",
                dict.path.as_ref().unwrap().to_string_lossy()
            ))
            .dim()
        ));
    }

    ctx.println("");

    env_logger::init_from_env(Env::new().default_filter_or(log_level));

    let data = Data::new(dictionary_map);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&data))
            .service(lookup::handle_lookup)
            .service(search::handle_search)
    })
    .bind(("127.0.0.1", *port))?
    .run()
    .await?;

    Ok(())
}
