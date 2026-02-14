# 编码转换工具

一个用 Rust 和 egui 构建的跨平台编码转换工具，支持多种进制转换、文本编码转换和位操作查看。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## 功能特性

- **进制转换**：二进制 ↔ 十进制 ↔ 十六进制，实时转换
- **文本编码**：ASCII ↔ 十六进制，支持不可打印字符显示
- **位查看器**：可视化位操作，支持点击切换
- **浮点数转换**：IEEE 754 标准，支持 f32 浮点数转换

## 快速开始

```bash
# 克隆并构建
git clone https://github.com/AnlangA/number-conversion-rs.git
cd number-conversion-rs
cargo build --release

# 运行
cargo run --release
```

## 系统要求

- Rust 1.70+
- Windows / macOS / Linux

## 项目结构

```
src/
├── app/           # 应用程序层
├── core/          # 核心业务逻辑
├── ui/            # 用户界面层
└── utils/         # 工具函数
```

## 开发

```bash
# 运行测试
cargo test
```

## 许可证

MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。
