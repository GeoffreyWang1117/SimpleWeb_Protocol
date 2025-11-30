//! # 第10章：可靠性机制
//!
//! 网络通信中保证可靠性的核心机制。
//!
//! ## 为什么需要可靠性机制？
//!
//! - 网络可能丢包
//! - 连接可能静默断开
//! - 数据可能乱序到达
//! - 数据可能重复到达
//!
//! ## 核心机制
//!
//! 1. **心跳（Heartbeat/Keepalive）**: 检测连接存活
//! 2. **确认（Acknowledgment）**: 确保数据到达
//! 3. **重传（Retransmission）**: 处理丢包
//! 4. **序列号（Sequence Number）**: 检测重复和乱序
//! 5. **超时（Timeout）**: 检测故障
//!
//! ## 本章内容
//!
//! - 心跳机制
//! - 确认与重传
//! - 滑动窗口
//! - 拥塞控制基础
//! - 选择性确认 (SACK)

#[path = "00_heartbeat.rs"]
pub mod ex00_heartbeat;
#[path = "01_ack_retransmit.rs"]
pub mod ex01_ack_retransmit;
#[path = "02_sliding_window.rs"]
pub mod ex02_sliding_window;
#[path = "03_congestion.rs"]
pub mod ex03_congestion;
#[path = "04_sack.rs"]
pub mod ex04_sack;

pub use ex00_heartbeat::*;
pub use ex01_ack_retransmit::*;
pub use ex02_sliding_window::*;
pub use ex03_congestion::*;
pub use ex04_sack::*;
