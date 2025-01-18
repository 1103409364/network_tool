#![windows_subsystem = "windows"]

use chrono::Local;
use log::{error, info};
use simplelog::*;
use single_instance::SingleInstance;
use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
use winit::event_loop::EventLoop;

// 添加模块引用
mod web_server;
use web_server::launch_web_server;

fn main() {
    // 创建 log 目录
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        fs::create_dir(log_dir).unwrap();
    }

    // 使用当前日期作为日志文件名
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = log_dir.join(format!("network_tool_{}.log", current_date));

    // 初始化日志系统
    let log_file = File::create(log_file_path).unwrap();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_rfc3339()  // 使用 RFC3339 格式的时间戳
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        log_file,
    )])
    .unwrap();

    // 确保程序单例运行
    let instance = SingleInstance::new("2eHYAHYbarsMt3f").unwrap();
    if !instance.is_single() {
        error!("程序已经在运行中");
        return;
    }

    // 创建托盘菜单
    let tray_menu = Menu::new();

    let quit_item = MenuItem::new("退出", true, None);
    let quit_id = quit_item.id().clone();
    tray_menu.append(&quit_item).unwrap();

    // 创建托盘图标，使用 rgba 格式，减少对 image 库的依赖。png 转 rgba 格式工具 https://convertio.co/zh/png-rgba/
    let icon = Icon::from_rgba(
        include_bytes!("./assets/icon.rgba").to_vec(),
        200,  // 宽度，根据你的实际图片尺寸调整 尺寸和图片不一致时，运行报错
        200   // 高度，根据你的实际图片尺寸调整
    ).expect("无法加载托盘图标");

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .with_tooltip("安全助手")
        .build()
        .unwrap();

    // 保持托盘图标的所有权
    let _tray_icon = Arc::new(tray_icon);

    // 创建事件循环
    let event_loop = EventLoop::new();

    // 启动 web 服务器
    info!("Starting...");
    launch_web_server();

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

    // 运行事件循环
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    });
}
