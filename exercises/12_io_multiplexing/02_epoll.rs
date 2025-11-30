//! # 练习 02: epoll
//!
//! epoll 是 Linux 特有的高性能 I/O 多路复用机制（2002年，Linux 2.5.44）。
//!
//! ## 核心改进
//!
//! 1. **O(1) 事件通知**: 只返回就绪的 fd，无需遍历
//! 2. **状态持久化**: fd 集合在内核维护，无需每次传递
//! 3. **边缘触发**: 支持更高效的边缘触发模式
//!
//! ## API
//!
//! ```c
//! // 创建 epoll 实例
//! int epoll_create1(int flags);
//!
//! // 控制 epoll（增删改 fd）
//! int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event);
//!
//! // 等待事件
//! int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
//! ```
//!
//! ## 触发模式
//!
//! ### 水平触发 (Level Triggered, LT)
//! - 默认模式
//! - 只要 fd 就绪就会通知
//! - 类似 select/poll
//!
//! ### 边缘触发 (Edge Triggered, ET)
//! - 只在状态变化时通知一次
//! - 必须一次性读取所有数据
//! - 更高效，但编程更复杂
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test io_epoll
//! ```

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

/// Epoll 操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollOp {
    /// 添加 fd
    Add,
    /// 修改 fd
    Modify,
    /// 删除 fd
    Delete,
}

/// Epoll 事件标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollEvents(u32);

impl EpollEvents {
    /// 可读
    pub const EPOLLIN: EpollEvents = EpollEvents(0x001);
    /// 可写
    pub const EPOLLOUT: EpollEvents = EpollEvents(0x004);
    /// 错误
    pub const EPOLLERR: EpollEvents = EpollEvents(0x008);
    /// 挂起
    pub const EPOLLHUP: EpollEvents = EpollEvents(0x010);
    /// 边缘触发
    pub const EPOLLET: EpollEvents = EpollEvents(1 << 31);
    /// 一次性（触发后自动删除）
    pub const EPOLLONESHOT: EpollEvents = EpollEvents(1 << 30);

    pub fn new() -> Self {
        EpollEvents(0)
    }

    /// 组合事件
    pub fn with(self, other: EpollEvents) -> Self {
        EpollEvents(self.0 | other.0)
    }

    /// 检查是否包含事件
    pub fn contains(&self, other: EpollEvents) -> bool {
        // TODO: 检查是否包含指定事件
        todo!("检查事件")
    }

    /// 是否是边缘触发
    pub fn is_edge_triggered(&self) -> bool {
        self.contains(Self::EPOLLET)
    }

    /// 是否可读
    pub fn is_readable(&self) -> bool {
        self.contains(Self::EPOLLIN)
    }

    /// 是否可写
    pub fn is_writable(&self) -> bool {
        self.contains(Self::EPOLLOUT)
    }
}

impl Default for EpollEvents {
    fn default() -> Self {
        Self::new()
    }
}

/// Epoll 事件
#[derive(Debug, Clone)]
pub struct EpollEvent {
    /// 事件类型
    pub events: EpollEvents,
    /// 关联的 fd
    pub fd: RawFd,
    /// 用户数据
    pub data: u64,
}

impl EpollEvent {
    pub fn new(fd: RawFd, events: EpollEvents) -> Self {
        // TODO: 创建 EpollEvent
        todo!("创建 EpollEvent")
    }

    /// 带用户数据
    pub fn with_data(fd: RawFd, events: EpollEvents, data: u64) -> Self {
        Self { events, fd, data }
    }
}

/// Epoll 实例
///
/// 模拟 epoll 的行为
pub struct Epoll {
    /// 注册的 fd 和事件
    interests: HashMap<RawFd, EpollEvents>,
    /// 就绪队列（模拟内核的就绪列表）
    ready_list: Vec<EpollEvent>,
}

impl Epoll {
    /// 创建 epoll 实例 (epoll_create1)
    pub fn create() -> io::Result<Self> {
        // TODO: 创建 epoll 实例
        todo!("创建 epoll")
    }

    /// 控制 epoll (epoll_ctl)
    pub fn ctl(&mut self, op: EpollOp, fd: RawFd, event: Option<EpollEvents>) -> io::Result<()> {
        // TODO: 实现 epoll_ctl
        //
        // - Add: 添加新 fd
        // - Modify: 修改已有 fd 的事件
        // - Delete: 删除 fd
        todo!("epoll_ctl")
    }

    /// 等待事件 (epoll_wait)
    pub fn wait(&mut self, max_events: usize, timeout: Option<Duration>) -> io::Result<Vec<EpollEvent>> {
        // TODO: 等待事件并返回就绪的 fd
        //
        // 真实实现会调用 libc::epoll_wait
        // 这里我们模拟返回
        todo!("epoll_wait")
    }

    /// 注册 fd（便捷方法）
    pub fn add(&mut self, fd: RawFd, events: EpollEvents) -> io::Result<()> {
        self.ctl(EpollOp::Add, fd, Some(events))
    }

