use std::{collections::HashMap, error::Error, fs::canonicalize, ops::Deref, path::PathBuf};

use actix_web::{App, HttpServer};
use clap::{command, Args};

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
    let port = args.port;
    let dictionaries = &args.dictionaries;
    let mut dictionary_map = HashMap::<String, String>::new();
    let alias_manager = &ctx.alias_manager;

    for dictionary in dictionaries {
        let path = alias_manager.get(&dictionary);

        if let Some(p) = path {
            dictionary_map.insert(dictionary.to_owned(), p.to_owned());
        } else {
            let pb = PathBuf::from(dictionary);

            if let Some(name) = pb.file_stem().map(|s| s.to_string_lossy().to_string()) {
                let p = canonicalize(&pb)?.to_string_lossy().to_string();
                dictionary_map.insert(name, p);
            }
        }
    }

    println!("{:?}", dictionary_map);
    ctx.println(format!("\n🟢 Listening on port {}\n", port));

    HttpServer::new(move || {
        App::new()
            .app_data(dictionary_map.to_owned())
            .service(lookup::handle_lookup)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
