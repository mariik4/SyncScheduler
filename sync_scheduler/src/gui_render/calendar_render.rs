// since always when wee need to update some data it need rerendering
// we use calendar render as a main render function,
// through which we can update all other components data
fn calendar_render(calendar_window: &CalendarWindow, calendar_data: RefMut<CalendarState>) {
    let slint_weeks = format_slint_date(&calendar_data.weeks);
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

    let dynamic_range_start = chrono::offset::Local::now().date_naive();
    let dynamic_range_end = dynamic_range_start + Duration::days(14);

    println!("start range {:?}", dynamic_range_start);
    println!("start range {:?}", dynamic_range_end);

    calendar_window.set_dynamic_range_start(Date {
        year: dynamic_range_start.year(),
        month: dynamic_range_start.month() as i32,
        day: dynamic_range_start.day() as i32,
    });
    calendar_window.set_dynamic_range_end(Date {
        year: dynamic_range_end.year(),
        month: dynamic_range_end.month() as i32,
        day: dynamic_range_end.day() as i32,
    });

    selected_date_render(calendar_window, calendar_data);
}

fn selected_date_render(calendar_window: &CalendarWindow, calendar_state: RefMut<CalendarState>) {
    if let Some(date_struct) = calendar_state.selected_date.clone() {
        let full_date = date_struct.full_date.expect("Full date does not exist");
        let date = slint_generatedCalendarWindow::Date {
            year: full_date.year(),
            month: full_date.month() as i32,
            day: full_date.day() as i32,
        };
        calendar_window.set_selected_date(date);

        let win_weak = calendar_window.as_weak();
        let tokio_handler = calendar_state.get_tokio_handler();
        let user_id = match calendar_state.get_user_id() {
            Some(id) => id,
            None => {
                eprint!("User not found");
                return;
            }
        };

        let _ = slint::spawn_local(async move {
            let join_result = tokio_handler
                .spawn(async move { refetch_events(&full_date, user_id).await })
                .await;
            let mut events_fetching_error = false;

            let events: Vec<_> = match join_result {
                Ok(Ok(evts)) => evts,
                Ok(Err(err_msg)) => {
                    eprintln!("Unable to load events: {}", err_msg);
                    events_fetching_error = true;
                    Vec::new()
                }
                Err(join_err) => {
                    eprintln!("Tokio task failed: {}", join_err);
                    events_fetching_error = true;
                    Vec::new()
                }
            };

            if let Some(win) = win_weak.upgrade() {
                println!("Events: {:?}", events);
                let slint_events = slint_format_events(events, full_date);
                println!("Slint events: {:?}", slint_events);
                win.set_events(ModelRc::new(VecModel::from(slint_events)));
                win.set_is_events_fetching_error(events_fetching_error);
            }
        });
    }
}

fn after_login_register_render(calendar_window: &CalendarWindow, user: &User) {
    calendar_window.set_is_login_layout_open(false);
    let name_letter = user.first_name.chars().next().unwrap_or(' ');
    let surname_letter = user.last_name.chars().next().unwrap_or(' ');

    let user_icon = format!(
        "{}{}",
        name_letter.to_uppercase(),
        surname_letter.to_uppercase()
    );
    calendar_window.set_user_icon(user_icon.into());
}
