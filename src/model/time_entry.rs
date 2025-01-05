use chrono::{NaiveDate, NaiveTime};

pub type TimeEntryId = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeEntryData {
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub date: NaiveDate,
    pub remark: String
}
impl TimeEntryData {
    pub fn with_start(&self, val: NaiveTime) -> TimeEntryData {
        Self {
            date: self.date,
            end: self.end,
            remark: self.remark.clone(),
            start: val
        }
    }

    pub fn with_end(&self, val: NaiveTime) -> TimeEntryData {
        Self {
            date: self.date,
            end: val,
            remark: self.remark.clone(),
            start: self.start
        }
    }
}

pub type TimeEntry = (TimeEntryId, TimeEntryData);