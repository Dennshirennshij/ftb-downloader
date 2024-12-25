use serde_json::Value;
use std::{
    error::Error,
    fmt::Debug,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
};

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
    fn parse_files(json: serde_json::Value) -> Vec<OnlineFile> {
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
        result
    }
    fn download(&self, output_dir: &Path) -> Result<(), Box<dyn Error>> {
        println!("Downloading {:?}", self.path);
        let mut path: PathBuf = output_dir.to_path_buf();
        path.push(self.path.clone());

        let _ = Command::new("curl")
            .arg("--silent")
            .arg(self.url.trim())
            .arg("--output")
            .arg(path)
            .arg("--create-dirs")
            .spawn()?;

        Ok(())
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct ProgressUpdate {
    pub downloaded_files: usize,
    pub total_files: usize,
    pub done: bool,
}

pub async fn download(
    pack_id: u32,
    release_id: u32,
    client: bool,
    out: PathBuf,
    update_channel: Option<Arc<Mutex<ProgressUpdate>>>,
) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!(
        "https://api.feed-the-beast.com/v1/modpacks/public/modpack/{}/{}/",
        pack_id, release_id
    ))
    .await?
    .text()
    .await?;
    let json: Value = serde_json::from_str(&res)?;
    let files = OnlineFile::parse_files(json["files"].clone());
    let total_files = files.len();

    if let Some(channel) = &update_channel {
        let mut lock = channel.lock().unwrap();
        lock.total_files = total_files;
        drop(lock);
    }

    for (i, file) in files
        .iter()
        .filter(|file| if client { file.client } else { file.server })
        .enumerate()
    {
        file.download(&out)?;
        if let Some(channel) = &update_channel {
            let mut lock = channel.lock().unwrap();
            lock.downloaded_files = i + 1;
            drop(lock);
        }
    }
    if let Some(channel) = &update_channel {
        let mut lock = channel.lock().unwrap();
        lock.done = true;
        drop(lock);
    }
    Ok(())
}
