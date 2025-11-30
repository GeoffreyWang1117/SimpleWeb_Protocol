//! # 练习 00: select
//!
//! select 是最古老的 I/O 多路复用机制（1983年，4.2BSD）。
//!
//! ## 工作原理
//!
//! ```text
//! fd_set readfds;    // 要监控可读的 fd 集合
//! fd_set writefds;   // 要监控可写的 fd 集合
//! fd_set exceptfds;  // 要监控异常的 fd 集合
//!
//! select(nfds, &readfds, &writefds, &exceptfds, timeout)
//!   |
//!   v
//! 阻塞等待，直到:
//! - 有 fd 就绪
//! - 超时
//! - 被信号中断
//! ```
//!
//! ## 限制
//!
//! 1. **fd 数量限制**: 通常最多 1024 个 fd (FD_SETSIZE)
//! 2. **效率问题**: 每次调用都要复制整个 fd_set
//! 3. **遍历开销**: 需要遍历所有 fd 检查就绪状态
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test io_select
//! ```

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

/// fd_set 的 Rust 实现
///
/// 在 C 中，fd_set 是一个位图，每个 bit 代表一个 fd
#[derive(Clone, Default)]
pub struct FdSet {
    fds: Vec<RawFd>,
}

impl FdSet {
    pub fn new() -> Self {
        Self { fds: Vec::new() }
    }

    /// 添加 fd 到集合 (FD_SET)
    pub fn set(&mut self, fd: RawFd) {
        // TODO: 添加 fd 到集合（避免重复）
        todo!("添加 fd")
    }

    /// 从集合移除 fd (FD_CLR)
    pub fn clear(&mut self, fd: RawFd) {
        // TODO: 从集合移除 fd
        todo!("移除 fd")
    }

    /// 检查 fd 是否在集合中 (FD_ISSET)
    pub fn is_set(&self, fd: RawFd) -> bool {
        // TODO: 检查 fd 是否存在
        todo!("检查 fd")
    }

    /// 清空集合 (FD_ZERO)
    pub fn zero(&mut self) {
        // TODO: 清空所有 fd
        todo!("清空集合")
    }

    /// 获取最大 fd + 1 (用于 select 的 nfds 参数)
    pub fn max_fd(&self) -> RawFd {
        // TODO: 返回最大 fd + 1，如果为空返回 0
        todo!("获取最大 fd")
    }

    /// 获取所有 fd
    pub fn fds(&self) -> &[RawFd] {
        &self.fds
    }
}

/// Select 事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectEvent {
    /// 可读
    Readable,
    /// 可写
    Writable,
    /// 异常
    Exception,
}

/// Select 结果
#[derive(Debug)]
pub struct SelectResult {
    /// 就绪的 fd 和事件
    pub ready: Vec<(RawFd, SelectEvent)>,
    /// 是否超时
    pub timed_out: bool,
}

/// 简化的 select 包装
///
/// 真正的 select 使用 libc，这里我们模拟其行为
pub struct Select {
    read_fds: FdSet,
    write_fds: FdSet,
    except_fds: FdSet,
}

impl Select {
    pub fn new() -> Self {
        Self {
            read_fds: FdSet::new(),
            write_fds: FdSet::new(),
            except_fds: FdSet::new(),
        }
    }

    /// 监控 fd 的可读事件
    pub fn watch_read(&mut self, fd: RawFd) {
        self.read_fds.set(fd);
    }

    /// 监控 fd 的可写事件
    pub fn watch_write(&mut self, fd: RawFd) {
        self.write_fds.set(fd);
    }

    /// 停止监控 fd
    pub fn unwatch(&mut self, fd: RawFd) {
        self.read_fds.clear(fd);
        self.write_fds.clear(fd);
        self.except_fds.clear(fd);
    }

    /// 执行 select（模拟）
    ///
    /// 真正的实现会调用 libc::select
    pub fn select(&mut self, _timeout: Option<Duration>) -> io::Result<SelectResult> {
        // TODO: 模拟 select 行为
        //
        // 在真实实现中，会调用:
        // libc::select(nfds, readfds, writefds, exceptfds, timeout)
        //
        // 这里我们返回一个简化的结果
        todo!("执行 select")
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用 select 的简单服务器
///
/// 演示如何用 select 处理多个连接
pub struct SelectServer {
    listener: TcpListener,
    connections: HashMap<RawFd, TcpStream>,
    select: Select,
}

impl SelectServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        let mut select = Select::new();
        select.watch_read(listener.as_raw_fd());

        Ok(Self {
            listener,
            connections: HashMap::new(),
            select,
        })
    }

    /// 运行一次事件循环
    pub fn poll_once(&mut self) -> io::Result<Vec<Vec<u8>>> {
        // TODO: 实现事件循环
        //
        // 1. 调用 select 等待事件
        // 2. 检查 listener 是否有新连接
        // 3. 检查每个连接是否有数据
        // 4. 返回收到的所有数据
        todo!("事件循环")
    }

    /// 获取连接数
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

/// select 的时间复杂度分析
pub struct SelectComplexity;

impl SelectComplexity {
    /// 每次 select 调用的时间复杂度
    pub fn per_call() -> &'static str {
        // TODO: 返回时间复杂度 "O(n)" 或 "O(1)"
        // 提示: select 需要遍历所有 fd
        todo!("时间复杂度")
    }

    /// 为什么 select 有 fd 数量限制？
    pub fn why_fd_limit() -> &'static str {
        // TODO: 解释原因
        // 提示: fd_set 是固定大小的位图
        todo!("fd 限制原因")
    }

    /// select 的主要缺点
    pub fn disadvantages() -> Vec<&'static str> {
        // TODO: 列出 select 的缺点
        todo!("select 缺点")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_select_fdset_basic() {
        let mut fds = FdSet::new();

        fds.set(1);
        fds.set(5);
        fds.set(3);

        assert!(fds.is_set(1));
        assert!(fds.is_set(5));
        assert!(fds.is_set(3));
        assert!(!fds.is_set(2));

        assert_eq!(fds.max_fd(), 6); // max fd + 1
    }

    #[test]
    fn test_io_select_fdset_clear() {
        let mut fds = FdSet::new();

        fds.set(1);
        fds.set(2);
        fds.clear(1);

        assert!(!fds.is_set(1));
        assert!(fds.is_set(2));
    }

    #[test]
    fn test_io_select_fdset_zero() {
        let mut fds = FdSet::new();

        fds.set(1);
        fds.set(2);
        fds.zero();

        assert!(!fds.is_set(1));
        assert!(!fds.is_set(2));
        assert_eq!(fds.max_fd(), 0);
    }

    #[test]
    fn test_io_select_complexity() {
        assert_eq!(SelectComplexity::per_call(), "O(n)");
        assert!(!SelectComplexity::disadvantages().is_empty());
    }
}
