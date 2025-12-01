//! # Rust Network Protocols
//!
//! 这是一个类似 rustlings 的网络协议学习项目。
//!
//! ## 模块结构
//!
//! ### 基础篇
//! - `intro` - 入门：Rust 基础与字节操作
//! - `binary` - 二进制基础：位操作、字节序、校验和
//!
//! ### 协议栈篇
//! - `ethernet` - 数据链路层：以太网帧、MAC 地址
//! - `ip` - 网络层：IPv4/IPv6 协议
//! - `udp` - 传输层：UDP 协议
//! - `tcp` - 传输层：TCP 协议
//!
//! ### 应用层篇
//! - `dns` - 应用层：DNS 协议
//! - `http` - 应用层：HTTP 协议
//! - `websocket` - 应用层：WebSocket 协议
//!
//! ### 高级篇
//! - `framing` - 消息分帧：定长、分隔符、长度前缀
//! - `reliability` - 可靠性：心跳、重传、滑动窗口、拥塞控制
//! - `socket_api` - Socket API：选项、非阻塞、原始套接字
//! - `io_multiplexing` - I/O 多路复用：select、poll、epoll
//! - `quic` - 现代协议：QUIC、0-RTT、连接迁移

// 引入所有练习模块
#[path = "../exercises/00_intro/mod.rs"]
pub mod intro;

#[path = "../exercises/01_binary/mod.rs"]
pub mod binary;

#[path = "../exercises/02_ethernet/mod.rs"]
pub mod ethernet;

#[path = "../exercises/03_ip/mod.rs"]
pub mod ip;

#[path = "../exercises/04_udp/mod.rs"]
pub mod udp;

#[path = "../exercises/05_tcp/mod.rs"]
pub mod tcp;

#[path = "../exercises/06_dns/mod.rs"]
pub mod dns;

#[path = "../exercises/07_http/mod.rs"]
pub mod http;

#[path = "../exercises/08_websocket/mod.rs"]
pub mod websocket;

#[path = "../exercises/09_framing/mod.rs"]
pub mod framing;

#[path = "../exercises/10_reliability/mod.rs"]
pub mod reliability;

#[path = "../exercises/11_socket_api/mod.rs"]
pub mod socket_api;

#[path = "../exercises/12_io_multiplexing/mod.rs"]
pub mod io_multiplexing;

#[path = "../exercises/13_quic/mod.rs"]
pub mod quic;

// 综合测验 - 小型项目级别应用题
#[path = "../exercises/quizzes/mod.rs"]
pub mod quizzes;
