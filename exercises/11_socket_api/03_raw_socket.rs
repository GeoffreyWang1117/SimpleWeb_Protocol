//! # 练习 03: 原始套接字
//!
//! 原始套接字允许直接访问网络层，绕过传输层。
//!
//! ## 用途
//!
//! - 实现自定义协议
//! - 网络诊断工具（ping, traceroute）
//! - 数据包捕获和注入
//! - 安全测试
//!
//! ## 权限要求
//!
//! - Linux: 需要 CAP_NET_RAW 能力或 root 权限
//! - Windows: 需要管理员权限
//! - macOS: 需要 root 权限
//!
//! ## 安全警告
//!
//! 原始套接字可以用于网络攻击，仅用于授权的测试和学习！
//!
//! ## 如何运行测试
//!
//! ```bash
//! # 需要 root 权限
//! sudo cargo test socket_raw
//! ```

/// ICMP 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
    /// Echo Reply (ping 响应)
    EchoReply = 0,
    /// Destination Unreachable
    DestUnreachable = 3,
    /// Echo Request (ping 请求)
    EchoRequest = 8,
    /// Time Exceeded (traceroute 使用)
    TimeExceeded = 11,
}

impl IcmpType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(IcmpType::EchoReply),
            3 => Some(IcmpType::DestUnreachable),
            8 => Some(IcmpType::EchoRequest),
            11 => Some(IcmpType::TimeExceeded),
            _ => None,
        }
    }
}

/// ICMP 头部
#[derive(Debug, Clone)]
pub struct IcmpHeader {
    /// 消息类型
    pub icmp_type: u8,
    /// 代码
    pub code: u8,
    /// 校验和
    pub checksum: u16,
    /// 标识符（用于匹配请求和响应）
    pub identifier: u16,
    /// 序列号
    pub sequence: u16,
}

impl IcmpHeader {
    /// ICMP 头部大小
    pub const SIZE: usize = 8;

    /// 创建 Echo Request
    pub fn echo_request(identifier: u16, sequence: u16) -> Self {
        // TODO: 创建 Echo Request 头部
        // type = 8, code = 0
        todo!("创建 ICMP Echo Request")
    }

    /// 创建 Echo Reply
    pub fn echo_reply(identifier: u16, sequence: u16) -> Self {
        // TODO: 创建 Echo Reply 头部
        todo!("创建 ICMP Echo Reply")
    }

    /// 从字节解析
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 ICMP 头部
        todo!("解析 ICMP 头部")
    }

    /// 序列化为字节
    pub fn to_bytes(&self) -> [u8; 8] {
        // TODO: 序列化 ICMP 头部
        todo!("序列化 ICMP 头部")
    }

    /// 计算校验和
    pub fn compute_checksum(data: &[u8]) -> u16 {
        // TODO: 计算 ICMP 校验和（与 IP 校验和相同）
        todo!("计算 ICMP 校验和")
    }
}

/// ICMP 数据包
#[derive(Debug, Clone)]
pub struct IcmpPacket {
    pub header: IcmpHeader,
    pub data: Vec<u8>,
}

impl IcmpPacket {
    /// 创建 Ping 数据包
    pub fn ping(identifier: u16, sequence: u16, data: Vec<u8>) -> Self {
        // TODO: 创建 ping 数据包
        // 需要计算校验和
        todo!("创建 Ping 数据包")
    }

    /// 从字节解析
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 ICMP 数据包
        todo!("解析 ICMP 数据包")
    }

    /// 序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 ICMP 数据包
        // 包含重新计算校验和
        todo!("序列化 ICMP 数据包")
    }

    /// 验证校验和
    pub fn verify_checksum(&self) -> bool {
        // TODO: 验证校验和
        todo!("验证校验和")
    }
}

/// 简单的 Ping 实现
///
/// 注意: 需要 root 权限
#[cfg(target_family = "unix")]
pub mod ping {
    use super::*;
    use std::io;
    use std::net::IpAddr;
    use std::time::{Duration, Instant};

