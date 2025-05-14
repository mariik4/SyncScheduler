async fn create_new_dynamic_event(
    slot_pre_data: DynamicEventPreData,
    slot: Option<Slot>,
    user_id: Uuid,
) -> Result<(), String> {
    let name = slot_pre_data.name.to_string();
    let description = slot_pre_data.description.to_string();
    let priority = slot_pre_data.priority;

    let slot_data = slot.ok_or("Slot not found!")?;

    let event = Event::new_dynamic(
        name,
        Some(description),
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
