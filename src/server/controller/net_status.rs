use actix_web::{get, HttpResponse, web};
// 引入 server/model/interfaces.rs 中的 InterfaceError
use crate::server::model::net_status::InterfaceError;
use crate::server::service::net_status;

/// 处理 GET /interfaces 请求
#[get("/interfaces")]
pub async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
    // 调用 server/service/net_status.rs 中的 get_interfaces 函数
    net_status::get_interfaces().await
}

/// 处理 GET /network_status 请求，支持传入 addr 查询参数
#[get("/network_status")]
pub async fn get_network_status(addr: web::Query<std::collections::HashMap<String, String>>) -> Result<HttpResponse, InterfaceError> {
    // 从查询参数中获取 addr 的值
    let target_addr = addr.get("addr").map(|s| s.to_string());
    // 调用 server/service/net_status.rs 中的 get_network_status 函数，并传递 target_addr
    net_status::get_network_status(target_addr).await
}
