include!("./gui_render/calendar_render.rs");
include!("./gui_render/selected_date.rs");
include!("./gui_render/data_formatter.rs");
include!("./calendar/calendar_state.rs");

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() {
    let calendar_window = CalendarWindow::new().unwrap();
    // Create multi mutable reference to the calendar state
    // to be able to pass the state to the callbacks
    let calendar_state = Rc::new(RefCell::new(CalendarState::new()));

    // Set calendar initial state
    {
        // due to the mutable state in the code below I create mutable Ref,
        // but in fact there is nothing to mutate
        let state = calendar_state.borrow_mut();
        update_calendar_gui(&calendar_window, state);
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
        update_calendar_gui(&window, state);
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
        update_calendar_gui(&window, state);
    });

    let weak_window = calendar_window.as_weak();
    let state_clone_for_select = Rc::clone(&calendar_state);
    calendar_window.on_select_date(move |day_id| {
        let window = weak_window.unwrap();
        let state = state_clone_for_select.borrow_mut();
        update_selected_date_gui(&window, state, &day_id);
    });

    calendar_window.run().unwrap();
}
