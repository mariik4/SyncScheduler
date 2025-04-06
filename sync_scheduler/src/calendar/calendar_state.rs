include!("./calendar_data.helpers.rs");

use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone)]
pub struct DayInfo {
    pub id: String,
    pub day_number: u32,
    pub full_date: Option<NaiveDate>,
    pub weekday_index: u32,
    pub is_selectd: bool,
    pub is_today: bool,
}

pub struct CalendarState {
    pub month: u32,
    pub year: i32,
    pub weeks: Vec<Vec<DayInfo>>,
    pub selected_date: Option<DayInfo>,
}

impl CalendarState {
    pub fn new() -> Self {
        let current_month = chrono::offset::Local::now().month();
        let current_year = chrono::offset::Local::now().year();

        let (weeks, today) = get_month_data(current_year, current_month);
        let mut selected_date = None;
        if let Some(mut day) = today {
            day.is_selectd = true;
            selected_date = Some(day);
            println!("Today: {:?}", selected_date);
        }
        Self {
            month: current_month,
            year: current_year,
            weeks,
            selected_date,
        }
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
        self.update_month_data();
    }

    pub fn previous_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
        self.update_month_data();
    }

    fn update_month_data(&mut self) {
        (self.weeks, _) = get_month_data(self.year, self.month);
    }

    pub fn select_date(&mut self, id: &str) {
        let prev_date_id = self.selected_date.as_ref().map(|day| day.id.clone());
        if prev_date_id.as_deref() == Some(id) {
            return;
        }

        for week in self.weeks.iter_mut() {
            for day in week.iter_mut() {
                if day.id == id {
                    day.is_selectd = true;
                    self.selected_date = Some(day.clone());
                }
                if let Some(ref prev_id) = prev_date_id {
                    if day.id == *prev_id {
                        day.is_selectd = false;
                    }
                }
            }
        }
    }
}

impl Default for CalendarState {
    fn default() -> Self {
        Self::new()
    }
}

// we can use them in the future to get data from the database to the additional fields
pub struct SelectedDateState {
    pub selected_date: DayInfo,
    // in the future we can pass here the events for the selected date
    // pub events: Vec<String>,
}
