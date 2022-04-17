#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

fn main() {
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  static mut APP_HIDDEN : bool = false;
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let menuitem_show_hide = CustomMenuItem::new("show_hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(menuitem_show_hide);

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
        let _appwindow = app.get_window("main").unwrap();
        //window.show().unwrap();
     
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
              let mut local_hidden = false;
              unsafe {
                local_hidden = APP_HIDDEN;
              }

              //window.hide().unwrap();
              if local_hidden == false
              {
                let _h = item_handle.set_title("Show").unwrap();
                window.hide().unwrap();
                local_hidden = true;
              }
              else
              {
                let _h = item_handle.set_title("Hide").unwrap();
                window.show().unwrap();
                local_hidden = false;
              }
              unsafe {
                APP_HIDDEN = local_hidden;
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