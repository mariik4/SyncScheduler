pub fn next_month_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
) {
    let window = weak_window.unwrap();
    let mut state = calendar_state.borrow_mut();
    state.next_month();
    calendar_render(&window, state);
}

pub fn prev_month_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
) {
    let window = weak_window.unwrap();
    let mut state = calendar_state.borrow_mut();
    state.previous_month();
    calendar_render(&window, state);
}

pub fn select_date_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    day_id: SharedString,
) {
    let window = weak_window.unwrap();
    let mut state = calendar_state.borrow_mut();
    state.select_date(&day_id.to_string());
    calendar_render(&window, state);
}
