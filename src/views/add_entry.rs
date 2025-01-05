

use chrono::{Local, NaiveDate, NaiveTime};
use eframe::App;
use egui::{Button, Color32, Grid, InnerResponse, Response, RichText, TextEdit, Widget};
use egui_extras::DatePickerButton;
use log::{info, warn};

use crate::{
    model::{error::ApplicationError, time_entry::{TimeEntry, TimeEntryData}}, Services, StateView, TitraResult, TitraView
};

use super::time_edit::TimeEdit;

pub struct AddEntry {
    date: NaiveDate,
    start: TimeEdit,
    end: TimeEdit,
    remark: String,
}



impl AddEntry {
    pub fn new() -> Self {
        Self {
            date: Local::now().date_naive(),
            start: TimeEdit::new(Some("Startzeit".to_owned())),
            end: TimeEdit::new_with_value(NaiveTime::from_hms_opt(17, 0, 0).unwrap(),Some("Endzeit".to_owned())),
            remark: "".to_owned(),
        }
    }

    pub fn validate(&self) -> Result<TimeEntryData, ApplicationError> {

        let start = self.start.validate()?;
        let end = self.end.validate()?;
         
        if end <= start {
            return Err(ApplicationError::InvalidRange);
        }

        Ok(TimeEntryData {
            start,
            end,
            date: self.date.clone(),
            remark: self.remark.clone()
        })
    }

    pub fn get_result(&self) -> TitraResult<TimeEntryData, ApplicationError> {
        let validated = self.validate();
        match validated {
            Ok(d) => TitraResult::Done(d),
            Err(e) => TitraResult::Error(e),
        }
    }
    
    fn get_unchanged(&self) -> TimeEntryData {
        self.validate().unwrap()
    }
}

impl TitraView<(), ApplicationError, Services> for AddEntry {
    fn show(&mut self, ui: &mut egui::Ui, services: &mut Services) -> TitraResult<(), ApplicationError> {
        let mut final_res = TitraResult::NoChange;
        ui.group(|ui| {
            ui.centered_and_justified(|ui|{
                Grid::new("new_grid").spacing((30.0, 2.0)).show(ui, |ui| {
                    let mut fn_res = TitraResult::NoChange;
                    ui.label("Tag");
                    let dpb = DatePickerButton::new(&mut self.date).id_salt("add_date");
                    ui.add(dpb);
                    ui.end_row();
                    let mut res = StateView::show(&mut self.start, ui);
                    ui.end_row();
                     res = res.combine_with(StateView::show(&mut self.end, ui));
                    ui.end_row();
                    ui.label("Bemerkung");
                    let res = ui.add(TextEdit::singleline(&mut self.remark).desired_width(240.0));
                    ui.end_row();

                    if res.changed() && res.has_focus() {
                        fn_res = TitraResult::InEdit
                    } else if res.changed() {
                        fn_res = self.get_result();
                    } else {
                        fn_res = TitraResult::NoChange;
                    }

                    let button = Button::new("+");

                    match fn_res {
                        TitraResult::Done(entry) => {
                            let response = ui.add(button);
                            if response.clicked() {
                                let res = services.time_service.add_entry(entry);
                                if let Err(err) = res {
                                    warn!("Failed to store entry: {:?}", err.to_string());
                                }
                                final_res = TitraResult::Done(());
                            }
                        }
                        TitraResult::Error(err) => {
                            ui.add_enabled(false, button).show_tooltip_text(RichText::new(err.to_string()).color(Color32::DARK_RED));
                        }
                        TitraResult::InEdit =>{ 
                            ui.add_enabled(false, button);
                        },
                        TitraResult::NoChange => {  let response = ui.add(button);
                            if response.clicked() {
                                let res = services.time_service.add_entry(self.get_unchanged());
                                if let Err(err) = res {
                                    warn!("Failed to store entry: {:?}", err.to_string());
                                }
                                final_res = TitraResult::Done(());
                            }
                        }
                    }
                    ui.end_row();
                });
            });


        });
        final_res
    }
}
