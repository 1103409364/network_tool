use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use image::load_from_memory;

fn main() {
    // 创建托盘菜单
    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("退出", true, None);
    tray_menu.append(&quit_item).unwrap();

    // 创建托盘图标
    let icon = load_from_memory(include_bytes!("../assets/icon.png")).unwrap();
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Windows 托盘程序")
        .build()
        .unwrap();

    // 保持托盘图标的所有权
    let tray_icon = Arc::new(tray_icon);

    // 处理菜单事件
    let tray_icon_clone = tray_icon.clone();
    let menu_channel = MenuEvent::receiver();
    std::thread::spawn(move || {
        while let Ok(event) = menu_channel.recv() {
            if event.id == quit_item.id() {
                // 退出程序
                std::process::exit(0);
            }
        }
    });

    // 保持程序运行
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
