use crate::server::model::net_status::{InterfaceError, InterfaceInfo, NetworkStatus};
use if_addrs::get_if_addrs;
use mac_address::mac_address_by_name;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpStream;
use tokio::time::Instant;

/// 返回所有活跃的网络接口信息。
///
/// 此函数获取系统中的所有网络接口，并过滤掉不活跃和本地回环接口。
/// 它还会尝试获取每个活跃接口的 MAC 地址。
///
/// # 返回值
///
/// * `Result<Vec<InterfaceInfo>, InterfaceError>`: 包含所有活跃网络接口信息的 `Vec`。
///   - 成功：返回一个 `Vec`，其中包含所有活跃网络接口的信息。
///   - 失败：返回一个 `InterfaceError`，表示获取接口信息时发生的错误。
///     例如 `GetIfAddrsError` 表示获取网络接口信息失败，`MacAddressError` 表示获取 MAC 地址失败。
pub fn get_interface_infos() -> Result<Vec<InterfaceInfo>, InterfaceError> {
    // 获取系统中的所有网络接口
    let interfaces = get_if_addrs().map_err(InterfaceError::GetIfAddrsError)?;
    let interface_infos: Vec<InterfaceInfo> = interfaces
        .into_iter()
        // 过滤掉不活跃和本地回环接口，以及 IPv6 地址
        .filter(|interface| {
            if let IpAddr::V4(ipv4) = interface.addr.ip() {
                return !interface.is_loopback()
                    && ipv4 != Ipv4Addr::new(0, 0, 0, 0)
                    && ipv4 != Ipv4Addr::new(127, 0, 0, 1)
                    && !ipv4.is_loopback();
            }
            false
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
    Ok(interface_infos)
}

/// 获取本机网络连接状态。
///
/// 此函数尝试连接到指定的地址（如果提供），否则连接到 www.baidu.com:80，
/// 以检查网络连通性。它还会尝试获取当前活跃网络接口的信息。
///
/// # 参数
///
/// * `target_addr` (Option<String>): 可选的目标地址，格式为 "host:port"。
///
/// # 返回值
///
/// * `Result<NetworkStatus, InterfaceError>`: 包含本机网络连接状态的 `NetworkStatus`。
///  - 成功：返回一个 `NetworkStatus`，包含是否连接到互联网、网络延迟和当前使用的网络接口信息。
/// - 失败：返回一个 `InterfaceError`，表示获取网络状态时发生的错误。
pub async fn get_network_status(
    target_addr: Option<String>,
) -> Result<NetworkStatus, InterfaceError> {
    let addr = target_addr.unwrap_or_else(|| "www.baidu.com:80".to_string());
    let start = Instant::now();
    let is_connected = TcpStream::connect(&addr).await.is_ok();
    let latency = start.elapsed().as_millis();

    let interface_infos: Vec<InterfaceInfo> = get_interface_infos()?;

    Ok(NetworkStatus {
        is_connected,
        interface_infos,
        latency: Some(latency),
    })
}
