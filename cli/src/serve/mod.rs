use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use clap::{command, Args, ValueEnum};
use env_logger::Env;
use odict::DictionaryFile;

use crate::CLIContext;

mod lookup;

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

    // Log level
    #[arg(short, long)]
    level: Option<LogLevel>,

    // List of dictionary paths or aliases to serve
    #[arg()]
    dictionaries: Vec<String>,
}

#[actix_web::main]
pub async fn serve(ctx: &mut CLIContext, args: &ServeArgs) -> Result<(), Box<dyn Error>> {
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

    let mut dictionary_map = HashMap::<String, DictionaryFile>::new();

    for dictionary in dictionaries {
        let dict = reader.read_from_path_or_alias_with_manager(&dictionary, &alias_manager)?;
        dictionary_map.insert(dictionary.to_owned(), dict);
    }

    ctx.println(format!("\n🟢 Listening on port {}\n", port));

    env_logger::init_from_env(
        Env::new().filter(format!("{}", level.as_ref().unwrap_or(&LogLevel::Info))),
    );

    let data = Data::new(dictionary_map);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .wrap(Logger::default())
            .service(lookup::handle_lookup)
    })
    .bind(("127.0.0.1", *port))?
    .run()
    .await?;

    Ok(())
}
