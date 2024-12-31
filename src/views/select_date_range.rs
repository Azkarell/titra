use std::time::Instant;

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime};
use egui::{Color32, RichText, Widget};
use egui_extras::DatePickerButton;

use crate::{ApplicationError, TitraView};

use super::{overview::NaiveDateExt, overview_table::DateRange};

#[derive(Clone)]
pub struct SelectDateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    start_error: Option<Instant>
}

impl SelectDateRange {
    pub fn new() -> Self {
        let now = Local::now();
        let start_date = now
            .date_naive()
            .with_day(1)
            .expect("Failed to get first of month");
        let end_date = now
            .date_naive()
            .with_day(now.date_naive().days_in_month())
            .expect("Failed to get end of month");
        Self {
            start_date,
            end_date,
            start_error: None
        }
    }

    pub fn validate(&self) -> Result<DateRange, ApplicationError> {
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
            let mut show_tool_tip = true;
            let response = ui
                .horizontal(|ui| {
                    ui.label("Start: ");
                    let dpb = DatePickerButton::new(&mut self.start_date).id_salt("start");
                    let response = ui.add(dpb);
                    
                    ui.label("End: ");

                    let dpb = DatePickerButton::new(&mut self.end_date).id_salt("end");
                    let response = ui.add(dpb);
    
           
                })
                .response;
            if let Err(err) = self.validate() {
                if self.start_error.is_none() {
                    response.show_tooltip_text(
                        RichText::new(format!("Ungültiger Zeitraum: {:?}", err.to_string()))
                            .color(Color32::DARK_RED),
                    );
                    self.start_error = Some(Instant::now())
                } else {
                    let Some(val) = &mut self.start_error else {
                        return;
                    };
                    if val.elapsed() < std::time::Duration::from_secs(2) {
                        response.show_tooltip_text(
                            RichText::new(format!("Ungültiger Zeitraum: {:?}", err.to_string()))
                                .color(Color32::DARK_RED),
                        );
                    } else if val.elapsed() < std::time::Duration::from_secs(5){
                        // do nothing
                    } else {
                        self.start_error = None;
                    }
                }
            }
            // });
        });
    }
}
