//! # 第6章：DNS 协议
//!
//! DNS（Domain Name System）是将域名转换为 IP 地址的分布式命名系统。
//!
//! ## DNS 工作原理
//!
//! 1. 客户端向 DNS 服务器发送查询请求
//! 2. DNS 服务器返回域名对应的 IP 地址
//! 3. 客户端使用 IP 地址建立连接
//!
//! ## DNS 使用的传输协议
//!
//! - UDP 端口 53（常规查询）
//! - TCP 端口 53（大型响应或区域传输）

pub mod ex00_header;
pub mod ex01_question;
pub mod ex02_response;

pub use ex00_header::*;
pub use ex01_question::*;
pub use ex02_response::*;
