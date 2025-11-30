//! # 练习 01: Socket 选项
//!
//! Socket 选项用于配置 socket 的行为。
//!
//! ## 常用选项
//!
//! | 选项 | 级别 | 说明 |
//! |------|------|------|
//! | SO_REUSEADDR | SOL_SOCKET | 允许重用地址 |
//! | SO_REUSEPORT | SOL_SOCKET | 允许重用端口 |
//! | SO_KEEPALIVE | SOL_SOCKET | 启用 keepalive |
//! | SO_RCVBUF | SOL_SOCKET | 接收缓冲区大小 |
//! | SO_SNDBUF | SOL_SOCKET | 发送缓冲区大小 |
//! | SO_LINGER | SOL_SOCKET | 关闭时的行为 |
//! | TCP_NODELAY | IPPROTO_TCP | 禁用 Nagle 算法 |
//! | TCP_KEEPIDLE | IPPROTO_TCP | Keepalive 空闲时间 |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test socket_options
//! ```

use std::io;
use std::net::TcpStream;
use std::time::Duration;

/// Socket 选项配置
#[derive(Debug, Clone, Default)]
pub struct SocketOptions {
    /// SO_REUSEADDR - 允许立即重用地址
    pub reuse_addr: Option<bool>,
    /// SO_REUSEPORT - 允许多个 socket 绑定同一端口
    pub reuse_port: Option<bool>,
    /// SO_KEEPALIVE - 启用 TCP keepalive
    pub keepalive: Option<bool>,
    /// SO_RCVBUF - 接收缓冲区大小
    pub recv_buffer_size: Option<usize>,
    /// SO_SNDBUF - 发送缓冲区大小
    pub send_buffer_size: Option<usize>,
    /// TCP_NODELAY - 禁用 Nagle 算法
    pub nodelay: Option<bool>,
    /// SO_LINGER - 关闭时等待
    pub linger: Option<Option<Duration>>,
    /// 读取超时
    pub read_timeout: Option<Option<Duration>>,
    /// 写入超时
    pub write_timeout: Option<Option<Duration>>,
}

impl SocketOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 SO_REUSEADDR
    pub fn reuse_addr(mut self, value: bool) -> Self {
        self.reuse_addr = Some(value);
        self
    }

    /// 设置 TCP_NODELAY
    pub fn nodelay(mut self, value: bool) -> Self {
        self.nodelay = Some(value);
        self
    }

    /// 设置 SO_KEEPALIVE
    pub fn keepalive(mut self, value: bool) -> Self {
        self.keepalive = Some(value);
        self
    }

    /// 设置接收缓冲区大小
    pub fn recv_buffer(mut self, size: usize) -> Self {
        self.recv_buffer_size = Some(size);
        self
    }

    /// 设置发送缓冲区大小
    pub fn send_buffer(mut self, size: usize) -> Self {
        self.send_buffer_size = Some(size);
        self
    }

    /// 设置读取超时
    pub fn read_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    /// 设置写入超时
    pub fn write_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.write_timeout = Some(timeout);
        self
    }

    /// 设置 SO_LINGER
    pub fn linger(mut self, linger: Option<Duration>) -> Self {
        self.linger = Some(linger);
        self
    }

    /// 应用选项到 TcpStream
    pub fn apply(&self, stream: &TcpStream) -> io::Result<()> {
        // TODO: 应用所有设置的选项
        //
        // 使用 TcpStream 的方法:
        // - set_nodelay()
        // - set_read_timeout()
        // - set_write_timeout()
        //
        // 其他选项需要使用 setsockopt 系统调用
        // 在 Rust 中可以使用 socket2 crate
        todo!("应用 socket 选项")
    }
}

/// Nagle 算法说明
///
/// Nagle 算法会将小数据包合并发送，以减少网络开销。
/// 但这会增加延迟，对于实时应用应该禁用。
///
/// 禁用 Nagle (TCP_NODELAY = true):
/// - 低延迟应用
/// - 交互式应用
/// - 游戏
///
/// 启用 Nagle (TCP_NODELAY = false):
/// - 批量数据传输
/// - 不关心延迟的应用
pub fn configure_for_low_latency(stream: &TcpStream) -> io::Result<()> {
    // TODO: 配置低延迟
    // 1. 禁用 Nagle (set_nodelay(true))
    // 2. 可能减小缓冲区
    todo!("配置低延迟")
}

