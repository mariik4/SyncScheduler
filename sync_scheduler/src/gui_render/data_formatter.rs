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
                        is_selected: day.is_selectd,
                        is_today: day.is_today,
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect()
}
