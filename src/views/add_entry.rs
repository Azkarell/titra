
use chrono::{Local, NaiveDate, NaiveTime, ParseError};
use egui::Color32;
use egui_extras::DatePickerButton;
use log::warn;
use thiserror::Error;

use crate::{
    export::ExportError, storage::{DataStorageError, TimeEntryData, TimeStorage}, ApplicationError, TitraView
};

use super::time_edit::TimeEdit;

pub struct AddEntry {
    date: NaiveDate,
    start: TimeEdit,
    end: TimeEdit,
    remark: String,
    storage: Box<dyn TimeStorage + Send>,
}





impl AddEntry {
    pub fn new(storage: Box<dyn TimeStorage + Send>) -> Self {
        Self {
            date: Local::now().date_naive(),
            start: TimeEdit::new("Start".to_owned()),
            end: TimeEdit::new("Ende".to_owned()),
            remark: "".to_owned(),
            storage,
        }
    }

    pub fn validate(&self) -> Result<TimeEntryData, ApplicationError> {

        let start = self.start.validate()?;
        let end = self.end.validate()?;
         
        if end <= start {
            return Err(ApplicationError::InvalidRange);
        }

        Ok(TimeEntryData {
            start,
            end,
            date: self.date.clone(),
            remark: if !self.remark.is_empty() {
                Some(self.remark.clone())
            } else {
                None
            },
        })
    }
}

impl TitraView for AddEntry {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Tag");
                let dpb = DatePickerButton::new(&mut self.date).id_salt("add_date");
                ui.add(dpb);

                self.start.show(ctx, frame, ui);
                self.end.show(ctx, frame, ui);

                ui.label("Bemerkung");
                ui.text_edit_singleline(&mut self.remark);

                let validated = self.validate();
                match validated {
                    Ok(entry) => {
                        if ui.button("Add").clicked() {
                            let res = self.storage.add_entry(entry);

                            if let Err(err) = res {
                                warn!("Failed to store entry: {:?}", err.to_string());
                            }
                        }
                    }
                    Err(message) => {
                        ui.colored_label(Color32::DARK_RED, message.to_string());
                    }
                }
            });
        });
    }
}
