//! # 练习 00: UDP 头部
//!
//! UDP 头部非常简单，只有 8 个字节。
//!
//! ## UDP 头部结构 (RFC 768)
//!
//! ```text
//!  0      7 8     15 16    23 24    31
//! +--------+--------+--------+--------+
//! |     Source      |   Destination   |
//! |      Port       |      Port       |
//! +--------+--------+--------+--------+
//! |     Length      |    Checksum     |
//! +--------+--------+--------+--------+
//! |                                   |
//! |          data octets ...          |
//! +-----------------------------------+
//! ```
//!
//! ## 字段说明
//!
//! - Source Port (16 bits): 源端口号
//! - Destination Port (16 bits): 目标端口号
//! - Length (16 bits): UDP 数据报总长度（头部 + 数据）
//! - Checksum (16 bits): 校验和（可选，在 IPv6 中必须）
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test udp_header
//! ```

/// UDP 头部结构
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpHeader {
    /// 源端口
    pub src_port: u16,
    /// 目标端口
    pub dst_port: u16,
    /// 总长度（头部 + 数据）
    pub length: u16,
    /// 校验和
    pub checksum: u16,
}

impl UdpHeader {
    /// UDP 头部固定长度
    pub const HEADER_LEN: usize = 8;

    /// 创建新的 UDP 头部
    ///
    /// # 参数
    /// * `src_port` - 源端口
    /// * `dst_port` - 目标端口
    /// * `data_len` - 数据长度（不包括头部）
    ///
    /// # 示例
    /// ```
    /// let header = UdpHeader::new(12345, 80, 100);
    /// assert_eq!(header.length, 108); // 8 + 100
    /// ```
    pub fn new(src_port: u16, dst_port: u16, data_len: u16) -> Self {
        // TODO: 创建 UDP 头部
        // length = 8 (头部) + data_len
        // checksum 初始为 0
        todo!("创建 UDP 头部")
    }

    /// 从字节切片解析 UDP 头部
    ///
    /// # 参数
    /// * `data` - 至少 8 字节的数据
    ///
    /// # 返回值
    /// 解析成功返回 Some(UdpHeader)，失败返回 None
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 UDP 头部
        //
        // 字节偏移:
        // 0-1: src_port (大端序)
        // 2-3: dst_port (大端序)
        // 4-5: length (大端序)
        // 6-7: checksum (大端序)
        todo!("解析 UDP 头部")
    }

    /// 将 UDP 头部序列化为字节
    pub fn to_bytes(&self) -> [u8; 8] {
        // TODO: 序列化 UDP 头部
        // 所有字段都是大端序
        todo!("序列化 UDP 头部")
    }

    /// 获取数据长度（不包括头部）
    pub fn data_len(&self) -> u16 {
        // TODO: 返回 length - 8
        todo!("获取数据长度")
    }
}

/// UDP 数据报（头部 + 数据）
#[derive(Debug, Clone)]
pub struct UdpDatagram {
    /// UDP 头部
    pub header: UdpHeader,
    /// 数据载荷
    pub data: Vec<u8>,
}

impl UdpDatagram {
    /// 创建新的 UDP 数据报
    pub fn new(src_port: u16, dst_port: u16, data: Vec<u8>) -> Self {
        // TODO: 创建 UDP 数据报
        // 注意: header.length 应该是 8 + data.len()
        todo!("创建 UDP 数据报")
    }

    /// 从字节切片解析 UDP 数据报
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 UDP 数据报
        //
        // 步骤:
        // 1. 解析头部
        // 2. 验证数据长度与 header.length 一致
        // 3. 提取数据部分
        todo!("解析 UDP 数据报")
    }

    /// 将 UDP 数据报序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 UDP 数据报
        // 头部 + 数据
        todo!("序列化 UDP 数据报")
    }

    /// 计算并设置校验和
    ///
    /// UDP 校验和需要包含伪头部（来自 IP 层）
    ///
    /// # 参数
    /// * `src_ip` - 源 IP 地址
    /// * `dst_ip` - 目标 IP 地址
    pub fn compute_checksum(&mut self, src_ip: [u8; 4], dst_ip: [u8; 4]) {
        // TODO: 计算 UDP 校验和
        //
        // 伪头部格式:
        // - 源 IP: 4 字节
        // - 目标 IP: 4 字节
        // - 零: 1 字节
        // - 协议: 1 字节 (UDP = 17)
        // - UDP 长度: 2 字节
        //
        // 校验和计算:
        // 1. 构造伪头部
        // 2. 将头部的 checksum 设为 0
        // 3. 连接: 伪头部 + UDP头部 + 数据
        // 4. 计算互联网校验和
        todo!("计算 UDP 校验和")
    }
}

