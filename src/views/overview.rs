use chrono::{Datelike, Local};
use egui::RichText;

use crate::{storage::TimeStorage, TitraView};

trait NaiveDateExt {
    fn days_in_month(&self) -> u32;
    fn days_in_year(&self) -> u32;
    fn is_leap_year(&self) -> bool;
}

impl NaiveDateExt for chrono::NaiveDate {
    fn days_in_month(&self) -> u32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn days_in_year(&self) -> u32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }
}

pub struct Overview {
    storage: Box<dyn TimeStorage + Send>,
    start_date: chrono::DateTime<Local>,
    end_date: chrono::DateTime<Local>,
}
impl Overview {
    pub fn new(storage: Box<dyn TimeStorage + Send>) -> Self {
        let now = Local::now();

        Self {
            end_date: now
                .with_day(now.date_naive().days_in_month())
                .expect("Could not get end day of month"),
            start_date: now.with_day(1).expect("Failed to get start date"),
            storage,
        }
    }
}

impl TitraView for Overview {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Start: ");
                ui.label(RichText::new(format!("{}", self.start_date.format("%"))));
                ui.label("End: ");
                ui.label(RichText::new(format!("{}", self.end_date.to_rfc3339())));
            });
        });
    }
}
