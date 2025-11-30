use std::path::PathBuf;
use std::time::Duration;

use crate::CLIContext;
use clap::{arg, command, Args};
use indicatif::{ProgressBar, ProgressStyle};
use odict::download::{DictionaryDownloader, DownloadOptions};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct DownloadArgs {
    #[arg(
        required = true,
        help = "Dictionary to download (e.g., 'wiktionary/eng')"
    )]
    dictionary: String,

    #[arg(
        short,
        long,
        help = "Directory to download the dictionary to (defaults to config directory)"
    )]
    output: Option<PathBuf>,

    #[arg(
        long,
        default_value_t = false,
        help = "Disable caching (always download fresh copy)"
    )]
    no_cache: bool,

    #[arg(
        short = 'r',
        long,
        default_value_t = 3,
        help = "Number of retry attempts for corrupted downloads"
    )]
    retries: u32,
}

pub async fn download(ctx: &mut CLIContext<'_>, args: &DownloadArgs) -> anyhow::Result<()> {
    let DownloadArgs {
        dictionary,
        output,
        no_cache,
        retries,
    } = args;

    let progress_bar = ProgressBar::new(0);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
    );

    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let mut downloader = DictionaryDownloader::default()
        .with_retries(*retries)
        .with_caching(!no_cache);

    if let Some(out_dir) = output {
        downloader = downloader.with_out_dir(out_dir);
    }

    let options = DownloadOptions::default().on_progress(|downloaded, total, _progress| {
        if let Some(total_bytes) = total {
            progress_bar.set_length(total_bytes);
        }
        progress_bar.set_position(downloaded);
    });

    match downloader.download_with_options(dictionary, &options).await {
        Ok(file) => {
            let dict = file.contents()?;

            progress_bar.finish_and_clear();

            ctx.println(format!(
                "✅ Successfully downloaded {} to {}",
                dict.name
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or(dictionary.to_string()),
                file.path().as_ref().unwrap().display()
            ));

            Ok(())
        }
        Err(err) => {
            progress_bar.finish_and_clear();
            Err(anyhow::Error::from(err))
        }
    }
}
