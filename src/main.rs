#![windows_subsystem = "windows"]

mod state;
mod storage;
mod titra;
mod views;
pub mod export;
pub mod user;

use dotenv::dotenv;
use egui::{IconData, ThemePreference};
use log::info;
use titra::*;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::BTreeMap;

fn main() {
    dotenv().ok();
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]).with_title("Titra").with_icon(IconData::default()),
 
        ..Default::default()
    };

    info!("Test");

    let text_styles: BTreeMap<_, _> = [
        (Heading, FontId::new(24.0, Proportional)),
        (Name("Heading2".into()), FontId::new(16.0, Proportional)),
        (Name("Context".into()), FontId::new(14.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
      ].into();
      
    eframe::run_native(
        "Titra",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(ThemePreference::Dark);
            cc.egui_ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
            Ok(Box::new(Titra::new(TitraConfig {
                root_dir: ".".into(),
                storage_impl: storage::StorageImplementation::Sqlite,
            })))
        }),
    )
    .unwrap();
}
