// since always when wee need to update some data it need rerendering
// we use calendar render as a main render function,
// through which we can update all other components data
fn calendar_render(
    calendar_window: &CalendarWindow,
    calendar_data: RefMut<CalendarState>,
    events_data: RefMut<EventsState>,
) {
    let slint_weeks = format_slint_data(&calendar_data.weeks);
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
    let title = format!(
        "{} {}",
        month_names[(calendar_data.month - 1) as usize],
        calendar_data.year
    );
    calendar_window.set_date_title(title.into());
    selected_date_render(calendar_window, calendar_data);
}

fn selected_date_render(calendar_window: &CalendarWindow, calendar_state: RefMut<CalendarState>) {
    let selected_date = &calendar_state.selected_date;

    match selected_date {
        Some(date) => {
            calendar_window.set_selected_date(
                date.full_date
                    .expect("Full date does not exists")
                    .to_string()
                    .into(),
            );
        }
        None => {
            calendar_window.set_selected_date("".into());
        }
    }
}
