use chrono::NaiveDate;
use egui::TextEdit;
use egui_extras::TableRow;
use log::info;

use crate::{
    model::{error::ApplicationError, time_entry::TimeEntryData},
    Services, StateView, TitraResult, TitraView,
};

use super::time_edit::TimeEdit;

#[derive(Clone, Debug)]
pub struct TimeEntryEdit {
    start: TimeEdit,
    end: TimeEdit,
    date: NaiveDate,
    remark: String,
}

impl TimeEntryEdit {
    pub fn validate(&mut self) -> Result<TimeEntryData, ApplicationError> {
        let start = self.start.validate()?;
        let end = self.end.validate()?;
        if start >= end {
            return Err(ApplicationError::InvalidRange);
        }
        Ok(TimeEntryData {
            date: self.date.clone(),
            end,
            start,
            remark: self.remark.clone(),
        })
    }
}

impl From<TimeEntryData> for TimeEntryEdit {
    fn from(value: TimeEntryData) -> Self {
        Self {
            date: value.date,
            end: TimeEdit::new_with_value(value.end, None),
            remark: value.remark,
            start: TimeEdit::new_with_value(value.start, None),
        }
    }
}

impl StateView<TimeEntryData, ApplicationError> for TimeEntryEdit {
    fn show(&mut self, ui: &mut egui::Ui) -> TitraResult<TimeEntryData, ApplicationError> {
        let change1 = match StateView::show(&mut self.start, ui) {
            TitraResult::InEdit => TitraResult::InEdit,
            TitraResult::Done(_) => match self.validate() {
                Ok(d) => TitraResult::Done(d),
                Err(e) => TitraResult::Error(e),
            },
            TitraResult::Error(e) => TitraResult::Error(e),
            TitraResult::NoChange => TitraResult::NoChange,
        };

        let change2 = match StateView::show(&mut self.end, ui) {
            TitraResult::InEdit => TitraResult::InEdit,
            TitraResult::Done(_) => match self.validate() {
                Ok(d) => TitraResult::Done(d),
                Err(e) => TitraResult::Error(e),
            },
            TitraResult::Error(e) => TitraResult::Error(e),
            TitraResult::NoChange => TitraResult::NoChange,
        };

        let response = ui.add_sized( (240.0, 30.0) ,TextEdit::singleline(&mut self.remark).desired_width(200.0));
        let change3 = if response.has_focus() {
            TitraResult::InEdit
        } else if response.lost_focus() {
            match self.validate() {
                Ok(d) => TitraResult::Done(d),
                Err(e) => TitraResult::Error(e),
            }
        } else {
            TitraResult::NoChange
        };
        let res = change1.combine_with(change2).combine_with(change3);

        res
    }
}
