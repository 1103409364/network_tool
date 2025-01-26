use crate::common::utils;
use crate::server::{
    controller::net_status::{get_interfaces, get_network_status},
    model::net_status::InterfaceError,
};
use actix_cors::Cors;
use actix_web::{rt, App, HttpServer};
use log::{error, info, warn};

// 服务器配置常量
const DEFAULT_PORT: u16 = 9425;
const MAX_PORT: u16 = 9898;
const BIND_ADDRESS: &str = "127.0.0.1";
const MAX_RETRIES: u32 = 10;

/// 配置CORS中间件
fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}

/// 在Windows系统上显示端口被占用的提示
#[cfg(target_os = "windows")]
fn show_port_error_dialog() {
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

/// Web 服务器的主入口函数
/// 负责启动 HTTP 服务器并配置所有路由
///
/// # 错误处理
/// - 如果指定端口被占用，会尝试使用其他端口，最多重试10次
/// - 在 Windows 系统上，如果端口被占用会显示提示框
/// - 优雅处理服务器启动和关闭
async fn start_web_server() -> Result<(), InterfaceError> {
    let port = utils::find_available_port(DEFAULT_PORT, MAX_PORT, MAX_RETRIES)?;

    if port != DEFAULT_PORT {
        warn!(
            "Port {} is not available, using port {}",
            DEFAULT_PORT, port
        );
        #[cfg(target_os = "windows")]
        show_port_error_dialog();
    }

    info!("Server starting at http://{}:{}", BIND_ADDRESS, port);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(configure_cors())
            .service(get_interfaces)
            .service(get_network_status)
    })
    .bind((BIND_ADDRESS, port))
    .map_err(|e| InterfaceError::GetIfAddrsError(std::io::Error::from(e)))?
    .shutdown_timeout(30); // 设置优雅关闭超时时间为30秒

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
        let rt = rt::System::new();
        rt.block_on(async {
            if let Err(e) = start_web_server().await {
                error!("Actix-web server error: {}", e);
            }
        });
    })
}
