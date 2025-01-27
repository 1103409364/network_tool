use crate::server::controller::net_status::*;
// use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn register_routes(cfg: &mut ServiceConfig) {
    // 默认版本，未分组
    register_network_routes(cfg);
    // API 版本分组
    // cfg.service(web::scope("/api/v1").configure(register_network_routes));
}
// 注册网络状态路由
fn register_network_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_interfaces).service(get_network_status);
}

