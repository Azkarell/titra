use std::collections::HashMap;

use chrono::NaiveDate;
use egui::{Color32, RichText};
use egui_extras::{Column, TableBuilder};
use log::{debug, info, warn};

use crate::{
    storage::{TimeEntryData, TimeEntryId, TimeStorage},
    ApplicationError, TitraView,
};

use super::{time_edit::TimeEdit, time_entry_edit::TimeEntryEdit};

pub type DateRange = (NaiveDate, NaiveDate);

pub struct OverviewTable {
    headers: Vec<String>,
    storage: Box<dyn TimeStorage + Send>,
    range: DateRange,
    edits: HashMap<TimeEntryId, TimeEntryEdit>,
}

impl OverviewTable {
    pub fn new(storage: Box<dyn TimeStorage + Send>, range: DateRange) -> Self {
        Self {
            headers: vec![
                "Tag".to_owned(),
                "Startzeit".to_owned(),
                "Endzeit".to_owned(),
                "Bemerkung".to_owned(),
                "Aktionen".to_owned(),
            ],
            storage,
            range,
            edits: HashMap::new(),
        }
    }

    pub fn set_range(&mut self, range: DateRange) {
        self.range = range;
    }
}

impl TitraView for OverviewTable {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_width(ui.available_width() - 250.0);
            ui.set_height(ui.available_height() - 250.0);

            let mut builder = TableBuilder::new(ui)
                .id_salt("overview_table")
                .auto_shrink(true);
            for _ in 0..self.headers.len() - 1 {
                builder =
                    builder.column(Column::auto_with_initial_suggestion(150.0).resizable(true));
            }
            builder
                .column(Column::remainder())
                .striped(true)
                .header(24.0, |mut header| {
                    for h in &self.headers {
                        header.col(|ui| {
                            ui.heading(h);
                        });
                    }
                })
                .body(|body| {
                    let data = self.storage.get_in_range(self.range);
                    if let Ok(entries) = data {
                        body.rows(20.0, entries.len(), |mut row| {
                            let (id, data) = &entries[row.index()];
                            let mut current_value: TimeEntryEdit = data.clone().into();
                            if let Some(edit) = self.edits.get(id) {
                                current_value = edit.clone();
                            }
                            let res = current_value.draw(&mut row, ctx, frame);
                            match res {
                                Ok(data) => {
                                    let _ = self.edits.remove_entry(id).unwrap();
                                    let res = self.storage.update_entry(*id, data);
                                    if let Err(err) = res {
                                        warn!("Failed to update entry: {err}")
                                    }
                                }
                                Err(err) => {
                                    self.edits.insert(*id, current_value);
                                    match err {
                                        ApplicationError::InEdit => {}
                                        _ => {
                                            row.response().show_tooltip_text(RichText::new(err.to_string()).color(Color32::DARK_RED));
                                        }
                                    }
                                }
                            }
                        });
                    } else {
                        info!("Failed to get data: {:?}", data.err().unwrap().to_string());
                    }
                });
        });
    }
}
