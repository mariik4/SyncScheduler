fn slots_render(window: &CalendarWindow, slots_state: RefMut<SlotsState>) {
    let is_pending = match slots_state.status {
        SlotsStatus::Pending => true,
        SlotsStatus::Success => false,
        SlotsStatus::Failed => false,
    };

    let formatted_slots = slint_format_slots(&slots_state.slots);

    window.set_slots(ModelRc::new(VecModel::from(formatted_slots)));
    window.set_slots_error(
        slots_state
            .error_message
            .clone()
            .unwrap_or("".to_string())
            .into(),
    );
    window.set_is_slots_loading(is_pending);
}
