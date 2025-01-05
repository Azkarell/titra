use crate::{Services, StaticView, TitraView};

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

impl StaticView for Failed {
    fn show(&mut self,  ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui|{
            ui.heading(&self.message)
        });
    }
}