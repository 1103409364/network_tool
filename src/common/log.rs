use chrono::Local;
use simplelog::*;
use std::fs::{self, File};
use std::path::Path;

// 初始化日志系统
pub fn config() {
    // 创建 log 目录（如果不存在）
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        fs::create_dir(log_dir).unwrap();
    }

    // 使用当前日期和时间作为日志文件名，格式：network_tool_YYYY-MM-DD_HH-MM-SS.log
    let current_date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_path = log_dir.join(format!("network_tool_{}.log", current_date));

    // 配置并初始化日志系统
    let log_file = File::create(log_file_path).unwrap();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_rfc3339() // 使用标准的 RFC3339 时间格式
            .set_target_level(LevelFilter::Error) // 设置目标日志级别
            .set_location_level(LevelFilter::Error) // 设置位置信息日志级别
            .build(),
        log_file,
    )])
    .unwrap();
}
