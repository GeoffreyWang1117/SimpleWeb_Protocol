//! # 练习 01: IPv4 头部
//!
//! IPv4 头部是每个 IP 数据包的开始部分，包含路由和处理所需的信息。
//!
//! ## IPv4 头部结构 (RFC 791)
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |Version|  IHL  |Type of Service|          Total Length         |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |         Identification        |Flags|      Fragment Offset    |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |  Time to Live |    Protocol   |         Header Checksum       |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                       Source Address                          |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                    Destination Address                        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                    Options                    |    Padding    |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//! ## 字段说明
//!
//! - Version (4 bits): IP 版本，IPv4 = 4
//! - IHL (4 bits): 头部长度，以 4 字节为单位，最小值 5（20 字节）
//! - Type of Service (8 bits): 服务类型/DSCP
//! - Total Length (16 bits): 整个数据包的长度（字节）
//! - Identification (16 bits): 分片标识
//! - Flags (3 bits): 控制分片
//! - Fragment Offset (13 bits): 分片偏移
//! - TTL (8 bits): 生存时间
//! - Protocol (8 bits): 上层协议（ICMP=1, TCP=6, UDP=17）
//! - Header Checksum (16 bits): 头部校验和
//! - Source/Destination Address (32 bits each): 源/目标 IP
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ip_ipv4_header
//! ```

use super::ex00_ipv4_address::Ipv4Address;

/// IPv4 头部中的协议字段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol {
    /// ICMP 协议
    Icmp = 1,
    /// TCP 协议
    Tcp = 6,
    /// UDP 协议
    Udp = 17,
    /// 其他协议
    Other(u8),
}

impl IpProtocol {
    /// 从 u8 创建协议类型
    pub fn from_u8(value: u8) -> Self {
        // TODO: 根据值返回对应的协议类型
        todo!("从 u8 创建协议类型")
    }

    /// 转换为 u8
    pub fn to_u8(&self) -> u8 {
        // TODO: 返回协议的 u8 值
        todo!("转换为 u8")
    }
}

/// IPv4 头部标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Flags {
    /// 保留位（必须为 0）
    pub reserved: bool,
    /// Don't Fragment 标志
    pub dont_fragment: bool,
    /// More Fragments 标志
    pub more_fragments: bool,
}

impl Ipv4Flags {
    /// 从 3 位值创建标志
    ///
    /// 位布局: [Reserved, DF, MF]
    pub fn from_bits(bits: u8) -> Self {
        // TODO: 解析 3 位标志
        // 提示:
        // - bit 2: reserved
        // - bit 1: don't fragment
        // - bit 0: more fragments
        todo!("从位创建标志")
    }

    /// 转换为 3 位值
    pub fn to_bits(&self) -> u8 {
        // TODO: 组合标志为 3 位
        todo!("转换为位")
    }
}

/// IPv4 头部结构
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    /// IP 版本（应为 4）
    pub version: u8,
    /// 头部长度（以 4 字节为单位）
    pub ihl: u8,
    /// 服务类型 / DSCP + ECN
    pub tos: u8,
    /// 总长度（整个数据包，包括头部和数据）
    pub total_length: u16,
    /// 标识符
    pub identification: u16,
    /// 标志
    pub flags: Ipv4Flags,
    /// 分片偏移（以 8 字节为单位）
    pub fragment_offset: u16,
    /// 生存时间
    pub ttl: u8,
    /// 协议
    pub protocol: IpProtocol,
    /// 头部校验和
    pub checksum: u16,
    /// 源 IP 地址
    pub src_addr: Ipv4Address,
    /// 目标 IP 地址
    pub dst_addr: Ipv4Address,
    /// 选项（可变长度）
    pub options: Vec<u8>,
}

impl Ipv4Header {
    /// 创建一个基本的 IPv4 头部（无选项）
    pub fn new(
        src_addr: Ipv4Address,
        dst_addr: Ipv4Address,
        protocol: IpProtocol,
        payload_len: u16,
    ) -> Self {
        // TODO: 创建基本的 IPv4 头部
        //
        // 默认值:
        // - version: 4
        // - ihl: 5 (20 字节，无选项)
        // - tos: 0
        // - total_length: 20 + payload_len
        // - identification: 0 (应该是随机的，但这里简化)
        // - flags: 不分片 (DF=1, MF=0)
        // - fragment_offset: 0
        // - ttl: 64 (常见默认值)
        // - checksum: 0 (稍后计算)
        // - options: 空
        todo!("创建 IPv4 头部")
    }

