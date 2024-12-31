
use chrono::{Local, NaiveDate, NaiveTime, ParseError};
use egui::Color32;
use egui_extras::DatePickerButton;
use log::warn;
use thiserror::Error;

use crate::{
    export::ExportError,
    storage::{DataStorageError, TimeEntryData, TimeStorage},
    TitraView,
};

pub struct AddEntry {
    date: NaiveDate,
    start: String,
    end: String,
    remark: String,
    storage: Box<dyn TimeStorage + Send>,
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Storage failure: {0}")]
    Storage(DataStorageError),
    #[error("Export failure: {0}")]
    Export(ExportError),
    #[error("Chrono error: {0}")]
    ChronoParseError(ParseError),
    #[error("Chrono timezone error: {0}")]
    ChronoeTimezoneError(String),
    #[error("Ungülitige start und endzeit")]
    InvalidRange,
}

impl From<ParseError> for ApplicationError {
    fn from(value: ParseError) -> Self {
        ApplicationError::ChronoParseError(value)
    }
}

impl AddEntry {
    pub fn new(storage: Box<dyn TimeStorage + Send>) -> Self {
        Self {
            date: Local::now().date_naive(),
            start: "00:00".to_owned(),
            end: "00:00".to_owned(),
            remark: "".to_owned(),
            storage,
        }
    }

    pub fn validate(&self) -> Result<TimeEntryData, ApplicationError> {
        let start = self
            .date
            .and_time(NaiveTime::parse_from_str(&self.start, "%R")?)
            .and_local_timezone(Local)
            .single();

        if start.is_none() {
            return Err(ApplicationError::ChronoeTimezoneError(
                "Startzeit konnte nicht gelesen werden".to_owned(),
            ));
        }

        let end = self
            .date
            .and_time(NaiveTime::parse_from_str(&self.end, "%R")?)
            .and_local_timezone(Local)
            .single();
        if end.is_none() {
            return Err(ApplicationError::ChronoeTimezoneError(
                "Endzeit konnte nicht gelesen werden".to_owned(),
            ));
        }

        let start = start.unwrap();
        let end = end.unwrap();
        if end <= start {
            return Err(ApplicationError::InvalidRange);
        }

        Ok(TimeEntryData {
            start,
            end,
            remark: if self.remark.len() > 0 {
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

                ui.label("Startzeit: ");
                ui.text_edit_singleline(&mut self.start);

                ui.label("Endzeit");
                ui.text_edit_singleline(&mut self.end);

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
