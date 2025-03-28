fn update_selected_date_gui(
    calendar_window: &CalendarWindow,
    mut calendar_state: RefMut<CalendarState>,
    id: &str,
) {
    calendar_state.get_date_info_by_id(id);
    let selected_date = &calendar_state.selected_date;

    match selected_date {
        Some(date) => {
            calendar_window.set_selected_date(
                date.full_date
                    .expect("Full date does not exists")
                    .to_string()
                    .into(),
            );
            update_calendar_gui(calendar_window, calendar_state);
        }
        None => {
            calendar_window.set_selected_date("".into());
        }
    }
}
