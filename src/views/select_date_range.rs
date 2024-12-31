use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime};
use egui::{Color32, RichText};
use egui_extras::DatePickerButton;

use crate::{ApplicationError, TitraView};

use super::{overview::NaiveDateExt, overview_table::DateRange};


#[derive(Clone)]
pub struct SelectDateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl SelectDateRange {
    pub fn new() -> Self {
        let now = Local::now();
        let start_date = now.date_naive().with_day(1).expect("Failed to get first of month");
        let end_date = now.date_naive().with_day(now.date_naive().days_in_month()).expect("Failed to get end of month");
        Self {   start_date, end_date }
    }

    pub fn validate(&self) -> Result<DateRange, ApplicationError>{
        if !self.is_valid() {
            Err(ApplicationError::InvalidRange)
        } else {
            Ok((self.start_date, self.end_date))
        }
    }

    pub fn is_valid(&self) -> bool {
        self.start_date <= self.end_date
    }
}



impl TitraView for SelectDateRange {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            // Grid::new("overview_input_grid").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Start: ");
                let dpb = DatePickerButton::new(&mut self.start_date).id_salt("start");
                ui.add(dpb);

                ui.label("End: ");

                let dpb = DatePickerButton::new(&mut self.end_date).id_salt("end");
                ui.add(dpb);


                if let Err(err) = self.validate() {
                    ui.label(RichText::new(format!("Ungültiger Zeitraum: {:?}", err.to_string())).color(Color32::DARK_RED));
                }
            });
               
            // });
        });    }
}