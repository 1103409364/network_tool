use crate::server::model::net_status::InterfaceError;
use std::net::TcpListener;

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
pub fn find_available_port(start: u16, end: u16, max_retries: u32) -> Result<u16, InterfaceError> {
    let mut retries = 0;
    for port in start..=end {
        retries += 1;
        if retries >= max_retries {
            return Err(InterfaceError::MaxRetriesExceeded);
        }
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => return Ok(port),
            Err(_) => continue,
        }
    }
    Err(InterfaceError::NoAvailablePort)
}
