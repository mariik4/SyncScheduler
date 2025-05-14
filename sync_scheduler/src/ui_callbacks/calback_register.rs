include!("./callbacks/login.callback.rs");
include!("./callbacks/registration.callback.rs");
include!("./callbacks/create_static_event.callback.rs");
include!("./callbacks/create_dynamic_event.callback.rs");
include!("./callbacks/slots_searching.callback.rs");
include!("./callbacks/calendar.callback.rs");

pub fn register_ui_callbacks(
    window: &CalendarWindow,
    calendar_state: Rc<RefCell<CalendarState>>,
    slots_state: Rc<RefCell<SlotsState>>,
) {
    // registrate callback for logic
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_collect_login(move |username: SharedString, password: SharedString| {
            login_callback(
                &weak_window,
                &calendar_state_clone,
                username.into(),
                password.into(),
            )
        });
    }

    // registrate callback for registration
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_collect_registration(
            move |name: SharedString,
                  surname: SharedString,
                  username: SharedString,
                  password: SharedString| {
                registration_callback(
                    &weak_window,
                    &calendar_state_clone,
                    name.into(),
                    surname.into(),
                    username.into(),
                    password.into(),
                );
            },
        );
    }

    // registrate callback for calendar next
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_next_month(move || {
            next_month_callback(&weak_window, &calendar_state_clone);
        });
    }

    // registrate callback for calendar previous
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_previous_month(move || {
            prev_month_callback(&weak_window, &calendar_state_clone);
        });
    }

    // registrate callback for date selection in the calendar
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_select_date(move |day_id| {
            select_date_callback(&weak_window, &calendar_state_clone, day_id);
        });
    }

    // registrate callback for static event creation
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        window.on_create_static_event(
            move |name: SharedString,
                  description: SharedString,
                  start_date: Date,
                  end_date: Date,
                  start_time: Time,
                  end_time: Time| {
                create_static_event_callback(
                    &weak_window,
                    &calendar_state_clone,
                    name.into(),
                    description.into(),
                    start_date,
                    end_date,
                    start_time,
                    end_time,
                );
            },
        );
    }

    // registrate callback for dynamic event creation
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        let slots_state_clone = Rc::clone(&slots_state);
        window.on_create_dynamic_event(move |slot_id: SharedString| {
            create_dynamic_event_callback(
                &weak_window,
                &calendar_state_clone,
                &slots_state_clone,
                slot_id.into(),
            );
        });
    }

    // registrate callback for slot searching
    {
        let weak_window = window.as_weak();
        let calendar_state_clone = Rc::clone(&calendar_state);
        let slots_state_clone = Rc::clone(&slots_state);
        window.on_start_slots_searching(
            move |name: SharedString,
                  description: SharedString,
                  duration: Time,
                  priority: i32,
                  selected_weekdays: ModelRc<i32>,
                  range_start: Date,
                  range_end: Date| {
                slot_searching_callback(
                    &weak_window,
                    &calendar_state_clone,
                    &slots_state_clone,
                    name.into(),
                    description.into(),
                    duration,
                    priority,
                    selected_weekdays,
                    range_start,
                    range_end,
                );
            },
        );
    }
}
