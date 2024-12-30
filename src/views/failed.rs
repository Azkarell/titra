use crate::TitraView;

pub struct Failed {
    message: String
}

impl Failed{
    pub fn new(message: String) -> Self{
        Self {
            message
        }
    }
}

impl TitraView for Failed {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui|{
            ui.heading(&self.message)
        });
    }
}