    /// 修改 fd（便捷方法）
    pub fn modify(&mut self, fd: RawFd, events: EpollEvents) -> io::Result<()> {
        self.ctl(EpollOp::Modify, fd, Some(events))
    }

    /// 删除 fd（便捷方法）
    pub fn delete(&mut self, fd: RawFd) -> io::Result<()> {
        self.ctl(EpollOp::Delete, fd, None)
    }
}

/// 边缘触发 vs 水平触发
pub struct TriggerModes;

impl TriggerModes {
    /// 解释水平触发
    pub fn level_triggered_explanation() -> &'static str {
        // TODO: 解释 LT 模式
        todo!("LT 解释")
    }

    /// 解释边缘触发
    pub fn edge_triggered_explanation() -> &'static str {
        // TODO: 解释 ET 模式
        todo!("ET 解释")
    }

    /// 边缘触发的注意事项
    pub fn edge_triggered_pitfalls() -> Vec<&'static str> {
        // TODO: 列出 ET 模式的注意事项
        // 提示: 必须非阻塞、必须读完所有数据
        todo!("ET 注意事项")
    }

    /// 模拟水平触发行为
    pub fn simulate_lt(buffer_has_data: bool, was_readable: bool) -> bool {
        // TODO: 返回是否应该触发事件
        // LT: 只要有数据就触发
        todo!("模拟 LT")
    }

    /// 模拟边缘触发行为
    pub fn simulate_et(buffer_has_data: bool, was_readable: bool) -> bool {
        // TODO: 返回是否应该触发事件
        // ET: 只在从无数据变为有数据时触发
        todo!("模拟 ET")
    }
}

/// 高性能 Epoll 服务器
#[cfg(target_os = "linux")]
pub struct EpollServer {
    listener: TcpListener,
    connections: HashMap<RawFd, TcpStream>,
    epoll: Epoll,
}

#[cfg(target_os = "linux")]
impl EpollServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        let mut epoll = Epoll::create()?;
        epoll.add(listener.as_raw_fd(), EpollEvents::EPOLLIN)?;

        Ok(Self {
            listener,
            connections: HashMap::new(),
            epoll,
        })
    }

    /// 使用边缘触发创建
    pub fn with_edge_trigger(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        let mut epoll = Epoll::create()?;
        epoll.add(
            listener.as_raw_fd(),
            EpollEvents::EPOLLIN.with(EpollEvents::EPOLLET),
        )?;

        Ok(Self {
            listener,
            connections: HashMap::new(),
            epoll,
        })
    }

    /// 运行事件循环
    pub fn run_once(&mut self) -> io::Result<Vec<Vec<u8>>> {
        // TODO: 实现 epoll 事件循环
        todo!("epoll 事件循环")
    }
}

/// epoll 的性能优势
pub struct EpollPerformance;

impl EpollPerformance {
    /// 为什么 epoll 是 O(1)？
    pub fn why_o1() -> &'static str {
        // TODO: 解释 epoll 的 O(1) 原理
        todo!("O(1) 原理")
    }

    /// epoll 的内部数据结构
    pub fn internal_structures() -> Vec<&'static str> {
        // TODO: 列出 epoll 使用的数据结构
        // 提示: 红黑树、就绪链表
        todo!("内部结构")
    }

    /// 在什么情况下 epoll 优于 poll？
    pub fn when_epoll_wins() -> &'static str {
        // TODO: 描述 epoll 的优势场景
        todo!("epoll 优势场景")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_epoll_events() {
        let events = EpollEvents::EPOLLIN.with(EpollEvents::EPOLLET);
        assert!(events.is_readable());
        assert!(events.is_edge_triggered());
        assert!(!events.is_writable());
    }

    #[test]
    fn test_io_epoll_event() {
        let event = EpollEvent::new(5, EpollEvents::EPOLLIN);
        assert_eq!(event.fd, 5);
        assert!(event.events.is_readable());
    }

    #[test]
    fn test_io_epoll_trigger_modes() {
        // 水平触发: 有数据就触发
        assert!(TriggerModes::simulate_lt(true, true));
        assert!(TriggerModes::simulate_lt(true, false));
        assert!(!TriggerModes::simulate_lt(false, true));

        // 边缘触发: 只在状态变化时触发
        assert!(TriggerModes::simulate_et(true, false)); // 从无到有
        assert!(!TriggerModes::simulate_et(true, true)); // 一直有
        assert!(!TriggerModes::simulate_et(false, false)); // 一直无
    }

    #[test]
    fn test_io_epoll_ctl() {
        let mut epoll = Epoll::create().unwrap();

        // 添加
        epoll.add(1, EpollEvents::EPOLLIN).unwrap();
        epoll.add(2, EpollEvents::EPOLLOUT).unwrap();

        // 修改
        epoll.modify(1, EpollEvents::EPOLLIN.with(EpollEvents::EPOLLOUT)).unwrap();

        // 删除
        epoll.delete(2).unwrap();
    }

    #[test]
    fn test_io_epoll_performance() {
        assert!(!EpollPerformance::internal_structures().is_empty());
    }
}
