pub mod common;
pub mod server;
pub mod client;

// 为什么需要添加 lib.rs：

// 1. 库和二进制分离

//    - main.rs 是二进制入口，只负责程序的运行
//    - lib.rs 是库入口，用于导出可重用的模块
//    - 测试需要以库的形式访问代码，而不是通过二进制

// 2. 模块可见性

//    - lib.rs 通过 pub mod 声明公共模块
//    - 这使得其他 crate（包括测试）可以访问这些模块
//    - 没有 lib.rs，测试就无法通过 crate 名称 network_tool 引用模块

// 3. 测试组织方式

//    - 集成测试（在 tests/ 目录下）被视为独立的 crate
//    - 它们需要像外部代码一样引用主项目
//    - lib.rs 提供了这个引用接口
