use std::{collections::HashMap, ops::Range};

use chrono::{Datelike, NaiveDate, TimeDelta};
use egui::{Align, Grid, Label, RichText, ScrollArea};
use log::{info, warn};

use crate::{
    extensions::naive_date_ext::NaiveDateExt,
    model::{
        error::ApplicationError,
        time_entry::{TimeEntry, TimeEntryId},
    },
    Services, StateView, StaticView, TitraResult, TitraView,
};

use super::time_entry_edit::TimeEntryEdit;

pub struct OverviewTable {
    header: OverviewHeader,
    footer: OverviewFooter,
    month_view: MonthView,
}

impl OverviewTable {
    pub fn new(at: NaiveDate) -> Self {
        Self {
            header: OverviewHeader {},
            footer: OverviewFooter {},
            month_view: MonthView::new(at, vec![]),
        }
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.month_view.set_date(date);
    }
}

#[derive(Clone)]
struct PlannedHourView {
    planned_hour: TimeDelta,
    repr: String,
    date: NaiveDate,
    actual_time: TimeDelta,
}

fn format_time_delta_hh_mm(delta: TimeDelta) -> String {
    format!("{:0>2}:{:0>2}", delta.num_hours(), delta.num_minutes() % 60)
}

impl PlannedHourView {
    fn new(date: NaiveDate, planned_hour: TimeDelta) -> Self {
        Self {
            date,
            planned_hour,
            repr: format_time_delta_hh_mm(planned_hour),
            actual_time: TimeDelta::zero(),
        }
    }

    fn set_actual_time(&mut self, delta: TimeDelta) {
        self.actual_time = delta;
    }
}

struct MonthView {
    date: NaiveDate,
    entries: HashMap<NaiveDate, Vec<TimeEntry>>,
    planned_hours: HashMap<NaiveDate, TimeDelta>,
    requires_refresh: bool,
    range: Range<usize>,
    flatten_entries: Vec<MonthViewEntry>,
}

struct MonthViewEntry {
    time: Option<(TimeEntryId, TimeEntryEdit)>,
    label: Option<String>,
    planned_time: Option<PlannedHourView>,
}

impl MonthView {
    fn new(date: NaiveDate, entries: Vec<TimeEntry>) -> Self {
        let map = Self::get_mapped_entries(date, entries);
        let mut ret = Self {
            date,
            entries: map,
            planned_hours: HashMap::new(),
            requires_refresh: true,
            range: 0..date.days_in_month() as usize,
            flatten_entries: vec![],
        };

        ret.flatten_entries();
        ret
    }

    pub fn set_displayed_range(&mut self, range: Range<usize>) {
        self.range = range
    }

    fn get_mapped_entries(
        date: NaiveDate,
        entries: Vec<TimeEntry>,
    ) -> HashMap<NaiveDate, Vec<TimeEntry>> {
        let range = date.as_month_range();
        let mut map = HashMap::new();
        for d in range.0.day()..=range.1.day() {
            let data = entries
                .iter()
                .filter(|e| e.1.date.day() == d)
                .cloned()
                .collect();
            map.insert(date.with_day(d).unwrap(), data);
        }
        map
    }

    fn set_entries(&mut self, entries: Vec<TimeEntry>) {
        self.entries.clear();
        self.entries
            .extend(Self::get_mapped_entries(self.date, entries));
        self.flatten_entries();
        self.set_displayed_range(0..self.rows());
    }

    fn set_planned_hours(&mut self, planned_hours: HashMap<NaiveDate, TimeDelta>) {
        self.planned_hours.clear();
        self.planned_hours.extend(planned_hours);
        self.flatten_entries();
        self.set_displayed_range(0..self.rows());
    }

    fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
        self.requires_refresh = true;
    }

    fn rows(&self) -> usize {
        self.flatten_entries.len()
    }

    fn flatten_entries(&mut self) {
        let mut res = vec![];
        for d in 1..=self.date.days_in_month() {
            let cur_date = self.date.with_day(d).unwrap();
            let entries_for_day = self.entries.get(&cur_date).unwrap();
            let mut planned = PlannedHourView::new(
                cur_date,
                *self
                    .planned_hours
                    .get(&cur_date)
                    .unwrap_or(&TimeDelta::zero()),
            );
            planned.set_actual_time(entries_for_day.iter().map(|e| e.1.end - e.1.start).sum());
            if entries_for_day.len() == 0 {
                res.push(MonthViewEntry {
                    time: None,
                    label: Some(cur_date.format("%x").to_string()),
                    planned_time: Some(planned),
                });
            } else if entries_for_day.len() == 1 {
                res.push(MonthViewEntry {
                    time: Some((
                        entries_for_day[0].0,
                        TimeEntryEdit::from(entries_for_day[0].1.clone()),
                    )),
                    label: Some(cur_date.format("%x").to_string()),
                    planned_time: Some(planned),
                });
            } else {
                let mut vec: Vec<MonthViewEntry> = entries_for_day
                    .iter()
                    .map(|e| MonthViewEntry {
                        time: Some((e.0, TimeEntryEdit::from(e.1.clone()))),
                        label: None,
                        planned_time: None,
                    })
                    .collect();
                vec[0].label = Some(cur_date.format("%x").to_string());
                vec[0].planned_time = Some(planned);
                res.extend(vec);
            }
        }

        self.flatten_entries = res;
    }
}

