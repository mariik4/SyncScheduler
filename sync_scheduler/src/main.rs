include!("./gui_render/calendar_render.rs");
include!("./gui_render/data_formatter.rs");
include!("./helpers/events.helpers.rs");
include!("./helpers/calendar.helpers.rs");
include!("./store/calendar.store.rs");
include!("./store/store.types.rs");
include!("./db/events.db.rs");
include!("./database/db_functions.rs");

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use dotenv::dotenv;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use tokio::runtime::Handle;

use slint::{Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() {
    dotenv().ok();

    let tokio_rt = tokio::runtime::Runtime::new().unwrap();
    let tokio_handler = tokio_rt.handle().clone();

    let calendar_window = CalendarWindow::new().unwrap();
    calendar_window.window().set_maximized(true);
    // Create multi mutable reference to the calendar state
    // to be able to pass the state to the callbacks
    let calendar_state = Rc::new(RefCell::new(CalendarState::new(tokio_handler)));
    // Set calendar initial state
    {
        // due to the mutable state in the code below I create mutable Ref,
        // but in fact there is nothing to mutate
        let calendar_state = calendar_state.borrow_mut();
        calendar_render(&calendar_window, calendar_state);
    }

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_next_month(move || {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        calendar_state.next_month();
        calendar_render(&window, calendar_state);
    });

    // Setup previous month button callback
    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    // Borrow ownership of all variables to ensure their lifecycle, when buttons are clicked
    // (since it can be out of the scope of the main variable)
    calendar_window.on_previous_month(move || {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        calendar_state.previous_month();
        calendar_render(&window, calendar_state);
    });

    let weak_window = calendar_window.as_weak();
    let calendar_state_ref_clone = Rc::clone(&calendar_state);
    calendar_window.on_select_date(move |day_id| {
        let window = weak_window.unwrap();
        let mut calendar_state = calendar_state_ref_clone.borrow_mut();
        calendar_state.select_date(&day_id);
        calendar_render(&window, calendar_state);
    });

    let calendar_state_clone = Rc::clone(&calendar_state);
    let weak_window = calendar_window.as_weak();
    calendar_window.on_collect_static(
        move |name: SharedString,
              description: SharedString,
              start_date: Date,
              end_date: Date,
              start_time: Time,
              end_time: Time| {
            let window = weak_window.unwrap();
            let handle = {
                let state = calendar_state_clone.borrow_mut();
                state.get_tokio_handler()
            };
            let state_rc = calendar_state_clone.clone();

            let _ = slint::spawn_local(async move {
                let join = handle.spawn(async move {
                    create_new_static_event(
                        name,
                        description,
                        start_date,
                        end_date,
                        start_time,
                        end_time,
                    )
                    .await
                });

                match join.await {
                    Ok(Ok(_)) => println!("Event created successfully"),
                    Ok(Err(e)) => eprintln!("Error creating event: {}", e),
                    Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
                }

                let state = state_rc.borrow_mut();
                calendar_render(&window, state);
            });
        },
    );

    let calendar_state_clone = Rc::clone(&calendar_state);
    let weak_window = calendar_window.as_weak();
    calendar_window.on_create_dynamic_event(
        move |name: SharedString,
              description: SharedString,
              duration: Time,
              priority: i32,
              selected_weekdays: ModelRc<i32>| {
            let window = weak_window.unwrap();
            let handle = {
                let state = calendar_state_clone.borrow_mut();
                state.get_tokio_handler()
            };
            let state_rc = calendar_state_clone.clone();

            let vecmodel: &slint::VecModel<i32> = selected_weekdays
                .as_any()
                .downcast_ref::<slint::VecModel<i32>>()
                .expect("ModelRc is not a VecModel");

            let weekdays: Vec<i32> = vecmodel
                .iter()
                .filter(|&idx| idx >= 0 && idx <= 6) // 6 — индекс воскресенья
                .collect();

            println!("Creating dynamic event");
            println!("Name: {}", name);
            println!("Description: {}", description);
            println!("Duration: {:?}", duration);
            println!("Priority: {}", priority);
            println!("Selected weekdays: {:?}", weekdays);

            // let _ = slint::spawn_local(async move {
            //     let join = handle.spawn(async move {
            //         create_new_dynamic_event(
            //             name,
            //             description,
            //             start_date,
            //             end_date,
            //             start_time,
            //             end_time,
            //         )
            //         .await
            //     });

            //     match join.await {
            //         Ok(Ok(_)) => println!("Event created successfully"),
            //         Ok(Err(e)) => eprintln!("Error creating event: {}", e),
            //         Err(join_e) => eprintln!("Tokio task failed: {}", join_e),
            //     }

            //     let state = state_rc.borrow_mut();
            //     calendar_render(&window, state);
            // });
        },
    );

    calendar_window.run().unwrap();
}
