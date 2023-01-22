#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {

    format!("Hello, {}! You've been greeted from Rust!", name)

}


mod timer;

use std::sync::{Arc, Mutex};

use timer::Timer;


use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent,Manager};



fn main() {



    tauri::Builder::default()
    .setup(|app|{
            let hundle=app.handle();
            let hundle2=app.handle();


            let  use_timer=Arc::new(Mutex::new(Timer::new()));

            let  use_timer_clone=Arc::clone(&use_timer);

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("start","Start"))
                        .add_item(CustomMenuItem::new("stop","Stop"))
                        .add_item(CustomMenuItem::new("menu3","Menu3"))
                        .add_item(CustomMenuItem::new("quit","Quit")),
                )
                .on_event(move|event|{if let SystemTrayEvent::MenuItemClick {  id, .. } =event{
                        if id == "quit"{
                            let tray_handle =hundle.tray_handle_by_id("main").unwrap();
                            tray_handle.destroy().unwrap();
                            hundle.exit(0);
                        }
                        

                        match id.as_str(){
                            "quit"=>{let tray_handle =hundle.tray_handle_by_id("main").unwrap();
                            tray_handle.destroy().unwrap();
                            hundle.exit(0);},
                            "start"=>{
                                use_timer_clone.lock().unwrap().start(100);
                            }
                            "stop"=>{
                                use_timer_clone.lock().unwrap().stop();
                                use_timer_clone.lock().unwrap().remining_time();
                                
                            },
                            _=>(),
                        }
                    }})
                .build(app).unwrap();

            let use_timer_clone2=Arc::clone(&use_timer);

            std::thread::spawn(move || 
                loop {
                
                hundle2
                    .emit_all("k-to-front", "ping frontend".to_string())
                    .unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))

            });
            Ok(())
            
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        
}



