use chrono::Duration;

use uuid::Uuid;

async fn create_new_static_event(
    name: String,
    description: String,
    start_date: Date,
    end_date: Date,
    start_time: Time,
    end_time: Time,
    user_id: Uuid,
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
        name,
        Some(description),
        NaiveDateTime::new(start_d, start_t),
        NaiveDateTime::new(end_d, end_t),
        user_id,
    );

    if let Err(err) = add_event_to_db(&event).await {
        eprintln!("DB error: {}", err);
    }

    Ok(())
}

async fn create_new_dynamic_event(
    slot_pre_data: DynamicEventPreData,
    slot: Option<Slot>,
    user_id: Uuid,
) -> Result<(), String> {
    let name = slot_pre_data.name.to_string();
    let description = slot_pre_data.description.to_string();
    let priority = slot_pre_data.priority;

    let slot_data = slot.ok_or_else(|| "Slot not found!")?;

    let event = Event::new_dynamic(
        name.into(),
        description.into(),
        NaiveDateTime::new(slot_data.date, slot_data.start_time),
        NaiveDateTime::new(slot_data.date, slot_data.end_time),
        priority,
        user_id,
    );

    if let Err(err) = add_event_to_db(&event).await {
        eprintln!("DB error: {}", err);
    }

    Ok(())
}

async fn search_for_slots(
    duration: Time,
    weekdays: Vec<i32>,
    range_start: Date,
    range_end: Date,
    user_id: Uuid,
) -> Result<Vec<Slot>, String> {
    let naive_duration = NaiveTime::from_hms_opt(duration.hour as u32, duration.minute as u32, 0)
        .ok_or_else(|| "Unsupported time format for duration time".to_owned())?;
    let naive_range_start = NaiveDate::from_ymd_opt(
        range_start.year,
        range_start.month as u32,
        range_start.day as u32,
    )
    .ok_or_else(|| "Unsupported time format for start range time".to_owned())?;
    let naive_range_end =
        NaiveDate::from_ymd_opt(range_end.year, range_end.month as u32, range_end.day as u32)
            .ok_or_else(|| "Unsupported time format for end range time".to_owned())?;

    let variants = find_dynamic_events_variants_by_weekdays(
        weekdays.iter().map(|&x| x as u32).collect(),
        naive_duration,
        naive_range_start,
        naive_range_end,
        user_id,
    )
    .await?;

    let slots = variants
        .iter()
        .map(|&slot| Slot {
            id: Uuid::new_v4().to_string(),
            start_time: slot.time(),
            end_time: slot.time()
                + Duration::hours(naive_duration.hour() as i64)
                + Duration::minutes(naive_duration.minute() as i64)
                + Duration::seconds(naive_duration.second() as i64),
            date: slot.date(),
            weekday: slot.weekday().to_string(),
        })
        .collect();

    Ok(slots)
}

async fn find_dynamic_events_variants_by_weekdays(
    selected_days: Vec<u32>,
    duration: NaiveTime,
    start_day: NaiveDate,
    end_day: NaiveDate,
    user_id: Uuid,
) -> Result<Vec<NaiveDateTime>, String> {
    let days_range = end_day.signed_duration_since(start_day).num_days() + 1;

    let mut days = Vec::new();

    for i in 0..days_range {
        let day = start_day + Duration::days(i);
        let day_of_week = day.weekday().num_days_from_monday();

        if selected_days.contains(&day_of_week) {
            days.push(day);
        }
    }

    let variants = find_dynamic_events_variants_by_naive_dates(days, duration, user_id)
        .await
        .map_err(|err| err.to_string())?;

    Ok(variants)
}

async fn find_dynamic_events_variants_by_naive_dates(
    selected_days: Vec<NaiveDate>,
    duration: NaiveTime,
    user_id: Uuid,
) -> Result<Vec<NaiveDateTime>, String> {
    let mut variants = Vec::new();

    let total_minutes = duration.hour() as i32 * 60 + duration.minute() as i32;
    if total_minutes > 16 * 60 {
        return Err("The available time window for scheduling events is from 6:00 AM to 10:00 PM. This means the maximum possible duration for a single event is 16 hours. Please select a shorter duration to allow the system find available slots.".to_owned());
    }

    for date in selected_days {
        let start_d = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
            .ok_or_else(|| "Unsupported date format for start date".to_owned())?;

        let events = get_events_in_day(user_id, start_d)
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
                start_time += Duration::minutes(60);
                end_time += Duration::minutes(60);
                continue;
            }

            start_time += Duration::minutes(15);
            end_time += Duration::minutes(15);
        }
    }
    Ok(variants)
}

async fn refetch_events(date: &NaiveDate, user_id: Uuid) -> Result<Vec<Event>, String> {
    match get_events_in_day(user_id, *date).await {
        Ok(events) => Ok(events),
        Err(err) => Err(format!(
            "Unable to load the data for the selected date: {}",
            err
        )),
    }
}
