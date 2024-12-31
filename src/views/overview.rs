
use chrono::Datelike;
use icu::datetime::input::DateInput;

use crate::{
    storage::TimeStorage, user::UserData, TitraView
};

use super::{add_entry::AddEntry, export::Export, overview_table::OverviewTable, select_date_range::SelectDateRange};

pub trait NaiveDateExt {
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
    select_date_range: SelectDateRange,
    edit: AddEntry,
    overview_table: OverviewTable,
    export: Export
}
impl Overview {
    pub fn new(storage: Box<dyn TimeStorage + Send>) -> Self {

        let cloned = storage.clone();
        let select_date_range = SelectDateRange::new();
        Self {
            edit: AddEntry::new(cloned),
            select_date_range: select_date_range.clone(),
            overview_table: OverviewTable::new(storage.clone(), (select_date_range.start_date, select_date_range.end_date)),
            export: Export::new(storage, 
                (select_date_range.start_date, select_date_range.end_date),
                 UserData::new("Daniel".to_owned(), "Reichswaldstraße".to_owned(), "91052".to_owned()))
        }
    }
}

impl TitraView for Overview {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
            self.select_date_range.show(ctx, frame, ui);
            self.overview_table.set_range((self.select_date_range.start_date, self.select_date_range.end_date));
            self.overview_table.show(ctx, frame, ui);

            ui.horizontal(|ui| {
                self.edit.show(ctx, frame, ui);
                self.export.set_range((self.select_date_range.start_date, self.select_date_range.end_date));
                self.export.show(ctx, frame, ui);
            });
            
           
    }
}

