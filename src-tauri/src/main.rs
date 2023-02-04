#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod timer;
use timer::Timer;


use std::sync::{Arc, Mutex};
use tauri::api::notification::Notification;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let hundle = app.handle();
            let hundle2 = app.handle();

            let use_timer = Arc::new(Mutex::new(Timer::new()));

            let use_timer_clone = Arc::clone(&use_timer);

            Notification::new(&app.config().tauri.bundle.identifier)
                .title("New message")
                .body("You've got a new message.")
                .show()
                .unwrap();

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("start_5", "Start 5分"))
                        .add_item(CustomMenuItem::new("start_25", "Start 25分"))
                        .add_item(CustomMenuItem::new("restert", "Restert"))
                        .add_item(CustomMenuItem::new("pause", "Pause"))
                        .add_item(CustomMenuItem::new("quit", "Quit")),
                )
                .on_event(move |event| {
                    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                        match id.as_str() {
                            "start_5" => {
                                use_timer_clone.lock().unwrap().start(300);
                            }
                            "start_25" => {
                                use_timer_clone.lock().unwrap().start(1500);
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

            let use_timer_clone3 = Arc::clone(&use_timer);

            let _hoge =
                app.listen_global("event-name", move |event| match event.payload().unwrap() {
                    "\"start_5\"" => {
                        use_timer_clone3.lock().unwrap().start(300);
                        println!("hogeoge");
                    }
                    "\"start_25\"" => {
                        use_timer_clone3.lock().unwrap().start(1500);
                    }
                    "\"stop\"" => {
                        use_timer_clone3.lock().unwrap().stop();
                    }
                    "\"pause\"" => {
                        use_timer_clone3.lock().unwrap().pause();
                    }
                    "\"restart\"" => {
                        use_timer_clone3.lock().unwrap().restart();
                    }
                    _ => {
                        println!("{:?}", event.payload());
                    }
                });

            let use_timer_clone2 = Arc::clone(&use_timer);
            use_timer_clone2.lock().unwrap().start(1000);

            std::thread::spawn(move || -> ! {
                loop {
                    let next_update_time: u64;
                    {
                        use_timer_clone2.lock().unwrap().update_remining_time();
                        let tmp_use_timer_clone2 = use_timer_clone2.lock().unwrap();
                        let tmp_reming_time = tmp_use_timer_clone2.remining_time();
                        //TODO:関数にしてやりたい.
                        hundle2
                            .emit_all(
                                "now-remining-time",
                                match tmp_reming_time {
                                    None => 0,
                                    _ => tmp_reming_time.unwrap().as_secs(),
                                },
                            )
                            .unwrap();

                        hundle2
                            .emit_all("is_runing", tmp_use_timer_clone2.is_runing)
                            .unwrap();
                        //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない もう作ったかも
                        if tmp_use_timer_clone2.is_runing {
                            next_update_time = tmp_use_timer_clone2.update_time_millis();
                        } else {
                            next_update_time = 500;
                            println!("hogehoge")
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(next_update_time));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("last_line");
}
