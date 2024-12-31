use chrono::{DateTime, Local};
use egui_extras::{Column, TableBuilder};
use log::{info, warn};

use crate::{storage::TimeStorage, TitraView};



pub struct OverviewTable {
    headers: Vec<String>,
    storage: Box<dyn TimeStorage + Send>,
    range: (DateTime<Local>, DateTime<Local>)
}

impl OverviewTable {
    pub fn new(storage: Box<dyn TimeStorage + Send>, range: (DateTime<Local>, DateTime<Local>) ) -> Self {
        Self {
            headers: vec!["Tag".to_owned(), "Startzeit".to_owned(), "Endzeit".to_owned(), "Bemerkung".to_owned(), "Aktionen".to_owned()],
            storage,
            range,
        }
    }

    pub fn set_range(&mut self, range: (DateTime<Local>, DateTime<Local>)) {
        self.range = range;
    }
}


impl TitraView for OverviewTable {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height()-250.0);
      
            let mut builder = TableBuilder::new(ui)
                .id_salt("overview_table")
                .auto_shrink(true);
            for _ in 0..self.headers.len() -1 {
                builder = builder.column(Column::auto_with_initial_suggestion(300.0).resizable(true));
            }
            builder.column(Column::remainder())
                .header(40.0, |mut header| {
                    for h in &self.headers {
                        header.col(|ui| {
                            ui.heading(h);
                        });
                    }
                  
                })
                .body(|body| {
                    let data = self.storage.get_in_range(
                        self.range.0,
                        self.range.1,
                    );
                    if let Ok(entries) = data {
                        body.rows(32.0, entries.len(), |mut row| {
                            let (id, data) = &entries[row.index()];
                          
                            row.col(|ui| {
                                ui.label(data.start.date_naive().format("%x").to_string());
                            });
                            row.col(|ui| {
                                ui.label(data.start.time().format("%R").to_string());
                            });
                            row.col(|ui| {
                                ui.label(data.end.time().format("%R").to_string());
                            });
                            row.col(|ui| {
                                ui.label(data.remark.as_ref().map_or("", |v| v));
                            });
                            row.col(|ui|{
                                if ui.button("x").clicked() {
                                    let res = self.storage.remove_entry(*id);
                                    if res.is_err() {
                                        warn!("Could not delete entry: {} - {}", *id, res.err().unwrap().to_string())
                                    }
                                }
                            });
                        });
                    } else {
                        info!("Failed to get data: {:?}", data.err().unwrap().to_string());
                    }
                });
        });
    }
}