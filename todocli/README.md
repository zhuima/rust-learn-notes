# Todo CLI

Todo CLI 是一个简单而强大的命令行待办事项管理工具，使用 Rust 编写。它允许您创建、查看、完成和删除待办事项，所有这些都可以通过简单的命令行界面完成。

## 功能

- 添加新的待办事项
- 列出所有待办事项
- 将待办事项标记为已完成
- 删除待办事项

## 安装

确保您的系统上安装了 Rust 和 Cargo。然后，按照以下步骤操作：

1. 克隆此仓库：

```rust
git clone https://github.com/yourusername/todocli.git
cd todocli
cargo run --
```

2. 构建项目：

```rust
cargo build --release
```

3. 可执行文件将在 `target/release` 目录中生成。

## 使用方法

### 添加新的待办事项

```rust
cargo run -- add --title "完成项目报告" --description "包括上周的进度总结"
```

### 列出所有待办事项

```rust
cargo run -- list
```

### 将待办事项标记为已完成

```rust
cargo run -- complete --id 1
```

### 删除待办事项

```rust
cargo run -- delete --id 1
```

## 数据存储

所有的待办事项都存储在项目根目录下的 `todos.json` 文件中。

## 依赖项

- structopt: 用于解析命令行参数
- serde 和 serde_json: 用于序列化和反序列化数据
- chrono: 用于处理日期和时间
