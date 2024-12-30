use std::{
    path::PathBuf,
    thread::{spawn, JoinHandle},
};

use eframe::App;
use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::{
    state::AppState,
    storage::{sqlite::SqliteStorage, DataStorageError, StorageImplementation, TimeStorage},
    views::{overview::Overview, scaffold::Scaffold},
};

pub trait TitraView {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut Ui);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TitraConfig {
    pub root_dir: PathBuf,
    pub storage_impl: StorageImplementation,
}

pub struct Titra {
    config: TitraConfig,
    state: AppState,
    threads: Vec<JoinHandle<()>>,
    init_thread: Option<JoinHandle<Result<Box<dyn TimeStorage + Send>, DataStorageError>>>,
}

impl Titra {
    pub fn new(config: TitraConfig) -> Self {
        Self {
            config,
            state: AppState::Init,
            threads: Vec::new(),
            init_thread: None,
        }
    }
}

impl App for Titra {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let mut scaffold = Scaffold::new(ctx, frame);

        match &mut self.state {
            AppState::Init => {
                let Some(init_thread) = &mut self.init_thread else {
                    let config_clone = self.config.clone();
                    self.init_thread = Some(spawn(move || {
                        return init(config_clone);
                    }));
                    return;
                };

                if init_thread.is_finished() {
                    let taken = self.init_thread.take().unwrap();
                    let Ok(join_res) = taken.join() else {
                        self.state = AppState::Failed("Error joining init thread".to_owned());
                        return;
                    };
                    match join_res {
                        Ok(res) => {
                            self.state = AppState::Overview(Overview::new(res))
                        }
                        Err(err) => {
                            self.state = AppState::Failed(err.to_string())
                        },
                    };
                    
                } else {
                    scaffold.loading();
                }
               
            }
            AppState::Overview(view) => scaffold.render(view),
            AppState::Failed(message) => scaffold.failed(message.clone()),
        }
    }
}

fn init(config: TitraConfig) -> Result<Box<dyn TimeStorage + Send>, DataStorageError>{
    match config.storage_impl {
        StorageImplementation::Sqlite => {
            
            match SqliteStorage::new(config.root_dir.clone()) {
                Ok(res) => return Ok(Box::new(res)),
                Err(err) => return Err(err),
            }
        }
    }

}
