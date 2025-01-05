use chrono::NaiveTime;
use egui::{Align, Color32, Id, Layout, RichText, TextEdit};
use log::info;

use crate::{ model::error::ApplicationError, Services, StateView, TitraResult, TitraView};

#[derive(Clone, Debug)]
pub struct TimeEdit {
    time: NaiveTime,
    repr: String,
    label: Option<String>,
}





impl TimeEdit {
    pub fn new(label: Option<String>) -> Self {
        let time = NaiveTime::default();
        Self::new_with_value(time, label)
    }

    pub fn new_with_value(time: NaiveTime, label: Option<String>) -> Self {
        let repr = time.format("%R").to_string();
        Self {
            time,
            repr,
            label,
        }
    }

    pub fn get_value(&self) -> NaiveTime {
        self.time
    }

    pub fn validate(&self) -> Result<NaiveTime, ApplicationError> {
        let time = NaiveTime::parse_from_str(&self.repr, "%R")?;
        Ok(time)
    }


}

impl StateView<NaiveTime, ApplicationError> for TimeEdit {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> TitraResult<NaiveTime, ApplicationError> {
            if let Some(l) = &self.label {
                ui.label(l);
            }
           
            let response = ui.add(
                TextEdit::singleline(&mut self.repr)
                    .desired_width(120.0)
                    .horizontal_align(Align::RIGHT),
            );
            if response.has_focus() {
                info!("here: {}", self.repr);
                TitraResult::InEdit
            } else if response.lost_focus() {
                match self.validate() {
                    Ok(v) => {
                        self.time = v;
                        info!("done");
                        TitraResult::Done(v)
                    },
                    Err(e) => TitraResult::Error(e),
                }
            } else {
                TitraResult::NoChange
            }
    }
}
