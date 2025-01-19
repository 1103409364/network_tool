use std::net::TcpListener;

use crate::common::utils::*;

#[test]
fn test_find_available_port() {
    // 测试正常情况
    let port = find_available_port(9000, 9100).unwrap();
    assert!(port >= 9000 && port <= 9100);
    // 测试端口被占用的情况
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    let next_port = find_available_port(port, port + 1).unwrap();
    let s = next_port;
    println!("{s}");
    assert!(next_port > port);
    drop(listener);
}
