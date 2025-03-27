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

#[derive(Clone)]
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
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    pub fn previous_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
    }
}

pub fn get_month_data(year: i32, month: u32) -> Vec<Vec<DayInfo>> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");
    let days_in_month = get_days_count_in_month(year, month) as u32;

    let mut weeks = Vec::new();
    let mut weeks_counter = 0;
    let mut remained_days = 0;

    for i in 0..days_in_month {
        let date = first_day
            .checked_add_days(Days::new(i as u64))
            .expect("Date overflow");

        // insert 0 to the beginning of the week array
        // when the month does not start from the Monday
        // Neede for the renderring offset
        if i == 0 {
            weeks.push(Vec::new() as Vec<DayInfo>);
            for _ in 0..date.weekday().num_days_from_monday() {
                // Month beggining empty day value
                let day_info = DayInfo {
                    day_number: 0,
                    full_date: None,
                    weekday_index: i,
                };
                weeks[weeks_counter].push(day_info)
            }
        }

        // Start filling array for the next week, since the day is Monday (index = 0)
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
        remained_days = 6 - date.weekday().num_days_from_monday();
    }

    // Fill the rest of the week with the empty daysa=1, b=4, c=4, d=-1, e=-14, мені потрібні графіки до цих задач, але замість x-y+14=0 використай x=y+14, і заштрихуй, будь ласка, область перетину
    // if the month is finished not in Sunday
    for _ in 0..remained_days {
        weeks[weeks_counter].push(DayInfo {
            day_number: 0,
            full_date: None,
            weekday_index: 0,
        });
    }

    weeks
}

pub fn get_days_count_in_month(year: i32, month: u32) -> i64 {
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
    .expect("Date issue")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date"))
    .num_days()
}

pub fn format_slint_data(
    weeks: Vec<Vec<DayInfo>>,
) -> std::vec::Vec<slint::ModelRc<slint_generatedCalendarWindow::SlintDay>> {
    weeks
        .into_iter()
        .map(|week| {
            ModelRc::new(VecModel::from(
                week.into_iter()
                    .map(|day| slint_generatedCalendarWindow::SlintDay {
                        day_number: day.day_number as i32,
                        is_current_date: false,
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect()
}

fn update_calendar_gui(calendar_window: &CalendarWindow, month: u32, year: i32) {
    let weeks = get_month_data(year, month);
    let slint_weeks = format_slint_data(weeks);
    let slint_weeks_model = ModelRc::new(VecModel::from(slint_weeks));
    calendar_window.set_weeks(slint_weeks_model);

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
    let current_month = chrono::offset::Local::now().month();
    let current_year = chrono::offset::Local::now().year();
    // Create multi mutable reference to the calendar state
    // to be able to pass the state to the callbacks
    let calendar_state = Rc::new(RefCell::new(CalendarState::new(
        current_month,
        current_year,
    )));

    // Set calendar initial state
    {
        let state = calendar_state.borrow();
        update_calendar_gui(&calendar_window, state.month, state.year);
    }

    // Setup next month button callback
    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_next_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.next_month();
        update_calendar_gui(&window, state.month, state.year);
    });

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_previous_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.previous_month();
        update_calendar_gui(&window, state.month, state.year);
    });

    calendar_window.run().unwrap();
}
