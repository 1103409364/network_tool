# network_tool

这个仓库包含网络工具项目的源代码。

## 描述

这是一个在 Windows 系统后台运行的系统托盘程序小工具，包含以下功能：

- 提供获取活跃网络接口信息的后端服务。
- 查找可用的端口。
- 启动一个 Web 服务器来展示网络接口信息。
- 通过系统托盘图标与用户交互。

## 使用方法

1. **构建项目**：

    ```bash
    cargo build --release
    ```

2. **运行应用程序**：

    ```bash
    cargo run --release
    ```

    运行后，一个系统托盘图标将会出现，表示程序已在后台运行。

3. 程序会持续在后台运行，并提供获取活跃网络接口信息的服务。
4. Web 服务器将在本地启动，可以通过浏览器访问查看网络接口信息。
5. 右键托盘图标可以退出当前程序。

## 项目架构

### 主程序 (`src/main.rs`)

程序的主入口，负责初始化日志系统、确保单例运行、创建托盘图标和菜单、启动 Web 服务器，以及运行事件循环来处理用户交互。

### 客户端模块 (`src/client/main.rs`)

客户端模块，负责处理客户端逻辑。

### 通用模块 (`src/common`)

包含通用功能，例如日志配置和端口查找。

- `log.rs`: 包含日志配置函数。
- `utils.rs`: 包含查找可用端口的函数 `find_available_port`。

### 服务器模块 (`src/server`)

包含服务器端逻辑，负责启动 Web 服务器和处理网络接口信息。

- `main.rs`: 包含启动 Web 服务器的函数 `start_web_server` 和 `run` 函数。

### 服务器控制器模块 (`src/server/controller`)

包含处理 HTTP 请求的控制器。

- `net_status.rs`: 包含获取网络接口信息的函数 `get_interfaces`。

### 服务器模型模块 (`src/server/model`)

包含数据模型。

- `net_status.rs`: 包含网络接口信息结构体 `InterfaceInfo`。

### 服务器服务模块 (`src/server/service`)

包含业务逻辑服务。

- `net_status.rs`: 包含获取网络接口信息的函数 `get_interfaces`。

### 资源文件 (`src/assets/icon.rgba`)

包含程序的图标文件。

### 日志文件

存储程序的运行日志，文件名包含当前日期。

## Web 服务器

该项目包含一个 Web 服务器，它能够：

- 展示网络接口信息。
- 允许用户通过 Web 界面与程序交互。

## 技术细节

该应用程序使用了以下 crate：

- `actix-cors`: 用于处理 CORS。
- `actix-web`: 用于创建 Web 服务器。
- `if_addrs`: 用于获取网络接口信息。
- `mac_address`: 用于获取 MAC 地址。
- `serde`: 用于序列化数据。
- `chrono`: 用于处理日期和时间。
- `log`: 用于日志记录。
- `simplelog`: 用于配置日志系统。
- `single_instance`: 用于确保程序单例运行。
- `tray_icon`: 用于创建系统托盘图标和菜单。
- `winit`: 用于创建事件循环。

## 贡献

欢迎贡献！请打开一个 issue 或提交一个 pull request。
