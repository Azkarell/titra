use crate::{model::error::ApplicationError, Services, StaticView, TitraView};

use super::{failed::Failed, loading::Loading};




pub struct Scaffold<'a, 'b> {
    ctx: &'a egui::Context,
    frame: &'b mut eframe::Frame
}


impl<'a,'b> Scaffold<'a,'b> {
    pub fn render(&mut self, view: &mut impl TitraView<(),ApplicationError, Services>, services: &mut Services){
        
        
        egui::CentralPanel::default().show(self.ctx, |ui| {
            ui.set_width(ui.available_width());
            let _ = view.show(ui, services);
            
        });

    

    } 

    pub fn loading(&mut self) {
        self.render(&mut Loading::new(), &mut Services::empty());
    }
    
    pub fn new(ctx: &'a egui::Context, frame: &'b mut eframe::Frame) -> Self {
        Self {
            ctx,
            frame
        }
    }
    
    pub fn failed(&mut self, msg: String) {
        self.render(&mut Failed::new(msg), &mut Services::empty());
    }
}

