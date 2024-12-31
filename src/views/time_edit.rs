use std::cell::RefCell;

use chrono::NaiveTime;
use eframe::App;
use egui::{Color32, RichText};

use crate::{ApplicationError, TitraView};




pub struct TimeEdit {
    time: NaiveTime,
    repr: String,
    label: String,
}

impl TimeEdit {
    pub fn new(label: String) -> Self {
        let time = NaiveTime::default();
        Self::new_with_value(time, label)
    }

    pub fn new_with_value(time: NaiveTime, label: String) -> Self {
        let repr = time.format("%R").to_string();
        Self { time, repr, label}

    }

    pub fn get_value(&self) -> NaiveTime {
        self.time
    }

    pub fn validate(&self) -> Result<NaiveTime, ApplicationError> {
        let time = NaiveTime::parse_from_str(&self.repr, "%R")?;
        Ok(time)
    }


}





impl TitraView for TimeEdit {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.label(&self.label);
        ui.text_edit_singleline(&mut self.repr);
        if let Err(err) = self.validate() {
            let label = egui::Label::new(RichText::new(format!("Eingabe ungültig: {}", err)).color(Color32::DARK_RED));
            ui.add(label);
        }
    }
}