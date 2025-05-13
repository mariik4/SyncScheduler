use chrono::Timelike;

pub fn format_slint_date(
    weeks: &[Vec<DayInfo>],
) -> std::vec::Vec<slint::ModelRc<slint_generatedCalendarWindow::SlintDay>> {
    weeks
        .iter()
        .map(|week| {
            ModelRc::new(VecModel::from(
                week.iter()
                    .map(|day| slint_generatedCalendarWindow::SlintDay {
                        day_number: day.day_number as i32,
                        id: day.id.to_string().into(),
                        is_selected: day.is_selected,
                        is_today: day.is_today,
                        events: {
                            let events = if let Some(preview) = day.events_preview.as_ref() {
                                preview
                                    .events
                                    .iter()
                                    .map(|event| slint_generatedCalendarWindow::SlintEventPreview {
                                        id: event.id.to_string().into(),
                                        name: event.name.to_string().into(),
                                    })
                                    .collect::<Vec<_>>()
                            } else {
                                vec![]
                            };
                            ModelRc::new(VecModel::from(events))
                        },
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect()
}

pub fn slint_format_events(
    events: Vec<Event>,
    display_date: NaiveDate,
) -> std::vec::Vec<slint_generatedCalendarWindow::SlintEvent> {
    events
        .iter()
        .map(|event| {
            let start_time = {
                let ev_date = event.start_time.date();
                let ev_time = if ev_date < display_date {
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                } else {
                    event.start_time.time()
                };

                Time {
                    hour: ev_time.hour() as i32,
                    minute: ev_time.minute() as i32,
                    second: ev_time.second() as i32,
                }
            };
            let end_time = {
                let ev_date = event.end_time.date();
                let ev_time = if ev_date > display_date {
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                } else {
                    event.end_time.time()
                };
                Time {
                    hour: ev_time.hour() as i32,
                    minute: ev_time.minute() as i32,
                    second: ev_time.second() as i32,
                }
            };

            slint_generatedCalendarWindow::SlintEvent {
                id: event.id.to_string().into(),
                name: event.name.to_string().into(),
                description: event.description.clone().unwrap_or_default().into(),
                start_time,
                end_time,
                event_type: event.event_type.to_string().into(),
                priority: event.priority as i32,
                postpone: event.postpone as i32,
            }
        })
        .collect()
}

pub fn slint_format_slots(
    slots: &Vec<Slot>,
) -> std::vec::Vec<slint_generatedCalendarWindow::SlintSlot> {
    slots
        .iter()
        .map(|slot| slint_generatedCalendarWindow::SlintSlot {
            id: slot.id.to_string().into(),
            start_time: Time {
                hour: slot.start_time.hour() as i32,
                minute: slot.start_time.minute() as i32,
                second: slot.start_time.second() as i32,
            },
            end_time: Time {
                hour: slot.end_time.hour() as i32,
                minute: slot.end_time.minute() as i32,
                second: slot.end_time.second() as i32,
            },
            date: Date {
                year: slot.date.year() as i32,
                month: slot.date.month() as i32,
                day: slot.date.day() as i32,
            },
            weekday: slot.weekday.clone().into(),
        })
        .collect()
}
