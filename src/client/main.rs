use log::info;
use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

// 添加 Default 以便App::default()来快速创建App实例
#[derive(Default)]
pub struct App {
    running: Arc<std::sync::atomic::AtomicBool>, // 是否运行中 Arc 用于多线程共享 AtomicBool 用于原子操作
                                                 // window: Option<winit::window::Window>, // 不需要窗口，这里这是利用事件循环退出程序
}

impl ApplicationHandler for App {
    // 当应用程序恢复运行时发出此信号。没有默认实现，所以必须实现
    fn resumed(&mut self, _: &ActiveEventLoop) {}
    // 当操作系统向 winit 窗口发送事件时触发。没有默认实现，所以必须实现
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
    // 当事件循环即将阻塞并等待新事件时发出。也可以使用 new_events：当操作系统有新的事件需要处理时，会触发此信号。
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if !self.running.load(std::sync::atomic::Ordering::SeqCst) {
            event_loop.exit(); // 如果程序退出，退出事件循环
        }
    }
    // 当事件循环正在关闭时发出。
    fn exiting(&mut self, _: &ActiveEventLoop) {
        info!("exiting");
    }
}

// 启动客户端程序
pub fn run() {
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

    // 创建事件循环，用于处理系统事件
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(err) => {
            log::error!("创建事件循环失败：{}", err);
            std::process::exit(1);
        }
    };

    let mut app = App {
        running: running.clone(),
    };
    // 运行主事件循环 在此表达式后的代码无法访问，阻塞主线程。事件循环退出后，回到主线程继续执行，直到程序退出
    event_loop.run_app(&mut app).expect("run app error."); // 0.2x 版本的 run 方法已经被废弃，使用 run_app 方法
}
