use actix_web::{get, web, HttpResponse};
// 引入 server/model/interfaces.rs 中的 InterfaceError
use crate::server::model::net_status::{InterfaceError, InterfaceInfo, NetworkStatusParams};
use crate::server::service::net_status;

/// 处理 GET /interfaces 请求
#[get("/interfaces")]
pub async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
    let interface_infos: Vec<InterfaceInfo> = net_status::get_interface_infos()?;
    // 检查是否找到了活跃的接口
    if interface_infos.is_empty() {
        Err(InterfaceError::NoActiveInterfaces)
    } else {
        Ok(HttpResponse::Ok().json(interface_infos))
    }
}

/// 获取网络连接状态
#[get("/network_status")]
pub async fn get_network_status(
    query: web::Query<NetworkStatusParams>,
) -> Result<HttpResponse, InterfaceError> {
    // 从查询参数中获取 addr 的值
    let target_addr = query.into_inner().addr;
    // 调用 server/service/net_status.rs 中的 get_network_status 函数
    let network_status = net_status::get_network_status(target_addr).await?; // ? 用于传播错误：如果 get_network_status 返回错误，则立即返回该错误
    Ok(HttpResponse::Ok().json(network_status))
}
