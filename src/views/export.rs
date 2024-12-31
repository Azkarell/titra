use std::thread::{spawn, JoinHandle};

use chrono::{DateTime, Local};
use egui::{Button, ComboBox};

use crate::{
    export::{excel::XlsxExporter, Exporter},
    storage::{DataStorageError, TimeStorage},
    user::UserData,
    TitraView,
};

use super::overview_table::DateRange;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

impl ExportFormat {
    pub fn get_exporter(&self) -> Box<dyn Exporter + Send> {
        match self {
            ExportFormat::Csv => todo!(),
            ExportFormat::Xlsx => Box::new(XlsxExporter::new()),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            ExportFormat::Csv => "Csv".to_owned(),
            ExportFormat::Xlsx => "Xlsx".to_owned(),
        }
    }
}

pub struct Export {
    storage: Box<dyn TimeStorage + Send>,
    export_format: ExportFormat,
    range: DateRange,
    user_data: UserData,
    current_export: Option<JoinHandle<()>>,
}

impl Export {
    pub fn new(
        storage: Box<dyn TimeStorage + Send>,
        range: DateRange,
        user_data: UserData,
    ) -> Self {
        Self {
            storage,
            export_format: ExportFormat::Xlsx,
            range,
            user_data,
            current_export: None,
        }
    }

    pub fn set_range(&mut self, range: DateRange) {
        self.range = range;
    }

    pub fn export(&mut self) -> Result<(), DataStorageError> {
        let clone = self.storage.clone();
        let user_data = self.user_data.clone();
        let exporter = self.export_format.get_exporter();
        let range = (self.range.0, self.range.1);
        let handle = spawn(move || {
            let data = clone.get_in_range(range);
            if data.is_err() {
                return;
            }
            exporter.export(data.unwrap(), user_data.clone()).unwrap();
        });

        self.current_export = Some(handle);

        Ok(())
    }

    pub fn check_finished(&mut self) {
        if let Some(handle) = &mut self.current_export {
            if handle.is_finished() {
                self.current_export.take().unwrap().join().unwrap();
            }
        }
    }
}

impl TitraView for Export {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ComboBox::from_label("ExportFormat")
                    .selected_text(self.export_format.as_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.export_format, ExportFormat::Xlsx, "Excel");
                        ui.selectable_value(&mut self.export_format, ExportFormat::Csv, "Csv");
                    });

                let button = Button::new("Export");
                if self.current_export.is_some() {
                    ui.add_enabled(false, button);
                    self.check_finished();
                } else if ui.add(button).clicked() {
                    self.export().unwrap();
                }
            });
        });
    }
}
