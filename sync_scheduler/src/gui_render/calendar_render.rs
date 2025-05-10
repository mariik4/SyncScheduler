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
    selected_date_render(calendar_window, calendar_data);
}

fn selected_date_render(calendar_window: &CalendarWindow, calendar_state: RefMut<CalendarState>) {
    // Снимем Option<Date> с состояния
    if let Some(date_struct) = calendar_state.selected_date.clone() {
        // Распаковываем full_date из Option или падаем с сообщением
        let full_date = date_struct.full_date.expect("Full date does not exist");
        // Обновляем заголовок выбранной даты
        calendar_window.set_selected_date(full_date.to_string().into());

        // Сохраним слабую ссылку на окно и хэндлер
        let win_weak = calendar_window.as_weak();
        let tokio_handler = calendar_state.get_tokio_handler();

        // Спавним локальную задачу
        let _ = slint::spawn_local(async move {
            // Запускаем fetch в Runtime и ждём завершения
            let join_result = tokio_handler
                .spawn(async move { refetch_events(&full_date).await })
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
    } else {
        calendar_window.set_selected_date("".into());
    }
}
