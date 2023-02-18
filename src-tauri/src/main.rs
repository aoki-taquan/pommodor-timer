#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 独自のモジュール これの中身に関しては中身に関してはそんなに気にしなくていい
mod timer;
use timer::Timer;

// 非同期処理回りのモジュール
use std::sync::{Arc, Mutex};

//使っているミドルウェアのモジュール
use tauri::{
    api::dialog, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

//音系

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let use_timer = Arc::new(Mutex::new(Timer::new()));

    //非同期で必要な処理
    let use_timer_to_sys_tray = Arc::clone(&use_timer);
    let use_timer_to_listen_global = Arc::clone(&use_timer);
    let use_timer_to_do_ask = Arc::clone(&use_timer);
    let use_timer_to_ask = Arc::clone(&use_timer);
    let use_timer_to_emit = Arc::clone(&use_timer);
    let use_timer_to_start = Arc::clone(&use_timer);

    let now_timer_long = Arc::new(Mutex::new(1500));

    //非同期で必要な処理
    let now_timer_long_to_sys_tray = Arc::clone(&now_timer_long);
    let now_timer_long_to_ask = Arc::clone(&now_timer_long);
    let now_timer_long_to_emit = Arc::clone(&now_timer_long);

    let do_alarm_work = Arc::new(Mutex::new(false));

    //非同期で必要な処理
    let do_alarm_work_to_ask = Arc::clone(&do_alarm_work);
    let do_alarm_work_to_emit = Arc::clone(&do_alarm_work);

    let app = tauri::Builder::default()
        .setup(move |app| {
            let app_hundle = app.handle();
            let app_hundle2 = app.handle();

            //システムトレイ(タスクトレイ)の処理
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
                                use_timer_to_sys_tray.lock().unwrap().start(10);
                                *now_timer_long_to_sys_tray.lock().unwrap() = 300;
                            }
                            "start_25" => {
                                use_timer_to_sys_tray.lock().unwrap().start(1500);
                                *now_timer_long_to_sys_tray.lock().unwrap() = 1500;
                            }
                            "stop" => {
                                use_timer_to_sys_tray.lock().unwrap().stop();
                            }
                            "pause" => {
                                use_timer_to_sys_tray.lock().unwrap().pause();
                            }
                            "restert" => {
                                use_timer_to_sys_tray.lock().unwrap().restart();
                            }
                            "quit" => {
                                //アプリの終了
                                let tray_handle = app_hundle.tray_handle_by_id("main").unwrap();
                                tray_handle.destroy().unwrap();
                                app_hundle.exit(0);
                            }
                            _ => (),
                        }
                    }
                })
                .build(app)
                .unwrap();
            //WebViewからの情報を受け取りそれの処理をする。
            app.listen_global("event-name", move |event| match event.payload().unwrap() {
                "\"start_5\"" => {
                    use_timer_to_listen_global.lock().unwrap().start(300);
                    *now_timer_long_to_emit.lock().unwrap() = 300;
                }
                "\"start_25\"" => {
                    use_timer_to_listen_global.lock().unwrap().start(1500);
                    *now_timer_long_to_emit.lock().unwrap() = 1500;
                }
                "\"stop\"" => {
                    use_timer_to_listen_global.lock().unwrap().stop();
                }
                "\"pause\"" => {
                    use_timer_to_listen_global.lock().unwrap().pause();
                }
                "\"restart\"" => {
                    use_timer_to_listen_global.lock().unwrap().restart();
                }
                _ => {
                    println!("{:?}", event.payload());
                }
            });

            // WebViewへの情報の送信
            //別なスレッドで処理して、定期的に実行させている
            std::thread::spawn(move || -> ! {
                loop {
                    let next_update_time: u64;
                    {
                        //アラームが出ている時に実行するとバグるから
                        if !*do_alarm_work_to_emit.lock().unwrap() {
                            {
                                use_timer_to_emit.lock().unwrap().update_remining_time();
                                let tmp_use_timer_to_emit = use_timer_to_emit.lock().unwrap();
                                let tmp_reming_time = tmp_use_timer_to_emit.remining_time();
                                //TODO:関数にしてやりたい.
                                app_hundle2
                                    .emit_all(
                                        "now-remining-time",
                                        match tmp_reming_time {
                                            None => 0,
                                            _ => tmp_reming_time.unwrap().as_secs(),
                                        },
                                    )
                                    .unwrap();

                                app_hundle2
                                    .emit_all("is_runing", tmp_use_timer_to_emit.is_runing)
                                    .unwrap();
                                //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない もう作ったかも
                                if tmp_use_timer_to_emit.is_runing {
                                    next_update_time = tmp_use_timer_to_emit.update_time_millis();
                                } else {
                                    next_update_time = 500;
                                }
                                match tmp_use_timer_to_emit.remining_time() {
                                    None => (),
                                    _ => (),
                                }
                            }
                        } else {
                            next_update_time = 500;
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_millis(next_update_time));
                }
            });

            //最初の値の設定
            {
                let mut tmp_use_timer_to_start = use_timer_to_start.lock().unwrap();
                tmp_use_timer_to_start.start(1500);
                tmp_use_timer_to_start.pause();
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let window = app.get_window("main");

    app.run(move |app_handle, _event| {
        let mut tmp_do_alarm_work_to_ask = do_alarm_work_to_ask.lock().unwrap();

        // アラームを作動させるべきかの判断をしている。
        let boool: bool;
        {
            let tmp_use_timer_to_do_ask = use_timer_to_do_ask.lock().unwrap();
            boool = match tmp_use_timer_to_do_ask.remining_time() {
                None => false,
                _ => {
                    tmp_use_timer_to_do_ask.remining_time().unwrap().as_secs() == 0
                        && tmp_use_timer_to_do_ask.is_runing
                        && *tmp_do_alarm_work_to_ask == false
                }
            };
        }

        // アラームの作動
        if boool {
            println!("runからのcall");
            *tmp_do_alarm_work_to_ask = true;

            //windowsを最前面にする
            _ = app_handle.get_window("main").unwrap().set_focus();

            //アラーム用の非同期処理で必要な部分
            let tmp_tmp_use_timer_to_ask = Arc::clone(&use_timer_to_ask);
            let do_alarm_work_ask_f = Arc::clone(&do_alarm_work);
            let tmp_now_timer_long_to_ask = Arc::clone(&now_timer_long_to_ask);
            std::thread::spawn(|| {
                //音をならす
                // WAVファイルを開く
                let file = File::open("sound/marimba.wav").unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();

                // 出力ストリームを作成する
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();

                // ソースをシンクに追加し、再生する
                sink.append(source);
                sink.play();
                // 再生が終了するまで待機する
                std::thread::sleep(std::time::Duration::from_millis(5000));
            });
            //音をならす
            // WAVファイルを開く

            dialog::ask(
                Some(&window.as_ref().unwrap()),
                "Pommodor Timer",
                "続けますか？",
                move |answer| {
                    //アラームの返答の処理
                    println!("{:?}", answer);

                    match answer {
                        true => {
                            let mut tmp_tmp_now_timer_long_to_ask =
                                tmp_now_timer_long_to_ask.lock().unwrap();

                            let tmp3_now_timer_long_to_ask;
                            {
                                tmp3_now_timer_long_to_ask = match *tmp_tmp_now_timer_long_to_ask {
                                    1500 => 300,
                                    300 => 1500,
                                    _ => 300,
                                };
                            }
                            *tmp_tmp_now_timer_long_to_ask = tmp3_now_timer_long_to_ask;
                            tmp_tmp_use_timer_to_ask
                                .lock()
                                .unwrap()
                                .start(*tmp_tmp_now_timer_long_to_ask);
                        }
                        false => tmp_tmp_use_timer_to_ask.lock().unwrap().stop(),
                    }
                    *do_alarm_work_ask_f.lock().unwrap() = false;
                },
            );
        }
    });
}
