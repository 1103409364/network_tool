use crate::server::model::net_status::{InterfaceError, InterfaceInfo, NetworkStatus};
use actix_web::HttpResponse;
use if_addrs::get_if_addrs;
use mac_address::mac_address_by_name;
use std::net::{IpAddr, Ipv4Addr};

/// 返回所有活跃的网络接口信息
/// # 返回值
/// - 成功：返回包含接口信息的 JSON 数组
/// - 失败：返回相应的错误信息
pub async fn get_interfaces() -> Result<HttpResponse, InterfaceError> {
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

use tokio::net::TcpStream;

/// 获取本机网络连接状态
pub async fn get_network_status(target_addr: Option<String>) -> Result<HttpResponse, InterfaceError> {
    let addr = target_addr.unwrap_or_else(|| "www.baidu.com:80".to_string());
    // 尝试连接到指定地址检查网络连通性
    let connected = TcpStream::connect(&addr).await.is_ok();

    let is_connected = connected;

    // 获取当前使用的网络接口信息
    let interfaces = get_if_addrs().map_err(InterfaceError::GetIfAddrsError)?;
    let active_interface = interfaces.into_iter().find(|interface| {
        if let IpAddr::V4(ipv4) = interface.addr.ip() {
            return !interface.is_loopback()
                && ipv4 != Ipv4Addr::new(0, 0, 0, 0)
                && ipv4 != Ipv4Addr::new(127, 0, 0, 1)
                && !ipv4.is_loopback();
        }
        false
    });

    let interface_info = active_interface.and_then(|interface| {
        mac_address_by_name(&interface.name)
            .ok()
            .flatten()
            .map(|mac| InterfaceInfo {
                mac_address: Some(mac.to_string()),
                interface_name: interface.name,
                ip_address: interface.addr.ip().to_string(),
                is_active: true,
            })
    });

    let network_status = NetworkStatus {
        is_connected,
        interface_info,
    };

    Ok(HttpResponse::Ok().json(network_status))
}
