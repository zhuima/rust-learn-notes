# IP Geolocation API

这是一个简单的 IP 地理位置 API，使用 Rust 和 Axum 框架构建。它可以根据给定的 IP 地址返回地理位置信息。

## 功能

- 提供 HTTP 接口来查询 IP 地址的地理位置信息
- 使用 ip2region 数据库进行快速的 IP 地理位置查询
- 返回包括国家、省份、城市和 ISP 在内的详细信息

## 前置要求

在开始之前，请确保你的系统中已安装以下软件：

- Rust（推荐使用最新的稳定版本）
- Cargo（Rust 的包管理器，通常随 Rust 一起安装）

你可以在 [Rust 官网](https://www.rust-lang.org/tools/install) 找到安装说明。

## 快速开始

1. 克隆此仓库：

   ```
   git clone https://github.com/your-username/ip_geolocation_api.git
   cd ip_geolocation_api
   ```

2. 下载 ip2region 数据库文件：

   从 [ip2region 官方仓库](https://github.com/lionsoul2014/ip2region/tree/master/data) 下载 `ip2region.xdb` 文件，并将其放在项目根目录下。

3. 构建并运行项目：

   ```
   cargo run
   ```

   如果一切正常，你应该会看到类似这样的输出：

   ```
   listening on 0.0.0.0:3000
   ```

4. 测试 API：

   打开新的终端窗口，运行以下命令：

   ```
   curl http://localhost:3000/ip
   ```

   你应该会看到一个包含你当前 IP 地址地理位置信息的 JSON 响应。

## API 使用

本 API 只有一个端点：

- `GET /ip`: 返回请求者 IP 地址的地理位置信息

响应格式示例：

```json
{
    "ip": "1.2.3.4",
    "region": "中国|0|北京|北京|电信",
    "country": "中国",
    "province": "北京",
    "city": "北京",
    "isp": "电信"
}
```

## 项目结构

- `src/main.rs`: 主程序文件，包含服务器设置和 API 处理逻辑
- `Cargo.toml`: 项目依赖和配置文件
- `ip2region.xdb`: IP 地理位置数据库文件（需要手动下载）

## 依赖

本项目主要依赖以下 Rust crates：

- `axum`: Web 框架
- `tokio`: 异步运行时
- `serde_json`: JSON 序列化/反序列化
- `ip2region`: IP 地理位置查询库

完整的依赖列表可以在 `Cargo.toml` 文件中找到。

## 贡献

欢迎贡献！如果你有任何改进建议或者发现了 bug，请创建一个 issue 或提交一个 pull request。

## 许可

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。
