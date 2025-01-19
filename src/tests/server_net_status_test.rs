use actix_web::{http::StatusCode, App};

use crate::server::{
    model::net_status::{InterfaceError, InterfaceInfo},
    service::net_status::*
};

#[test]
fn test_interface_error() {
    // 测试错误类型的转换和显示
    let err = InterfaceError::NoActiveInterfaces;
    assert_eq!(err.to_string(), "No active network interfaces found");

    let err = InterfaceError::NoAvailablePort;
    assert_eq!(err.to_string(), "Failed to find available port");
}

#[actix_web::test]
async fn test_get_interfaces() {
    // 测试获取接口信息
    let app = actix_web::test::init_service(App::new().service(
        actix_web::web::resource("/interfaces").route(actix_web::web::get().to(get_interfaces)),
    ))
    .await;

    let req = actix_web::test::TestRequest::get()
        .uri("/interfaces")
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;

    match resp.status() {
        StatusCode::OK => {
            let body = actix_web::test::read_body(resp).await;
            assert!(!body.is_empty());
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            // 如果没有活跃接口的情况
            let error_body = actix_web::test::read_body(resp).await;
            assert!(!error_body.is_empty());
        }
        _ => panic!("意外的状态码: {}", resp.status()),
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

#[actix_web::test]
async fn test_get_network_status_no_addr() {
    // 测试不带 addr 参数获取网络连接状态
    let app = actix_web::test::init_service(
        App::new().service(
            actix_web::web::resource("/network_status")
                .route(actix_web::web::get().to(get_network_status)),
        ),
    )
    .await;

    let req = actix_web::test::TestRequest::get()
        .uri("/network_status")
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_network_status_with_addr() {
    // 测试带 addr 参数获取网络连接状态
    let app = actix_web::test::init_service(
        App::new().service(
            actix_web::web::resource("/network_status")
                .route(actix_web::web::get().to(get_network_status)),
        ),
    )
    .await;

    let req = actix_web::test::TestRequest::get()
        .uri("/network_status?addr=192.168.1.1")
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = actix_web::test::read_body(resp).await;
    assert!(!body.is_empty());
}
