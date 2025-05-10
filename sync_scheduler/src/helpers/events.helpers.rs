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

async fn refetch_events(date: &NaiveDate) -> Result<Vec<Event>, String> {
    match get_events_in_day(Uuid::nil(), *date).await {
        Ok(events) => Ok(events),
        Err(err) => Err(format!(
            "Unable to load the data for the selected date: {}",
            err
        )),
    }
}
