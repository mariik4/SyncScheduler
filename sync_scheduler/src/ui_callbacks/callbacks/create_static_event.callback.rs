pub fn create_static_event_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    name: String,
    description: String,
    start_date: Date,
    end_date: Date,
    start_time: Time,
    end_time: Time,
) {
    let window = weak_window.unwrap();

    let handle = {
        let state = calendar_state.borrow_mut();
        state.get_tokio_handler()
    };
    let state = calendar_state.borrow();
    let user_id = match state.get_user_id() {
        Some(id) => id,
        None => {
            eprint!("User not found");
            return;
        }
    };
    let state_rc = calendar_state.clone();

    let _ = slint::spawn_local(async move {
        let join = handle.spawn(async move {
            create_new_static_event(
                name,
                description,
                start_date,
                end_date,
                start_time,
                end_time,
                user_id,
            )
            .await
        });

        match join.await {
            Ok(Ok(_)) => println!("Event created successfully"),
            Ok(Err(e)) => eprintln!("Error creating event: {}", e),
            Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
        }

        let state = state_rc.borrow_mut();
        calendar_render(&window, state);
    });
}
