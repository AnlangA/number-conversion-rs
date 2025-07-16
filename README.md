# 编码转换工具 (Number Conversion Tool)

一个用Rust和egui构建的跨平台编码转换工具，支持多种进制转换、文本编码转换和位操作查看。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

## 功能特性

### 🔢 进制转换
- **二进制 ↔ 十进制 ↔ 十六进制**：支持常用进制之间的相互转换
- **实时转换**：输入即时显示转换结果
- **格式化输出**：自动添加分隔符提高可读性
- **输入验证**：智能检测输入格式并提供错误提示

### 📝 文本转换
- **ASCII ↔ 十六进制**：文本字符与十六进制编码的相互转换
- **可视化显示**：不可打印字符以 `[0xXX]` 格式显示
- **批量处理**：支持长文本的批量转换

### 🔍 位查看器
- **可视化位操作**：直观显示二进制位，支持点击切换
- **自定义字段分组**：可配置位字段分割方式
- **3D按钮效果**：美观的位按钮界面
- **统计信息**：显示位数统计和比例信息

### 🧮 浮点数转换
- **IEEE 754支持**：f32浮点数与十六进制表示的转换
- **结构分析**：详细分析浮点数的符号位、指数位、尾数位
- **特殊值处理**：正确处理NaN、无穷大等特殊值

## 技术架构

### 模块化设计
```
src/
├── app/                    # 应用程序层
│   ├── application.rs      # 主应用程序结构
│   └── config.rs          # 配置管理
├── core/                  # 核心业务逻辑层
│   ├── models/            # 数据模型
│   ├── converters/        # 转换器
│   └── errors.rs          # 错误处理
├── ui/                    # 用户界面层
│   ├── components/        # UI组件
│   └── pages/            # 页面
└── utils/                 # 工具函数
    ├── formatting.rs      # 格式化工具
    └── validation.rs      # 验证工具
```

### 核心特性
- **类型安全**：使用Rust的类型系统确保转换的正确性
- **错误处理**：完善的错误处理机制，用户友好的错误提示
- **性能优化**：高效的转换算法，支持大数值处理
- **可扩展性**：模块化设计，易于添加新的转换功能

## 安装和使用

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/AnlangA/number-conversion-rs.git
cd number-conversion-rs

# 构建项目
cargo build --release

# 运行程序
cargo run --release
```

### 系统要求
- Rust 1.70 或更高版本
- 支持的操作系统：Windows、macOS、Linux

## 使用指南

### 进制转换
1. 选择"进制转换"标签页
2. 在相应的输入框中输入数值
3. 转换结果会实时显示在下方

### 文本转换
1. 选择"字符转换"标签页
2. 输入ASCII文本或十六进制编码
3. 查看转换结果

### 位查看器
1. 选择"bit查看"标签页
2. 输入十六进制数据
3. 配置字段分组（可选）
4. 点击位按钮进行编辑

## 开发

### 项目结构
- **应用程序层**：处理UI逻辑和用户交互
- **核心业务层**：实现转换算法和数据处理
- **工具层**：提供通用的格式化和验证功能

### 添加新功能
1. 在 `core/converters/` 中添加新的转换器
2. 在 `ui/pages/` 中创建对应的UI页面
3. 更新导航组件以包含新页面

### 运行测试
```bash
cargo test
```

## 贡献

欢迎提交Issue和Pull Request！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 更新日志

### v0.2.0
- 重构项目架构，采用模块化设计
- 改进错误处理机制
- 优化UI组件结构
- 添加配置管理功能
- 完善文档和测试

### v0.1.0
- 初始版本
- 基本的进制转换功能
- 文本编码转换
- 位查看器功能

## 致谢

- [egui](https://github.com/emilk/egui) - 优秀的即时模式GUI框架
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) - egui的应用程序框架
- Rust社区的支持和贡献
