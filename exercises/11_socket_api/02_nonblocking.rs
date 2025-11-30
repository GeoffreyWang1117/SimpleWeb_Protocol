//! # 练习 02: 非阻塞 I/O
//!
//! 非阻塞 I/O 是高性能网络编程的基础。
//!
//! ## 阻塞 vs 非阻塞
//!
//! ### 阻塞 I/O
//! ```text
//! read() ---> 等待数据 ---> 返回数据
//!             ^^^^^^^
//!             线程阻塞
//! ```
//!
//! ### 非阻塞 I/O
//! ```text
//! read() ---> 立即返回 (EWOULDBLOCK 或 数据)
//! ```
//!
//! ## 非阻塞模式的使用模式
//!
//! 1. **轮询**: 反复调用 read()
//! 2. **select/poll/epoll**: 等待多个 socket
//! 3. **事件驱动**: 结合事件循环
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test socket_nonblocking
//! ```

use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

/// 非阻塞读取结果
#[derive(Debug)]
pub enum NonBlockingResult<T> {
    /// 成功获取数据
    Ready(T),
    /// 没有数据可读（EWOULDBLOCK）
    WouldBlock,
    /// 连接关闭
    Closed,
    /// 发生错误
    Error(io::Error),
}

/// 非阻塞 TCP 连接
pub struct NonBlockingTcp {
    stream: TcpStream,
}

impl NonBlockingTcp {
    /// 创建非阻塞连接
    pub fn connect(addr: &str) -> io::Result<Self> {
        // TODO: 创建非阻塞连接
        // 1. 创建 TcpStream
        // 2. 设置为非阻塞模式
        todo!("创建非阻塞连接")
    }

    /// 从现有连接创建
    pub fn from_stream(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self { stream })
    }

    /// 非阻塞读取
    pub fn try_read(&mut self, buf: &mut [u8]) -> NonBlockingResult<usize> {
        // TODO: 实现非阻塞读取
        //
        // 处理错误类型:
        // - WouldBlock: 没有数据
        // - 返回 0: 连接关闭
        // - 其他错误: 真正的错误
        todo!("非阻塞读取")
    }

    /// 非阻塞写入
    pub fn try_write(&mut self, data: &[u8]) -> NonBlockingResult<usize> {
        // TODO: 实现非阻塞写入
        todo!("非阻塞写入")
    }

    /// 尝试读取直到 WouldBlock 或缓冲区满
    pub fn read_available(&mut self, buf: &mut Vec<u8>) -> NonBlockingResult<usize> {
        // TODO: 读取所有可用数据
        // 循环调用 try_read 直到 WouldBlock
        todo!("读取所有可用数据")
    }

    /// 获取底层 stream 的引用
    pub fn get_ref(&self) -> &TcpStream {
        &self.stream
    }
}

/// 非阻塞连接建立
///
/// connect() 在非阻塞模式下会立即返回，连接在后台建立
pub struct NonBlockingConnect {
    stream: TcpStream,
    connected: bool,
}

impl NonBlockingConnect {
    /// 开始非阻塞连接
    pub fn start(addr: &str) -> io::Result<Self> {
        // TODO: 开始非阻塞连接
        //
        // 在非阻塞模式下，connect() 可能返回:
        // - Ok(()): 连接立即成功（很少见）
        // - Err(EINPROGRESS/EWOULDBLOCK): 连接正在进行
        // - Err(其他): 真正的错误
        todo!("开始非阻塞连接")
    }

    /// 检查连接是否完成
    pub fn poll(&mut self) -> io::Result<bool> {
        // TODO: 检查连接状态
        //
        // 方法:
        // 1. 尝试获取 socket 错误 (SO_ERROR)
        // 2. 或者尝试写入 0 字节
        todo!("检查连接状态")
    }

