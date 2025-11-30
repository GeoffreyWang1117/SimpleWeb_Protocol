//! # 第9章：消息分帧（Framing）
//!
//! TCP 是字节流协议，没有消息边界的概念。应用层需要自己定义消息边界。
//!
//! ## 为什么需要分帧？
//!
//! TCP 提供的是无边界的字节流：
//! - 发送 "Hello" + "World" 可能被接收为 "HelloWorld" 或 "Hel" + "loWorld"
//! - 应用层必须自己确定消息的边界
//!
//! ## 常见的分帧策略
//!
//! 1. **固定长度**: 每条消息固定 N 字节
//! 2. **分隔符**: 使用特殊字符分隔消息（如 \n, \r\n, \0）
//! 3. **长度前缀**: 消息头部包含长度信息
//! 4. **自描述格式**: JSON, Protocol Buffers 等
//!
//! ## 本章内容
//!
//! - 固定长度分帧
//! - 分隔符分帧
//! - 长度前缀分帧（TLV格式）
//! - 混合分帧策略
//! - 变长整数编码

#[path = "00_fixed_length.rs"]
pub mod ex00_fixed_length;
#[path = "01_delimiter.rs"]
pub mod ex01_delimiter;
#[path = "02_length_prefixed.rs"]
pub mod ex02_length_prefixed;
#[path = "03_codec.rs"]
pub mod ex03_codec;
#[path = "04_varint.rs"]
pub mod ex04_varint;

pub use ex00_fixed_length::*;
pub use ex01_delimiter::*;
pub use ex02_length_prefixed::*;
pub use ex03_codec::*;
pub use ex04_varint::*;
