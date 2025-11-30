//! # 练习 01: poll
//!
//! poll 是 select 的改进版本（1986年，System V）。
//!
//! ## 改进点
//!
//! 1. **无 fd 数量限制**: 使用数组而非位图
//! 2. **事件分离**: 请求事件和返回事件分开
//!
//! ## 数据结构
//!
//! ```c
//! struct pollfd {
//!     int   fd;      // 文件描述符
//!     short events;  // 请求的事件
//!     short revents; // 返回的事件
//! };
//! ```
//!
//! ## 事件类型
//!
//! | 事件 | 说明 |
//! |------|------|
//! | POLLIN | 可读 |
//! | POLLOUT | 可写 |
//! | POLLERR | 错误 |
//! | POLLHUP | 挂起 |
//! | POLLNVAL | 无效 fd |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test io_poll
//! ```

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

/// Poll 事件标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollEvents(u16);

impl PollEvents {
    /// 无事件
    pub const NONE: PollEvents = PollEvents(0);
    /// 可读
    pub const POLLIN: PollEvents = PollEvents(0x001);
    /// 可写
    pub const POLLOUT: PollEvents = PollEvents(0x004);
    /// 错误
    pub const POLLERR: PollEvents = PollEvents(0x008);
    /// 挂起（对方关闭）
    pub const POLLHUP: PollEvents = PollEvents(0x010);
    /// 无效 fd
    pub const POLLNVAL: PollEvents = PollEvents(0x020);

    /// 创建事件集
    pub fn new() -> Self {
        Self::NONE
    }

    /// 添加事件
    pub fn with(self, other: PollEvents) -> Self {
        PollEvents(self.0 | other.0)
    }

    /// 检查是否包含事件
    pub fn contains(&self, other: PollEvents) -> bool {
        // TODO: 检查是否包含指定事件
        todo!("检查事件")
    }

    /// 是否可读
    pub fn is_readable(&self) -> bool {
        self.contains(Self::POLLIN)
    }

    /// 是否可写
    pub fn is_writable(&self) -> bool {
        self.contains(Self::POLLOUT)
    }

    /// 是否有错误
    pub fn has_error(&self) -> bool {
        self.contains(Self::POLLERR) || self.contains(Self::POLLNVAL)
    }

    /// 是否挂起
    pub fn is_hangup(&self) -> bool {
        self.contains(Self::POLLHUP)
    }
}

impl Default for PollEvents {
    fn default() -> Self {
        Self::new()
    }
}

/// pollfd 结构
#[derive(Debug, Clone)]
pub struct PollFd {
    /// 文件描述符
    pub fd: RawFd,
    /// 请求的事件
    pub events: PollEvents,
    /// 返回的事件
    pub revents: PollEvents,
}

impl PollFd {
    /// 创建 pollfd
    pub fn new(fd: RawFd, events: PollEvents) -> Self {
        // TODO: 创建 PollFd
        todo!("创建 PollFd")
    }

    /// 检查是否就绪
    pub fn is_ready(&self) -> bool {
        self.revents.0 != 0
    }

    /// 清除返回事件
    pub fn clear_revents(&mut self) {
        self.revents = PollEvents::NONE;
    }
}

/// Poll 包装器
pub struct Poll {
    fds: Vec<PollFd>,
    fd_index: HashMap<RawFd, usize>,
}

impl Poll {
    pub fn new() -> Self {
        Self {
            fds: Vec::new(),
            fd_index: HashMap::new(),
        }
    }

    /// 注册 fd
    pub fn register(&mut self, fd: RawFd, events: PollEvents) {
        // TODO: 注册 fd 和感兴趣的事件
        // 如果 fd 已存在，更新事件
        todo!("注册 fd")
    }

    /// 注销 fd
    pub fn unregister(&mut self, fd: RawFd) {
        // TODO: 移除 fd
        todo!("注销 fd")
    }

    /// 修改 fd 的事件
    pub fn modify(&mut self, fd: RawFd, events: PollEvents) {
        // TODO: 修改已注册 fd 的事件
        todo!("修改事件")
    }