    /// 等待连接完成（带超时）
    pub fn wait(mut self, timeout: std::time::Duration) -> io::Result<TcpStream> {
        // TODO: 等待连接完成
        todo!("等待连接完成")
    }

    /// 获取 stream（仅在连接成功后调用）
    pub fn into_stream(self) -> Option<TcpStream> {
        if self.connected {
            Some(self.stream)
        } else {
            None
        }
    }
}

/// 非阻塞 accept
pub struct NonBlockingListener {
    listener: TcpListener,
}

impl NonBlockingListener {
    /// 创建非阻塞监听器
    pub fn bind(addr: &str) -> io::Result<Self> {
        // TODO: 创建非阻塞监听器
        todo!("创建非阻塞监听器")
    }

    /// 尝试接受连接
    pub fn try_accept(&self) -> NonBlockingResult<(TcpStream, std::net::SocketAddr)> {
        // TODO: 非阻塞 accept
        todo!("非阻塞 accept")
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
}

/// 简单的轮询循环
///
/// 演示如何使用非阻塞 I/O 处理多个连接
pub struct SimplePoller {
    listeners: Vec<NonBlockingListener>,
    connections: Vec<NonBlockingTcp>,
}

impl SimplePoller {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// 添加监听器
    pub fn add_listener(&mut self, listener: NonBlockingListener) {
        self.listeners.push(listener);
    }

    /// 添加连接
    pub fn add_connection(&mut self, conn: NonBlockingTcp) {
        self.connections.push(conn);
    }

    /// 轮询一次
    ///
    /// 返回:
    /// - 新连接
    /// - 收到数据的连接索引和数据
    pub fn poll_once(&mut self) -> (Vec<TcpStream>, Vec<(usize, Vec<u8>)>) {
        // TODO: 实现轮询
        //
        // 1. 检查所有监听器是否有新连接
        // 2. 检查所有连接是否有数据
        // 3. 返回结果
        //
        // 注意: 这是一个简化的实现，真正的轮询应该使用 select/poll/epoll
        todo!("轮询")
    }
}

impl Default for SimplePoller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_socket_nonblocking_read() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = thread::spawn(move || {
            let (conn, _) = listener.accept().unwrap();
            thread::sleep(Duration::from_millis(100));
            drop(conn);
        });

        let stream = TcpStream::connect(addr).unwrap();
        let mut nb = NonBlockingTcp::from_stream(stream).unwrap();

        // 应该返回 WouldBlock，因为没有数据
        let mut buf = [0u8; 1024];
        match nb.try_read(&mut buf) {
            NonBlockingResult::WouldBlock => (), // 预期结果
            other => panic!("Expected WouldBlock, got {:?}", other),
        }

        handle.join().unwrap();
    }

    #[test]
    fn test_socket_nonblocking_listener() {
        let listener = NonBlockingListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // 没有连接时应该返回 WouldBlock
        match listener.try_accept() {
            NonBlockingResult::WouldBlock => (),
            other => panic!("Expected WouldBlock, got {:?}", other),
        }

        // 创建连接
        let _client = TcpStream::connect(addr).unwrap();
        thread::sleep(Duration::from_millis(10));

        // 现在应该能接受连接
        match listener.try_accept() {
            NonBlockingResult::Ready((_, _)) => (),
            other => panic!("Expected Ready, got {:?}", other),
        }
    }

    #[test]
    fn test_socket_nonblocking_write() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = thread::spawn(move || {
            let (mut conn, _) = listener.accept().unwrap();
            let mut buf = [0u8; 1024];
            let _ = conn.read(&mut buf);
        });

        let stream = TcpStream::connect(addr).unwrap();
        let mut nb = NonBlockingTcp::from_stream(stream).unwrap();

        // 写入应该成功
        match nb.try_write(b"Hello") {
            NonBlockingResult::Ready(n) => assert_eq!(n, 5),
            other => panic!("Expected Ready, got {:?}", other),
        }

        handle.join().unwrap();
    }
}
