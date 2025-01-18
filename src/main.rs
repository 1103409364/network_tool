use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
use image::io::Reader as ImageReader;

fn main() {
    // 创建托盘菜单
    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("退出", true, None);
    let quit_id = quit_item.id().clone();
    tray_menu.append(&quit_item).unwrap();

    // 创建托盘图标
    let icon_data = include_bytes!("../assets/icon.png");
    let image = ImageReader::new(std::io::Cursor::new(icon_data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let rgba = image.into_rgba8();
    let icon = Icon::from_rgba(
        rgba.as_raw().to_vec(),
        rgba.width(),
        rgba.height()
    ).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .with_tooltip("安全中心")
        .build()
        .unwrap();

    // 保持托盘图标的所有权
    let tray_icon = Arc::new(tray_icon);

    // 处理菜单事件
    let menu_channel = MenuEvent::receiver();
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();

    std::thread::spawn(move || {
        while let Ok(event) = menu_channel.recv() {
            if event.id == quit_id {
                running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });

    // 保持程序运行
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
