use chrono::NaiveDate;
use egui_extras::TableRow;

use crate::{storage::TimeEntryData, ApplicationError, TitraView};

use super::time_edit::TimeEdit;

#[derive(Clone, Debug)]
pub struct TimeEntryEdit {
    start: TimeEdit,
    end: TimeEdit,
    date: NaiveDate,
    remark: String,
    is_touched: bool,
    is_done: bool
}

impl TimeEntryEdit {
    pub fn validate(&mut self) -> Result<TimeEntryData, ApplicationError> {
        
        if self.start.is_touched() {
            self.is_touched = true;
        }
        if self.end.is_touched() {
            self.is_touched = true;
        }

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
    
    fn mark_touched(&mut self) {
        self.is_touched = true;
    }
    pub fn is_touched(&self) -> bool {
        self.is_touched
    }

    pub fn draw(&mut self, row: &mut TableRow, ctx: &egui::Context, frame: &mut eframe::Frame) -> Result<TimeEntryData, ApplicationError> {
        self.is_done = true;
        row.col(|ui| {
            ui.label(self.date.format("%x").to_string());
        });
        row.col(|ui| {
            ui.set_max_width(90.0);
            self.start.show(ctx, frame, ui);
            if !self.start.is_done() {
                self.is_done = false;
            }
        });
        row.col(|ui| {
            ui.set_max_width(90.0);
            self.end.show(ctx, frame, ui);
            if !self.end.is_done() {
                self.is_done = false;
            }
        });
        row.col(|ui| {
            let response = ui.text_edit_singleline(&mut self.remark);
            if response.changed() {
                self.mark_touched();
            }
            if response.has_focus() {
                self.is_done = false;
            }
        });
        let res = self.validate();
        if self.is_done && self.is_touched {
            return res;
        } else {
            return Err(ApplicationError::InEdit);
        }
      

    }
}

impl From<TimeEntryData> for TimeEntryEdit {
    fn from(value: TimeEntryData) -> Self {
        Self {
            date: value.date,
            end: TimeEdit::new_with_value(value.end, None),
            remark: value.remark,
            start: TimeEdit::new_with_value(value.start, None),
            is_touched: false,
            is_done: true
        }
    }
}

