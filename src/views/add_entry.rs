
use core::f32;

use chrono::{Local, NaiveDate, NaiveTime, ParseError};
use egui::{Button, Color32, Grid, RichText, TextEdit};
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
            start: TimeEdit::new(Some("Startzeit".to_owned())),
            end: TimeEdit::new_with_value(NaiveTime::from_hms_opt(17, 0, 0).unwrap(),Some("Endzeit".to_owned())),
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
            remark: self.remark.clone()
        })
    }
}

impl TitraView for AddEntry {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.centered_and_justified(|ui|{
                Grid::new("new_grid").spacing((30.0, 2.0)).show(ui, |ui| {
                    ui.label("Tag");
                    let dpb = DatePickerButton::new(&mut self.date).id_salt("add_date");
                    ui.add(dpb);
                    ui.end_row();
                    self.start.show(ctx, frame, ui);
                    ui.end_row();
                    self.end.show(ctx, frame, ui);
                    ui.end_row();
                    ui.label("Bemerkung");
                    ui.add(TextEdit::singleline(&mut self.remark).desired_width(240.0));
                    ui.end_row();
                    let validated = self.validate();
                    let button = Button::new("+");

                    match validated {
                        Ok(entry) => {
                            let response = ui.add(button);
                            if response.clicked() {
                                let res = self.storage.add_entry(entry);
                                if let Err(err) = res {
                                    warn!("Failed to store entry: {:?}", err.to_string());
                                }
                            }
                        }
                        Err(message) => {
                            ui.add_enabled(false, button).show_tooltip_text(RichText::new(message.to_string()).color(Color32::DARK_RED));
                        }
                    }
                    ui.end_row();
                });
            });


        });
    }
}
