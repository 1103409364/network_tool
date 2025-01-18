use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use mac_address::mac_address_by_name;
use serde::Serialize;
use if_addrs::get_if_addrs;
use std::net::TcpListener;

// 网络接口信息的数据结构
#[derive(Serialize)]
struct InterfaceInfo {
    // MAC 地址，可能为 None（如果无法获取）
    mac_address: Option<String>,
    // 网络接口名称（如 "eth0", "en0" 等）
    interface_name: String,
    // IP 地址
    ip_address: String,
    // 接口是否活跃
    is_active: bool,
}

// 处理 GET /interfaces 请求
// 返回所有活跃的网络接口信息
#[get("/interfaces")]
async fn get_interfaces() -> impl Responder {
    // 获取系统中的所有网络接口
    match get_if_addrs() {
        Ok(interfaces) => {
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
                    let mac = mac_address_by_name(&interface.name)
                        .ok()           // 将 Result 转换为 Option
                        .flatten()      // 展平嵌套的 Option
                        .map(|mac| mac.to_string());

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
                HttpResponse::NotFound().body("No active network interfaces found")
            } else {
                HttpResponse::Ok().json(interface_infos)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// 查找可用的端口
/// 返回一个可用的端口号，如果没有可用的端口则返回 None
fn find_available_port() -> Option<u16> {
    for port in 8080..9000 {
        // TcpListener::bind 创建的 TcpListener 对象在离开作用域时会自动被 drop，从而释放占用的端口。因此，我们不需要显式地调用 drop。
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => return Some(port),
            Err(_) => continue,
        }
    }
    None
}

// 程序入口点
#[actix_web::main]
async fn get_interfaces() -> std::io::Result<()> {
    let port = find_available_port().expect("No available ports found");
    println!("Starting server at http://127.0.0.1:{}", port);

    // 创建并启动 HTTP 服务器
    HttpServer::new(|| {
        App::new()
            .service(get_interfaces)  // 注册 /interfaces 端点
    })
    .bind(("127.0.0.1", port))?     // 绑定到本地端口
    .run()                          // 运行服务器
    .await
}
