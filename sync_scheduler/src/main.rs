include!("./gui_render/calendar_render.rs");
include!("./gui_render/data_formatter.rs");
include!("./calendar/calendar_state.rs");
include!("./events/events.helpers.rs");
include!("./events/events_state.rs");

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() {
    let calendar_window = CalendarWindow::new().unwrap();
    calendar_window.window().set_maximized(true);
    // Create multi mutable reference to the calendar state
    // to be able to pass the state to the callbacks
    let calendar_state = Rc::new(RefCell::new(CalendarState::new()));
    let events_state: Rc<RefCell<EventsState>> = Rc::new(RefCell::new(EventsState::new()));

    // Set calendar initial state
    {
        // due to the mutable state in the code below I create mutable Ref,
        // but in fact there is nothing to mutate
        let state = calendar_state.borrow_mut();
        calendar_render(&calendar_window, state);
    }

    // Setup next month button callback
    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_next_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.next_month();
        calendar_render(&window, state);
    });

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let state_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_previous_month(move || {
        let window = weak_window.unwrap();
        let mut state = state_clone.borrow_mut();
        state.previous_month();
        calendar_render(&window, state);
    });

    let weak_window = calendar_window.as_weak();
    let state_clone_for_select = Rc::clone(&calendar_state);
    calendar_window.on_select_date(move |day_id| {
        let window = weak_window.unwrap();
        let mut state = state_clone_for_select.borrow_mut();
        state.select_date(&day_id);
        calendar_render(&window, state);
    });

    let events_state_clone = Rc::clone(&events_state);
    let calendar_state_clone = Rc::clone(&calendar_state);
    calendar_window.on_collect_static(
        move |name: SharedString,
              description: SharedString,
              start_date: Date,
              end_date: Date,
              start_time: Time,
              end_time: Time| {
            let event = create_new_static_event(
                name,
                description,
                start_date,
                end_date,
                start_time,
                end_time,
            );

            print!("Creating new static event\n");

            let mut test = events_state_clone.borrow_mut();
            test.add_event(event);
            test.fetch_events(calendar_state_clone.borrow().selected_date.clone().unwrap());
        },
    );

    calendar_window.run().unwrap();
}
