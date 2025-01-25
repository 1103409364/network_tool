use log::error;
use serde::Serialize;
use thiserror::Error;

/// 定义应用程序可能遇到的错误类型
#[derive(Error, Debug)]
#[allow(dead_code)]
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

    /// 达到最大重试次数
    #[error("Maximum port retry attempts exceeded")]
    MaxRetriesExceeded,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("{0}")]
    Unknown(String),
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

/// 本机网络连接状态
#[derive(Serialize)]
pub struct NetworkStatus {
    /// 是否已连接到互联网
    pub is_connected: bool,
    /// 网络延迟 (ms)
    pub latency: Option<u128>,
    /// 当前使用的网络接口信息
    pub interface_infos: Vec<InterfaceInfo>,
}

/// 查询参数的数据结构
#[derive(serde::Deserialize)]
pub struct NetworkStatusParams {
    pub addr: Option<String>,
}
