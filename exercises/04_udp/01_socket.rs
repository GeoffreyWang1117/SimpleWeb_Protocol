//! # 练习 01: UDP Socket
//!
//! 使用 Rust 标准库的 `std::net::UdpSocket` 进行 UDP 通信。
//!
//! ## UDP Socket 基础
//!
//! ```rust
//! use std::net::UdpSocket;
//!
//! // 绑定到本地端口
//! let socket = UdpSocket::bind("127.0.0.1:0")?; // 0 表示自动分配
//!
//! // 发送数据
//! socket.send_to(b"Hello", "127.0.0.1:8080")?;
//!
//! // 接收数据
//! let mut buf = [0u8; 1024];
//! let (len, src) = socket.recv_from(&mut buf)?;
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test udp_socket
//! ```

use std::io::{self, ErrorKind};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

/// 创建一个绑定到指定地址的 UDP socket
///
/// # 参数
/// * `addr` - 要绑定的地址，格式为 "IP:PORT"
///
/// # 返回值
/// 成功返回 UdpSocket，失败返回错误
///
/// # 示例
/// ```
/// let socket = create_socket("127.0.0.1:0")?; // 绑定到随机端口
/// let socket = create_socket("0.0.0.0:8080")?; // 绑定到所有接口的 8080 端口
/// ```
pub fn create_socket(addr: &str) -> io::Result<UdpSocket> {
    // TODO: 使用 UdpSocket::bind 创建 socket
    todo!("创建 UDP socket")
}

/// 获取 socket 绑定的本地地址
///
/// # 参数
/// * `socket` - UDP socket
///
/// # 返回值
/// 本地 SocketAddr
pub fn get_local_addr(socket: &UdpSocket) -> io::Result<SocketAddr> {
    // TODO: 使用 socket.local_addr()
    todo!("获取本地地址")
}

/// 发送数据到指定地址
///
/// # 参数
/// * `socket` - UDP socket
/// * `data` - 要发送的数据
/// * `addr` - 目标地址
///
/// # 返回值
/// 发送的字节数
pub fn send_data(socket: &UdpSocket, data: &[u8], addr: &str) -> io::Result<usize> {
    // TODO: 使用 socket.send_to()
    todo!("发送数据")
}

/// 接收数据
///
/// # 参数
/// * `socket` - UDP socket
/// * `buf` - 接收缓冲区
///
/// # 返回值
/// (接收的字节数, 发送方地址)
pub fn receive_data(socket: &UdpSocket, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    // TODO: 使用 socket.recv_from()
    todo!("接收数据")
}

/// 设置 socket 的读取超时
///
/// # 参数
/// * `socket` - UDP socket
/// * `timeout` - 超时时间
pub fn set_read_timeout(socket: &UdpSocket, timeout: Option<Duration>) -> io::Result<()> {
    // TODO: 使用 socket.set_read_timeout()
    todo!("设置读取超时")
}

/// 设置 socket 为非阻塞模式
///
/// # 参数
/// * `socket` - UDP socket
/// * `nonblocking` - 是否非阻塞
pub fn set_nonblocking(socket: &UdpSocket, nonblocking: bool) -> io::Result<()> {
    // TODO: 使用 socket.set_nonblocking()
    todo!("设置非阻塞模式")
}

/// 连接到指定地址（可选的 UDP "连接"）
///
/// 连接后可以使用 send/recv 而不是 send_to/recv_from
///
/// # 参数
/// * `socket` - UDP socket
/// * `addr` - 目标地址
pub fn connect_to(socket: &UdpSocket, addr: &str) -> io::Result<()> {
    // TODO: 使用 socket.connect()
    todo!("连接到地址")
}

/// 简单的 UDP echo 客户端
///
/// 发送消息到服务器，等待并返回响应
///
/// # 参数
/// * `message` - 要发送的消息
/// * `server_addr` - 服务器地址
/// * `timeout` - 等待响应的超时时间
///
/// # 返回值
/// 服务器的响应消息
pub fn echo_client(message: &str, server_addr: &str, timeout: Duration) -> io::Result<String> {
    // TODO: 实现 echo 客户端
    //
    // 步骤:
    // 1. 创建 socket（绑定到随机端口）
    // 2. 设置读取超时
    // 3. 发送消息
    // 4. 接收响应
    // 5. 将响应转换为字符串
    todo!("实现 echo 客户端")
}

/// 简单的 UDP echo 服务器（处理单个请求）
///
/// # 参数
/// * `bind_addr` - 绑定地址
///
/// # 返回值
/// (收到的消息, 客户端地址)
pub fn echo_server_once(bind_addr: &str) -> io::Result<(String, SocketAddr)> {
    // TODO: 实现简单的 echo 服务器
    //
    // 步骤:
    // 1. 创建 socket 并绑定
    // 2. 接收数据
    // 3. 将相同的数据发送回客户端
    // 4. 返回收到的消息和客户端地址
    todo!("实现 echo 服务器")
}

