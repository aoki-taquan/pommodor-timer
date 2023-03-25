#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 独自のモジュール これの中身に関しては中身に関してはそんなに気にしなくていい
mod timer;
// use tauri::async_runtime::Mutex;
use tauri::State;
use timer::Timer;

// 非同期処理回りのモジュール
use std::sync::{Arc, Mutex};

//まだ実装していない
struct NowTimerLonger(u64);

//使っているミドルウェアのモジュール
use tauri::{
    api::dialog, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

//音楽を再生する

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

#[tauri::command]
fn get_new_time_longer(now_time_longer: State<Mutex<NowTimerLonger>>) -> u64 {
    now_time_longer.lock().unwrap().0
}

#[tauri::command]
fn chenge_now_time_longer(now_time_longer: State<Mutex<NowTimerLonger>>, new_time_longer: u64) {
    now_time_longer.lock().unwrap().0 = new_time_longer;
}

#[tauri::command]
fn remining_time(timer: State<Mutex<Timer>>) -> Option<std::time::Duration> {
    timer.lock().unwrap().remining_time()
}

#[tauri::command]
fn start_timer(timer: State<Mutex<Timer>>, set_time_second: u64) {
    timer.lock().unwrap().start(set_time_second);
}

#[tauri::command]
fn stop_timer(timer: State<Mutex<Timer>>) {
    timer.lock().unwrap().stop();
}

#[tauri::command]
fn pause_timer(timer: State<Mutex<Timer>>) {
    timer.lock().unwrap().pause();
}

#[tauri::command]
fn restart_timer(timer: State<Mutex<Timer>>) {
    timer.lock().unwrap().restart();
}

#[tauri::command]
fn is_runing_timer(timer: State<Mutex<Timer>>) -> bool {
    timer.lock().unwrap().is_runing
}

#[tauri::command]
fn update_time_millisecond(timer: State<Mutex<Timer>>) -> u64 {
    timer.lock().unwrap().update_time_millis()
}

fn main() {
    let do_alarm_work = Arc::new(Mutex::new(false));

    //非同期で必要な処理
    let do_alarm_work_to_ask = Arc::clone(&do_alarm_work);
    let do_alarm_work_to_emit = Arc::clone(&do_alarm_work);

    let app = tauri::Builder::default()
        .manage(Mutex::new(NowTimerLonger(1500)))
        .manage(Mutex::new(Timer::new()))
        .setup(move |app| {
            let app_hundle = app.handle();
            let app_hundle2 = app.handle();
            let app_hundle3 = app.handle();

            #[cfg(target_os = "macos")]
            // don't show on the taskbar/springboard
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

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
                                start_timer(app_hundle.state(), 300);
                                chenge_now_time_longer(app_hundle.state(), 300);
                            }
                            "start_25" => {
                                start_timer(app_hundle.state(), 1500);
                                chenge_now_time_longer(app_hundle.state(), 1500);
                            }
                            "stop" => {
                                stop_timer(app_hundle.state());
                            }
                            "pause" => {
                                pause_timer(app_hundle.state());
                            }
                            "restert" => {
                                restart_timer(app_hundle.state());
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
                    start_timer(app_hundle3.state(), 300);
                    chenge_now_time_longer(app_hundle3.state(), 300);
                }
                "\"start_25\"" => {
                    start_timer(app_hundle3.state(), 1500);
                    chenge_now_time_longer(app_hundle3.state(), 1500);
                }
                "\"stop\"" => {
                    stop_timer(app_hundle3.state());
                }
                "\"pause\"" => {
                    pause_timer(app_hundle3.state());
                }
                "\"restart\"" => {
                    restart_timer(app_hundle3.state());
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
                                let tmp_reming_time = remining_time(app_hundle2.state());
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
                                    .emit_all("is_runing", is_runing_timer(app_hundle2.state()))
                                    .unwrap();
                                //TODO:timer.rs側に実装した関数を呼び足すようにする.ただまだ作っていない もう作ったかも
                                if is_runing_timer(app_hundle2.state()) {
                                    next_update_time = update_time_millisecond(app_hundle2.state());
                                } else {
                                    next_update_time = 500;
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
                start_timer(app.state(), 1500);
                chenge_now_time_longer(app.state(), 1500);
                pause_timer(app.state());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let window = app.get_window("main");

    app.run(move |app_handle, _event| {
        let mut tmp_do_alarm_work_to_ask = do_alarm_work_to_ask.lock().unwrap();

        let file_path = app_handle
            .path_resolver()
            .resolve_resource("../assets/sound/marimba.wav")
            .expect("file not find");

        // アラームを作動させるべきかの判断をしている。
        let boool: bool;
        {
            boool = match remining_time(app_handle.state()) {
                None => false,
                _ => {
                    remining_time(app_handle.state()).unwrap().as_secs() == 0
                        && is_runing_timer(app_handle.state())
                        && *tmp_do_alarm_work_to_ask == false
                }
            };
        }

        // アラームの作動
        if boool {
            *tmp_do_alarm_work_to_ask = true;

            //windowsを最前面にする
            _ = app_handle.get_window("main").unwrap().set_focus();

            //アラーム用の非同期処理で必要な部分
            //TODO:こんなもん必要ない様にしたい

            let do_alarm_work_ask_f = Arc::clone(&do_alarm_work);

            std::thread::spawn(move || {
                //音をならす
                // WAVファイルを開く
                let file = File::open(&file_path).unwrap();
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
            //アラームのダイアログを出す
            //hundle必要？
            let app_handle2 = app_handle.clone();
            dialog::ask(
                Some(&window.as_ref().unwrap()),
                "Pomodoro Timer",
                "続けますか？",
                move |answer| {
                    //アラームの返答の処理
                    match answer {
                        true => {
                            let tmp_now_timer_long = match get_new_time_longer(
                                app_handle2.state::<Mutex<NowTimerLonger>>(),
                            ) {
                                1500 => 300,
                                300 => 1500,
                                _ => {
                                    panic!("now_timer_longerの値がおかしいです。")
                                }
                            };

                            chenge_now_time_longer(
                                app_handle2.state::<Mutex<NowTimerLonger>>(),
                                tmp_now_timer_long,
                            );
                            start_timer(app_handle2.state::<Mutex<Timer>>(), tmp_now_timer_long);
                        }

                        false => stop_timer(app_handle2.state::<Mutex<Timer>>()),
                    }
                    *do_alarm_work_ask_f.lock().unwrap() = false;
                },
            );
        }
    });
}
