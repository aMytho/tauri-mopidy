#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use tauri::{SystemTray, SystemTrayEvent, Manager};

mod tray;
mod title_bar;
mod server_connection;

fn main() {
    // Store the tray menu state
    let tray_menu = tray::regenerate_menu(HashMap::new());

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // Handle menu clicks
                println!("The id is {}", id);
                let window = app.get_window("main").unwrap();
                tray::handle_tray_click(id, window, app);
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Destroyed => {
                // Handle window closure
                event.window().app_handle().tray_handle()
                .set_menu(tray::regenerate_menu(event.window().windows()))
                .unwrap();
                event.window().emit_all(
                    "closed", event.window().label().to_string())
                .unwrap();
                println!("Window was destroyed");
            }
            _ => {
                println!("Window event {:?}", event.event());
            }
          })
        .menu(title_bar::generate_title_menu())
        .invoke_handler(tauri::generate_handler![add_menu_entry, add_connection,
            request_connections, delete_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//Runs when after a user makes a window. Called from the frontnend.
//Function redraws the tray menu
#[tauri::command]
fn add_menu_entry(app: tauri::AppHandle, name: String) {
    println!("Attempting to create {}", name);
    app.tray_handle().set_menu(tray::regenerate_menu(app.windows())).unwrap();
}

#[tauri::command]
fn add_connection(protocol: server_connection::Protocol, address: String, port: String, extension: String, name: String) {
    println!("{}, {:?}, {}, {}", address, protocol, extension, name);

    let conn = server_connection::ServerConnection {
        protocol,
        address,
        port,
        extension,
        name
    };
    server_connection::add_connection(conn);
}

#[tauri::command]
fn request_connections() -> Vec<server_connection::ServerConnection> {
    return server_connection::get_connections();
}

#[tauri::command]
fn delete_connection(name: String) {
    server_connection::delete_connection(name);
}
