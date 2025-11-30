//! # 第12章：I/O 多路复用
//!
//! 理解高性能网络编程的核心技术：I/O 多路复用。
//!
//! ## 为什么需要 I/O 多路复用？
//!
//! 传统的阻塞 I/O 每个连接需要一个线程，这在高并发场景下不可行：
//!
//! ```text
//! 传统模型:
//! Client 1 <---> Thread 1 <---> Server
//! Client 2 <---> Thread 2 <---> Server
//! ...
//! Client N <---> Thread N <---> Server
//!
//! 问题: 10000 个连接 = 10000 个线程 = 内存耗尽
//! ```
//!
//! I/O 多路复用允许单个线程处理多个连接：
//!
//! ```text
//! Client 1 \
//! Client 2  >---> Event Loop ---> Single Thread
//! Client N /
//! ```
//!
//! ## 技术演进
//!
//! | 技术 | 年份 | 平台 | 特点 |
//! |------|------|------|------|
//! | select | 1983 | 跨平台 | fd 数量限制 (1024) |
//! | poll | 1986 | Unix | 无 fd 数量限制 |
//! | epoll | 2002 | Linux | O(1) 事件通知 |
//! | kqueue | 2000 | BSD/macOS | 类似 epoll |
//! | IOCP | 1993 | Windows | 完成端口模型 |
//!
//! ## 本章内容
//!
//! - select: 最古老的多路复用
//! - poll: select 的改进
//! - epoll: Linux 高性能方案
//! - 事件循环: 构建自己的事件驱动系统

#[path = "00_select.rs"]
pub mod ex00_select;
#[path = "01_poll.rs"]
pub mod ex01_poll;
#[path = "02_epoll.rs"]
pub mod ex02_epoll;
#[path = "03_event_loop.rs"]
pub mod ex03_event_loop;

pub use ex00_select::*;
pub use ex01_poll::*;
pub use ex02_epoll::*;
pub use ex03_event_loop::*;
