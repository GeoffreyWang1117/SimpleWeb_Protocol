//! # 第8章：WebSocket 协议
//!
//! WebSocket 提供了在单个 TCP 连接上进行全双工通信的能力。
//!
//! ## WebSocket 特点
//!
//! - 全双工通信（客户端和服务器可以同时发送数据）
//! - 低延迟（保持连接，无需每次请求都建立新连接）
//! - 二进制和文本支持
//! - 通过 HTTP Upgrade 机制建立连接
//!
//! ## 使用场景
//!
//! - 实时聊天
//! - 在线游戏
//! - 股票行情
//! - 协作编辑

#[path = "00_frame.rs"]
pub mod ex00_frame;
#[path = "01_handshake.rs"]
pub mod ex01_handshake;

pub use ex00_frame::*;
pub use ex01_handshake::*;
