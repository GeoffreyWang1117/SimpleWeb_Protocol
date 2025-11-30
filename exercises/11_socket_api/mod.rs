//! # 第11章：Socket API
//!
//! 深入理解 BSD Socket API，这是几乎所有网络编程的基础。
//!
//! ## Socket API 起源
//!
//! - 1983 年在 4.2BSD 中引入
//! - 成为事实上的网络编程标准
//! - POSIX 标准化
//!
//! ## 核心系统调用
//!
//! ```text
//! socket()  -> 创建 socket
//! bind()    -> 绑定地址
//! listen()  -> 开始监听
//! accept()  -> 接受连接
//! connect() -> 发起连接
//! send()    -> 发送数据
//! recv()    -> 接收数据
//! close()   -> 关闭 socket
//! ```
//!
//! ## 本章内容
//!
//! - Socket 创建和配置
//! - Socket 选项
//! - 阻塞与非阻塞 I/O
//! - 原始套接字

#[path = "00_basics.rs"]
pub mod ex00_basics;
#[path = "01_options.rs"]
pub mod ex01_options;
#[path = "02_nonblocking.rs"]
pub mod ex02_nonblocking;
#[path = "03_raw_socket.rs"]
pub mod ex03_raw_socket;

pub use ex00_basics::*;
pub use ex01_options::*;
pub use ex02_nonblocking::*;
pub use ex03_raw_socket::*;
