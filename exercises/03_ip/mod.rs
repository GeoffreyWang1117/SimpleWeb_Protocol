//! # 第3章：IP 协议
//!
//! IP（Internet Protocol）是网络层的核心协议，负责将数据包从源主机路由到目标主机。
//!
//! 本章将学习：
//! - IPv4 地址的结构和分类
//! - IPv4 头部格式
//! - IPv6 基础

#[path = "00_ipv4_address.rs"]
pub mod ex00_ipv4_address;
#[path = "01_ipv4_header.rs"]
pub mod ex01_ipv4_header;
#[path = "02_ipv6.rs"]
pub mod ex02_ipv6;

pub use ex00_ipv4_address::*;
pub use ex01_ipv4_header::*;
pub use ex02_ipv6::*;
