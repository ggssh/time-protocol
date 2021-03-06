#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, IconData};
use std::path::Path;
use time_protocol::{time_client::Client, tool::vec_to_string};

const ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.png");

fn main() {
    // windows settings
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    // let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.png");
    let (icon_rgba, icon_width, icon_height) = get_icon(Path::new(ICON_PATH));
    options.icon_data = Some(IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    });

    eframe::run_native(
        "TIME protocol test demo",
        options,
        Box::new(|_cc| Box::new(TimeApp::default())),
    );
}

struct TimeApp {
    address: String,
    port: String,
    messages: Vec<String>,
}

impl Default for TimeApp {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: "37".to_string(),
            messages: Vec::new(),
        }
    }
}

impl eframe::App for TimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("20198614 yyz Time Client");

                ui.label("address: ");
                ui.text_edit_singleline(&mut self.address);
                ui.label("port:");
                ui.text_edit_singleline(&mut self.port);

                if ui.button("Update Time").clicked() {
                    let mut client = Client::default();
                    client.set_address(&self.address);
                    client.set_port(self.port.parse::<u32>().unwrap_or(37));
                    let time = client.update().unwrap_or(
                        "Some errors occurred. Please check port and address.".to_string(),
                    );

                    // ????????????
                    // #[cfg(debug_assertions)]
                    // println!("{:#?}", &client);

                    self.messages.push(format!(
                        "[TIME Protocol] from {}:{} update time: {}",
                        self.address, self.port, time
                    ));
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut vec_to_string(&self.messages))
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::DARK_GREEN)
                            .interactive(false)
                            .desired_rows(5),
                    )
                })
            });
        });
    }
}

fn get_icon(path: &Path) -> (Vec<u8>, u32, u32) {
    let image = image::open(path)
        .expect("Failed to open icon path")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
}