/// 配置高吞吐量
pub fn configure_for_throughput(stream: &TcpStream) -> io::Result<()> {
    // TODO: 配置高吞吐量
    // 1. 启用 Nagle (set_nodelay(false))
    // 2. 增大缓冲区
    todo!("配置高吞吐量")
}

/// SO_LINGER 选项说明
///
/// 控制 close() 的行为:
///
/// - linger = None: 默认行为，close() 立即返回
/// - linger = Some(Duration::ZERO): 发送 RST，立即关闭
/// - linger = Some(duration): close() 阻塞直到数据发送完成或超时
#[derive(Debug, Clone, Copy)]
pub enum LingerBehavior {
    /// 默认行为：后台发送
    Default,
    /// 立即关闭，发送 RST
    Abort,
    /// 等待发送完成
    Wait(Duration),
}

impl LingerBehavior {
    /// 转换为 linger 选项值
    pub fn to_linger(&self) -> Option<Duration> {
        match self {
            LingerBehavior::Default => None,
            LingerBehavior::Abort => Some(Duration::ZERO),
            LingerBehavior::Wait(d) => Some(*d),
        }
    }
}

/// 获取当前 socket 选项
pub struct SocketInfo;

impl SocketInfo {
    /// 获取接收缓冲区大小
    pub fn recv_buffer_size(_stream: &TcpStream) -> io::Result<usize> {
        // TODO: 获取接收缓冲区大小
        // 需要使用 getsockopt
        todo!("获取接收缓冲区大小")
    }

    /// 获取发送缓冲区大小
    pub fn send_buffer_size(_stream: &TcpStream) -> io::Result<usize> {
        // TODO: 获取发送缓冲区大小
        todo!("获取发送缓冲区大小")
    }

    /// 获取 TCP_NODELAY 状态
    pub fn nodelay(stream: &TcpStream) -> io::Result<bool> {
        stream.nodelay()
    }

    /// 获取读取超时
    pub fn read_timeout(stream: &TcpStream) -> io::Result<Option<Duration>> {
        stream.read_timeout()
    }

    /// 获取写入超时
    pub fn write_timeout(stream: &TcpStream) -> io::Result<Option<Duration>> {
        stream.write_timeout()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    fn create_connected_pair() -> io::Result<(TcpStream, TcpStream)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;

        let client = TcpStream::connect(addr)?;
        let (server, _) = listener.accept()?;

        Ok((client, server))
    }

    #[test]
    fn test_socket_options_nodelay() {
        let (client, _server) = create_connected_pair().unwrap();

        client.set_nodelay(true).unwrap();
        assert!(SocketInfo::nodelay(&client).unwrap());

        client.set_nodelay(false).unwrap();
        assert!(!SocketInfo::nodelay(&client).unwrap());
    }

    #[test]
    fn test_socket_options_timeout() {
        let (client, _server) = create_connected_pair().unwrap();

        let timeout = Some(Duration::from_secs(5));
        client.set_read_timeout(timeout).unwrap();
        assert_eq!(SocketInfo::read_timeout(&client).unwrap(), timeout);

        client.set_write_timeout(timeout).unwrap();
        assert_eq!(SocketInfo::write_timeout(&client).unwrap(), timeout);
    }

    #[test]
    fn test_socket_options_builder() {
        let (client, _server) = create_connected_pair().unwrap();

        let opts = SocketOptions::new()
            .nodelay(true)
            .read_timeout(Some(Duration::from_secs(10)))
            .write_timeout(Some(Duration::from_secs(10)));

        opts.apply(&client).unwrap();

        assert!(client.nodelay().unwrap());
    }

    #[test]
    fn test_socket_options_low_latency() {
        let (client, _server) = create_connected_pair().unwrap();

        configure_for_low_latency(&client).unwrap();
        assert!(client.nodelay().unwrap());
    }
}
