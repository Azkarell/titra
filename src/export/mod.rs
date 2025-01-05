use thiserror::Error;

use crate::{model::time_entry::TimeEntry, user::UserData};

pub mod excel;


#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExportError {
    #[error("Could not export data: {0}")]
    Unknown(String)
}



pub trait Exporter {
    fn export(&self, data: Vec<TimeEntry>, user_data: UserData) -> Result<(), ExportError>;
}