use std::{cell::RefCell, rc::Rc};

use chrono::{Datelike, Days, NaiveDate};
use slint::{ModelRc, VecModel};

slint::include_modules!();

#[derive(Debug)]
pub struct DayInfo {
    pub day_number: u32,
    pub full_date: Option<NaiveDate>,
    pub weekday_index: u32,
}

pub struct SlintDayInfo {
    pub day_number: u32,
    pub is_current_date: bool,
}

pub struct CalendarState {
    pub month: u32,
    pub year: i32,
}

impl CalendarState {
    pub fn new(month: u32, year: i32) -> Self {
        Self { month, year }
    }

    pub fn next_month(&mut self) {
        let (month, year) = calculate_next_month_and_year(self.month, self.year);
        self.month = month;
        self.year = year;
    }

    pub fn previous_month(&mut self) {
        let (month, year) = calculate_previous_month_and_year(self.month, self.year);
        self.month = month;
        self.year = year;
    }
}

pub fn get_days_of_month(year: i32, month: u32) -> Vec<Vec<DayInfo>> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");
    let days_in_month = get_days_from_month(year, month) as u32;

    let mut weeks = Vec::new();
    let mut weeks_counter = 0;
    let mut remained_days = 0;

    for i in 0..days_in_month {
        let date = first_day
            .checked_add_days(Days::new(i as u64))
            .expect("Date overflow");

        if i == 0 {
            weeks.push(Vec::new() as Vec<DayInfo>);
            for j in 0..date.weekday().num_days_from_monday() {
                let day_info = DayInfo {
                    day_number: 0,
                    full_date: None,
                    weekday_index: i,
                };
                weeks[weeks_counter].push(day_info)
            }
        }

        // insert new week array if week_day index equals 0 (Monday)
        if date.weekday().num_days_from_monday() == 0 {
            weeks_counter += 1;
            weeks.push(Vec::new() as Vec<DayInfo>);
        };

        let day_info = DayInfo {
            day_number: date.day(),
            full_date: Some(date),
            weekday_index: date.weekday().num_days_from_monday(),
        };
        weeks[weeks_counter].push(day_info);

        remained_days = date.weekday().num_days_from_sunday();
    }

    for _ in 0..remained_days {
        weeks[weeks_counter].push(DayInfo {
            day_number: 0,
            full_date: None,
            weekday_index: 0,
        });
    }

    weeks
}

pub fn calculate_next_month_and_year(month: u32, year: i32) -> (u32, i32) {
    if month == 12 {
        (1, year + 1)
    } else {
        (month + 1, year)
    }
}
pub fn calculate_previous_month_and_year(month: u32, year: i32) -> (u32, i32) {
    if month == 1 {
        (12, year - 1)
    } else {
        (month - 1, year)
    }
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .expect("huina")
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

pub fn format_slint_data(weeks: Vec<Vec<DayInfo>>) -> Vec<Vec<SlintDayInfo>> {
    weeks
        .into_iter()
        .map(|week| {
            week.into_iter()
                .map(|day| SlintDayInfo {
                    day_number: day.day_number,
                    is_current_date: false,
                })
                .collect()
        })
        .collect()
}

fn update_calendar(calendar_window: &CalendarWindow, month: u32, year: i32) {
    let weeks = get_days_of_month(year, month);
    let slint_weeks = format_slint_data(weeks);
    let nested_model: Vec<_> = slint_weeks
        .into_iter()
        .map(|week| {
            ModelRc::new(VecModel::from(
                week.into_iter()
                    .map(|day| slint_generatedCalendarWindow::SlintDay {
                        day_number: day.day_number as i32,
                        is_current_date: day.is_current_date,
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect();
    let slint_weeks_model = ModelRc::new(VecModel::from(nested_model));
    calendar_window.set_weeks(slint_weeks_model);

    // Update month/year title
    let month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let title = format!("{} {}", month_names[(month - 1) as usize], year);
    calendar_window.set_date_title(title.into());
}

fn main() {
    let calendar_window = CalendarWindow::new().unwrap();
    let calendar_state = Rc::new(RefCell::new(CalendarState::new(3, 2025))); // Initial month/year

    // Initial update
    {
        let state = calendar_state.borrow();
        update_calendar(&calendar_window, state.month, state.year);
    }

    // Setup button callbacks
    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    calendar_window.on_next_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.next_month();
        update_calendar(&window, state.month, state.year);
    });

    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    calendar_window.on_previous_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.previous_month();
        update_calendar(&window, state.month, state.year);
    });

    calendar_window.run().unwrap();
}
