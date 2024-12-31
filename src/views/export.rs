use std::thread::JoinHandle;

use chrono::{DateTime, Local};
use egui::{Button, ComboBox};

use crate::{export::{excel::XlsxExporter, Exporter}, storage::{DataStorageError, TimeStorage}, user::UserData, TitraView};


#[derive(Debug, PartialEq, PartialOrd)]
pub enum ExportFormat {
    Csv,
    Xlsx
}

impl ExportFormat {
    pub fn get_exporter(&self) -> Box<dyn Exporter> {
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
    range: (DateTime<Local>, DateTime<Local>),
    user_data: UserData,
    current_export: Option<JoinHandle<()>>
}

impl Export {
    pub fn new(storage: Box<dyn TimeStorage + Send>, range: (DateTime<Local>, DateTime<Local>), user_data: UserData) -> Self {
        Self {
            storage,
            export_format: ExportFormat::Xlsx,
            range,
            user_data,
            current_export: None
        }
    }

    pub fn set_range(&mut self, range: (DateTime<Local>, DateTime<Local>)){
        self.range = range;
    }

    pub fn set_export_format(&mut self, format: ExportFormat) {
        self.export_format = format;
    }

    pub fn export(&self) -> Result<(), DataStorageError> {
        spawn(||{
            let data = self.storage.get_in_range(self.range.0, self.range.1)?;
            let exporter = self.export_format.get_exporter();
            exporter.export(data, self.user_data.clone()).unwrap();
        })
       
        Ok(())
    }

}


impl TitraView for Export {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.group(|ui|{
            ui.vertical(|ui|{
                ComboBox::from_label("ExportFormat")
                .selected_text(self.export_format.as_string())
                .show_ui(ui, |ui|{
                    ui.selectable_value(&mut self.export_format, ExportFormat::Xlsx, "Excel");
                    ui.selectable_value(&mut self.export_format, ExportFormat::Csv, "Csv");
                });
                
                let button = Button::new("Export");
                if self.current_export.is_some() {
                    ui.add_enabled(false, button);
                } else if ui.add(button).clicked(){
                    self.export().unwrap();
                }
            });
       
        });
    }
}