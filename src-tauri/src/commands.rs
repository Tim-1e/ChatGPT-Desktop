use std::process::Command;
use tauri::Manager;

/// # 在文件管理器中展示指定路径
/// 打开系统默认的文件管理器，并将视图定位到指定的路径。
/// 对于Windows系统，会使用`explorer`命令；
/// 对于Linux系统，会使用`xdg-open`或`dbus-send`命令；
/// 对于macOS系统，会使用`open`命令。
/// # 参数
/// * `path` - 要展示的路径
#[tauri::command]
pub async fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs::metadata;
        use std::path::PathBuf;
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:file://{path}").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

/// # 关闭启动画面
/// 在页面加载完成后，关闭启动画面，并显示主窗口。
/// # 参数
/// * `window` - Tauri应用的窗口实例
// 页面加载
#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
    // 关闭启动视图
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // 展示主视图
    window.get_window("main").unwrap().show().unwrap();
}

/// # 获取用户的语言设置
/// 获取用户在操作系统中设置的语言，并以字符串的形式返回。
/// 如果无法获取到用户的语言设置，会返回`"en-US"`。
/// # 返回值
/// 返回一个字符串，表示用户的语言设置。
// 获取当前系统语言
#[tauri::command]
pub fn get_user_language() -> String {
    let current_locale = current_locale::current_locale();

    if current_locale.is_ok() {
        return current_locale.ok().unwrap();
    }

    return "en-US".to_string();
}
