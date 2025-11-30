//! # Rust Network Protocols
//!
//! 这是一个类似 rustlings 的网络协议学习项目。
//!
//! ## 模块结构
//!
//! - `intro` - 入门：Rust 基础与字节操作
//! - `binary` - 二进制基础：位操作、字节序、校验和
//! - `ethernet` - 数据链路层：以太网帧、MAC 地址
//! - `ip` - 网络层：IPv4/IPv6 协议
//! - `udp` - 传输层：UDP 协议
//! - `tcp` - 传输层：TCP 协议
//! - `dns` - 应用层：DNS 协议
//! - `http` - 应用层：HTTP 协议
//! - `websocket` - 应用层：WebSocket 协议

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
