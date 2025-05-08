use uuid::Uuid;

fn create_new_static_event(
    name: SharedString,
    description: SharedString,
    start_date: Date,
    end_date: Date,
    start_time: Time,
    end_time: Time,
) -> Option<Event> {
    let start_d = NaiveDate::from_ymd_opt(
        start_date.year,
        start_date.month as u32,
        start_date.day as u32,
    )?;
    let start_t = NaiveTime::from_hms_opt(
        start_time.hour as u32,
        start_time.minute as u32,
        start_time.second as u32,
    )?;

    let end_d = NaiveDate::from_ymd_opt(end_date.year, end_date.month as u32, end_date.day as u32)?;
    let end_t = NaiveTime::from_hms_opt(
        end_time.hour as u32,
        end_time.minute as u32,
        end_time.second as u32,
    )?;

    let event = Event {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: description.to_string(),
        start: NaiveDateTime::new(start_d, start_t),
        end: NaiveDateTime::new(end_d, end_t),
        event_type: "static".to_string(),
    };

    create_static_event(&event);
    Some(event)
}
