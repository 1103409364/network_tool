#![windows_subsystem = "windows"] // 在 Windows 上禁用控制台窗口

use log::{error, info};  // 使用 log 模块的 error 宏
use single_instance::SingleInstance; // 单实例库

mod client;
mod common;
mod server; // 导入 mod server 模块。文件即模块，不需要额外声明 目录下有 mod.rs 文件也是模块

#[cfg(test)]
mod tests; // 导入测试模块 条件编译，只有在测试模式下才编译测试代码，只有当你运行 cargo test 命令时，这个模块才会被编译和执行

// Application instance identifier
const APP_GUID: &str = "2eHYAHYbarsMt3f";

fn main() {
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
    common::log::config(); // 配置日志
    server::main::run();
    client::main::run();
}
