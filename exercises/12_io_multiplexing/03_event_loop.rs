//! # 练习 03: 事件循环
//!
//! 基于 I/O 多路复用构建事件驱动系统。
//!
//! ## 事件循环模式
//!
//! ```text
//! loop {
//!     events = poll_for_events(timeout);
//!     for event in events {
//!         handler = get_handler(event.fd);
//!         handler.handle(event);
//!     }
//! }
//! ```
//!
//! ## Reactor 模式
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │             Reactor                  │
//! │  ┌─────────┐    ┌──────────────┐    │
//! │  │ Selector│───>│Event Handlers│    │
//! │  └─────────┘    └──────────────┘    │
//! │       │              │               │
//! │       v              v               │
//! │  ┌─────────────────────┐            │
//! │  │   Handle Table      │            │
//! │  │  fd -> handler      │            │
//! │  └─────────────────────┘            │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## 跨平台抽象
//!
//! | 平台 | 实现 |
//! |------|------|
//! | Linux | epoll |
//! | macOS/BSD | kqueue |
//! | Windows | IOCP |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test io_event_loop
//! ```

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

/// 事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interest {
    /// 可读事件
    Readable,
    /// 可写事件
    Writable,
    /// 读写事件
    Both,
}

impl Interest {
    /// 是否包含可读
    pub fn is_readable(&self) -> bool {
        matches!(self, Interest::Readable | Interest::Both)
    }

    /// 是否包含可写
    pub fn is_writable(&self) -> bool {
        matches!(self, Interest::Writable | Interest::Both)
    }
}

/// 就绪事件
#[derive(Debug, Clone)]
pub struct Event {
    /// 事件源 ID
    pub token: Token,
    /// 是否可读
    pub readable: bool,
    /// 是否可写
    pub writable: bool,
    /// 是否出错
    pub error: bool,
    /// 是否关闭
    pub closed: bool,
}

impl Event {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            readable: false,
            writable: false,
            error: false,
            closed: false,
        }
    }

    /// 是否可读
    pub fn is_readable(&self) -> bool {
        self.readable
    }

    /// 是否可写
    pub fn is_writable(&self) -> bool {
        self.writable
    }

    /// 是否需要处理
    pub fn is_active(&self) -> bool {
        // TODO: 检查事件是否需要处理
        todo!("检查事件")
    }
}

/// 事件源标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token(pub usize);

/// 跨平台选择器（抽象层）
pub struct Selector {
    /// fd 到 token 的映射
    fd_to_token: HashMap<RawFd, Token>,
    /// token 到 fd 的映射
    token_to_fd: HashMap<Token, RawFd>,
    /// 感兴趣的事件
    interests: HashMap<RawFd, Interest>,
    /// 下一个 token
    next_token: usize,
}

impl Selector {
    pub fn new() -> io::Result<Self> {
        // TODO: 创建选择器
        // 在真实实现中，这里会调用 epoll_create/kqueue
        todo!("创建选择器")
    }

    /// 注册 fd
    pub fn register(&mut self, fd: RawFd, interest: Interest) -> io::Result<Token> {
        // TODO: 注册 fd 并返回 token
        todo!("注册 fd")
    }

    /// 重新注册（修改事件）
    pub fn reregister(&mut self, token: Token, interest: Interest) -> io::Result<()> {
        // TODO: 修改 token 对应的事件
        todo!("重新注册")
    }

    /// 注销 fd
    pub fn deregister(&mut self, token: Token) -> io::Result<()> {
        // TODO: 注销 token
        todo!("注销")
    }

