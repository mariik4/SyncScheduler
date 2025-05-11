use chrono::Duration;
use uuid::Uuid;

async fn create_new_static_event(
    name: SharedString,
    description: SharedString,
    start_date: Date,
    end_date: Date,
    start_time: Time,
    end_time: Time,
) -> Result<(), String> {
    let start_d = NaiveDate::from_ymd_opt(
        start_date.year,
        start_date.month as u32,
        start_date.day as u32,
    )
    .ok_or_else(|| "Unsupported date format for start date".to_owned())?;
    let start_t = NaiveTime::from_hms_opt(
        start_time.hour as u32,
        start_time.minute as u32,
        start_time.second as u32,
    )
    .ok_or_else(|| "Unsupported time format for start time".to_owned())?;

    let end_d = NaiveDate::from_ymd_opt(end_date.year, end_date.month as u32, end_date.day as u32)
        .ok_or_else(|| "Unsupported date format for end date".to_owned())?;
    let end_t = NaiveTime::from_hms_opt(
        end_time.hour as u32,
        end_time.minute as u32,
        end_time.second as u32,
    )
    .ok_or_else(|| "Unsupported time format for end time".to_owned())?;

    let event = Event::new_static(
        name.into(),
        Some(description.into()),
        NaiveDateTime::new(start_d, start_t),
        NaiveDateTime::new(end_d, end_t),
        Uuid::nil(),
    );

    if let Err(err) = add_event_to_db(&event).await {
        eprintln!("DB error: {}", err);
    }

    Ok(())
}

async fn find_dynamic_events_variants_by_weekdays(
    selected_days: Vec<u32>,
    duration: NaiveTime,
    start_day: NaiveDate,
    end_day: NaiveDate,
) -> Result<Vec<NaiveDateTime>, String> {
    let days_range = (end_day.signed_duration_since(start_day).num_days() + 1) as i64;

    let mut days = Vec::new();

    for i in 0..days_range {
        let day = start_day + Duration::days(i);
        let day_of_week = day.weekday().num_days_from_monday() as u32;

        if selected_days.contains(&day_of_week) {
            days.push(day);
        }
    }

    let variants = find_dynamic_events_variants_by_naive_dates(days, duration)
        .await
        .map_err(|err| err.to_string())?;

    Ok(variants)
}

async fn find_dynamic_events_variants_by_naive_dates(
    selected_days: Vec<NaiveDate>,
    duration: NaiveTime,
) -> Result<Vec<NaiveDateTime>, String> {
    let mut variants = Vec::new();
    for date in selected_days {
        let start_d = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
            .ok_or_else(|| "Unsupported date format for start date".to_owned())?;

        let events = get_events_in_day(Uuid::nil(), start_d)
            .await
            .map_err(|err| err.to_string())?;

        let mut start_time = NaiveTime::from_hms_opt(6, 0, 0)
            .ok_or_else(|| "Unsupported time format for start time".to_owned())?;

        let mut end_time = NaiveTime::from_hms_opt(
            start_time.hour() + duration.hour(),
            start_time.minute() + duration.minute(),
            start_time.second() + duration.second(),
        )
        .ok_or_else(|| "Unsupported time format for end time".to_owned())?;

        while end_time
            < NaiveTime::from_hms_opt(22, 0, 0)
                .ok_or_else(|| "Unsupported time format for comparison".to_owned())?
        {
            let mut is_free = true;

            for event in &events {
                if (event.start_time >= NaiveDateTime::new(start_d, start_time)
                    && event.start_time <= NaiveDateTime::new(start_d, end_time))
                    || (event.end_time >= NaiveDateTime::new(start_d, start_time)
                        && event.end_time <= NaiveDateTime::new(start_d, end_time))
                    || (event.start_time <= NaiveDateTime::new(start_d, start_time)
                        && event.end_time >= NaiveDateTime::new(start_d, end_time))
                {
                    // Event overlaps with the time slot
                    is_free = false;
                    break;
                }
            }
            if is_free {
                let variant = NaiveDateTime::new(start_d, start_time);
                variants.push(variant);
                start_time = start_time + Duration::minutes(60);
                end_time = end_time + Duration::minutes(60);
                continue;
            }

            start_time = start_time + Duration::minutes(15);
            end_time = end_time + Duration::minutes(15);
        }
    }
    Ok(variants)
}

async fn refetch_events(date: &NaiveDate) -> Result<Vec<Event>, String> {
    match get_events_in_day(Uuid::nil(), *date).await {
        Ok(events) => Ok(events),
        Err(err) => Err(format!(
            "Unable to load the data for the selected date: {}",
            err
        )),
    }
}
