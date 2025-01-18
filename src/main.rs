#![windows_subsystem = "windows"] // 在 Windows 上禁用控制台窗口
use log::{error, info};
use single_instance::SingleInstance;
mod client; // 导入 mod client 模块。
mod common;
mod server; // 导入 mod server 模块。文件即模块，不需要额外声明 目录下有 mod.rs 文件也是模块
#[cfg(test)]
// 条件编译，只有在测试模式下才编译测试代码，只有当你运行 cargo test 命令时，这个模块才会被编译和执行
mod tests; // 导入测试模块
fn main() {
    // 确保程序只运行一个实例
    let instance = SingleInstance::new("2eHYAHYbarsMt3f").unwrap();
    if !instance.is_single() {
        error!("程序已经在运行中");
        return;
    }
    common::log::config();
    info!("starting...");
    // 启动 Web 服务器
    server::main::run(); // use server::main::run;
    client::main::run(); // 启动客户端程序
}
