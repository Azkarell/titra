use crate::{views::overview::Overview, Services};



pub enum AppState {
    Init,
    Loaded(Overview, Services),
    Failed(String)
}