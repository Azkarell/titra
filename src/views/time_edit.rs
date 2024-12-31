
use chrono::NaiveTime;
use egui::{Align, Color32, Layout, RichText, TextEdit};
use log::info;

use crate::{ApplicationError, TitraView};



#[derive(Clone, Debug)]
pub struct TimeEdit {
    time: NaiveTime,
    repr: String,
    label: Option<String>,
    is_touched: bool,
    is_done: bool,
}

impl TimeEdit {
    pub fn new(label: Option<String>) -> Self {
        let time = NaiveTime::default();
        Self::new_with_value(time, label)
    }

    pub fn new_with_value(time: NaiveTime, label: Option<String>) -> Self {
        let repr = time.format("%R").to_string();
        Self { time, repr, label, is_touched: false, is_done: false}

    }


    pub fn get_value(&self) -> NaiveTime {
        self.time
    }

    pub fn validate(&self) -> Result<NaiveTime, ApplicationError> {
        let time = NaiveTime::parse_from_str(&self.repr, "%R")?;
        Ok(time)
    }


    pub fn is_touched(&self) -> bool {
        self.is_touched
    }
    pub fn is_done(&self) -> bool {
        self.is_done
    }

}





impl TitraView for TimeEdit {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if let Some(l) = &self.label {
           ui.label(l); 
        }
            let response = ui.add(TextEdit::singleline(&mut self.repr).desired_width(90.0).horizontal_align(Align::RIGHT));
            if response.changed() {
                self.is_touched = true;
            } else {
                self.is_touched = false;
            }
            self.is_done = !response.has_focus();
      
    }
}