include!("./events/static_events.helpers.rs");
include!("./events/dynamic_events.helpers.rs");
include!("./events/slots.helpers.rs");

use chrono::Duration;
use uuid::Uuid;

async fn refetch_events(date: &NaiveDate, user_id: Uuid) -> Result<Vec<Event>, String> {
    match get_events_in_day(user_id, *date).await {
        Ok(events) => Ok(events),
        Err(err) => Err(format!(
            "Unable to load the data for the selected date: {}",
            err
        )),
    }
}
