// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager as _, SystemTray, SystemTrayEvent, WindowBuilder, WindowEvent};
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray = SystemTray::new();

    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    if let Some(tray) = app.get_window("tray") {
                        if tray.is_visible().is_ok_and(|is_visible| is_visible) {
                            let _ = tray.hide();
                        } else {
                            let _ = tray.set_focus();
                        }
                    } else {
                        // build a window
                        let window = WindowBuilder::new(
                            app,
                            "tray",
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .inner_size(400 as f64, 600 as f64)
                        .decorations(false)
                        .focused(true)
                        .always_on_top(true)
                        .build();

                        if let Ok(window) = window {
                            let _ = window.move_window(Position::TrayCenter);

                            let window_handler = window.clone();

                            window.on_window_event(move |event| match event {
                                WindowEvent::Focused(focused) if !focused => {
                                    let _ = window_handler.hide();
                                }
                                _ => {}
                            });
                        }
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error building tauri application");

    // keeps the app out of the dock on mac
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, _event| {});
}
