use crate::{state::AppState, TitraView};

use super::{failed::Failed, loading::Loading};




pub struct Scaffold<'a, 'b> {
    ctx: &'a egui::Context,
    frame: &'b mut eframe::Frame
}


impl<'a,'b> Scaffold<'a,'b> {
    pub fn render(&mut self, view: &mut impl TitraView){
        
        egui::CentralPanel::default().show(self.ctx, |ui| {
            
            view.show(self.ctx, self.frame, ui);
            
        });

    } 

    pub fn loading(&mut self,) {
        self.render(&mut Loading::new());
    }
    
    pub fn new(ctx: &'a egui::Context, frame: &'b mut eframe::Frame) -> Self {
        Self {
            ctx,
            frame
        }
    }
    
    pub fn failed(&mut self, msg: String) {
        self.render(&mut Failed::new(msg));
    }
}

