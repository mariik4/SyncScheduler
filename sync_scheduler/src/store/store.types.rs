#[derive(Debug, Clone)]
pub struct EventPreview {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DayEventsPreview {
    pub events: Vec<EventPreview>,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct DayInfo {
    pub id: String,
    pub day_number: u32,
    pub full_date: Option<NaiveDate>,
    pub weekday_index: u32,
    pub is_selected: bool,
    pub is_today: bool,
    pub events_preview: Option<DayEventsPreview>,
}
