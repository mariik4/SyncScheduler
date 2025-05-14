pub fn login_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    username: String,
    password: String,
) {
    let window = weak_window.unwrap();
    let handler = {
        let state = calendar_state.borrow_mut();
        state.get_tokio_handler()
    };
    let calendar_clone = calendar_state.clone();

    let _ = slint::spawn_local(async move {
        let join_handle = handler
            .spawn(async move { check_login_of_user(username.into(), password.into()).await });

        let mut calendar_state = calendar_clone.borrow_mut();
        match join_handle.await {
            Ok(Ok(user)) => {
                calendar_state.login_user(&user);
                after_login_register_render(&window, &user);
                calendar_render(&window, calendar_state);
            }
            Ok(Err(e)) => eprintln!("Error in check_login_of_user: {}", e),
            Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
        }
    });
}
