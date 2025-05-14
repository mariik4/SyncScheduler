pub fn registration_callback(
    weak_window: &slint::Weak<CalendarWindow>,
    calendar_state: &Rc<RefCell<CalendarState>>,
    name: String,
    surname: String,
    username: String,
    password: String,
) {
    let window = weak_window.unwrap();
    let handle = {
        let state = calendar_state.borrow_mut();
        state.get_tokio_handler()
    };
    let calendar_rc = calendar_state.clone();

    let _ = slint::spawn_local(async move {
        let join_handle = handle
            .spawn(async move { create_new_user_on_db(username, name, surname, password).await });

        match join_handle.await {
            Ok(Ok(user)) => {
                let mut calendar_state = calendar_rc.borrow_mut();
                calendar_state.login_user(&user);
                after_login_register_render(&window, &user);
                calendar_render(&window, calendar_state);
            }
            Ok(Err(e)) => eprintln!("Error in check_login_of_user: {}", e),
            Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
        }
    });
}
