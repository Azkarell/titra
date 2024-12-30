
mod titra;
mod storage;
mod views;
mod state;
use titra::*;

fn main() {

    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };


    eframe::run_native("Titra", options, Box::new(|cc| Ok(Box::new(Titra::new(TitraConfig{
        root_dir: ".".into(),
        storage_impl: storage::StorageImplementation::Sqlite
    }, ))))).unwrap();

}


