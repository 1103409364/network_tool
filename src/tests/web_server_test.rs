use crate::web_server::*;
use std::net::TcpListener;

#[test]
fn test_find_available_port() {
    // 测试正常情况
    let port = find_available_port(9000, 9100).unwrap();
    assert!(port >= 9000 && port <= 9100);

    // 测试端口被占用的情况
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    let next_port = find_available_port(port, port + 1).unwrap();
    assert!(next_port > port);
    drop(listener);
}

#[test]
fn test_interface_error() {
    // 测试错误类型的转换和显示
    let err = InterfaceError::NoActiveInterfaces;
    assert_eq!(
        err.to_string(),
        "No active network interfaces found"
    );

    let err = InterfaceError::NoAvailablePort;
    assert_eq!(
        err.to_string(),
        "Failed to find available port"
    );
}

#[actix_web::test]
async fn test_get_interfaces() {
    // 测试获取接口信息
    let resp = get_interfaces().await;
    
    match resp {
        Ok(response) => {
            let body = response.into_body();
            assert!(body.size() > 0);
        },
        Err(e) => {
            // 如果没有活跃接口，应该返回 NoActiveInterfaces 错误
            match e {
                InterfaceError::NoActiveInterfaces => (),
                _ => panic!("Unexpected error: {}", e),
            }
        }
    }
}

#[test]
fn test_interface_info_serialization() {
    // 测试 InterfaceInfo 结构体的序列化
    let info = InterfaceInfo {
        mac_address: Some("00:11:22:33:44:55".to_string()),
        interface_name: "test0".to_string(),
        ip_address: "192.168.1.1".to_string(),
        is_active: true,
    };

    let serialized = serde_json::to_string(&info).unwrap();
    assert!(serialized.contains("00:11:22:33:44:55"));
    assert!(serialized.contains("test0"));
    assert!(serialized.contains("192.168.1.1"));
    assert!(serialized.contains("true"));
} 