    /// 从字节切片解析 IPv4 头部
    ///
    /// # 参数
    /// * `data` - 原始数据（至少 20 字节）
    ///
    /// # 返回值
    /// 解析成功返回 Some((Ipv4Header, header_len))，失败返回 None
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 IPv4 头部
        //
        // 步骤:
        // 1. 检查数据至少 20 字节
        // 2. 提取第一个字节的高 4 位作为版本
        // 3. 提取第一个字节的低 4 位作为 IHL
        // 4. 验证版本 == 4
        // 5. 计算头部长度 = IHL * 4
        // 6. 检查数据足够容纳整个头部
        // 7. 提取所有字段
        // 8. 返回 (header, header_length)
        //
        // 字段偏移:
        // 0: version(4) + ihl(4)
        // 1: tos
        // 2-3: total_length
        // 4-5: identification
        // 6-7: flags(3) + fragment_offset(13)
        // 8: ttl
        // 9: protocol
        // 10-11: checksum
        // 12-15: src_addr
        // 16-19: dst_addr
        // 20+: options
        todo!("解析 IPv4 头部")
    }

    /// 将 IPv4 头部序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化头部为字节
        //
        // 步骤:
        // 1. 计算实际 IHL（考虑选项）
        // 2. 构建每个字段
        // 3. 添加选项和填充
        todo!("序列化 IPv4 头部")
    }

    /// 计算头部长度（字节）
    pub fn header_len(&self) -> usize {
        // TODO: 返回 IHL * 4
        todo!("计算头部长度")
    }

    /// 计算并设置头部校验和
    ///
    /// 使用互联网校验和算法（RFC 1071）
    pub fn compute_checksum(&mut self) {
        // TODO: 计算头部校验和
        //
        // 步骤:
        // 1. 将 checksum 字段设为 0
        // 2. 将头部序列化为字节
        // 3. 使用互联网校验和算法计算
        // 4. 设置 checksum 字段
        todo!("计算校验和")
    }

    /// 验证头部校验和
    pub fn verify_checksum(&self) -> bool {
        // TODO: 验证校验和是否正确
        // 对整个头部（包括校验和）计算校验和，结果应为 0xFFFF
        todo!("验证校验和")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_ipv4_header_protocol() {
        assert_eq!(IpProtocol::from_u8(1), IpProtocol::Icmp);
        assert_eq!(IpProtocol::from_u8(6), IpProtocol::Tcp);
        assert_eq!(IpProtocol::from_u8(17), IpProtocol::Udp);

        assert_eq!(IpProtocol::Tcp.to_u8(), 6);
    }

    #[test]
    fn test_ip_ipv4_header_flags() {
        let flags = Ipv4Flags::from_bits(0b010);
        assert!(!flags.reserved);
        assert!(flags.dont_fragment);
        assert!(!flags.more_fragments);

        let flags2 = Ipv4Flags::from_bits(0b001);
        assert!(flags2.more_fragments);

        assert_eq!(flags.to_bits(), 0b010);
    }

    #[test]
    fn test_ip_ipv4_header_new() {
        let src = Ipv4Address::new(192, 168, 1, 1);
        let dst = Ipv4Address::new(192, 168, 1, 2);

        let header = Ipv4Header::new(src, dst, IpProtocol::Tcp, 100);

        assert_eq!(header.version, 4);
        assert_eq!(header.ihl, 5);
        assert_eq!(header.total_length, 120); // 20 + 100
        assert_eq!(header.ttl, 64);
    }

    #[test]
    fn test_ip_ipv4_header_parse() {
        // 一个真实的 IPv4 头部示例
        let data = [
            0x45, // Version=4, IHL=5
            0x00, // TOS
            0x00, 0x3c, // Total Length = 60
            0x1c, 0x46, // Identification
            0x40, 0x00, // Flags=DF, Fragment Offset=0
            0x40, // TTL = 64
            0x06, // Protocol = TCP
            0xb1, 0xe6, // Checksum
            0xac, 0x10, 0x0a, 0x63, // Source: 172.16.10.99
            0xac, 0x10, 0x0a, 0x0c, // Dest: 172.16.10.12
        ];

        let (header, len) = Ipv4Header::parse(&data).unwrap();

        assert_eq!(len, 20);
        assert_eq!(header.version, 4);
        assert_eq!(header.ihl, 5);
        assert_eq!(header.total_length, 60);
        assert_eq!(header.ttl, 64);
        assert_eq!(header.protocol, IpProtocol::Tcp);
        assert!(header.flags.dont_fragment);
        assert!(!header.flags.more_fragments);
    }

    #[test]
    fn test_ip_ipv4_header_to_bytes() {
        let src = Ipv4Address::new(192, 168, 1, 1);
        let dst = Ipv4Address::new(192, 168, 1, 2);

        let mut header = Ipv4Header::new(src, dst, IpProtocol::Udp, 8);
        header.identification = 0x1234;
        header.compute_checksum();

        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), 20);
        assert_eq!(bytes[0], 0x45); // version=4, ihl=5
        assert_eq!(bytes[9], 17); // UDP
    }

    #[test]
    fn test_ip_ipv4_header_checksum() {
        let src = Ipv4Address::new(192, 168, 1, 1);
        let dst = Ipv4Address::new(192, 168, 1, 2);

        let mut header = Ipv4Header::new(src, dst, IpProtocol::Tcp, 0);
        header.compute_checksum();

        assert!(header.verify_checksum());

        // 篡改数据后校验和应该失败
        header.ttl = 32;
        assert!(!header.verify_checksum());
    }
}
