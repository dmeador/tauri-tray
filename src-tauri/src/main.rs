#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

use std::sync::Mutex;
//use tauri::async_runtime::Mutex;
use std::sync::Arc;

fn main() {
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let app_hidden = Arc::new(Mutex::<bool>::new(false));
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let menuitem_show_hide = CustomMenuItem::new("show_hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(menuitem_show_hide);

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event( move |app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
        //let _appwindow = app.get_window("main").unwrap();

      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
      }
      SystemTrayEvent::MenuItemClick { id,  .. } => {
          let item_handle = app.tray_handle().get_item(&id);
          match id.as_str() {
            "quit" => {
              std::process::exit(0);
            }
            "show_hide" => {
              let window = app.get_window("main").unwrap();
              let arc  = Arc::clone(&app_hidden);
              let mut guard = arc.lock().unwrap();
              let local_hidden = &mut *guard;

              if *local_hidden == false
              {
                *local_hidden = true;
                window.hide().unwrap();
                let _h = item_handle.set_title("Show").unwrap();
              }
              else
              {
                *local_hidden = false;
                window.show().unwrap();
                let _h = item_handle.set_title("Hide").unwrap();
              }

            }

            _ => {}
          }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}