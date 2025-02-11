use crate::downloader::{download, ProgressUpdate};
use eframe::egui::{CentralPanel, ViewportBuilder};
use egui::{Button, ProgressBar};
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime::Runtime;

struct InputApp {
    pack_id: String,
    release_id: String,
    path: Option<PathBuf>,
    updates: Option<Arc<Mutex<ProgressUpdate>>>,
}
impl Default for InputApp {
    fn default() -> Self {
        Self {
            pack_id: "0".to_string(),
            release_id: "0".to_string(),
            path: None,
            updates: None,
        }
    }
}
impl eframe::App for InputApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let download_server = Button::new("Download server files");
        let download_client = Button::new("Download client files");

        // Remove the value from the option when the download is completed
        if let Some(update) = &self.updates {
            let clone = update.clone();
            let lock = clone.lock().unwrap();
            if lock.done {
                self.updates = None;
            }
            drop(lock);
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Download a FTB Modpack here");
            ui.horizontal(|ui| {
                ui.label("Pack ID: ");
                if self.updates.is_some() {
                    ui.label(&self.pack_id);
                } else {
                    ui.text_edit_singleline(&mut self.pack_id);
                }
            });
            if self.pack_id.parse::<u32>().is_err() {
                ui.label("Pack ID is not a number!!!");
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Release ID: ");
                if self.updates.is_some() {
                    ui.label(&self.release_id);
                } else {
                    ui.text_edit_singleline(&mut self.release_id);
                }
            });
            if self.release_id.parse::<u32>().is_err() {
                ui.label("Release ID is not a number!!!");
            }
            ui.separator();

            if let Some(path) = &self.path {
                ui.label(path.to_str().unwrap());
            }
            if ui.button("Select Folder").clicked() {
                if let Some(picked_path) = FileDialog::new().pick_folder() {
                    self.path = Some(picked_path);
                }
            }

            ui.separator();
            if let Some(update) = &self.updates {
                let lock = update.lock().unwrap();

                let progress = lock.downloaded_files as f32 / lock.total_files as f32;
                ui.add(ProgressBar::new(progress));

                drop(lock);
            } else {
                ui.horizontal(|ui| {
                    if ui.add(download_server).clicked()
                        && !(self.pack_id.parse::<u32>().is_err()
                            || self.release_id.parse::<u32>().is_err()
                            || self.path.is_none())
                    {
                        let pack = self.pack_id.parse::<u32>().unwrap();
                        let release = self.release_id.parse::<u32>().unwrap();

                        let progress = Arc::new(Mutex::new(ProgressUpdate::default()));
                        self.updates = Some(progress.clone());

                        spawn_thread(pack, release, false, self.path.clone().unwrap(), progress);
                    }
                    if ui.add(download_client).clicked()
                        && !(self.pack_id.parse::<u32>().is_err()
                            || self.release_id.parse::<u32>().is_err()
                            || self.path.is_none())
                    {
                        let pack = self.pack_id.parse::<u32>().unwrap();
                        let release = self.release_id.parse::<u32>().unwrap();

                        let progress = Arc::new(Mutex::new(ProgressUpdate::default()));
                        self.updates = Some(progress.clone());

                        spawn_thread(pack, release, true, self.path.clone().unwrap(), progress);
                    }
                });
            }
        });
    }
}

fn spawn_thread(
    pack: u32,
    release: u32,
    client: bool,
    output: PathBuf,
    progress: Arc<Mutex<ProgressUpdate>>,
) {
    let _handle = thread::spawn(move || {
        println!("Spawned thread");
        let result = Runtime::new().unwrap().block_on(download(
            pack,
            release,
            client,
            output,
            Some(progress),
        ));
        match result {
            Ok(()) => println!("No errors"),
            Err(e) => println!("Error: {:?}", e),
        }
        println!("Closed thread");
    });
}

pub fn gui_main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([200.0, 150.0]),
        ..Default::default()
    };
    eframe::run_native(
        "FTB Downloader",
        options,
        Box::new(|_cc| Ok(Box::new(InputApp::default()))),
    )
    .expect("Failed to create the GUI");
}
