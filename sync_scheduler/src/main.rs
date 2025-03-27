use chrono::{Datelike, Days, NaiveDate};
use slint::{ModelRc, VecModel};

slint::include_modules!();

#[derive(Debug)]
pub struct DayInfo {
    pub day_number: u32,
    pub full_date: Option<NaiveDate>,
    pub weekday_index: u32, // Monday=0, Sunday=6
}

pub struct SlintDayInfo {
    pub day_number: u32,
    pub is_current_date: bool,
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

fn main() {
    let calendarWindow = CalendarWindow::new().unwrap();

    let weeks = get_days_of_month(2025, 3);
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
    calendarWindow.set_weeks(slint_weeks_model);

    calendarWindow.run().unwrap();
}
