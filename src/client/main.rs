use chrono::Local;
use simplelog::*;
use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
use winit::event_loop::EventLoop;

// 启动客户端程序
pub fn run() {
    // 初始化日志系统
    // 创建 log 目录（如果不存在）
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        fs::create_dir(log_dir).unwrap();
    }

    // 使用当前日期作为日志文件名，格式：network_tool_YYYY-MM-DD.log
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = log_dir.join(format!("network_tool_{}.log", current_date));

    // 配置并初始化日志系统
    let log_file = File::create(log_file_path).unwrap();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_rfc3339() // 使用标准的 RFC3339 时间格式
            .set_target_level(LevelFilter::Error) // 设置目标日志级别
            .set_location_level(LevelFilter::Error) // 设置位置信息日志级别
            .build(),
        log_file,
    )])
    .unwrap();

    // 创建系统托盘菜单
    let tray_menu = Menu::new();

    // 添加退出菜单项
    let quit_item = MenuItem::new("退出", true, None);
    let quit_id = quit_item.id().clone(); // 保存退出菜单项的 ID，用于后续事件处理
    tray_menu.append(&quit_item).unwrap();

    // 创建托盘图标
    // 使用 RGBA 格式的图标数据，避免额外的图片处理依赖
    let icon = Icon::from_rgba(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/icon.rgba")).to_vec(),
        200, // 图标宽度
        200, // 图标高度
    )
    .expect("无法加载托盘图标");

    // 构建托盘图标
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .with_tooltip("安全助手") // 鼠标悬停时显示的提示文本
        .build()
        .unwrap();

    // 将托盘图标包装在 Arc 中以在多个线程间共享
    let _tray_icon = Arc::new(tray_icon);

    // 创建事件循环，用于处理系统事件
    let event_loop = EventLoop::new();

    // 处理菜单事件
    let menu_channel = MenuEvent::receiver();
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();

    // 创建新线程处理菜单事件
    std::thread::spawn(move || {
        while let Ok(event) = menu_channel.recv() {
            if event.id == quit_id {
                // 如果点击了退出菜单项
                running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });
    // TODO: server 退出是否需要单独处理？windows 点击退出后整个进程退出，依赖操作系统行为？
    // 运行主事件循环
    event_loop.run(move |_event, _, control_flow| {
        // 设置事件循环为等待模式，减少 CPU 使用
        *control_flow = winit::event_loop::ControlFlow::Wait;

        // 检查是否应该退出程序
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    });
}
