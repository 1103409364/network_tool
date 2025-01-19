use actix_web::{get, web, HttpResponse};
// 引入 server/model/interfaces.rs 中的 InterfaceError
use crate::server::model::net_status::{InterfaceError, InterfaceInfo, NetworkStatus};
use crate::server::service::net_status;
use utoipa::IntoParams;

/// 获取网络接口信息
#[utoipa::path(
    get,
    path = "/interfaces",
    responses(
        (status = 200, description = "获取成功", body = [InterfaceInfo]),
        (status = 500, description = "服务器错误")
    )
)]
#[get("/interfaces")]
pub async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
    // 调用 server/service/net_status.rs 中的 get_interfaces 函数
    net_status::get_interfaces().await
}

#[derive(IntoParams, serde::Deserialize)]
pub struct NetworkStatusParams {
    addr: Option<String>,
}

/// 获取网络连接状态
#[utoipa::path(
    get,
    path = "/network_status",
    params(("addr" = Option<String>, Query, description = "Target network address (optional)")),
    responses(
        (status = 200, description = "获取成功", body = [NetworkStatus]),
        (status = 500, description = "服务器错误")
    )
)]
#[get("/network_status")]
pub async fn get_network_status(
    query: web::Query<NetworkStatusParams>,
) -> Result<HttpResponse, InterfaceError> {
    // 从查询参数中获取 addr 的值
    let target_addr = query.into_inner().addr;
    // 调用 server/service/net_status.rs 中的 get_network_status 函数，并传递 target_addr
    net_status::get_network_status(target_addr).await
}
