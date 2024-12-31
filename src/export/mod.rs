use thiserror::Error;

use crate::{storage::TimeEntry, user::UserData};

pub mod excel;


#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Could not export data: {0}")]
    Unknown(String)
}



pub trait Exporter {
    fn export(&self, data: Vec<TimeEntry>, user_data: UserData) -> Result<(), ExportError>;
}