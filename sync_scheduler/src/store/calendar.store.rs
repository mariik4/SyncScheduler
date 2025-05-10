pub struct CalendarState {
    pub month: u32,
    pub year: i32,
    pub weeks: Vec<Vec<DayInfo>>,
    pub selected_date: Option<DayInfo>,
    pub tokio_handler: Handle,
}

impl CalendarState {
    pub fn new(tokio_handler: Handle) -> Self {
        let now = chrono::Local::now();
        let current_month = now.month();
        let current_year = now.year();

        let (weeks, today) = match get_month_data(current_year, current_month) {
            Some((w, t)) => (w, t),
            None => {
                eprintln!("Unable to load month data!");
                return CalendarState::default();
            }
        };

        let mut selected_date = None;
        if let Some(mut day) = today {
            day.is_selected = true;
            selected_date = Some(day);
        }

        Self {
            month: current_month,
            year: current_year,
            weeks,
            selected_date,
            tokio_handler,
        }
    }

    pub fn get_tokio_handler(&self) -> Handle {
        self.tokio_handler.clone()
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
        self.update_month_data();
    }

    pub fn previous_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
        self.update_month_data();
    }

    fn update_month_data(&mut self) {
        match get_month_data(self.year, self.month) {
            Some((weeks, today)) => {
                self.weeks = weeks;
                if let Some(mut day) = today {
                    day.is_selected = true;
                    self.selected_date = Some(day);
                }
            }
            None => {
                eprintln!("Failed to update month data");
            }
        }
    }

    pub fn select_date(&mut self, id: &str) {
        let prev_date_id = self.selected_date.as_ref().map(|day| day.id.clone());
        if prev_date_id.as_deref() == Some(id) {
            return;
        }

        for week in self.weeks.iter_mut() {
            for day in week.iter_mut() {
                if day.id == id {
                    day.is_selected = true;
                    self.selected_date = Some(day.clone());
                }
                if let Some(ref prev_id) = prev_date_id {
                    if day.id == *prev_id {
                        day.is_selected = false;
                    }
                }
            }
        }
    }
}

impl Default for CalendarState {
    fn default() -> Self {
        let tokio_rt = tokio::runtime::Runtime::new().unwrap();
        let tokio_handler = tokio_rt.handle().clone();

        Self::new(tokio_handler)
    }
}
