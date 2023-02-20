#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTray};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // let hello = CustomMenuItem::new("hello".to_string(), "Hello");
    // let goodbye = CustomMenuItem::new("goodbye".to_string(), "Goodbye");

    // let submenu = Submenu::new("Menu", Menu::new().add_item(hello).add_item(goodbye));

    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_submenu(submenu);

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        // .menu(menu)
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
