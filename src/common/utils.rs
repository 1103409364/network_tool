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
pub fn find_available_port(start: u16, end: u16) -> Result<u16, InterfaceError> {
    // pub(crate) 表示该项（函数、结构体等）只在当前 crate （包）内可见
    for port in start..=end {
        // TcpListener::bind 创建的 TcpListener 对象在离开作用域时会自动被 drop，从而释放占用的端口。因此，我们不需要显式地调用 drop。
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => return Ok(port),
            Err(_) => continue,
        }
    }
    Err(InterfaceError::NoAvailablePort)
}
