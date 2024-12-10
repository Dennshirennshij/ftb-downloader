mod gui;

use clap::Parser;
use serde_json::Value;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pack_id: u32,
    #[arg(short, long)]
    release: u32,
    #[arg(short, long)]
    out: PathBuf,
    #[arg(short, long, default_value = "true")]
    client: bool,
    #[arg(long, default_value = "false")]
    cli: bool,
}

fn parse_targets(targets: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    loop {
        let current_target = targets[i].clone();
        if current_target["id"].as_i64().is_none() {
            break;
        }

        let id = current_target["id"].as_i64().unwrap();
        let name = current_target["name"].as_str().unwrap();
        let version = current_target["version"].as_str().unwrap();
        let r#type = current_target["type"].as_str().unwrap();
        let updated = current_target["updated"].as_u64().unwrap();

        println!(
            "i: {}\nid: {}\nname: {}\nversion: {}\ntype: {}\nupdated: {}\n\n\n",
            i, id, name, version, r#type, updated
        );

        i += 1;
    }
    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct OnlineFile {
    path: PathBuf,
    url: String,
    server: bool,
    client: bool,
}
impl OnlineFile {
    fn from_json(json: serde_json::Value) -> Result<Self, Box<dyn Error>> {
        if json["clientonly"].as_bool().is_none() {
            return Err("Error".into());
        }
        let client_only = json["clientonly"].as_bool().unwrap();
        let server_only = json["serveronly"].as_bool().unwrap();
        println!("Parsing File: {:?}", json);
        let mut path = PathBuf::from_str(json["path"].as_str().unwrap())?;
        path.push(json["name"].as_str().unwrap());

        let url = match json["mirrors"][0].as_str() {
            Some(url) => url,
            None => json["url"].as_str().unwrap(),
        }
        .to_string();

        Ok(OnlineFile {
            path,
            url,
            server: !client_only,
            client: !server_only,
        })
    }
    fn parse_files(json: serde_json::Value) -> Result<Vec<OnlineFile>, Box<dyn Error>> {
        let mut result: Vec<OnlineFile> = Vec::new();
        let mut i = 0;
        loop {
            println!("i: {}", i);
            if let Ok(file) = OnlineFile::from_json(json[i].clone()) {
                result.push(file);
            } else {
                break;
            }
            i += 1;
            println!();
        }
        Ok(result)
    }
    fn download(&self, output_dir: &Path) -> Result<(), Box<dyn Error>> {
        println!("Downloading {:?}", self.path);
        let mut path: PathBuf = output_dir.to_path_buf();
        path.push(self.path.clone());

        let _ = Command::new("curl")
            .arg(self.url.trim())
            .arg("--output")
            .arg(path)
            .arg("--create-dirs")
            .spawn()?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // Parse the arguments
    // or open the gui
    let args = Args::parse(); 
    let (pack_id, release_id, client, out) = match args.cli {
        true => {
            (args.pack_id, args.release, args.client, args.out)
        },
        false => {
            panic!("Gui not yet implemented");
        },
    };

    // Get the modpack index
    let res = reqwest::get(format!(
        "https://api.feed-the-beast.com/v1/modpacks/public/modpack/{}/{}/",
        pack_id, release_id
    ))
    .await?
    .text()
    .await?;

    // Parse the index of the given modpack into a struct
    let json: Value = serde_json::from_str(&res)?;
    let files = OnlineFile::parse_files(json["files"].clone())?;
    for file in files.iter().filter(|file| {
        if client {
            file.client
        } else {
            file.server
        }
    }) {
        file.download(&out)?;
    }

    // Parse the targets (minecraft version, forge version & java version)
    let targets = json["targets"].clone();
    parse_targets(targets)?;

    Ok(())
}
