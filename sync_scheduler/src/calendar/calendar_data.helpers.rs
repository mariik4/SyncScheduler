use chrono::Days;

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
                    id: "".to_string(),
                    day_number: 0,
                    full_date: None,
                    weekday_index: i,
                    is_selectd: false,
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
            is_selectd: false,
        };
        weeks[weeks_counter].push(day_info);
        remained_days = 6 - date.weekday().num_days_from_monday();
    }

    // Fill the rest of the week with the empty daysa=1, b=4, c=4, d=-1, e=-14, мені потрібні графіки до цих задач, але замість x-y+14=0 використай x=y+14, і заштрихуй, будь ласка, область перетину
    // if the month is finished not in Sunday
    for _ in 0..remained_days {
        weeks[weeks_counter].push(DayInfo {
            id: "".to_string(),
            day_number: 0,
            full_date: None,
            weekday_index: 0,
            is_selectd: false,
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
