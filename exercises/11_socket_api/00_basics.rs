//! # 练习 00: Socket 基础
//!
//! 理解 socket 的创建、地址绑定和基本 I/O 操作。
//!
//! ## Socket 地址族
//!
//! - `AF_INET`: IPv4
//! - `AF_INET6`: IPv6
//! - `AF_UNIX`: Unix 域套接字
//!
//! ## Socket 类型
//!
//! - `SOCK_STREAM`: TCP，面向连接的流
//! - `SOCK_DGRAM`: UDP，无连接的数据报
//! - `SOCK_RAW`: 原始套接字
//!
//! ## TCP 服务器流程
//!
//! ```text
//! socket() -> bind() -> listen() -> accept() -> read/write -> close()
//! ```
//!
//! ## TCP 客户端流程
//!
//! ```text
//! socket() -> connect() -> read/write -> close()
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test socket_basics
//! ```

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs};

/// Socket 地址包装器
///
/// 演示如何处理不同类型的地址
#[derive(Debug, Clone)]
pub enum SocketAddress {
    /// IPv4 地址
    V4(std::net::SocketAddrV4),
    /// IPv6 地址
    V6(std::net::SocketAddrV6),
}

impl SocketAddress {
    /// 从字符串解析地址
    ///
    /// 支持格式:
    /// - "127.0.0.1:8080"
    /// - "[::1]:8080"
    pub fn parse(s: &str) -> io::Result<Self> {
        // TODO: 解析地址字符串
        todo!("解析 socket 地址")
    }

    /// 转换为标准 SocketAddr
    pub fn to_socket_addr(&self) -> SocketAddr {
        match self {
            SocketAddress::V4(a) => SocketAddr::V4(*a),
            SocketAddress::V6(a) => SocketAddr::V6(*a),
        }
    }

    /// 检查是否是 IPv4
    pub fn is_ipv4(&self) -> bool {
        matches!(self, SocketAddress::V4(_))
    }

    /// 检查是否是回环地址
    pub fn is_loopback(&self) -> bool {
        // TODO: 检查是否是回环地址
        todo!("检查回环地址")
    }
}

/// 简单的 TCP 服务器
///
/// 演示基本的 socket 服务器操作
pub struct SimpleTcpServer {
    listener: TcpListener,
}

impl SimpleTcpServer {
    /// 创建并绑定服务器
    ///
    /// # 参数
    /// * `addr` - 绑定地址，如 "127.0.0.1:8080" 或 "0.0.0.0:8080"
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        // TODO: 创建 TCP 监听器
        todo!("绑定服务器")
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    /// 接受一个连接
    ///
    /// 这会阻塞直到有新连接
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        // TODO: 接受连接
        todo!("接受连接")
    }

    /// 设置为非阻塞模式
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.listener.set_nonblocking(nonblocking)
    }
}

/// 简单的 TCP 客户端
pub struct SimpleTcpClient {
    stream: TcpStream,
}

impl SimpleTcpClient {
    /// 连接到服务器
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        // TODO: 连接到服务器
        todo!("连接服务器")
    }

    /// 发送数据
    pub fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        // TODO: 发送数据
        todo!("发送数据")
    }

    /// 接收数据
    pub fn receive(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // TODO: 接收数据
        todo!("接收数据")
    }

    /// 发送全部数据（处理短写）
    pub fn send_all(&mut self, data: &[u8]) -> io::Result<()> {
        // TODO: 确保发送所有数据
        // write_all 会自动处理短写
        todo!("发送全部数据")
    }

    /// 接收指定长度的数据
    pub fn receive_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        // TODO: 确保接收指定长度
        // read_exact 会自动处理短读
        todo!("接收指定长度")
    }

    /// 获取对端地址
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.stream.local_addr()
    }

    /// 关闭写入端（发送 FIN）
    pub fn shutdown_write(&self) -> io::Result<()> {
        // TODO: 优雅关闭写入端
        todo!("关闭写入")
    }

    /// 关闭读取端
    pub fn shutdown_read(&self) -> io::Result<()> {
        // TODO: 关闭读取端
        todo!("关闭读取")
    }
}

/// 解析端口号
///
/// 支持格式:
/// - 数字: "8080"
/// - 服务名: "http" (需要查询 /etc/services)
pub fn parse_port(s: &str) -> Option<u16> {
    // TODO: 解析端口号
    // 先尝试解析为数字
    // 否则尝试查询服务名（可选）
    todo!("解析端口号")
}

/// 获取可用的本地端口
///
/// 绑定到端口 0 让操作系统分配一个可用端口
pub fn get_available_port() -> io::Result<u16> {
    // TODO: 获取可用端口
    // 绑定到 127.0.0.1:0，然后获取分配的端口
    todo!("获取可用端口")
}

/// 检查端口是否可用
pub fn is_port_available(port: u16) -> bool {
    // TODO: 尝试绑定端口，成功则可用
    todo!("检查端口可用性")
}

/// Echo 服务器（处理单个连接）
pub fn echo_once(listener: &TcpListener) -> io::Result<()> {
    // TODO: 实现简单的 echo 服务
    // 1. 接受连接
    // 2. 读取数据
    // 3. 写回相同的数据
    // 4. 关闭连接
    todo!("Echo 服务")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_socket_basics_address_parse() {
        let addr = SocketAddress::parse("127.0.0.1:8080").unwrap();
        assert!(addr.is_ipv4());
        assert!(addr.is_loopback());
    }

    #[test]
    fn test_socket_basics_server_bind() {
        let server = SimpleTcpServer::bind("127.0.0.1:0").unwrap();
        let addr = server.local_addr().unwrap();
        assert!(addr.port() > 0);
    }

    #[test]
    fn test_socket_basics_connect() {
        let server = SimpleTcpServer::bind("127.0.0.1:0").unwrap();
        let addr = server.local_addr().unwrap();

        // 在另一个线程中接受连接
        let handle = thread::spawn(move || {
            let (mut conn, _) = server.accept().unwrap();
            let mut buf = [0u8; 1024];
            let n = conn.read(&mut buf).unwrap();
            conn.write_all(&buf[..n]).unwrap();
        });

        // 客户端连接
        let mut client = SimpleTcpClient::connect(addr).unwrap();
        client.send(b"Hello").unwrap();

        let mut buf = [0u8; 1024];
        let n = client.receive(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"Hello");

        handle.join().unwrap();
    }

    #[test]
    fn test_socket_basics_port() {
        let port = get_available_port().unwrap();
        assert!(port > 0);
        assert!(is_port_available(port));
    }

    #[test]
    fn test_socket_basics_parse_port() {
        assert_eq!(parse_port("8080"), Some(8080));
        assert_eq!(parse_port("80"), Some(80));
        assert_eq!(parse_port("invalid"), None);
    }
}
