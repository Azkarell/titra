
use crate::{Services, StaticView, TitraView};

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

impl StaticView for Loading {
    fn show(&mut self, ui: &mut egui::Ui) {
        if !self.ctx.is_done {
            ui.horizontal_centered(|ui|{
                ui.spinner();
            });
        }
    }
}