impl TitraView<(), ApplicationError, Services> for MonthViewEntry {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        services: &mut Services,
    ) -> TitraResult<(), ApplicationError> {
        match &mut self.label {
            Some(l) => ui.label(l.clone()),
            None => ui.label(""),
        };
        let change1 = match &mut self.time {
            Some((id, edit)) => {
                let res = StateView::show(edit, ui).then(|d| {
                    if let Err(err) = services.time_service.update_entry(*id, d) {
                        warn!("Failed to update: {err}");
                    };
                });
                let res2 = if ui.button("x").clicked() {
                    _ = services.time_service.remove_entry(*id);
                    TitraResult::Done(())
                } else {
                    TitraResult::NoChange
                };
                res.combine_with(res2)
            }
            None => {
                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("");
                TitraResult::NoChange
            }
        };

        let change2 = match &mut self.planned_time {
            Some(e) => {
                StaticView::show(e, ui);
                TitraResult::NoChange
            }
            None => {
                ui.label("");
                ui.label("");
                TitraResult::NoChange
            }
        };

        change1.combine_with(change2)
    }
}

struct OverviewHeader {}

struct OverviewFooter {}

impl StaticView for PlannedHourView {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(format_time_delta_hh_mm(self.actual_time));
        ui.label(format_time_delta_hh_mm(self.planned_hour));
    }
}

impl TitraView<(), ApplicationError, Services> for MonthView {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        services: &mut Services,
    ) -> TitraResult<(), ApplicationError> {
        if self.requires_refresh {
            info!("Refreshing");
            if let Ok(entries) = services
                .time_service
                .get_in_range(self.date.as_month_range())
            {
                self.set_entries(entries);
            } else {
                self.set_entries(vec![]);
            }
            if let Ok(planned_hours) = services.hour_service.get_range(self.date.as_month_range()) {
                self.set_planned_hours(planned_hours);
            } else {
                self.set_planned_hours(HashMap::new());
            }
            self.requires_refresh = false;
        }

        for d in self.flatten_entries[self.range.clone()].iter_mut() {
            match d.show(ui, services) {
                TitraResult::Done(_) => self.requires_refresh = true,
                _ => {}
            }
            ui.end_row();
        }

        TitraResult::NoChange
    }
}

impl TitraView<(), ApplicationError, Services> for OverviewTable {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        services: &mut Services,
    ) -> TitraResult<(), ApplicationError> {
        ui.vertical(|ui|{
         

            Grid::new("header")
            .num_columns(7)
            .min_col_width(120.0)
            .show(ui, |ui| {
                StaticView::show(&mut self.header, ui);
            });
        ScrollArea::vertical()
            .max_width(ui.available_width())
            .max_height(ui.available_height() - 250.0)
            .show_rows(
                ui,
                ui.spacing().interact_size.y,
                self.month_view.rows(),
                |ui, range| {
                  
                    Grid::new("overview_all")
                        .striped(true)
                        .num_columns(7)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            self.month_view.set_displayed_range(range);
                            self.month_view.show(ui, services);
                        });
                },
            );
        Grid::new("footer")
            .num_columns(7)
            .show(ui, |ui| {
                StaticView::show(&mut self.footer, ui);
            });

        });

        TitraResult::NoChange
    }
}

impl StaticView for OverviewHeader {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Date  ");
        ui.heading("Beginn");
        ui.heading("Ende  ");
        ui.add_sized(
            (240.0, 25.0),
            Label::new(RichText::new("Bemerkung").heading()).halign(Align::LEFT),
        );
        ui.separator();
        ui.heading("Ist   ");
        ui.heading("Soll  ");
    }
}

impl StaticView for OverviewFooter {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("total");
        ui.heading("");
    }
}
