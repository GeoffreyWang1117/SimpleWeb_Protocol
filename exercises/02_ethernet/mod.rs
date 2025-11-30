//! # 第2章：以太网（Ethernet）
//!
//! 以太网是最常用的局域网技术，工作在 OSI 模型的数据链路层。
//!
//! 本章将学习：
//! - MAC 地址的结构和表示
//! - 以太网帧的格式
//! - EtherType 字段

#[path = "00_mac_address.rs"]
pub mod ex00_mac_address;
#[path = "01_frame.rs"]
pub mod ex01_frame;

pub use ex00_mac_address::*;
pub use ex01_frame::*;
