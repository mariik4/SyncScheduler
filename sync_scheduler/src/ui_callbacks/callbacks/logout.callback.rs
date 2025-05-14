pub fn logout_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
) {
    let _window = weak_window.unwrap();
    let mut state = calendar_state.borrow_mut();
    state.logout_user();
}
