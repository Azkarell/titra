
use crate::TitraView;

pub struct Loading {
    ctx: LoadingContext,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            ctx: LoadingContext { is_done: false },
        }
    }
}

pub struct LoadingContext {
    is_done: bool,
}

impl TitraView for Loading {
    fn show(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if !self.ctx.is_done {
            ui.horizontal_centered(|ui|{
                ui.spinner();
            });
        }
    }
}
