//! # 第4章：UDP 协议
//!
//! UDP（User Datagram Protocol）是一个简单的传输层协议，
//! 提供无连接、不可靠的数据传输服务。
//!
//! ## UDP 的特点
//!
//! - 无连接：不需要建立连接就可以发送数据
//! - 不可靠：不保证数据到达，不保证顺序
//! - 低开销：头部只有 8 字节
//! - 快速：没有拥塞控制和流量控制
//!
//! ## 适用场景
//!
//! - DNS 查询
//! - 实时音视频
//! - 在线游戏
//! - IoT 设备通信

pub mod ex00_header;
pub mod ex01_socket;

pub use ex00_header::*;
pub use ex01_socket::*;
