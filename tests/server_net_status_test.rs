use network_tool::server::model::net_status::InterfaceError;
use network_tool::server::service::net_status::*;

#[test]
fn test_interface_error() {
    // 测试错误类型的转换和显示
    let err = InterfaceError::NoActiveInterfaces;
    assert_eq!(err.to_string(), "No active network interfaces found");

    let err = InterfaceError::NoAvailablePort;
    assert_eq!(err.to_string(), "Failed to find available port");
}

#[test]
fn test_interface_error_display() {
    let err = InterfaceError::PermissionDenied;
    assert_eq!(err.to_string(), "Permission denied");

    let err = InterfaceError::Unknown("Unexpected error".to_string());
    assert_eq!(err.to_string(), "Unexpected error");
}

#[test]
fn test_get_interface_infos() {
    let result = get_interface_infos();
    assert!(result.is_ok());
    if let Ok(infos) = result {
        // 至少应该返回一个网络接口信息
        assert!(!infos.is_empty());
        // 检查返回的 InterfaceInfo 中的字段是否被正确填充 (只做基本类型检查)
        for info in infos {
            assert!(!info.interface_name.is_empty());
            assert!(info.mac_address.is_some());
            assert!(!info.ip_address.is_empty());
            assert_eq!(info.is_active, true);
        }
    }
}

#[test]
fn test_get_interface_infos_no_interfaces() {
    // Mock the scenario where no network interfaces are available
    // This requires modifying the get_interface_infos function to allow dependency injection or mocking
    let result = get_interface_infos();
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "No active network interfaces found");
    }
}

#[tokio::test]
async fn test_get_network_status() {
    let result = get_network_status(None).await;
    assert!(result.is_ok());
    if let Ok(status) = result {
        // 检查是否连接到互联网 (这个测试在没有网络连接的情况下可能会失败)
        // 在实际的测试中，可能需要 mock 网络连接
        assert_eq!(status.is_connected, true);
        // 检查延迟是否存在
        assert!(status.latency.is_some());
        // 检查接口信息是否返回
        assert!(!status.interface_infos.is_empty());
    }
}

#[tokio::test]
async fn test_get_network_status_no_connection() {
    // Mock the scenario where there is no internet connection
    // This requires modifying the get_network_status function to allow dependency injection or mocking
    let result = get_network_status(None).await;
    assert!(result.is_ok());
    if let Ok(status) = result {
        assert_eq!(status.is_connected, false);
        assert!(status.latency.is_none());
        assert!(status.interface_infos.is_empty());
    }
}

#[tokio::test]
async fn test_get_network_status_with_latency() {
    // 模拟有延迟的情况
    let result = get_network_status(None).await;
    assert!(result.is_ok());
    if let Ok(status) = result {
        assert_eq!(status.is_connected, true);
        assert!(status.latency.is_some());
        assert!(!status.interface_infos.is_empty());
    }
}

#[tokio::test]
async fn test_get_network_status_with_specific_interface() {
    // Test getting network status for a specific interface
    let result = get_network_status(Some("eth0".to_string())).await;
    assert!(result.is_ok());
    if let Ok(status) = result {
        assert_eq!(status.is_connected, true);
        assert!(status.latency.is_some());
        assert!(!status.interface_infos.is_empty());
        assert!(status
            .interface_infos
            .iter()
            .any(|info| info.interface_name == "eth0"));
    }
}
