use actix_web::{get, HttpResponse};
// 引入 server/model/interfaces.rs 中的 InterfaceError
use crate::server::model::net_status::InterfaceError;
use crate::server::service::net_status;

/// 处理 GET /interfaces 请求
#[get("/interfaces")]
pub async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
    // 调用 server/service/net_status.rs 中的 get_interfaces 函数
    net_status::get_interfaces().await
}

/// 处理 GET /network_status 请求
#[get("/network_status")]
pub async fn get_network_status() -> Result<HttpResponse, InterfaceError> {
    // 调用 server/service/net_status.rs 中的 get_network_status 函数
    net_status::get_network_status().await
}