    /// 等待事件
    pub fn select(&mut self, timeout: Option<Duration>) -> io::Result<Vec<Event>> {
        // TODO: 等待事件并返回就绪的事件
        // 真实实现会调用 epoll_wait/kevent
        todo!("等待事件")
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// 事件处理器 trait
pub trait EventHandler {
    /// 处理可读事件
    fn on_readable(&mut self, token: Token) -> io::Result<()>;
    /// 处理可写事件
    fn on_writable(&mut self, token: Token) -> io::Result<()>;
    /// 处理错误
    fn on_error(&mut self, token: Token, err: io::Error);
    /// 处理关闭
    fn on_close(&mut self, token: Token);
}

/// 简单的事件循环
pub struct EventLoop {
    selector: Selector,
    running: bool,
}

impl EventLoop {
    pub fn new() -> io::Result<Self> {
        // TODO: 创建事件循环
        todo!("创建事件循环")
    }

    /// 注册 fd
    pub fn register(&mut self, fd: RawFd, interest: Interest) -> io::Result<Token> {
        self.selector.register(fd, interest)
    }

    /// 运行一次事件循环
    pub fn poll(&mut self, timeout: Option<Duration>) -> io::Result<Vec<Event>> {
        // TODO: 执行一次轮询
        todo!("轮询")
    }

    /// 运行事件循环
    pub fn run<H: EventHandler>(&mut self, handler: &mut H) -> io::Result<()> {
        // TODO: 实现事件循环
        //
        // while self.running {
        //     let events = self.poll(Some(Duration::from_millis(100)))?;
        //     for event in events {
        //         if event.is_readable() {
        //             handler.on_readable(event.token)?;
        //         }
        //         // ... 处理其他事件
        //     }
        // }
        todo!("运行事件循环")
    }

    /// 停止事件循环
    pub fn stop(&mut self) {
        self.running = false;
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// 基于事件循环的 TCP 服务器
pub struct TcpServer {
    listener: TcpListener,
    listener_token: Token,
    connections: HashMap<Token, TcpStream>,
    event_loop: EventLoop,
    read_buffers: HashMap<Token, Vec<u8>>,
}

impl TcpServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        let mut event_loop = EventLoop::new()?;
        let listener_token = event_loop.register(listener.as_raw_fd(), Interest::Readable)?;

        Ok(Self {
            listener,
            listener_token,
            connections: HashMap::new(),
            event_loop,
            read_buffers: HashMap::new(),
        })
    }

    /// 处理新连接
    fn accept(&mut self) -> io::Result<Token> {
        // TODO: 接受新连接并注册
        todo!("接受连接")
    }

    /// 处理客户端数据
    fn handle_read(&mut self, token: Token) -> io::Result<Option<Vec<u8>>> {
        // TODO: 读取并处理数据
        todo!("读取数据")
    }

    /// 运行一次
    pub fn poll_once(&mut self) -> io::Result<Vec<(Token, Vec<u8>)>> {
        // TODO: 实现服务器事件循环
        todo!("服务器事件循环")
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
}

/// 定时器支持
pub struct Timer {
    /// 到期时间
    pub deadline: std::time::Instant,
    /// 关联的 token
    pub token: Token,
    /// 是否重复
    pub repeat: bool,
    /// 重复间隔
    pub interval: Option<Duration>,
}

impl Timer {
    /// 创建一次性定时器
    pub fn once(delay: Duration, token: Token) -> Self {
        // TODO: 创建一次性定时器
        todo!("创建定时器")
    }

    /// 创建重复定时器
    pub fn repeat(interval: Duration, token: Token) -> Self {
        // TODO: 创建重复定时器
        todo!("创建重复定时器")
    }

    /// 是否已过期
    pub fn is_expired(&self) -> bool {
        // TODO: 检查是否过期
        todo!("检查过期")
    }

    /// 重置定时器（用于重复定时器）
    pub fn reset(&mut self) {
        // TODO: 重置到下一个间隔
        todo!("重置定时器")
    }
}

/// 带定时器的事件循环
pub struct TimerEventLoop {
    event_loop: EventLoop,
    timers: Vec<Timer>,
}

impl TimerEventLoop {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            event_loop: EventLoop::new()?,
            timers: Vec::new(),
        })
    }

    /// 添加定时器
    pub fn add_timer(&mut self, timer: Timer) {
        self.timers.push(timer);
    }

    /// 获取最近的定时器超时
    fn next_timeout(&self) -> Option<Duration> {
        // TODO: 计算到最近定时器的时间
        todo!("计算超时")
    }

    /// 处理过期定时器
    fn process_timers(&mut self) -> Vec<Token> {
        // TODO: 处理所有过期的定时器
        // 返回触发的定时器 token
        todo!("处理定时器")
    }
}

impl Default for TimerEventLoop {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// 真实世界的框架对比
pub struct RealWorldFrameworks;

impl RealWorldFrameworks {
    /// Tokio 的 reactor 实现
    pub fn tokio_approach() -> &'static str {
        // TODO: 描述 Tokio 的实现方式
        todo!("Tokio 方式")
    }

    /// mio 的设计
    pub fn mio_design() -> &'static str {
        // TODO: 描述 mio 的设计
        todo!("mio 设计")
    }

    /// async-std 的方式
    pub fn async_std_approach() -> &'static str {
        // TODO: 描述 async-std
        todo!("async-std 方式")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_event_loop_interest() {
        assert!(Interest::Readable.is_readable());
        assert!(!Interest::Readable.is_writable());
        assert!(Interest::Both.is_readable());
        assert!(Interest::Both.is_writable());
    }

    #[test]
    fn test_io_event_loop_event() {
        let mut event = Event::new(Token(1));
        event.readable = true;

        assert!(event.is_readable());
        assert!(!event.is_writable());
        assert!(event.is_active());
    }

    #[test]
    fn test_io_event_loop_selector() {
        let mut selector = Selector::new().unwrap();

        let token1 = selector.register(1, Interest::Readable).unwrap();
        let token2 = selector.register(2, Interest::Writable).unwrap();

        assert_ne!(token1, token2);

        selector.deregister(token1).unwrap();
    }

    #[test]
    fn test_io_event_loop_timer() {
        let timer = Timer::once(Duration::from_millis(100), Token(1));
        assert!(!timer.is_expired());

        let repeat_timer = Timer::repeat(Duration::from_secs(1), Token(2));
        assert!(repeat_timer.repeat);
    }
}
