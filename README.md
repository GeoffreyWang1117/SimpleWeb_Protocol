# Rust Network Protocols | Rust 网络协议学习

[English](#english) | [中文](#中文)

---

<a name="english"></a>
## English

Welcome to **Rust Network Protocols**! This is an interactive learning project similar to [rustlings](https://github.com/rust-lang/rustlings), designed to help you learn network protocols from scratch.

### Learning Path

This project is organized by network protocol layers, from low-level to high-level, from simple to complex:

#### Fundamentals

| Chapter | Topic | Difficulty | Description |
|---------|-------|------------|-------------|
| 00 | intro | ★ | Getting started: Rust basics and byte operations |
| 01 | binary | ★★ | Binary fundamentals: bit operations, endianness, checksum, CRC |

#### Protocol Stack

| Chapter | Topic | Difficulty | Description |
|---------|-------|------------|-------------|
| 02 | ethernet | ★★ | Data link layer: Ethernet frames, MAC addresses |
| 03 | ip | ★★★ | Network layer: IPv4/IPv6 protocols, CIDR |
| 04 | udp | ★★★ | Transport layer: UDP protocol |
| 05 | tcp | ★★★★ | Transport layer: TCP protocol, three-way handshake, state machine |

#### Application Layer

| Chapter | Topic | Difficulty | Description |
|---------|-------|------------|-------------|
| 06 | dns | ★★★ | Application layer: DNS protocol |
| 07 | http | ★★★★ | Application layer: HTTP/1.1 protocol, chunked encoding |
| 08 | websocket | ★★★★ | Application layer: WebSocket protocol |

#### Advanced Topics

| Chapter | Topic | Difficulty | Description |
|---------|-------|------------|-------------|
| 09 | framing | ★★★ | Message framing: fixed-length, delimiter, length-prefixed, varint |
| 10 | reliability | ★★★★ | Reliability: heartbeat, ACK/retransmit, sliding window, congestion control, SACK |
| 11 | socket_api | ★★★ | Socket API: options, non-blocking I/O, raw sockets |
| 12 | io_multiplexing | ★★★★ | I/O multiplexing: select, poll, epoll, event loop |
| 13 | quic | ★★★★★ | Modern protocol: QUIC, stream multiplexing, 0-RTT, connection migration, flow control |

#### Quizzes (Mini Projects)

| Quiz | Topic | Difficulty | Description |
|------|-------|------------|-------------|
| Quiz 1 | Ping Tool | ★★★★ | ICMP Echo Request/Reply, checksum calculation, RTT statistics |
| Quiz 2 | HTTP Downloader | ★★★★★ | URL parsing, HTTP client, chunked encoding, progress tracking |
| Quiz 3 | Chat Protocol | ★★★★★ | Protocol design, message framing, session management, heartbeat |
| Quiz 4 | DNS Client | ★★★★★ | DNS query/response, name compression, record parsing, caching |
| Quiz 5 | Reliable UDP | ★★★★★ | Connection state machine, sliding window, congestion control |

### Getting Started

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Clone the Project

```bash
git clone <your-repo-url>
cd rust-network-protocols
```

#### 3. Run the Exercise Checker

```bash
cargo run
```

#### 4. Start Practicing

Each exercise file contains `// TODO:` or `todo!()` markers. Fill in the correct code to make the tests pass.

### Verify Your Answers

```bash
# Run all tests
cargo test

# Run tests for a specific chapter
cargo test binary

# Run tests for a specific exercise
cargo test bits
```

### Recommended Resources

- [RFC 791 - IPv4](https://tools.ietf.org/html/rfc791)
- [RFC 793 - TCP](https://tools.ietf.org/html/rfc793)
- [RFC 9000 - QUIC](https://tools.ietf.org/html/rfc9000)
- [RFC 9114 - HTTP/3](https://tools.ietf.org/html/rfc9114)

### Learning Goals

After completing all exercises, you will be able to:

- ✅ Understand network protocol stack architecture
- ✅ Parse and construct packets at each layer
- ✅ Implement simple network applications in Rust
- ✅ Master TCP/IP protocol suite concepts
- ✅ Understand I/O multiplexing and modern protocols like QUIC

---

<a name="中文"></a>
## 中文

欢迎来到 **Rust Network Protocols**！这是一个类似 [rustlings](https://github.com/rust-lang/rustlings) 的交互式学习项目，帮助你从零开始学习网络协议。

### 学习路线

本项目按照网络协议栈的层次，由底层到上层、由简单到复杂的顺序组织：

#### 基础篇

| 章节 | 主题 | 难度 | 描述 |
|------|------|------|------|
| 00 | intro | ★ | 入门：Rust 基础与字节操作 |
| 01 | binary | ★★ | 二进制基础：位操作、字节序、校验和、CRC |

#### 协议栈篇

| 章节 | 主题 | 难度 | 描述 |
|------|------|------|------|
| 02 | ethernet | ★★ | 数据链路层：以太网帧、MAC 地址 |
| 03 | ip | ★★★ | 网络层：IPv4/IPv6 协议、CIDR |
| 04 | udp | ★★★ | 传输层：UDP 协议 |
| 05 | tcp | ★★★★ | 传输层：TCP 协议、三次握手、状态机 |

#### 应用层篇

| 章节 | 主题 | 难度 | 描述 |
|------|------|------|------|
| 06 | dns | ★★★ | 应用层：DNS 协议 |
| 07 | http | ★★★★ | 应用层：HTTP/1.1 协议、分块编码 |
| 08 | websocket | ★★★★ | 应用层：WebSocket 协议 |

#### 高级篇

| 章节 | 主题 | 难度 | 描述 |
|------|------|------|------|
| 09 | framing | ★★★ | 消息分帧：定长、分隔符、长度前缀、变长整数 |
| 10 | reliability | ★★★★ | 可靠性：心跳、ACK重传、滑动窗口、拥塞控制、SACK |
| 11 | socket_api | ★★★ | Socket API：选项、非阻塞I/O、原始套接字 |
| 12 | io_multiplexing | ★★★★ | I/O多路复用：select、poll、epoll、事件循环 |
| 13 | quic | ★★★★★ | 现代协议：QUIC、流复用、0-RTT、连接迁移、流量控制 |

#### 综合测验 (小型项目)

| 测验 | 主题 | 难度 | 描述 |
|------|------|------|------|
| Quiz 1 | Ping 工具 | ★★★★ | ICMP Echo 请求/响应、校验和计算、RTT 统计 |
| Quiz 2 | HTTP 下载器 | ★★★★★ | URL 解析、HTTP 客户端、分块编码、进度追踪 |
| Quiz 3 | 聊天协议 | ★★★★★ | 协议设计、消息分帧、会话管理、心跳机制 |
| Quiz 4 | DNS 客户端 | ★★★★★ | DNS 查询/响应、名称压缩、记录解析、缓存 |
| Quiz 5 | 可靠 UDP | ★★★★★ | 连接状态机、滑动窗口、拥塞控制 |

### 开始学习

#### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. 克隆项目

```bash
git clone <your-repo-url>
cd rust-network-protocols
```

#### 3. 运行练习检查器

```bash
cargo run
```

#### 4. 开始做题

每个练习文件中都有 `// TODO:` 或 `todo!()` 标记，你需要填写正确的代码使测试通过。

### 验证答案

```bash
# 运行所有测试
cargo test

# 运行特定章节的测试
cargo test binary

# 运行特定练习的测试
cargo test bits
```

### 练习格式

```rust
//! # 位操作基础
//!
//! 在这个练习中，你将学习如何使用 Rust 进行位操作。

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
    }
}
```

### 推荐资源

#### 基础协议
- [RFC 791 - Internet Protocol (IPv4)](https://tools.ietf.org/html/rfc791)
- [RFC 8200 - Internet Protocol (IPv6)](https://tools.ietf.org/html/rfc8200)
- [RFC 793 - Transmission Control Protocol (TCP)](https://tools.ietf.org/html/rfc793)
- [RFC 768 - User Datagram Protocol (UDP)](https://tools.ietf.org/html/rfc768)

#### 应用层协议
- [RFC 1035 - Domain Names (DNS)](https://tools.ietf.org/html/rfc1035)
- [RFC 9110 - HTTP Semantics](https://tools.ietf.org/html/rfc9110)
- [RFC 6455 - WebSocket Protocol](https://tools.ietf.org/html/rfc6455)

#### 现代协议
- [RFC 9000 - QUIC Transport Protocol](https://tools.ietf.org/html/rfc9000)
- [RFC 9001 - QUIC TLS](https://tools.ietf.org/html/rfc9001)
- [RFC 9114 - HTTP/3](https://tools.ietf.org/html/rfc9114)

#### 可靠性机制
- [RFC 5681 - TCP Congestion Control](https://tools.ietf.org/html/rfc5681)
- [RFC 2018 - TCP Selective Acknowledgment](https://tools.ietf.org/html/rfc2018)

### 学习目标

完成所有练习后，你将能够：

- ✅ 理解网络协议栈的分层架构
- ✅ 手动解析和构造各层协议的数据包
- ✅ 使用 Rust 实现简单的网络应用
- ✅ 理解 TCP/IP 协议族的核心概念
- ✅ 掌握消息分帧和可靠性机制的设计
- ✅ 理解高性能 I/O 多路复用技术
- ✅ 了解 QUIC 等现代传输协议
- ✅ 为更高级的网络编程打下坚实基础

### 学习建议

1. **按顺序学习**：练习是按照依赖关系排列的，后面的练习可能依赖前面的知识
2. **动手实验**：每个练习都可以独立运行，尝试修改代码看看会发生什么
3. **查阅 RFC**：每个协议都有对应的 RFC 文档，遇到困难时可以查阅
4. **使用抓包工具**：配合 Wireshark 等工具，可以更直观地理解协议

---

## Project Structure | 项目结构

```
rust-network-protocols/
├── Cargo.toml              # Project config | 项目配置
├── README.md               # This file | 本文件
├── info.toml               # Exercise metadata | 练习元数据
├── src/
│   ├── main.rs             # Exercise checker | 练习检查器
│   └── lib.rs              # Library entry | 库入口
└── exercises/
    ├── 00_intro/           # Introduction | 入门
    ├── 01_binary/          # Binary basics | 二进制基础
    ├── 02_ethernet/        # Ethernet | 以太网
    ├── 03_ip/              # IP protocol | IP 协议
    ├── 04_udp/             # UDP | UDP 协议
    ├── 05_tcp/             # TCP | TCP 协议
    ├── 06_dns/             # DNS | DNS 协议
    ├── 07_http/            # HTTP | HTTP 协议
    ├── 08_websocket/       # WebSocket | WebSocket 协议
    ├── 09_framing/         # Framing | 消息分帧
    ├── 10_reliability/     # Reliability | 可靠性机制
    ├── 11_socket_api/      # Socket API
    ├── 12_io_multiplexing/ # I/O multiplexing | I/O 多路复用
    ├── 13_quic/            # QUIC protocol | QUIC 协议
    └── quizzes/            # Mini projects | 综合测验
```

## License | 许可证

MIT License

---

**Happy Learning! | 祝你学习愉快！**

If you have any questions, feel free to open an issue or PR!

如有问题，欢迎提 Issue 或 PR！
