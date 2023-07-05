// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use std::{sync::Arc, process::Child};
use tokio::sync::Mutex;
use voiceInput::VoiceInput;

#[cfg(target_os = "windows")]
use window_shadows::set_shadow;

mod commands;
mod tray;
mod voiceInput;
mod read_file;
mod stt;
use stt::IatRecorder;
mod back_end;

/// 主函数
/// 初始化Tauri应用，并注册相关的命令处理函数和事件处理函数。
/// 如果在运行Tauri应用的过程中出现错误，会打印错误消息并退出。
fn main() {    
    // 创建一个VoiceInput实例，并使用Arc和Mutex将其包装起来，
    // 以便在多个线程间共享和修改。是我们音频接收重要方法
    let voice_input_state = Arc::new(Mutex::new(None::<VoiceInput>));

    // let mut child_process: Option<Child> = None;
    // child_process=Some(back_end::start_backend().await);

    // 创建Tauri应用
    tauri::Builder::default()
        .manage(voice_input_state)
        .setup(move |_app| {
            #[cfg(target_os = "macos")]
            _app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            #[cfg(target_os = "windows")]
            set_shadow(&_app.get_window("main").unwrap(), true).expect("Unsupported platform!");

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_window("main").unwrap();
                //window.open_devtools();
            }
            Ok(())
        })
        .system_tray(tray::main_menu())
        .on_system_tray_event(tray::handler)
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.unminimize().unwrap();
            window.set_focus().unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            commands::show_in_folder,
            commands::close_splashscreen,
            commands::get_user_language,
            voiceInput::init_voice_input,
            voiceInput::send_voice_input,
            voiceInput::recv_voice_input,
            read_file::open_document
        ])
        .on_system_tray_event(tray::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
