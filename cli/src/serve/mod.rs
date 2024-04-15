use std::{
    collections::HashMap, error::Error, fs::canonicalize, ops::Deref, path::PathBuf, rc::Rc,
    sync::Arc,
};

use actix_web::{App, HttpServer};
use clap::{command, Args};
use odict::ArchivedDictionary;

use crate::CLIContext;

mod lookup;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct ServeArgs {
    #[arg(short, default_value_t = 5005, help = "Port to listen on")]
    port: u16,

    #[arg(help = "List of dictionary paths or aliases to serve")]
    dictionaries: Vec<String>,
}

#[actix_web::main]
pub async fn serve(ctx: &mut CLIContext, args: &ServeArgs) -> Result<(), Box<dyn Error>> {
    let ServeArgs { port, dictionaries } = args;

    let CLIContext {
        alias_manager,
        reader,
        ..
    } = ctx;

    let mut dictionary_map = HashMap::<String, ArchivedDictionary>::new();

    println!("{:?}", dictionary_map);
    for dictionary in dictionaries {
        let dict = reader.read_from_path_or_alias_with_manager(&dictionary, &alias_manager)?;
        dictionary_map.insert(dictionary.to_owned(), dict.to_archive()?);
    }

    ctx.println(format!("\n🟢 Listening on port {}\n", port));

    let r = Arc::new(dictionary_map);

    HttpServer::new(move || {
        App::new()
            .app_data(r.clone())
            .service(lookup::handle_lookup)
    })
    .bind(("127.0.0.1", *port))?
    .run()
    .await?;

    Ok(())
}
