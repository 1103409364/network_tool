#![windows_subsystem = "windows"] // 在 Windows 上禁用控制台窗口
use log::error; // 使用 log 模块的 error 宏
use single_instance::SingleInstance; // 单实例库
mod client; // 导入 mod client 模块。
mod common; // 导入 mod common 模块。
mod server; // 导入 mod server 模块。文件即模块，不需要额外声明 目录下有 mod.rs 文件也是模块

#[cfg(test)]
mod tests; // 导入测试模块 // 条件编译，只有在测试模式下才编译测试代码，只有当你运行 cargo test 命令时，这个模块才会被编译和执行

fn main() {
    // 确保程序只运行一个实例
    let instance = SingleInstance::new("2eHYAHYbarsMt3f").unwrap();
    if !instance.is_single() {
        error!("The program is already running.");
        return;
    }
    common::log::config(); // 配置日志
    server::main::run(); // // 启动 Web 服务器 use server::main::run;
    client::main::run(); // 启动客户端程序 事件循环阻塞主线程
}
