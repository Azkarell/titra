use std::str::FromStr;

use chrono::{DateTime, Datelike, Local, MappedLocalTime, NaiveTime, TimeZone, Timelike};
use egui::{Grid, RichText};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use icu::{
    calendar::Gregorian,
    datetime::{
        input::{DateInput, DateTimeInput, IsoTimeInput, LocalizedDateTimeInput},
        options::length,
        DateTimeFormatter, DateTimeFormatterOptions,
    },
    locid::Locale,
};
use sys_locale::get_locale;

use crate::{storage::TimeStorage, TitraView};

trait NaiveDateExt {
    fn days_in_month(&self) -> u32;
    fn days_in_year(&self) -> u32;
    fn is_leap_year(&self) -> bool;
}

impl NaiveDateExt for chrono::NaiveDate {
    fn days_in_month(&self) -> u32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn days_in_year(&self) -> u32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }
}

pub struct Overview {
    storage: Box<dyn TimeStorage + Send>,
    start_date: chrono::DateTime<Local>,
    end_date: chrono::DateTime<Local>,
    formatter: DateTimeFormatter,
}
impl Overview {
    pub fn new(storage: Box<dyn TimeStorage + Send>) -> Self {
        let now = Local::now();
        let sys_locale = get_locale().expect("failed to read locale");
        let options = DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
            length::Date::Medium,
            length::Time::Short,
        ));

        let formatter = DateTimeFormatter::try_new(
            &Locale::from_str(&sys_locale)
                .expect("Failed to read locale")
                .into(),
            options,
        )
        .expect("Failed to create formatter");
        Self {
            end_date: now
                .with_day(now.date_naive().days_in_month())
                .expect("Could not get end day of month"),
            start_date: now.with_day(1).expect("Failed to get start date"),
            storage,
            formatter: formatter,
        }
    }
}

impl TitraView for Overview {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.group(|ui| {
                Grid::new("overview_input_grid").show(ui, |ui| {
                    ui.label("Start: ");
                    let mut naiveDate = self.start_date.date_naive();
                    let dpb = DatePickerButton::new(&mut naiveDate).id_salt("start");
                    ui.add(dpb);
                    self.start_date = naiveDate.and_time(NaiveTime::default()).and_local_timezone(Local).earliest().unwrap();

                    ui.end_row();
                    ui.label("End: ");

                    let mut naiveDate = self.end_date.date_naive();
                    let dpb = DatePickerButton::new(&mut naiveDate).id_salt("end");
                    ui.add(dpb);
                    self.end_date = naiveDate.and_time(NaiveTime::default()).and_local_timezone(Local).earliest().unwrap();

                    ui.end_row();
                });
            });

            ui.group(|ui|{
                TableBuilder::new(ui)
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto())
                    .header(40.0, |mut header| {
                        header.col(|ui|{
                            ui.heading("Tag");
                        });
                        header.col(|ui|{
                            ui.heading("Startzeit");
                        });
                        header.col(|ui|{
                            ui.heading("Endzeit");
                        });
                        header.col(|ui|{
                            ui.heading("Bemerkung");
                        });
                    }).body(|mut body| {
                        if let Ok(entries) = self.storage.get_in_range(self.start_date, self.end_date) {
                            body.rows(32.0, entries.len(), |mut row| {
                                let (id, data) = &entries[row.index()];
                                row.col(|ui| {
                                    ui.label(data.start.date_naive().format("%Y-%M-%d").to_string());
                                });
                                row.col(|ui| {
                                    ui.label(data.start.time().format("%hh:%mm").to_string());
                                });
                                row.col(|ui| {
                                    ui.label(data.end.time().format("%hh:%mm").to_string());                                });
                            });
                        }
                    });
    
            });
        });
    }
}

pub trait IcuChronoExt: Sized {
    fn to_icu(&self) -> icu::calendar::DateTime<Gregorian>;
    fn from_icu(other: icu::calendar::DateTime<Gregorian>) -> Self;
}

impl IcuChronoExt for DateTime<Local> {
    fn to_icu(&self) -> icu::calendar::DateTime<Gregorian> {
        icu::calendar::DateTime::try_new_gregorian_datetime(
            self.year(),
            self.month().try_into().unwrap(),
            self.day().try_into().unwrap(),
            self.hour().try_into().unwrap(),
            self.minute().try_into().unwrap(),
            self.second().try_into().unwrap(),
        )
        .unwrap()
    }

    fn from_icu(other: icu::calendar::DateTime<Gregorian>) -> Self {
        let local = Local;
        let res = local.with_ymd_and_hms(
            other.year().unwrap().number.into(),
            other.month().unwrap().ordinal.into(),
            other.day_of_month().unwrap().0.into(),
            other.hour().unwrap().number().into(),
            other.minute().unwrap().number().into(),
            other.second().unwrap().number().into(),
        );
        res.earliest().unwrap()
    }
}
