pub fn format_slint_data(
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
