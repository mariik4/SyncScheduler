#[derive(Clone, Debug)]
struct Slot {
    id: String,
    start_time: NaiveTime,
    end_time: NaiveTime,
    date: NaiveDate,
    weekday: String,
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
    pub slots: Vec<Slot>,
    pub error_message: Option<String>,
}

impl SlotsState {
    pub fn new() -> Self {
        Self {
            status: SlotsStatus::Success,
            slots: Vec::new(),
            error_message: None,
        }
    }

    pub fn set_pending(&mut self) {
        self.status = SlotsStatus::Pending;
        self.slots.clear();
        self.error_message = None;
    }

    pub fn set_success(&mut self, slots: Vec<Slot>) {
        self.status = SlotsStatus::Success;
        self.slots = slots;
        self.error_message = None;
    }

    pub fn set_failed(&mut self, msg: impl Into<String>) {
        self.status = SlotsStatus::Failed;
        self.slots.clear();
        self.error_message = Some(msg.into());
    }
}

impl Default for SlotsState {
    fn default() -> Self {
        Self::new()
    }
}
