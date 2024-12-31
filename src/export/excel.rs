use std::collections::HashMap;

use chrono::Datelike;
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet};

use crate::user::{self, UserData};

use super::Exporter;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum XlsxFormats {
    Header,
    
}


pub struct XlsxExporter{
    formats: HashMap<XlsxFormats, Format>
}

impl XlsxExporter {
    pub fn new() -> Self {
        Self {  
            formats: HashMap::from([
                (XlsxFormats::Header, Format::new().set_font_size(30.0).set_background_color(Color::Cyan))])
        }
    }
}

impl Exporter for XlsxExporter {
    fn export(&self, data: Vec<crate::storage::TimeEntry>, user_data: UserData) -> Result<(), super::ExportError> {
        
        let mut wb = Workbook::new();
        let sheet = wb.add_worksheet();
        let month = format!("{}-{}", data[0].1.end.year(), data[0].1.start.month());
        sheet.set_name(month.clone()).unwrap();
        generate_header(&self.formats, sheet, &user_data);


        wb.save(format!("./{} {}.xlsx", user_data.name.clone(), month )).expect("Failed to save");
        Ok(())
    }
}

fn generate_header(formats: &HashMap<XlsxFormats, Format>, sheet: &mut Worksheet, user_data: &UserData) {
    sheet.write_with_format(0, 0, user_data.name.clone(), formats.get(&XlsxFormats::Header).unwrap()).unwrap();
    sheet.set_column_width(0, 32.0).unwrap();
}