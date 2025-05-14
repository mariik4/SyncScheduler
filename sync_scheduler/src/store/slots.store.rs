#[derive(Clone, Debug)]
pub struct Slot {
    id: String,
    start_time: NaiveTime,
    end_time: NaiveTime,
    date: NaiveDate,
    weekday: String,
}

#[derive(Clone, Debug)]
struct DynamicEventPreData {
    name: String,
    description: String,
    priority: i64,
}

#[derive(Clone, Debug)]
pub enum SlotsStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Clone, Debug)]
pub struct SlotsState {
    pub status: SlotsStatus,
    slots: Vec<Slot>,
    pub error_message: Option<String>,
    event_data: Option<DynamicEventPreData>,
}

impl SlotsState {
    pub fn new() -> Self {
        Self {
            status: SlotsStatus::Success,
            slots: Vec::new(),
            error_message: None,
            event_data: None,
        }
    }

    fn set_pending(&mut self, event_data: DynamicEventPreData) {
        self.status = SlotsStatus::Pending;
        self.slots.clear();
        self.error_message = None;
        self.event_data = Some(event_data);
    }

    fn set_success(&mut self, slots: Vec<Slot>) {
        self.status = SlotsStatus::Success;
        self.slots = slots;
        self.error_message = None;
    }

    fn set_failed(&mut self, msg: impl Into<String>) {
        self.status = SlotsStatus::Failed;
        self.slots.clear();
        self.error_message = Some(msg.into());
    }

    fn reset(&mut self) {
        self.status = SlotsStatus::Success;
        self.slots.clear();
        self.error_message = None;
        self.event_data = None;
    }
}

impl Default for SlotsState {
    fn default() -> Self {
        Self::new()
    }
}
