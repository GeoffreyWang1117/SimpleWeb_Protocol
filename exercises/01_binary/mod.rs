//! # 第1章：二进制基础
//!
//! 网络协议的数据都是以二进制形式传输的。
//! 本章将学习处理二进制数据的核心技能：
//!
//! - 位操作（获取、设置、清除特定位）
//! - 字节序（大端序与小端序）
//! - 校验和计算
//! - CRC 循环冗余校验

#[path = "00_bits.rs"]
pub mod ex00_bits;
#[path = "01_endianness.rs"]
pub mod ex01_endianness;
#[path = "02_checksum.rs"]
pub mod ex02_checksum;
#[path = "03_crc.rs"]
pub mod ex03_crc;

pub use ex00_bits::*;
pub use ex01_endianness::*;
pub use ex02_checksum::*;
pub use ex03_crc::*;
