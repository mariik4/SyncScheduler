use chrono::Days;

pub fn get_month_data(year: i32, month: u32) -> Option<(Vec<Vec<DayInfo>>, Option<DayInfo>)> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)?;
    let days_in_month = get_days_count_in_month(year, month)? as u32;
    let today = chrono::offset::Local::now().date_naive();
    let mut today_obj: Option<DayInfo> = None;

    let mut weeks = Vec::new();
    let mut weeks_counter = 0;
    let mut remained_days = 0;

    for i in 0..days_in_month {
        let date = first_day
            .checked_add_days(Days::new(i as u64))
            .expect("Date overflow");

        // insert 0 to the beginning of the week array
        // when the month does not start from the Monday
        // Need for the renderring offset
        if i == 0 {
            weeks.push(Vec::new() as Vec<DayInfo>);
            for _ in 0..date.weekday().num_days_from_monday() {
                // Month beggining empty day value
                let day_info = DayInfo {
                    id: "".to_string(),
                    day_number: 0,
                    full_date: None,
                    weekday_index: i,
                    is_selected: false,
                    is_today: false,
                    events_preview: None,
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
            id: date.to_string(),
            day_number: date.day(),
            full_date: Some(date),
            weekday_index: date.weekday().num_days_from_monday(),
            is_selected: false,
            is_today: date == today,
            events_preview: Some(DayEventsPreview {
                events: Vec::new(),
                count: 0,
            }),
        };
        if day_info.is_today {
            today_obj = Some(day_info.clone());
        }
        weeks[weeks_counter].push(day_info);
        remained_days = 6 - date.weekday().num_days_from_monday();
    }

    // Fill the rest of the week with the empty days

    // if the month is finished not in Sunday
    for _ in 0..remained_days {
        weeks[weeks_counter].push(DayInfo {
            id: "".to_string(),
            day_number: 0,
            full_date: None,
            weekday_index: 0,
            is_selected: false,
            is_today: false,
            events_preview: None,
        });
    }

    Some((weeks, today_obj))
}

pub fn get_days_count_in_month(year: i32, month: u32) -> Option<i64> {
    let month_start_date = NaiveDate::from_ymd_opt(year, month, 1)?;
    let month = NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )?;

    Some(month.signed_duration_since(month_start_date).num_days())
}
