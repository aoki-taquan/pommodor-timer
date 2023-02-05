#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod timer;
use timer::Timer;

use std::sync::{Arc, Mutex};

use tauri::{
    api::dialog, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
fn main() {
    let use_timer = Arc::new(Mutex::new(Timer::new()));

    let use_timer_clone = Arc::clone(&use_timer);
    let use_timer_clone2 = Arc::clone(&use_timer);
    let use_timer_clone3 = Arc::clone(&use_timer);
    let use_timer_clone4 = Arc::clone(&use_timer);
    let use_timer_clone5 = Arc::clone(&use_timer);
    let use_timer_clone6 = Arc::clone(&use_timer);

    let now_timer_long = Arc::new(Mutex::new(1500));

    let now_timer_long_clone = Arc::clone(&now_timer_long);
    let now_timer_long_clone2 = Arc::clone(&now_timer_long);
    let now_timer_long_clone3 = Arc::clone(&now_timer_long);

    let app = tauri::Builder::default()
        .setup(move |app| {
            let hundle = app.handle();

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("start_5", "5 minutes"))
                        .add_item(CustomMenuItem::new("start_25", "25 minutes"))
                        .add_native_item(SystemTrayMenuItem::Separator)
                        .add_item(CustomMenuItem::new("restert", "Restert"))
                        .add_item(CustomMenuItem::new("pause", "Pause"))
                        .add_native_item(SystemTrayMenuItem::Separator)
                        .add_item(CustomMenuItem::new("quit", "Quit")),
                )
                .on_event(move |event| {
                    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                        match id.as_str() {
                            "start_5" => {
                                use_timer_clone.lock().unwrap().start(300);
                                *now_timer_long_clone.lock().unwrap() = 300;
                                // now_timer_long_clone
                            }
                            "start_25" => {
                                use_timer_clone.lock().unwrap().start(1500);
                                *now_timer_long_clone.lock().unwrap() = 1500;
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

            let _hoge =
                app.listen_global("event-name", move |event| match event.payload().unwrap() {
                    "\"start_5\"" => {
                        use_timer_clone3.lock().unwrap().start(300);
                        *now_timer_long_clone3.lock().unwrap() = 300;
                    }
                    "\"start_25\"" => {
                        use_timer_clone3.lock().unwrap().start(1500);
                        *now_timer_long_clone3.lock().unwrap() = 1500;
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
            // {
            use_timer_clone6.lock().unwrap().start(1500);
            // }
            // std::thread::spawn(move || -> ! {
            // loop {
            //     let next_update_time: u64;
            //     {
            //         use_timer_clone2.lock().unwrap().update_remining_time();
            //         let tmp_use_timer_clone2 = use_timer_clone2.lock().unwrap();
            //         let tmp_reming_time = tmp_use_timer_clone2.remining_time();
            //         //TODO:関数にしてやりたい.
            //         hundle2
            //             .emit_all(
            //                 "now-remining-time",
            //                 match tmp_reming_time {
            //                     None => 0,
            //                     _ => tmp_reming_time.unwrap().as_secs(),
            //                 },
            //             )
            //             .unwrap();

            //         hundle2
            //             .emit_all("is_runing", tmp_use_timer_clone2.is_runing)
            //             .unwrap();
            //         //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない もう作ったかも
            //         if tmp_use_timer_clone2.is_runing {
            //             next_update_time = tmp_use_timer_clone2.update_time_millis();
            //         } else {
            //             next_update_time = 500;
            //         }
            //         match tmp_use_timer_clone2.remining_time() {
            //             None => (),
            //             _ => (),
            //         }
            //     }
            //         std::thread::sleep(std::time::Duration::from_millis(next_update_time));
            //     }
            // });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let window = app.get_window("main");

    let n = Arc::new(Mutex::new(0));
    let m = Arc::clone(&n);
    app.run(move |app_handle, _event| {
        let mut l = m.lock().unwrap();
        let boool: bool;
        {
            let tmp_use_timer_clone4 = use_timer_clone4.lock().unwrap();
            boool = match tmp_use_timer_clone4.remining_time() {
                None => false,
                _ => {
                    tmp_use_timer_clone4.remining_time().unwrap().as_secs() == 0
                        && tmp_use_timer_clone4.is_runing
                        && *l == 0
                }
            };
        }
        if boool {
            println!("runからのcall");
            *l = *l + 1;

            let tmp_tmp_use_timer_clone5 = Arc::clone(&use_timer_clone5);
            let o = Arc::clone(&n);
            let tmp_now_timer_long_clone2 = Arc::clone(&now_timer_long_clone2);
            _ = app_handle.get_window("main").unwrap().set_focus();
            dialog::ask(
                Some(&window.as_ref().unwrap()),
                "Pommodor Timer",
                "続けますか？",
                move |answer| {
                    println!("{:?}", answer);

                    match answer {
                        true => {
                            let mut tmp_tmp_now_timer_long_clone2 =
                                tmp_now_timer_long_clone2.lock().unwrap();

                            let tmp3_now_timer_long_clone2;
                            {
                                tmp3_now_timer_long_clone2 = match *tmp_tmp_now_timer_long_clone2 {
                                    1500 => 300,
                                    300 => 1500,
                                    _ => 300,
                                };
                            }
                            *tmp_tmp_now_timer_long_clone2 = tmp3_now_timer_long_clone2;
                            tmp_tmp_use_timer_clone5
                                .lock()
                                .unwrap()
                                .start(*tmp_tmp_now_timer_long_clone2);
                        }
                        false => tmp_tmp_use_timer_clone5.lock().unwrap().stop(),
                    }
                    *o.lock().unwrap() = 0;
                },
            );
        }
        {
            use_timer_clone2.lock().unwrap().update_remining_time();
            let tmp_use_timer_clone2 = use_timer_clone2.lock().unwrap();
            let tmp_reming_time = tmp_use_timer_clone2.remining_time();
            //TODO:関数にしてやりたい.
            app_handle
                .emit_all(
                    "now-remining-time",
                    match tmp_reming_time {
                        None => 0,
                        _ => tmp_reming_time.unwrap().as_secs(),
                    },
                )
                .unwrap();

            app_handle
                .emit_all("is_runing", tmp_use_timer_clone2.is_runing)
                .unwrap();
            //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない もう作ったかも

            match tmp_use_timer_clone2.remining_time() {
                None => (),
                _ => (),
            }
        }
    });
}
