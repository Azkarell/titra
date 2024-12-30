use crate::views::overview::{self, Overview};



pub enum AppState {
    Init,
    Overview(Overview),
    Failed(String)
}