#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 独自のモジュール これの中身に関しては中身に関してはそんなに気にしなくていい
mod timer;
// use tauri::async_runtime::Mutex;
use tauri::AppHandle;
use tauri::State;
use timer::Timer;
// 非同期処理回りのモジュール
use std::sync::Mutex;
use std::time;
use std::time::Duration;

//まだ実装していない
struct NowTimerLonger(u64);

struct EmitTime(Option<Duration>);

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

#[tauri::command]
fn get_emit_time(emit_time: State<Mutex<EmitTime>>) -> Option<Duration> {
    emit_time.lock().unwrap().0
}

#[tauri::command]
fn chenge_emit_time(emit_time: State<Mutex<EmitTime>>, new_emit_time: Option<Duration>) {
    emit_time.lock().unwrap().0 = new_emit_time;
}

fn main() {
    let app = tauri::Builder::default()
        .manage(Mutex::new(NowTimerLonger(0)))
        .manage(Mutex::new(Timer::new()))
        .manage(Mutex::new(EmitTime(None)))
        .setup(move |app| {
            let app_hundle = app.handle();
            let app_hundle3 = app.handle();

            #[cfg(target_os = "macos")]
            // don't show on the taskbar/springboard
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            //システムトレイ(タスクトレイ)の処理
            SystemTray::new()
                .with_id("main")
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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            remining_time,
            is_runing_timer,
            start_timer,
            stop_timer,
            pause_timer,
            restart_timer,
            chenge_emit_time
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let window = app.get_window("main");

    app.run(move |app_handle, _event| {
        // app_handle.state::<Mutex<Count>>().lock().unwrap().0 += 1;
        // println!("{:?}", app_handle.state::<Mutex<Count>>().lock().unwrap().0);

        match (
            remining_time(app_handle.state()),
            get_emit_time(app_handle.state()),
        ) {
            (Some(x), Some(y)) if x.as_secs() == y.as_secs() => {
                // println!("hogehoge");
                // time_emit_all(&app_handle.clone());
            }
            (None, None) => {}
            (_, _) => {
                println!("pugepuge");
                time_emit_all(&app_handle.clone());
            }
        }

        if remining_time(app_handle.state()) != get_emit_time(app_handle.state()) {
            // time_emit_all(&app_handle.clone());
        }

        // アラームを作動させるべきかの判断をしている。
        let boool: bool;
        {
            boool = match remining_time(app_handle.state()) {
                None => false,
                _ => {
                    remining_time(app_handle.state()) == Some(Duration::from_secs(0))
                        && is_runing_timer(app_handle.state())
                }
            };
        }

        // アラームの作動
        if boool {
            //windowsを最前面にする
            _ = app_handle.get_window("main").unwrap().set_focus();

            pause_timer(app_handle.state());

            let app_handle_f = app_handle.clone();
            std::thread::spawn(move || {
                //音をならす
                // WAVファイルを開く
                let file_path = app_handle_f
                    .path_resolver()
                    .resolve_resource("../assets/sound/marimba.wav")
                    .expect("file not find");
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
                                    println!(
                                        "{:?}",
                                        get_new_time_longer(
                                            app_handle2.state::<Mutex<NowTimerLonger>>(),
                                        )
                                    );
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
                },
            );
        }
    });
}

fn time_emit_all(app_handle: &AppHandle) {
    let tmp_reming_time = remining_time(app_handle.state());

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
        .emit_all("is_runing", is_runing_timer(app_handle.state()))
        .unwrap();
    chenge_emit_time(app_handle.state(), tmp_reming_time);
}