    /// 执行 poll
    pub fn poll(&mut self, timeout: Option<Duration>) -> io::Result<Vec<(RawFd, PollEvents)>> {
        // TODO: 执行 poll 并返回就绪的 fd
        //
        // 在真实实现中，会调用:
        // libc::poll(fds.as_mut_ptr(), fds.len(), timeout_ms)
        //
        // 返回值: 就绪的 fd 数量
        // - > 0: 有 fd 就绪
        // - = 0: 超时
        // - < 0: 错误
        todo!("执行 poll")
    }

    /// 获取注册的 fd 数量
    pub fn len(&self) -> usize {
        self.fds.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.fds.is_empty()
    }
}

impl Default for Poll {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用 poll 的回显服务器
pub struct PollEchoServer {
    listener: TcpListener,
    poll: Poll,
    connections: HashMap<RawFd, TcpStream>,
    listener_fd: RawFd,
}

impl PollEchoServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let listener_fd = listener.as_raw_fd();

        let mut poll = Poll::new();
        poll.register(listener_fd, PollEvents::POLLIN);

        Ok(Self {
            listener,
            poll,
            connections: HashMap::new(),
            listener_fd,
        })
    }

    /// 接受新连接
    fn accept_connection(&mut self) -> io::Result<()> {
        // TODO: 接受新连接并注册到 poll
        todo!("接受连接")
    }

    /// 处理客户端数据
    fn handle_client(&mut self, fd: RawFd) -> io::Result<Option<Vec<u8>>> {
        // TODO: 读取数据并回显
        // 返回收到的数据（用于测试）
        todo!("处理客户端")
    }

    /// 运行一次事件循环
    pub fn poll_once(&mut self, timeout: Option<Duration>) -> io::Result<Vec<Vec<u8>>> {
        // TODO: 实现事件循环
        //
        // 1. 调用 poll
        // 2. 处理 listener 的新连接
        // 3. 处理客户端数据
        // 4. 处理断开连接
        todo!("事件循环")
    }

    /// 获取连接数
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
}

/// poll 与 select 的对比
pub struct PollVsSelect;

impl PollVsSelect {
    /// poll 相对于 select 的优势
    pub fn poll_advantages() -> Vec<&'static str> {
        // TODO: 列出 poll 的优势
        todo!("poll 优势")
    }

    /// poll 仍然存在的问题
    pub fn poll_disadvantages() -> Vec<&'static str> {
        // TODO: 列出 poll 的缺点
        // 提示: 仍然需要遍历所有 fd
        todo!("poll 缺点")
    }

    /// 何时使用 select vs poll
    pub fn when_to_use() -> (&'static str, &'static str) {
        // TODO: 返回 (使用 select 的场景, 使用 poll 的场景)
        todo!("使用场景")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_poll_events() {
        let events = PollEvents::POLLIN.with(PollEvents::POLLOUT);
        assert!(events.contains(PollEvents::POLLIN));
        assert!(events.contains(PollEvents::POLLOUT));
        assert!(!events.contains(PollEvents::POLLERR));
    }

    #[test]
    fn test_io_poll_fd() {
        let pfd = PollFd::new(5, PollEvents::POLLIN);
        assert_eq!(pfd.fd, 5);
        assert!(pfd.events.is_readable());
        assert!(!pfd.events.is_writable());
    }

    #[test]
    fn test_io_poll_register() {
        let mut poll = Poll::new();

        poll.register(1, PollEvents::POLLIN);
        poll.register(2, PollEvents::POLLOUT);

        assert_eq!(poll.len(), 2);

        poll.unregister(1);
        assert_eq!(poll.len(), 1);
    }

    #[test]
    fn test_io_poll_vs_select() {
        let advantages = PollVsSelect::poll_advantages();
        assert!(!advantages.is_empty());

        let disadvantages = PollVsSelect::poll_disadvantages();
        assert!(!disadvantages.is_empty());
    }
}
