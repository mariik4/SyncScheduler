pub struct EventsState {
    selected_date_events: Vec<Event>,
}

impl EventsState {
    pub fn new() -> Self {
        Self {
            selected_date_events: Vec::new(),
        }
    }

    pub fn refetch_events(&mut self, date: &DayInfo) {
        fetch_events_for_selected_date(date);

        self.selected_date_events = Vec::new()
    }
}

impl Default for EventsState {
    fn default() -> Self {
        Self::new()
    }
}
