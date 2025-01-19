use crate::common::utils;
use crate::server::{controller::net_status::get_interfaces, model::net_status::InterfaceError};
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::{error, info, warn};

/// Web 服务器的主入口函数
/// 负责启动 HTTP 服务器并配置所有路由
///
/// # 错误处理
/// - 如果指定端口被占用，会尝试使用其他端口
/// - 在 Windows 系统上，如果端口被占用会显示提示框
#[actix_web::main]
async fn start_web_server() -> Result<(), InterfaceError> {
    const START: u16 = 9425;
    let port = utils::find_available_port(START, 9898)?;
    // 判断 port 不等于 START，提示端口被占用
    if port != START {
        warn!("Port {} is not available, using port {}", START, port);
        // windows 系统弹出错误提示框 条件编译
        if cfg!(target_os = "windows") {
            use std::process::Command;
            Command::new("cmd")
                .args(&[
                    "/C",
                    "start",
                    "mshta",
                    "javascript:alert('端口被占用，请重启后重试');close();",
                ])
                .output()
                .expect("failed to execute process");
        }
    }

    info!("Server starting at http://127.0.0.1:{}", port);

    let server = HttpServer::new(|| {
        // 配置 CORS
        let cors = Cors::default()
            .allow_any_origin() // 允许所有来源
            .allow_any_method() // 允许所有 HTTP 方法
            .allow_any_header() // 允许所有请求头
            .max_age(3600); // 预检请求的缓存时间（秒）

        App::new()
            .wrap(cors) // 添加 CORS 中间件
            .service(get_interfaces)
    })
    .bind(("127.0.0.1", port))
    .map_err(|e| InterfaceError::GetIfAddrsError(std::io::Error::from(e)))?;

    let result = server.run().await;

    info!("Web server has stopped");

    result.map_err(|e| InterfaceError::GetIfAddrsError(std::io::Error::from(e)))
}

/// 启动 Web 服务器的公共函数
/// 在新线程中启动服务器，避免阻塞主线程
///
/// 如果服务器启动失败，会记录错误信息但不会导致程序崩溃
pub fn run() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        if let Err(e) = start_web_server() {
            error!("Failed to start web server: {}", e);
        }
    })
}
