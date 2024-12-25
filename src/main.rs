mod downloader;
mod gui;

use clap::Parser;
use core::panic;
use std::{error::Error, fmt::Debug, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pack_id: Option<u32>,
    #[arg(short, long)]
    release: Option<u32>,
    #[arg(short, long)]
    out: Option<PathBuf>,
    #[arg(short, long, default_value = "true")]
    client: Option<bool>,
    #[arg(long, default_value = "false")]
    cli: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse the arguments
    // or open the gui
    let args = Args::parse();

    if args.cli {
        if args.client.is_none() {
            panic!("Client?");
        }
        if args.out.is_none() {
            panic!("Output dir?");
        }
        if args.pack_id.is_none() {
            panic!("Pack ID?");
        }
        if args.release.is_none() {
            panic!("Release ID?");
        }
        downloader::download(
            args.pack_id.unwrap(),
            args.release.unwrap(),
            args.client.unwrap(),
            args.out.unwrap(),
            None,
        )
        .await?;
    } else {
        gui::gui_main();
    }

    Ok(())
}
