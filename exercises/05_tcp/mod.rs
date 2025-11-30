//! # 第5章：TCP 协议
//!
//! TCP（Transmission Control Protocol）是可靠的、面向连接的传输层协议。
//!
//! ## TCP 的特点
//!
//! - 面向连接：通信前需要建立连接
//! - 可靠传输：确保数据按序、完整地到达
//! - 流量控制：防止发送方发送过快
//! - 拥塞控制：避免网络拥塞
//!
//! ## 本章内容
//!
//! - TCP 头部结构
//! - TCP 标志位
//! - 三次握手和四次挥手
//! - TCP 状态机

#[path = "00_header.rs"]
pub mod ex00_header;
#[path = "01_flags.rs"]
pub mod ex01_flags;
#[path = "02_handshake.rs"]
pub mod ex02_handshake;
#[path = "03_state_machine.rs"]
pub mod ex03_state_machine;

pub use ex00_header::*;
pub use ex01_flags::*;
pub use ex02_handshake::*;
pub use ex03_state_machine::*;