/// 常见的 UDP 端口号
pub mod well_known_ports {
    /// DNS
    pub const DNS: u16 = 53;
    /// DHCP 服务器
    pub const DHCP_SERVER: u16 = 67;
    /// DHCP 客户端
    pub const DHCP_CLIENT: u16 = 68;
    /// TFTP
    pub const TFTP: u16 = 69;
    /// NTP
    pub const NTP: u16 = 123;
    /// SNMP
    pub const SNMP: u16 = 161;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_header_new() {
        let header = UdpHeader::new(12345, 80, 100);
        assert_eq!(header.src_port, 12345);
        assert_eq!(header.dst_port, 80);
        assert_eq!(header.length, 108); // 8 + 100
        assert_eq!(header.checksum, 0);
    }

    #[test]
    fn test_udp_header_parse() {
        let data = [
            0x30, 0x39, // src_port = 12345
            0x00, 0x50, // dst_port = 80
            0x00, 0x6c, // length = 108
            0x00, 0x00, // checksum = 0
        ];

        let header = UdpHeader::parse(&data).unwrap();
        assert_eq!(header.src_port, 12345);
        assert_eq!(header.dst_port, 80);
        assert_eq!(header.length, 108);
    }

    #[test]
    fn test_udp_header_to_bytes() {
        let header = UdpHeader::new(12345, 80, 100);
        let bytes = header.to_bytes();

        assert_eq!(
            bytes,
            [
                0x30, 0x39, // src_port
                0x00, 0x50, // dst_port
                0x00, 0x6c, // length
                0x00, 0x00, // checksum
            ]
        );
    }

    #[test]
    fn test_udp_header_data_len() {
        let header = UdpHeader::new(1000, 2000, 50);
        assert_eq!(header.data_len(), 50);
    }

    #[test]
    fn test_udp_datagram_new() {
        let data = vec![1, 2, 3, 4, 5];
        let datagram = UdpDatagram::new(1234, 5678, data.clone());

        assert_eq!(datagram.header.src_port, 1234);
        assert_eq!(datagram.header.dst_port, 5678);
        assert_eq!(datagram.header.length, 13); // 8 + 5
        assert_eq!(datagram.data, data);
    }

    #[test]
    fn test_udp_datagram_parse() {
        let bytes = [
            0x04, 0xd2, // src_port = 1234
            0x16, 0x2e, // dst_port = 5678
            0x00, 0x0d, // length = 13
            0x00, 0x00, // checksum
            0x01, 0x02, 0x03, 0x04, 0x05, // data
        ];

        let datagram = UdpDatagram::parse(&bytes).unwrap();
        assert_eq!(datagram.header.src_port, 1234);
        assert_eq!(datagram.header.dst_port, 5678);
        assert_eq!(datagram.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_udp_datagram_to_bytes() {
        let datagram = UdpDatagram::new(1234, 5678, vec![0xAA, 0xBB]);
        let bytes = datagram.to_bytes();

        assert_eq!(bytes.len(), 10); // 8 + 2
        assert_eq!(&bytes[8..], &[0xAA, 0xBB]);
    }

    #[test]
    fn test_udp_datagram_checksum() {
        let mut datagram = UdpDatagram::new(1234, 5678, vec![0x00, 0x01]);
        let src_ip = [192, 168, 1, 1];
        let dst_ip = [192, 168, 1, 2];

        datagram.compute_checksum(src_ip, dst_ip);
        assert_ne!(datagram.header.checksum, 0);
    }
}
