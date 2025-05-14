include!("./gui_render/calendar_render.rs");
include!("./gui_render/data_formatter.rs");
include!("./helpers/events.helpers.rs");
include!("./helpers/calendar.helpers.rs");
include!("./store/calendar.store.rs");
include!("./store/store.types.rs");
include!("./database/db_functions.rs");
include!("./store/slots.store.rs");
include!("./gui_render/slots.render.rs");
include!("./ui_callbacks/calback_register.rs");

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use dotenv::dotenv;
use slint::{Model, ModelRc, SharedString, VecModel};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use tokio::runtime::Handle;

slint::include_modules!();

fn main() {
    dotenv().ok();

    let tokio_rt = tokio::runtime::Runtime::new().unwrap();
    let tokio_handler = tokio_rt.handle().clone();

    let calendar_window = CalendarWindow::new().unwrap();
    calendar_window.window().set_maximized(true);
    // Create multi mutable reference to the calendar state
    // to be able to pass the state to the callbacks
    let calendar_state = Rc::new(RefCell::new(CalendarState::new(tokio_handler)));
    let slots_state = Rc::new(RefCell::new(SlotsState::new()));
    // Set calendar initial state
    {
        // due to the mutable state in the code below I create mutable Ref,
        // but in fact there is nothing to mutate
        let calendar_state = calendar_state.borrow_mut();
        calendar_render(&calendar_window, calendar_state);
    }

    register_ui_callbacks(&calendar_window, calendar_state, slots_state);

    calendar_window.run().unwrap();
}