/// 广播消息到本地网络
///
/// # 参数
/// * `socket` - UDP socket
/// * `message` - 要广播的消息
/// * `port` - 目标端口
///
/// # 注意
/// 需要先设置 socket 的 SO_BROADCAST 选项
pub fn broadcast_message(socket: &UdpSocket, message: &[u8], port: u16) -> io::Result<usize> {
    // TODO: 广播消息
    //
    // 步骤:
    // 1. 设置 broadcast 选项: socket.set_broadcast(true)
    // 2. 发送到广播地址 255.255.255.255:port
    todo!("广播消息")
}

/// 加入多播组
///
/// # 参数
/// * `socket` - UDP socket
/// * `multicast_addr` - 多播地址（如 "239.0.0.1"）
///
/// # 返回值
/// 成功返回 Ok(())
pub fn join_multicast(socket: &UdpSocket, multicast_addr: &str) -> io::Result<()> {
    // TODO: 加入多播组
    //
    // 使用 socket.join_multicast_v4()
    // 需要解析多播地址为 Ipv4Addr
    todo!("加入多播组")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_udp_socket_create() {
        let socket = create_socket("127.0.0.1:0").unwrap();
        let addr = get_local_addr(&socket).unwrap();
        assert!(addr.port() > 0);
    }

    #[test]
    fn test_udp_socket_send_receive() {
        // 创建两个 socket
        let server = create_socket("127.0.0.1:0").unwrap();
        let client = create_socket("127.0.0.1:0").unwrap();

        let server_addr = get_local_addr(&server).unwrap();
        let server_addr_str = format!("{}:{}", server_addr.ip(), server_addr.port());

        // 客户端发送
        let sent = send_data(&client, b"Hello, UDP!", &server_addr_str).unwrap();
        assert_eq!(sent, 11);

        // 服务器接收
        let mut buf = [0u8; 1024];
        let (len, src) = receive_data(&server, &mut buf).unwrap();

        assert_eq!(len, 11);
        assert_eq!(&buf[..len], b"Hello, UDP!");
    }

    #[test]
    fn test_udp_socket_timeout() {
        let socket = create_socket("127.0.0.1:0").unwrap();
        set_read_timeout(&socket, Some(Duration::from_millis(100))).unwrap();

        let mut buf = [0u8; 1024];
        let result = receive_data(&socket, &mut buf);

        // 应该超时
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut);
        }
    }

    #[test]
    fn test_udp_socket_nonblocking() {
        let socket = create_socket("127.0.0.1:0").unwrap();
        set_nonblocking(&socket, true).unwrap();

        let mut buf = [0u8; 1024];
        let result = receive_data(&socket, &mut buf);

        // 非阻塞模式下没有数据应该返回 WouldBlock
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::WouldBlock);
        }
    }

    #[test]
    fn test_udp_socket_connect() {
        let server = create_socket("127.0.0.1:0").unwrap();
        let client = create_socket("127.0.0.1:0").unwrap();

        let server_addr = get_local_addr(&server).unwrap();
        let server_addr_str = format!("{}:{}", server_addr.ip(), server_addr.port());

        // 连接到服务器
        connect_to(&client, &server_addr_str).unwrap();

        // 连接后可以使用 send 而不是 send_to
        // 但这个测试只验证 connect 不会出错
    }

    #[test]
    fn test_udp_socket_echo() {
        // 启动服务器线程
        let server = create_socket("127.0.0.1:0").unwrap();
        let server_addr = get_local_addr(&server).unwrap();

        let handle = thread::spawn(move || {
            let mut buf = [0u8; 1024];
            if let Ok((len, src)) = receive_data(&server, &mut buf) {
                let _ = server.send_to(&buf[..len], src);
            }
        });

        // 客户端发送并接收
        thread::sleep(Duration::from_millis(10));

        let addr_str = format!("127.0.0.1:{}", server_addr.port());
        let response = echo_client("Test message", &addr_str, Duration::from_secs(1));

        handle.join().unwrap();

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), "Test message");
    }

    #[test]
    fn test_udp_socket_broadcast() {
        let socket = create_socket("0.0.0.0:0").unwrap();

        // 尝试广播（可能在某些环境下失败，所以只测试不会 panic）
        let result = broadcast_message(&socket, b"Broadcast!", 9999);
        // 某些系统可能不允许广播，所以我们只检查函数能正常调用
        let _ = result;
    }
}
