# 编码转换工具

一个用 Rust 和 egui 构建的跨平台编码转换工具，支持多种进制转换、文本编码转换、位查看与位操作，以及多进制表达式计算。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## 功能特性

- **进制转换**：二进制 ↔ 十进制 ↔ 十六进制，实时转换
- **文本编码**：ASCII ↔ 十六进制，支持不可打印字符显示
- **位查看器**：支持十六进制 / 位串 / 十进制联动查看，支持点击切换、按位取反、逻辑移位、循环移位、撤销与重做
- **浮点数转换**：IEEE 754 标准，支持 f32 浮点数转换与分析
- **表达式计算器**：支持二进制、八进制、十进制、十六进制表达式计算，并自动转换显示结果

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
├── app/           # 应用程序入口、配置与启动逻辑
├── backend/       # 异步计算与前后端消息通信
├── core/          # 核心业务逻辑（位操作、计算引擎等）
├── frontend/      # 页面状态管理与本地交互逻辑
└── ui/            # 用户界面层与可复用组件
```

## 开发

```bash
# 运行测试
cargo test
```

如果你要使用表达式计算器，请确保当前 Python 环境可用；项目会优先查找本地虚拟环境中的 Python，并依赖 `scripts/calc_engine.py` 所需的 `sympy` 包。

## 许可证

MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。
