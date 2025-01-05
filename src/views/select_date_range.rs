use std::time::Instant;

use chrono::{ Datelike, Local, NaiveDate};
use egui::ComboBox;
use log::info;

use crate::{extensions::naive_date_ext::NaiveDateExt, model::{date_range::DateRange, error::ApplicationError}, Services, StateView, TitraResult, TitraView};


#[derive(Clone)]
pub struct SelectDateRange {
    pub date: NaiveDate,
    state: SelectDateRangeState,
}

impl SelectDateRange {
    pub fn new() -> Self {
        info!("creating range");
        let now = Local::now();

        Self {
            date: now.date_naive(),
            state: SelectDateRangeState {
                picker: YearMonthPicker::new(now.date_naive(), "year_month_picker".to_owned()),
            },
        }
    }


    pub fn get_range(&self) -> DateRange {
        (self.date.with_day(1).unwrap(), self.date.with_day(self.date.days_in_month()).unwrap())
    }
    

}

fn get_month_name(month: u32) -> Option<&'static str> {
    match month {
        1 => Some("Januar"),
        2 => Some("Februar"),
        3 => Some("März"),
        4 => Some("April"),
        5 => Some("Mai"),
        6 => Some("Juni"),
        7 => Some("Juli"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("Oktober"),
        11 => Some("November"),
        12 => Some("Dezember"),
        _ => None,
    }
}

#[derive(Clone)]
struct SelectDateRangeState {
    picker: YearMonthPicker,
}

#[derive(Clone)]
struct MonthPicker {
    pub salt: String,
    pub current_month: u32,
}

impl MonthPicker {
    fn new(current_month: u32, salt: String) -> Self {
        Self {
            current_month,
            salt,
        }
    }
}

impl StateView<u32, ApplicationError> for MonthPicker {
    fn show(&mut self, ui: &mut egui::Ui) -> TitraResult<u32, ApplicationError> {

        let response = ComboBox::from_id_salt(&self.salt)
            .selected_text(get_month_name(self.current_month).unwrap())
            .show_ui(ui, |ui| {
                let before = self.current_month;
                for i in 1..=12 {
                    ui.selectable_value(&mut self.current_month, i, get_month_name(i).unwrap());
                }
                (self.current_month, before != self.current_month)
            });
            if let Some((v, changed)) = response.inner {
                if changed {
                    TitraResult::Done(v)
                } else {
                    TitraResult::NoChange
                }
            } else {
                TitraResult::NoChange
            }

      
    }
}

#[derive(Clone)]
struct YearPicker {
    pub year_string: String,
    pub year: i32,
}

impl YearPicker {
    fn new(year: i32) -> Self {
        Self {
            year_string: year.to_string(),
            year,
        }
    }
}

impl StateView<i32, ApplicationError> for YearPicker {
    fn show(&mut self, ui: &mut egui::Ui) -> TitraResult<i32, ApplicationError>{
        if ui.text_edit_singleline(&mut self.year_string).lost_focus() {
            if let Ok(y) = self.year_string.parse::<i32>() {
                self.year = y;
                TitraResult::Done(y)
            } else {
                TitraResult::InEdit
            }
        } else {
            TitraResult::NoChange
        }
    }
}

#[derive(Clone)]
struct YearMonthPicker {
    pub month: MonthPicker,
    pub year: YearPicker,
    pub date: NaiveDate,

}

impl YearMonthPicker {
    fn new(date: NaiveDate, salt: String) -> Self {
        Self {
            month: MonthPicker::new(date.month(), salt),
            year: YearPicker::new(date.year()),
            date,
        }
    }
}

impl StateView<NaiveDate, ApplicationError> for YearMonthPicker {
    fn show(&mut self, ui: &mut egui::Ui) -> TitraResult<NaiveDate, ApplicationError> {
        let month = StateView::show(&mut self.month, ui);
        let year = StateView::show(&mut self.year, ui);
        
        match (month, year) {
            (TitraResult::InEdit, _) => TitraResult::InEdit,
            (_, TitraResult::InEdit) => TitraResult::InEdit,
            (TitraResult::Error(e), _) => TitraResult::Error(e),
            (_, TitraResult::Error(e)) => TitraResult::Error(e),
            (TitraResult::Done(d1), TitraResult::Done(d2)) => {
                self.date = self.date.with_month(d1).unwrap().with_year(d2).unwrap();
                TitraResult::Done(self.date)
            }
            (TitraResult::NoChange, TitraResult::NoChange) => TitraResult::NoChange,
            (TitraResult::NoChange, TitraResult::Done(y)) => {
                self.date = self.date.with_year(y).unwrap();
                TitraResult::Done(self.date)
            },
            (TitraResult::Done(m), TitraResult::NoChange) => {
                self.date = self.date.with_month(m).unwrap();
                TitraResult::Done(self.date)
            }
        }
    }
}

impl StateView<NaiveDate, ApplicationError> for SelectDateRange {
    fn show(&mut self, ui: &mut egui::Ui) -> TitraResult<NaiveDate, ApplicationError>{
        let res = ui.group(|ui| {
            // Grid::new("overview_input_grid").show(ui, |ui| {
            let response = ui
                .horizontal(|ui| {

                    let res = StateView::show(&mut self.state.picker, ui);
                    res
                   
                });
            response.inner
            // });
        });
        res.inner
    }
}
