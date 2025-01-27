// 在 Windows 上禁用控制台窗口
#![windows_subsystem = "windows"]
// 使用 log 模块的 error 宏
use log::{error, info};
// 单实例库
use single_instance::SingleInstance;

mod client;
mod common;
// 导入 mod server 模块。文件即模块，不需要额外声明 目录下有 mod.rs 文件也是模块
mod server;
// 导入测试模块 条件编译，只有在测试模式下才编译测试代码，只有当你运行 cargo test 命令时，这个模块才会被编译和执行
#[cfg(test)]
mod tests;

// Application instance identifier using cargo environment variables
const APP_GUID: &str = concat!(env!("CARGO_PKG_NAME"), "_", env!("CARGO_PKG_VERSION"));

include!(concat!(env!("OUT_DIR"), "/built.rs"));

fn main() {
    // 初始化日志配置
    common::log::config();
    info!("Build Time UTC: {}", BUILT_TIME_UTC);
    info!("Git Version: {}", GIT_VERSION.unwrap_or("unknown"));
    info!("Target: {}", TARGET);
    // info!("APP_GUID: {}", APP_GUID);
    // Ensure only one instance is running
    let instance = match SingleInstance::new(APP_GUID) {
        Ok(instance) => instance,
        Err(err) => {
            error!("Failed to create program instance: {}", err);
            return;
        }
    };

    if !instance.is_single() {
        error!("Program is already running");
        return;
    }

    info!("Program instance started successfully");
    server::main::run();
    client::main::run();
}
