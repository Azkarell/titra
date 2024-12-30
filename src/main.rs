
mod titra;
mod storage;
mod views;
mod state;
use dotenv::dotenv;
use log::info;
use titra::*;

fn main() {

    dotenv().ok();
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    info!("Test");

    eframe::run_native("Titra", options, Box::new(|cc| Ok(Box::new(Titra::new(TitraConfig{
        root_dir: ".".into(),
        storage_impl: storage::StorageImplementation::Sqlite
    }, ))))).unwrap();

}


