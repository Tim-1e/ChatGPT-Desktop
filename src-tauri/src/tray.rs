use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

/// # 构建系统托盘菜单
/// 此函数用于构建并返回一个`SystemTray`对象，该对象描述了系统托盘菜单的配置。
/// 它包含两个菜单项：“Quit”和“Hide”，并在两者之间有一个分隔符。
/// # 返回
/// 返回一个构建完成的`SystemTray`对象。
// 加载菜单
pub fn main_menu() -> SystemTray {
        // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
        let quit = CustomMenuItem::new("quit".to_string(), "Quit");
        let hide = CustomMenuItem::new("hide".to_string(), "Hide");
        let tray_menu = SystemTrayMenu::new()
            .add_item(quit)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}

/// # 系统托盘事件处理函数
/// 此函数用于处理从系统托盘接收到的事件。
/// 它会根据事件的类型执行相应的操作，例如处理菜单项的点击事件。
/// # 参数
/// - `app`: 一个 `AppHandle` 对象的引用，它提供了与应用交互的方法。
/// - `event`: 接收到的 `SystemTrayEvent` 对象，表示系统托盘发生的事件。
// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap(){
                    window.hide().unwrap();
                    item_handle.set_title("Show").unwrap();
                }
                else{
                    window.show().unwrap();
                    item_handle.set_title("Hide").unwrap();
                }
            }
            _ => {}
            }
        }
        _ => {}
    };
}
