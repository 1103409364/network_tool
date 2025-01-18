use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer};
use if_addrs::get_if_addrs;
use log::{error, info, warn};
use mac_address::mac_address_by_name;
use serde::Serialize;
use std::net::TcpListener;
use thiserror::Error;

/// 定义应用程序可能遇到的错误类型
#[derive(Error, Debug)]
pub enum InterfaceError {
    /// 获取网络接口信息失败
    #[error("Failed to get network interfaces: {0}")]
    GetIfAddrsError(#[from] std::io::Error),
    
    /// 获取 MAC 地址失败
    #[error("Failed to get MAC address: {0}")]
    MacAddressError(#[from] mac_address::MacAddressError),
    
    /// 未找到活跃的网络接口
    #[error("No active network interfaces found")]
    NoActiveInterfaces,
    
    /// 未找到可用的端口
    #[error("Failed to find available port")]
    NoAvailablePort,
}

impl actix_web::ResponseError for InterfaceError {}

/// 网络接口信息的数据结构
/// 用于序列化和返回给客户端的接口信息
#[derive(Serialize)]
pub struct InterfaceInfo {
    /// MAC 地址，可能为 None（如果无法获取）
    pub mac_address: Option<String>,
    /// 网络接口名称（如 "eth0", "en0" 等）
    pub interface_name: String,
    /// IP 地址
    pub ip_address: String,
    /// 接口是否活跃
    pub is_active: bool,
}

/// 处理 GET /interfaces 请求
/// 返回所有活跃的网络接口信息
/// 
/// # 返回值
/// - 成功：返回包含接口信息的 JSON 数组
/// - 失败：返回相应的错误信息
#[get("/interfaces")]
async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
    // 获取系统中的所有网络接口
    let interfaces = get_if_addrs().map_err(InterfaceError::GetIfAddrsError)?;

    // 将接口列表转换为 InterfaceInfo 结构的 Vec
    let interface_infos: Vec<InterfaceInfo> = interfaces
        .into_iter()
        // 过滤掉不活跃和本地回环接口
        .filter(|interface| {
            let ip = interface.addr.ip().to_string();
            !interface.is_loopback()  // 过滤掉回环接口
                && ip != "0.0.0.0"    // 过滤掉未配置 IP 的接口
                && ip != "127.0.0.1"  // 过滤掉 IPv4 回环地址
                // && ip != "::1"        // 过滤掉 IPv6 回环地址
                // 过滤掉 IPv6 地址，判断包含冒号的情况
                && !ip.contains(':')
        })
        // 过滤并映射：只保留能获取到 MAC 地址的接口
        .filter_map(|interface| {
            // 尝试获取接口的 MAC 地址
            let mac = match mac_address_by_name(&interface.name) {
                Ok(Some(mac)) => Some(mac.to_string()),
                Ok(None) => None,
                Err(_) => None,
            };

            // 只返回有 MAC 地址的接口信息
            mac.map(|mac_addr| InterfaceInfo {
                mac_address: Some(mac_addr),
                interface_name: interface.name,
                ip_address: interface.addr.ip().to_string(),
                is_active: true,
            })
        })
        .collect();

    // 检查是否找到了活跃的接口
    if interface_infos.is_empty() {
        Err(InterfaceError::NoActiveInterfaces)
    } else {
        Ok(HttpResponse::Ok().json(interface_infos))
    }
}

/// 查找可用的端口
/// 在指定的端口范围内查找第一个可用的端口
/// 
/// # 参数
/// - start: 起始端口号
/// - end: 结束端口号
/// 
/// # 返回值
/// - Ok(port): 找到的可用端口
/// - Err(InterfaceError): 未找到可用端口
pub(crate) fn find_available_port(start: u16, end: u16) -> Result<u16, InterfaceError> { // pub(crate) 表示该项（函数、结构体等）只在当前 crate （包）内可见
    for port in start..end {
        // TcpListener::bind 创建的 TcpListener 对象在离开作用域时会自动被 drop，从而释放占用的端口。因此，我们不需要显式地调用 drop。
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => return Ok(port),
            Err(_) => continue,
        }
    }
    Err(InterfaceError::NoAvailablePort)
}

/// Web 服务器的主入口函数
/// 负责启动 HTTP 服务器并配置所有路由
/// 
/// # 错误处理
/// - 如果指定端口被占用，会尝试使用其他端口
/// - 在 Windows 系统上，如果端口被占用会显示提示框
#[actix_web::main]
async fn start_web_server() -> Result<(), InterfaceError> {
    const START: u16 = 9425;
    let port = find_available_port(START, 9898)?;
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
    info!("Starting at http://127.0.0.1:{}", port);

    HttpServer::new(|| {
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
    .map_err(|e| InterfaceError::GetIfAddrsError(std::io::Error::from(e)))?
    .run()
    .await
    .map_err(|e| InterfaceError::GetIfAddrsError(std::io::Error::from(e)))
}

/// 启动 Web 服务器的公共函数
/// 在新线程中启动服务器，避免阻塞主线程
/// 
/// 如果服务器启动失败，会记录错误信息但不会导致程序崩溃
pub fn launch_web_server() {
    std::thread::spawn(|| {
        if let Err(e) = start_web_server() {
            error!("Failed to start web server: {}", e);
        }
    });
}
