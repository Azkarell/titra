use crate::views::overview::{Overview};



pub enum AppState {
    Init,
    Overview(Overview),
    Failed(String)
}