//! # 第13章：QUIC 协议
//!
//! QUIC 是下一代传输协议，HTTP/3 的基础。
//!
//! ## QUIC 历史
//!
//! - 2012: Google 内部开发 "Quick UDP Internet Connections"
//! - 2016: IETF 开始标准化
//! - 2021: RFC 9000 发布（QUIC 传输协议）
//! - 2022: RFC 9114 发布（HTTP/3）
//!
//! ## 为什么需要 QUIC？
//!
//! TCP 的问题：
//!
//! 1. **队头阻塞**: 一个丢包会阻塞所有流
//! 2. **握手延迟**: TCP + TLS 需要 2-3 个 RTT
//! 3. **连接迁移**: IP 变化需要重新建立连接
//! 4. **中间件僵化**: 防火墙/NAT 难以升级
//!
//! ## QUIC 特性
//!
//! ```text
//! ┌────────────────────────────────────┐
//! │            HTTP/3                   │
//! ├────────────────────────────────────┤
//! │              QUIC                   │
//! │  ┌──────┬──────┬──────┬─────────┐  │
//! │  │流复用│ TLS  │拥塞  │连接迁移 │  │
//! │  │      │ 1.3  │控制  │        │  │
//! │  └──────┴──────┴──────┴─────────┘  │
//! ├────────────────────────────────────┤
//! │              UDP                    │
//! └────────────────────────────────────┘
//! ```
//!
//! ## 本章内容
//!
//! - QUIC 基础概念
//! - 流复用
//! - 0-RTT/1-RTT 握手
//! - 连接迁移

#[path = "00_basics.rs"]
pub mod ex00_basics;
#[path = "01_streams.rs"]
pub mod ex01_streams;
#[path = "02_handshake.rs"]
pub mod ex02_handshake;
#[path = "03_migration.rs"]
pub mod ex03_migration;

pub use ex00_basics::*;
pub use ex01_streams::*;
pub use ex02_handshake::*;
pub use ex03_migration::*;
