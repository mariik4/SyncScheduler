pub fn create_dynamic_event_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    slos_state: &Rc<RefCell<SlotsState>>,
    slot_id: String,
) {
    let window = weak_window.unwrap();

    let handle = {
        let state = calendar_state.borrow_mut();
        state.get_tokio_handler()
    };
    let state_rc = calendar_state.clone();
    let slots_rc = slos_state.clone();

    let (event_data, slot_data) = {
        let slots_state = slots_rc.borrow();

        let slot = slots_state
            .slots
            .iter()
            .find(|s| s.id == slot_id.to_string())
            .cloned();

        let event_data = slots_state.event_data.clone().expect("Missing event data");
        (event_data, slot)
    };

    let state = calendar_state.borrow();
    let user_id = match state.get_user_id() {
        Some(id) => id,
        None => {
            eprint!("User not found");
            return;
        }
    };

    let _ = slint::spawn_local(async move {
        let join = handle
            .spawn(async move { create_new_dynamic_event(event_data, slot_data, user_id).await });

        match join.await {
            Ok(Ok(_)) => println!("Event created successfully"),
            Ok(Err(e)) => eprintln!("Error creating event: {}", e),
            Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
        }

        {
            let mut slots_state = slots_rc.borrow_mut();
            // reset data for the new searching
            slots_state.reset();
            slots_render(&window, slots_state);
        }

        let state = state_rc.borrow_mut();
        calendar_render(&window, state);
    });
}
