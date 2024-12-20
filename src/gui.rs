use std::path::PathBuf;
use eframe::egui::{CentralPanel, Context, TopBottomPanel, ViewportBuilder};

struct TestApp {
    name: String,
    age: u32,
}
impl Default for TestApp {
    fn default() -> Self {
        Self {
            name: "Max Mustermann".to_string(),
            age: 42,
        }
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Give me your data now!");

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.separator();

            ui.label(format!("Your name is {}.", &self.name));

        });
    }
}

pub fn open_test_gui() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Test application",
        native_options,
        Box::new(|_cc| Ok(Box::new(TestApp::default()))),
    )
}

pub fn open_gui() -> (u32, u32, bool, PathBuf) {
    todo!()
}
