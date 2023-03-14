use std::{collections::HashMap};

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, Window, AppHandle, Manager, SystemTraySubmenu};

fn generate_submenu(name: &String) -> SystemTraySubmenu {
    let hide = CustomMenuItem::new(format!("{}hide", name), "Hide");
    let show = CustomMenuItem::new(format!("{}show", name), "Show");
    let close = CustomMenuItem::new(format!("{}clos", name), "Close");
    let menu = SystemTrayMenu::new()
        .add_item(close)
        .add_item(hide)
        .add_item(show);

    return SystemTraySubmenu::new(name, menu);
}

pub fn regenerate_menu(windows: HashMap<String, Window>) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();
    for val in windows.values() {
        let label = val.label().to_string();
        if label != "main" {
            menu = menu.add_submenu(generate_submenu(&label));
            println!("Generating {}", val.label());
        }
    }
    let quit = CustomMenuItem::new("mainquit", "Quit");
    let hide = CustomMenuItem::new("mainhide", "Hide");
    let home = CustomMenuItem::new("mainhome", "Server Select");
    menu = menu
        .add_item(home)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit);

    return menu;
}

pub fn handle_tray_click(id:String, window: Window, app: &AppHandle) {
    match &id[id.len()-4..] {
        "hide" => app.get_window(&remove_action(id)).unwrap().hide().unwrap(),
        "show" => app.get_window(&remove_action(id)).unwrap().show().unwrap(),
        // Shortened to clos because of my lack of rust knowledge
        "clos" => app.get_window(&remove_action(id)).unwrap().close().unwrap(),
        "quit" => window.close().unwrap(),
        "home" => {
            println!("Building new window");
            match app.get_window("main") {
                Some(window) => {
                    if !window.is_visible().unwrap() {
                        window.show().unwrap();
                    }
                    return;
                },
                None => {
                    tauri::WindowBuilder::new(
                        app, id,
                        tauri::WindowUrl::App("index.html".into())
                    ).build().unwrap();
                }
            }
        }
        _ => println!("No match for {}", id.as_str())
      }
}

fn remove_action(id: String) -> String {
    return id[0..id.len() - 4].to_string();
}