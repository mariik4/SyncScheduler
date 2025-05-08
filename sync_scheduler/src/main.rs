include!("./gui_render/calendar_render.rs");
include!("./gui_render/data_formatter.rs");
include!("./helpers/events.helpers.rs");
include!("./helpers/calendar.helpers.rs");
include!("./store/calendar.store.rs");
include!("./store/events.store.rs");
include!("./store/store.types.rs");
include!("./db/events.db.rs");

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};

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
    let events_state = Rc::new(RefCell::new(EventsState::new()));

    // Set calendar initial state
    {
        // due to the mutable state in the code below I create mutable Ref,
        // but in fact there is nothing to mutate
        let calendar_state = calendar_state.borrow_mut();
        let events_state = events_state.borrow_mut();
        calendar_render(&calendar_window, calendar_state, events_state);
    }

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    let events_state_ref_clone = Rc::clone(&events_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_next_month(move || {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        let mut events_state = events_state_ref_clone.borrow_mut();
        calendar_state.next_month();
        calendar_render(&window, calendar_state, events_state);
    });

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    let events_state_ref_clone = Rc::clone(&events_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_previous_month(move || {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        let mut events_state = events_state_ref_clone.borrow_mut();
        calendar_state.previous_month();
        calendar_render(&window, calendar_state, events_state);
    });

    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    let events_state_ref_clone = Rc::clone(&events_state);
    calendar_window.on_select_date(move |day_id| {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        let mut events_state = events_state_ref_clone.borrow_mut();
        calendar_state.select_date(&day_id);
        if let Some(ref date) = calendar_state.selected_date {
            events_state.refetch_events(date);
        }
        calendar_render(&window, calendar_state, events_state);
    });

    let calendar_state_clone = Rc::clone(&calendar_state);
    let events_state_clone = Rc::clone(&events_state);
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

            // let mut test = events_state_clone.borrow_mut();
            // test.add_event(event);
            // test.fetch_events(calendar_state_clone.borrow().selected_date.clone().unwrap());
        },
    );

    calendar_window.run().unwrap();
}