    /// Ping 结果
    #[derive(Debug)]
    pub struct PingResult {
        /// 目标地址
        pub addr: IpAddr,
        /// 序列号
        pub sequence: u16,
        /// 往返时间
        pub rtt: Duration,
        /// TTL
        pub ttl: u8,
    }

    /// Ping 客户端
    pub struct Pinger {
        identifier: u16,
        sequence: u16,
    }

    impl Pinger {
        /// 创建 Pinger
        pub fn new() -> Self {
            // TODO: 创建 Pinger
            // 使用进程 ID 作为 identifier
            todo!("创建 Pinger")
        }

        /// 发送一个 ping
        ///
        /// # 参数
        /// * `addr` - 目标地址
        /// * `timeout` - 超时时间
        pub fn ping(&mut self, _addr: IpAddr, _timeout: Duration) -> io::Result<PingResult> {
            // TODO: 实现 ping
            //
            // 步骤:
            // 1. 创建原始套接字 (SOCK_RAW, IPPROTO_ICMP)
            // 2. 构造 ICMP Echo Request
            // 3. 发送数据包
            // 4. 等待响应
            // 5. 解析响应，计算 RTT
            //
            // 注意: 这需要 root 权限
            todo!("实现 ping")
        }
    }

    impl Default for Pinger {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// 数据包捕获（只读原始套接字）
#[cfg(target_family = "unix")]
pub mod capture {
    use std::io;

    /// 捕获的数据包
    #[derive(Debug)]
    pub struct CapturedPacket {
        /// IP 头部
        pub ip_header: Vec<u8>,
        /// 传输层数据
        pub transport_data: Vec<u8>,
        /// 捕获时间
        pub timestamp: std::time::Instant,
    }

    /// 简单的数据包捕获器
    pub struct PacketCapture {
        // socket fd
    }

    impl PacketCapture {
        /// 创建捕获器
        ///
        /// # 参数
        /// * `protocol` - 要捕获的协议 (IPPROTO_ICMP, IPPROTO_TCP, etc.)
        pub fn new(_protocol: i32) -> io::Result<Self> {
            // TODO: 创建原始套接字用于捕获
            // 使用 SOCK_RAW
            todo!("创建数据包捕获器")
        }

        /// 捕获一个数据包
        pub fn capture(&self) -> io::Result<CapturedPacket> {
            // TODO: 接收并解析数据包
            todo!("捕获数据包")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_raw_icmp_header() {
        let header = IcmpHeader::echo_request(1234, 1);
        assert_eq!(header.icmp_type, 8);
        assert_eq!(header.code, 0);
        assert_eq!(header.identifier, 1234);
        assert_eq!(header.sequence, 1);
    }

    #[test]
    fn test_socket_raw_icmp_serialize() {
        let header = IcmpHeader::echo_request(1234, 1);
        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), 8);

        let parsed = IcmpHeader::parse(&bytes).unwrap();
        assert_eq!(parsed.icmp_type, header.icmp_type);
        assert_eq!(parsed.identifier, header.identifier);
        assert_eq!(parsed.sequence, header.sequence);
    }

    #[test]
    fn test_socket_raw_icmp_packet() {
        let packet = IcmpPacket::ping(1234, 1, b"Hello".to_vec());
        let bytes = packet.to_bytes();

        let parsed = IcmpPacket::parse(&bytes).unwrap();
        assert!(parsed.verify_checksum());
        assert_eq!(parsed.data, b"Hello");
    }

    #[test]
    fn test_socket_raw_checksum() {
        // 测试校验和计算
        let data = [0x08, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01];
        let checksum = IcmpHeader::compute_checksum(&data);

        // 校验和应该使整个数据的校验和为 0xFFFF
        let mut with_checksum = data.to_vec();
        with_checksum[2] = (checksum >> 8) as u8;
        with_checksum[3] = checksum as u8;

        let verify = IcmpHeader::compute_checksum(&with_checksum);
        assert_eq!(verify, 0xFFFF);
    }
}
