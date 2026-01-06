use futures::StreamExt;
use iced::{Event, Subscription, Task, window};
use std::path::Path;
use tokio_stream::wrappers::ReceiverStream;

use common_debug::debug_dev;
use status::setup_status_emitter;
use status::status_event::StatusEvent;

use crate::app_state::{AppMessage, AppState};
use crate::app_status::StatusMessage;
use crate::app_task::save_bom_logs_async;
use crate::app_task::scan_app_async;
use crate::app_task::set_input_path;
use crate::app_task::set_output_path;
use crate::app_task::trash_app_async;
use crate::app_task::{add_app, open_loc_async};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::DropFile(path) => {
            state.reset();

            let (reporter, rx) = setup_status_emitter(10);

            state.input_file = path.clone();
            let add_app =
                Task::perform(
                    add_app(state.input_file.clone(), Some(reporter)),
                    |res| match res {
                        Ok(cleaner) => AppMessage::ScanApp(Ok(cleaner)),
                        Err(err) => {
                            let event = StatusEvent::new()
                                .with_stage("Failed")
                                .with_message(err.to_string());
                            AppMessage::Status(StatusMessage::Event(event))
                        }
                    },
                );
            let rx_stream = ReceiverStream::new(rx);

            let status_task = Task::run(
                rx_stream.map(|event| AppMessage::Status(StatusMessage::Event(event))),
                |msg| msg,
            );

            Task::batch(vec![add_app, status_task])
        }

        AppMessage::OpenFile => {
            state.reset();

            Task::perform(set_input_path(), |res| match res {
                Ok(path) => AppMessage::DropFile(path.to_path_buf()),
                Err(e) => {
                    let event = StatusEvent::new()
                        .with_stage("Failed")
                        .with_message(e.to_string());
                    AppMessage::Status(StatusMessage::Event(event))
                }
            })
        }

        AppMessage::ScanApp(cleaner) => {
            if let Ok(app_input) = cleaner {
                let (reporter, rx) = setup_status_emitter(10);

                let scan_task =
                    Task::perform(scan_app_async(app_input, Some(reporter)), |res| match res {
                        Ok(cleaner) => AppMessage::UpdateCleaner(cleaner),
                        Err(err) => {
                            let event = StatusEvent::new()
                                .with_stage("Failed")
                                .with_message(err.to_string());
                            AppMessage::Status(StatusMessage::Event(event))
                        }
                    });

                let rx_stream = ReceiverStream::new(rx);

                let progress_task = Task::run(
                    rx_stream.map(|event| AppMessage::Status(StatusMessage::Event(event))),
                    |msg| msg,
                );

                return Task::batch(vec![scan_task, progress_task]);
            }
            Task::none()
        }

        AppMessage::UpdateCleaner(cleaner) => {
            state.cleaner = cleaner;
            let founded = state.cleaner.app_data.all_found_entries().len();
            let event = StatusEvent::new()
                .with_stage("Completed:")
                .with_message(format!("{} item founded", founded));
            Task::done(AppMessage::Status(StatusMessage::Event(event)))
        }

        AppMessage::OpenSelectedPath(index) => {
            state.selected_file = Some(index);
            debug_dev!("Clicked path: {:?}", index);

            let entries = state.cleaner.app_data.all_found_entries();

            if let Some((_i, (path, _label))) = entries.get(index) {
                let path = path.clone();
                return Task::perform(open_loc_async(path), |_| AppMessage::NoOperations);
            }
            Task::none()
        }

        AppMessage::BrowseOutput => Task::perform(set_output_path(), |res| match res {
            Ok(path) => AppMessage::OutputFile(Ok(path)),
            Err(e) => {
                let event = StatusEvent::new().with_message(e.to_string());
                AppMessage::Status(StatusMessage::Event(event))
            }
        }),

        AppMessage::OutputFile(result) => {
            match result {
                Ok(path) => {
                    state.output_file = (*path).clone();
                    state.status.message = Some("folder selected".to_string());
                }
                Err(e) => {
                    state.status.message = Some(e.to_string());
                }
            }
            Task::none()
        }

        AppMessage::ExportFile => {
            let output_dir = if !state.output_file.as_os_str().is_empty() {
                state.output_file.clone()
            } else {
                let home = std::env::var("HOME").unwrap();
                Path::new(&home).join("Desktop")
            };
            let cleaner = state.cleaner.clone();
            Task::perform(save_bom_logs_async(cleaner, output_dir), |res| match res {
                Ok(()) => {
                    let event = StatusEvent::new()
                        .with_stage("Success")
                        .with_message("Bom file saved".to_string());
                    AppMessage::Status(StatusMessage::Event(event))
                }
                Err(err) => {
                    let event = StatusEvent::new()
                        .with_stage("Failed")
                        .with_message(err.to_string());
                    AppMessage::Status(StatusMessage::Event(event))
                }
            })
        }

        AppMessage::TrashApp => {
            let cleaner = state.cleaner.clone();
            Task::perform(trash_app_async(cleaner), |res| match res {
                Ok(()) => AppMessage::DeletedApp,
                Err(err) => {
                    let event = StatusEvent::new()
                        .with_stage("Failed")
                        .with_message(err.to_string());
                    AppMessage::Status(StatusMessage::Event(event))
                }
            })
        }

        AppMessage::DeletedApp => {
            state.reset();
            state.status.message = Some("App moved to Trash".to_string());
            Task::none()
        }

        AppMessage::ClearList => {
            state.reset();
            Task::none()
        }

        AppMessage::Status(msg) => state.status.update(msg).map(AppMessage::Status),

        AppMessage::NoOperations => Task::none(),
    }
}

pub fn subscription(_state: &AppState) -> Subscription<AppMessage> {
    iced::event::listen().map(|event| match event {
        Event::Window(window::Event::FileDropped(path)) => AppMessage::DropFile(path),
        _ => AppMessage::NoOperations,
    })
}
