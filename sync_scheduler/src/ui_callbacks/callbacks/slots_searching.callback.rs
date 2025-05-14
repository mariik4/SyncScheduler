pub fn slot_searching_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    slots_state: &Rc<RefCell<SlotsState>>,
    name: String,
    description: String,
    duration: Time,
    priority: i32,
    selected_weekdays: ModelRc<i32>,
    range_start: Date,
    range_end: Date,
) {
    let window = weak_window.unwrap();
    let handle = {
        let state = calendar_state.borrow_mut();
        state.get_tokio_handler()
    };
    let slots_rc = slots_state.clone();
    {
        let event_data = DynamicEventPreData {
            name,
            description,
            priority: priority as i64,
        };
        let mut slots_state = slots_rc.borrow_mut();
        slots_state.set_pending(event_data);
        slots_render(&window, slots_state);
    }
    let state = calendar_state.borrow();
    let user_id = match state.get_user_id() {
        Some(id) => id,
        None => {
            eprint!("User not found");
            return;
        }
    };

    let vecmodel: &slint::VecModel<i32> = selected_weekdays
        .as_any()
        .downcast_ref::<slint::VecModel<i32>>()
        .expect("ModelRc is not a VecModel");

    let weekdays: Vec<i32> = vecmodel
        .iter()
        .filter(|&idx| (0..=6).contains(&idx))
        .collect();

    let _ = slint::spawn_local(async move {
        let join_result = handle
            .spawn(async move {
                search_for_slots(duration, weekdays, range_start, range_end, user_id).await
            })
            .await;

        let mut slots_state = slots_rc.borrow_mut();
        match join_result {
            Ok(Ok(slots)) => {
                slots_state.set_success(slots);
            }
            Ok(Err(e)) => {
                eprintln!("Error creating event: {}", e);
                slots_state.set_failed(e.to_string());
            }
            Err(join_e) => {
                eprintln!("Tokio task failed: {}", join_e);
            }
        };

        slots_render(&window, slots_state);
    });
}
