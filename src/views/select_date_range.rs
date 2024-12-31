use chrono::{DateTime, Datelike, Local, NaiveTime};
use egui_extras::DatePickerButton;

use crate::TitraView;

use super::overview::NaiveDateExt;


#[derive(Clone)]
pub struct SelectDateRange {
    pub start_date: DateTime<Local>,
    pub end_date: DateTime<Local>,
}

impl SelectDateRange {
    pub fn new() -> Self {
        let now = Local::now();
        Self {   end_date: now
            .with_day(now.date_naive().days_in_month())
            .expect("Could not get end day of month"),
        start_date: now.with_day(1).expect("Failed to get start date"), }
    }
}



impl TitraView for SelectDateRange {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            // Grid::new("overview_input_grid").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Start: ");
                let mut naiveDate = self.start_date.date_naive();
                let dpb = DatePickerButton::new(&mut naiveDate).id_salt("start");
                ui.add(dpb);
                self.start_date = naiveDate
                    .and_time(NaiveTime::default())
                    .and_local_timezone(Local)
                    .earliest()
                    .unwrap();

                ui.end_row();
                ui.label("End: ");

                let mut naiveDate = self.end_date.date_naive();
                let dpb = DatePickerButton::new(&mut naiveDate).id_salt("end");
                ui.add(dpb);
                self.end_date = naiveDate
                    .and_time(NaiveTime::from_hms_opt(23,59, 59).unwrap())
                    .and_local_timezone(Local)
                    .earliest()
                    .unwrap();

                ui.end_row();
            });
               
            // });
        });    }
}