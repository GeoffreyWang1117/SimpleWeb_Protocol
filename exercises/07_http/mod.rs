//! # 第7章：HTTP 协议
//!
//! HTTP（Hypertext Transfer Protocol）是 Web 的基础协议。
//!
//! ## HTTP/1.1 特点
//!
//! - 基于文本的协议
//! - 请求-响应模式
//! - 无状态（通过 Cookie 等机制可以维持状态）
//! - 默认使用 TCP 端口 80（HTTPS 使用 443）
//!
//! ## 本章内容
//!
//! - HTTP 请求解析和构建
//! - HTTP 响应解析和构建
//! - 简单的 HTTP 客户端

pub mod ex00_request;
pub mod ex01_response;
pub mod ex02_client;

pub use ex00_request::*;
pub use ex01_response::*;
pub use ex02_client::*;
