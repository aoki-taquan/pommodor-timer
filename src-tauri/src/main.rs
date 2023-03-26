#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 独自のモジュール これの中身に関しては中身に関してはそんなに気にしなくていい
mod timer;
use timer::Timer;

use std::{sync::Mutex, time::Duration};

struct NowTimerLonger(u64);
struct EmitTime(Option<Duration>);

//使っているミドルウェアのモジュール
use tauri::{api::dialog, AppHandle, Manager, State, SystemTray, SystemTrayEvent};

//音楽再生のモジュール
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

#[tauri::command]
fn get_new_time_longer(now_time_longer: State<Mutex<NowTimerLonger>>) -> u64 {
    now_time_longer.lock().unwrap().0
}

#[tauri::command]
fn chenge_now_time_longer(now_time_longer: State<Mutex<NowTimerLonger>>, new_time_longer: u64) {
    now_time_longer.lock().unwrap().0 = new_time_longer;
}

#[tauri::command]
fn remining_time(timer: State<Mutex<Timer>>) -> Option<Duration> {
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
fn get_emit_time(emit_time: State<Mutex<EmitTime>>) -> Option<Duration> {
    emit_time.lock().unwrap().0
}

#[tauri::command]
fn chenge_emit_time(emit_time: State<Mutex<EmitTime>>, new_emit_time: Option<Duration>) {
    emit_time.lock().unwrap().0 = new_emit_time;
}

fn main() {
    let mut timer = Timer::new();
    timer.start(1500);
    let app = tauri::Builder::default()
        .manage(Mutex::new(NowTimerLonger(1500)))
        .manage(Mutex::new(timer))
        .manage(Mutex::new(EmitTime(None)))
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            // don't show on the taskbar/springboard
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_hundle = app.handle();
            //システムトレイ(タスクトレイ)の処理
            SystemTray::new()
                .with_id("main")
                .on_event(move |event| match event {
                    SystemTrayEvent::LeftClick { position, size, .. } => {
                        let main_window = app_hundle.get_window("main").unwrap();
                        let visible = main_window.is_visible().unwrap();
                        if visible {
                            main_window.hide().unwrap();
                        } else {
                            let window_size = main_window.outer_size().unwrap();
                            let physical_pos = tauri::PhysicalPosition {
                                x: position.x as i32 + (size.width as i32 / 2)
                                    - (window_size.width as i32 / 2),
                                y: position.y as i32 - window_size.height as i32,
                            };

                            let _ =
                                main_window.set_position(tauri::Position::Physical(physical_pos));
                            main_window.show().unwrap();
                            main_window.set_focus().unwrap();
                        }
                    }
                    _ => (),
                })
                .build(app)
                .unwrap();

            let main_window = app.get_window("main").unwrap();
            let main_window_cloen = main_window.clone();
            main_window.on_window_event(move |event| match event {
                tauri::WindowEvent::Focused(false) => {
                    main_window_cloen.hide().unwrap();
                }
                _ => (),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            remining_time,
            is_runing_timer,
            start_timer,
            stop_timer,
            pause_timer,
            restart_timer,
            chenge_now_time_longer
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(move |app_handle, _event| {
        match (
            remining_time(app_handle.state()),
            get_emit_time(app_handle.state()),
        ) {
            (Some(x), Some(y)) if x.as_secs() == y.as_secs() => {}
            (None, None) => {}
            (_, _) => time_emit_all(&app_handle.clone()),
        }

        let reming_time_0 = app_handle
            .state::<Mutex<Timer>>()
            .lock()
            .unwrap()
            .reming_time_0();

        if reming_time_0 && is_runing_timer(app_handle.state()) {
            alarm(&app_handle.clone());
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

fn alarm(app_handle: &AppHandle) {
    let main_window = app_handle.get_window("main").unwrap();
    //windowsを最前面にする
    main_window.set_focus().unwrap();

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
        std::thread::sleep(Duration::from_millis(5000));
    });
    //アラームのダイアログを出す
    //hundle必要？
    let app_handle2 = app_handle.clone();
    dialog::ask(
        Some(&main_window),
        "Pomodoro Timer",
        "続けますか？",
        move |answer| {
            //アラームの返答の処理
            match answer {
                true => {
                    let tmp_now_timer_long =
                        match get_new_time_longer(app_handle2.state::<Mutex<NowTimerLonger>>()) {
                            1500 => 300,
                            300 => 1500,
                            _ => panic!("now_timer_longerの値がおかしいです。"),
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
