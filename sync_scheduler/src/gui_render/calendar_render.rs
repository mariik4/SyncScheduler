fn update_calendar_gui(calendar_window: &CalendarWindow, calendar_data: RefMut<CalendarState>) {
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
}
