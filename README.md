# 🌐 Rust Network Protocols - Rustlings 风格的网络协议学习

欢迎来到 **Rust Network Protocols**！这是一个类似 [rustlings](https://github.com/rust-lang/rustlings) 的交互式学习项目，帮助你从零开始学习网络协议。

## 📚 学习路线

本项目按照网络协议栈的层次，由底层到上层、由简单到复杂的顺序组织：

| 章节 | 主题 | 难度 | 描述 |
|------|------|------|------|
| 00 | intro | ⭐ | 入门：Rust 基础与字节操作 |
| 01 | binary | ⭐⭐ | 二进制基础：位操作、字节序、校验和 |
| 02 | ethernet | ⭐⭐ | 数据链路层：以太网帧、MAC 地址 |
| 03 | ip | ⭐⭐⭐ | 网络层：IPv4/IPv6 协议 |
| 04 | udp | ⭐⭐⭐ | 传输层：UDP 协议 |
| 05 | tcp | ⭐⭐⭐⭐ | 传输层：TCP 协议、三次握手、流量控制 |
| 06 | dns | ⭐⭐⭐ | 应用层：DNS 协议 |
| 07 | http | ⭐⭐⭐⭐ | 应用层：HTTP/1.1 协议 |
| 08 | websocket | ⭐⭐⭐⭐ | 应用层：WebSocket 协议 |

## 🚀 开始学习

### 1. 安装 Rust

如果你还没有安装 Rust，请先安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 克隆项目

```bash
git clone <your-repo-url>
cd rust-network-protocols
```

### 3. 运行练习检查器

```bash
cargo run
```

或者构建后运行：

```bash
cargo build --release
./target/release/netlings
```

### 4. 开始做题

每个练习文件中都有 `// TODO:` 或 `todo!()` 标记，你需要填写正确的代码使测试通过。

## 📝 练习说明

### 练习格式

每个练习文件的结构如下：

```rust
// exercises/01_binary/00_bits.rs

//! # 位操作基础
//!
//! 在这个练习中，你将学习如何使用 Rust 进行位操作。
//!
//! ## 知识点
//! - 位与 (&)、位或 (|)、位异或 (^)
//! - 左移 (<<) 和右移 (>>)

/// 获取一个字节的第 n 位（从右往左，0-indexed）
pub fn get_bit(byte: u8, n: u8) -> bool {
    // TODO: 实现这个函数
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit(0b10101010, 1), true);
        assert_eq!(get_bit(0b10101010, 0), false);
    }
}
```

### 如何验证你的答案

在项目根目录运行：

```bash
# 运行所有测试
cargo test

# 运行特定章节的测试
cargo test binary

# 运行特定练习的测试
cargo test bits
```

## 🔧 项目结构

```
rust-network-protocols/
├── Cargo.toml              # 项目配置
├── README.md               # 本文件
├── info.toml               # 练习元数据配置
├── src/
│   ├── main.rs             # 练习检查器主程序
│   └── lib.rs              # 库入口
└── exercises/
    ├── 00_intro/           # 入门练习
    ├── 01_binary/          # 二进制基础
    ├── 02_ethernet/        # 以太网协议
    ├── 03_ip/              # IP 协议
    ├── 04_udp/             # UDP 协议
    ├── 05_tcp/             # TCP 协议
    ├── 06_dns/             # DNS 协议
    ├── 07_http/            # HTTP 协议
    └── 08_websocket/       # WebSocket 协议
```

## 💡 学习建议

1. **按顺序学习**：练习是按照依赖关系排列的，后面的练习可能依赖前面的知识
2. **动手实验**：每个练习都可以独立运行，尝试修改代码看看会发生什么
3. **查阅 RFC**：每个协议都有对应的 RFC 文档，遇到困难时可以查阅
4. **使用抓包工具**：配合 Wireshark 等工具，可以更直观地理解协议

## 📖 推荐资源

- [RFC 791 - Internet Protocol (IPv4)](https://tools.ietf.org/html/rfc791)
- [RFC 793 - Transmission Control Protocol (TCP)](https://tools.ietf.org/html/rfc793)
- [RFC 768 - User Datagram Protocol (UDP)](https://tools.ietf.org/html/rfc768)
- [RFC 1035 - Domain Names (DNS)](https://tools.ietf.org/html/rfc1035)
- [RFC 2616 - HTTP/1.1](https://tools.ietf.org/html/rfc2616)
- [RFC 6455 - WebSocket Protocol](https://tools.ietf.org/html/rfc6455)

## 🎯 目标

完成所有练习后，你将能够：

- ✅ 理解网络协议栈的分层架构
- ✅ 手动解析和构造各层协议的数据包
- ✅ 使用 Rust 实现简单的网络应用
- ✅ 理解 TCP/IP 协议族的核心概念
- ✅ 为更高级的网络编程打下坚实基础

## 📜 许可证

MIT License

---

**祝你学习愉快！** 🎉

如有问题，欢迎提 Issue 或 PR！
