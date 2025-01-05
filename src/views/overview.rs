use log::debug;

use crate::{ extensions::naive_date_ext::NaiveDateExt, model::error::ApplicationError, user::UserData, Services, StateView, StaticView, TitraResult, TitraView};

use super::{
    add_entry::AddEntry, export::Export, overview_table::OverviewTable,
    select_date_range::SelectDateRange,
};


pub struct Overview {
    select_date_range: SelectDateRange,
    edit: AddEntry,
    overview_table: OverviewTable,
    export: Export,
}
impl Overview {
    pub fn new() -> Self {
        debug!("Init Overview");
        let select_date_range = SelectDateRange::new();
        Self {
            edit: AddEntry::new(),
            select_date_range: select_date_range.clone(),
            overview_table: OverviewTable::new( select_date_range.date),
            export: Export::new(
                select_date_range.get_range(),
                UserData::new(
                    "Daniel".to_owned(),
                    "Reichswaldstraße".to_owned(),
                    "91052".to_owned(),
                ),
            ),
        }
    }
}

impl TitraView<(), ApplicationError, Services> for Overview {
    fn show(&mut self, ui: &mut egui::Ui, services: &mut Services) -> TitraResult<(), ApplicationError> {

        let res = StateView::show(&mut self.select_date_range, ui);
        match res {
            crate::TitraResult::Done(d) => {
                self.overview_table.set_date(d);
                self.export.set_range(d.as_month_range());
            },
            _ => {}
        }
        ui.group(|ui|{
            ui.set_width(ui.available_width());
            self.overview_table.show(ui, services);
        });

        ui.horizontal(|ui| {

            match self.edit.show( ui, services) {
                TitraResult::Done(_) => {
                    self.overview_table.set_date(self.select_date_range.date);
                } 
                _ => {}
            }
            self.export.show( ui, services);
        });

        TitraResult::NoChange
    }
}
