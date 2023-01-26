#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod timer;
use timer::Timer;

use std::sync::{Arc, Mutex};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let hundle = app.handle();
            let hundle2 = app.handle();

            let use_timer = Arc::new(Mutex::new(Timer::new()));

            let use_timer_clone = Arc::clone(&use_timer);

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("start", "Start"))
                        .add_item(CustomMenuItem::new("stop", "Stop"))
                        .add_item(CustomMenuItem::new("restert", "Restert"))
                        .add_item(CustomMenuItem::new("quit", "Quit")),
                )
                .on_event(move |event| {
                    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                        match id.as_str() {
                            "start" => {
                                use_timer_clone.lock().unwrap().start(100);
                            }
                            "stop" => {
                                use_timer_clone.lock().unwrap().stop();
                            }
                            "pause" => {
                                use_timer_clone.lock().unwrap().pause();
                            }
                            "restert" => {
                                use_timer_clone.lock().unwrap().restart();
                            }
                            "quit" => {
                                let tray_handle = hundle.tray_handle_by_id("main").unwrap();
                                tray_handle.destroy().unwrap();
                                hundle.exit(0);
                            }
                            _ => (),
                        }
                    }
                })
                .build(app)
                .unwrap();

            let use_timer_clone2 = Arc::clone(&use_timer);
            use_timer_clone2.lock().unwrap().start(1000);

            std::thread::spawn(move || loop {
                let tmp_use_timer_clone2 = use_timer_clone2.lock().unwrap();
                let tmp_reming_time = tmp_use_timer_clone2.remining_time();
                
                hundle2
                    .emit_all(
                        "now-remining-time",
                        match tmp_reming_time {
                            None => 0,
                            _ => tmp_reming_time.unwrap().as_secs()},
                    )
                    .unwrap();
                let remiing_time_millis=match tmp_reming_time {
                            None => 999,
                            _ => tmp_reming_time.unwrap().as_millis(),
                };

                //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない.
                let next_update_time=0;
                
                std::thread::sleep(std::time::Duration::from_millis(next_update_time));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
