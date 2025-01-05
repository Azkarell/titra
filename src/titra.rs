use std::{
    path::PathBuf,
    thread::{spawn, JoinHandle},
};

use eframe::App;
use egui::{Ui};
use serde::{Deserialize, Serialize};

use crate::{
    model::{date_range::DateRange, error::ApplicationError, time_entry::{TimeEntry, TimeEntryData, TimeEntryId}}, state::AppState, storage::{cache::CachedStorage, error::DataStorageError, null::NullService, sqlite::SqliteStorage, PlannedHoursStorage, StorageImplementation, TimeStorage}, views::{overview::Overview, scaffold::Scaffold}
};


#[derive(Clone)]
pub struct Services {
    pub time_service: Box<dyn TimeStorage + Send>,
    pub hour_service: Box<dyn PlannedHoursStorage + Send>,
}

impl Services {
    pub fn new(time_service: Box<dyn TimeStorage + Send>, hour_service: Box<dyn PlannedHoursStorage + Send>) -> Self {
        Self { time_service, hour_service }
    }
    
    pub(crate) fn empty() -> Self {
        Self {
            time_service: Box::new(NullService),
            hour_service: Box::new(NullService)
        }
    }
    
}


#[derive(Debug, PartialEq, Eq)]
pub enum TitraResult<T,E> {
    InEdit,
    Done(T),
    Error(E),
    NoChange
}

impl<T,E> TitraResult<T,E> {
    pub fn map_err<U, O: FnOnce(E) -> U>(self, f: O) -> TitraResult<T, U> {
        match self {
            TitraResult::InEdit => TitraResult::InEdit,
            TitraResult::Done(d) => TitraResult::Done(d),
            TitraResult::Error(e) => TitraResult::Error(f(e)),
            TitraResult::NoChange => TitraResult::NoChange,
        }
    }

    pub fn then<U, O: FnOnce(T) -> U>(self, f: O)-> TitraResult<U, E> {
        match self {
            TitraResult::InEdit => TitraResult::InEdit,
            TitraResult::Done(d) => TitraResult::Done(f(d)),
            TitraResult::Error(e) => TitraResult::Error(e),
            TitraResult::NoChange   => TitraResult::NoChange
        }
    }


    pub fn combine_with(self, other: TitraResult<T,E>) -> TitraResult<T,E> {
        match (self, other) {
            (TitraResult::InEdit, _) => TitraResult::InEdit,
            (_, TitraResult::InEdit) => TitraResult::InEdit,
            (TitraResult::Error(e), _) => TitraResult::Error(e),
            (_, TitraResult::Error(e)) => TitraResult::Error(e),
            (TitraResult::Done(d), _) => TitraResult::Done(d),
            (_, TitraResult::Done(d)) => TitraResult::Done(d),
            (TitraResult::NoChange, TitraResult::NoChange) => TitraResult::NoChange
        }
    }
}

pub trait TitraView<T, E, S> {
    fn show(&mut self, ui: &mut Ui, services: &mut S) -> TitraResult<T,E>;
}

pub trait StaticView {
    fn show(&mut self, ui: &mut Ui);
}

impl<S: StaticView, E> StateView<(), E> for S {
    fn show(&mut self, ui: &mut Ui) -> TitraResult<(),E> {
        self.show(ui);
        TitraResult::Done(())
    }
}



pub trait StateView<T, E>{
    fn show(&mut self, ui: &mut Ui) -> TitraResult<T,E>;
}

impl<V: StateView<T,E>, T, E, S> TitraView<T,E,S> for V {
    fn show(&mut self, ui: &mut Ui, _: &mut S) -> TitraResult<T,E> {
        self.show(ui)
    }
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
    init_thread: Option<JoinHandle<Result<Services, DataStorageError>>>,
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
                        init(config_clone)
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
                            self.state = AppState::Loaded(Overview::new(), res)
                        }
                        Err(err) => {
                            self.state = AppState::Failed(err.to_string())
                        },
                    };
                    
                } else {
                    scaffold.loading();
                }
               
            }
            AppState::Loaded(view, services) => scaffold.render(view, services),
            AppState::Failed(message) => scaffold.failed(message.clone()),
        }
    }
}

fn init(config: TitraConfig) -> Result<Services, DataStorageError>{
    match config.storage_impl {
        StorageImplementation::Sqlite => {
            let sqlite = SqliteStorage::new(config.root_dir.clone())?;
            let time_service: Box<dyn TimeStorage + Send> = Box::new(CachedStorage::new_time(sqlite.clone()));
            let hour_service: Box<dyn PlannedHoursStorage + Send> = Box::new(CachedStorage::new_hours(sqlite));
            Ok(Services::new(time_service, hour_service))
        }
    }

}
