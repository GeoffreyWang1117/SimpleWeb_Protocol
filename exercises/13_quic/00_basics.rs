//! # 练习 00: QUIC 基础
//!
//! 理解 QUIC 协议的基本概念和数据包格式。
//!
//! ## QUIC vs TCP
//!
//! | 特性 | TCP | QUIC |
//! |------|-----|------|
//! | 传输层 | 内核 | 用户空间 |
//! | 加密 | 可选(TLS) | 强制(TLS 1.3) |
//! | 流复用 | 无 | 原生支持 |
//! | 队头阻塞 | 有 | 无 |
//! | 连接迁移 | 不支持 | 支持 |
//! | 握手延迟 | 1-3 RTT | 0-1 RTT |
//!
//! ## QUIC 数据包类型
//!
//! | 类型 | 用途 |
//! |------|------|
//! | Initial | 握手初始包 |
//! | Handshake | 握手包 |
//! | 0-RTT | 早期数据 |
//! | 1-RTT | 应用数据 |
//! | Retry | 地址验证 |
//! | Version Negotiation | 版本协商 |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test quic_basics
//! ```

/// QUIC 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(pub u32);

impl Version {
    /// QUIC v1 (RFC 9000)
    pub const V1: Version = Version(0x00000001);
    /// QUIC v2 (RFC 9369)
    pub const V2: Version = Version(0x6b3343cf);
    /// 版本协商
    pub const NEGOTIATION: Version = Version(0x00000000);

    /// 是否是有效版本
    pub fn is_valid(&self) -> bool {
        // TODO: 检查是否是支持的版本
        todo!("检查版本")
    }

    /// 是否是版本协商包
    pub fn is_negotiation(&self) -> bool {
        self.0 == 0
    }
}

/// 连接 ID
///
/// QUIC 使用连接 ID 而非 IP:Port 标识连接
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionId {
    /// ID 数据（0-20 字节）
    data: Vec<u8>,
}

impl ConnectionId {
    /// 最大长度
    pub const MAX_LEN: usize = 20;

    /// 创建连接 ID
    pub fn new(data: Vec<u8>) -> Option<Self> {
        // TODO: 验证长度并创建
        // 长度必须 <= 20
        todo!("创建连接 ID")
    }

    /// 生成随机连接 ID
    pub fn random(len: usize) -> Option<Self> {
        // TODO: 生成指定长度的随机 ID
        todo!("生成随机 ID")
    }

    /// 空连接 ID
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// 获取长度
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 获取数据
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// QUIC 数据包头类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    /// Initial 包（握手开始）
    Initial,
    /// 0-RTT 包（早期数据）
    ZeroRtt,
    /// Handshake 包
    Handshake,
    /// Retry 包（地址验证）
    Retry,
    /// 1-RTT 包（短头）
    OneRtt,
    /// Version Negotiation
    VersionNegotiation,
}

impl PacketType {
    /// 从头部第一个字节解析
    pub fn from_first_byte(byte: u8, version: Option<Version>) -> Option<Self> {
        // TODO: 解析数据包类型
        //
        // 头部格式:
        // - bit 7: 头部形式 (1=长头, 0=短头)
        // - bit 6: 固定位 (长头=1, 短头=1)
        // - bit 4-5: 长头类型
        //   - 00: Initial
        //   - 01: 0-RTT
        //   - 10: Handshake
        //   - 11: Retry
        //
        // Version = 0 表示 Version Negotiation
        todo!("解析包类型")
    }

    /// 是否使用长头
    pub fn uses_long_header(&self) -> bool {
        // TODO: 判断是否使用长头
        // 只有 1-RTT 使用短头
        todo!("判断头部类型")
    }
}

/// QUIC 长头（用于握手）
///
/// ```text
/// Long Header:
/// +-+-+-+-+-+-+-+-+
/// |1|1|T T|X X X X|  第一字节
/// +-+-+-+-+-+-+-+-+
/// |    Version    |  4 字节
/// +-+-+-+-+-+-+-+-+
/// | DCID Len (8)  |  1 字节
/// +-+-+-+-+-+-+-+-+
/// |     DCID      |  0-20 字节
/// +-+-+-+-+-+-+-+-+
/// | SCID Len (8)  |  1 字节
/// +-+-+-+-+-+-+-+-+
/// |     SCID      |  0-20 字节
/// +-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone)]
pub struct LongHeader {
    /// 数据包类型
    pub packet_type: PacketType,
    /// 版本
    pub version: Version,
    /// 目标连接 ID
    pub dcid: ConnectionId,
    /// 源连接 ID
    pub scid: ConnectionId,
}

impl LongHeader {
    /// 解析长头
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析长头并返回消耗的字节数
        todo!("解析长头")
    }

    /// 序列化长头
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化长头
        todo!("序列化长头")
    }

    /// 获取头部大小
    pub fn size(&self) -> usize {
        // TODO: 计算头部大小
        // 1 (first byte) + 4 (version) + 1 (dcid len) + dcid + 1 (scid len) + scid
        todo!("头部大小")
    }
}

/// QUIC 短头（用于应用数据）
///
/// ```text
/// Short Header:
/// +-+-+-+-+-+-+-+-+
/// |0|1|S|R|R|K|P P|  第一字节
/// +-+-+-+-+-+-+-+-+
/// |     DCID      |  变长（由对方决定）
/// +-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone)]
pub struct ShortHeader {
    /// 自旋位（用于 RTT 测量）
    pub spin: bool,
    /// Key Phase（密钥轮换）
    pub key_phase: bool,
    /// 目标连接 ID
    pub dcid: ConnectionId,
}

impl ShortHeader {
    /// 解析短头
    pub fn parse(data: &[u8], dcid_len: usize) -> Option<(Self, usize)> {
        // TODO: 解析短头
        // dcid_len 需要从连接状态获取
        todo!("解析短头")
    }

    /// 序列化短头
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化短头
        todo!("序列化短头")
    }
}

/// QUIC 数据包
#[derive(Debug, Clone)]
pub enum QuicPacket {
    /// 长头数据包
    Long {
        header: LongHeader,
        /// 包号
        packet_number: u64,
        /// 载荷
        payload: Vec<u8>,
    },
    /// 短头数据包
    Short {
        header: ShortHeader,
        /// 包号
        packet_number: u64,
        /// 载荷
        payload: Vec<u8>,
    },
}

impl QuicPacket {
    /// 判断是长头还是短头
    pub fn is_long_header(first_byte: u8) -> bool {
        // TODO: 判断头部形式
        // 第一字节的最高位: 1=长头, 0=短头
        todo!("判断头部形式")
    }

    /// 获取包号
    pub fn packet_number(&self) -> u64 {
        match self {
            QuicPacket::Long { packet_number, .. } => *packet_number,
            QuicPacket::Short { packet_number, .. } => *packet_number,
        }
    }
}

/// QUIC 帧类型
///
/// QUIC 载荷由一个或多个帧组成
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// PADDING (0x00)
    Padding,
    /// PING (0x01)
    Ping,
    /// ACK (0x02-0x03)
    Ack,
    /// CRYPTO (0x06)
    Crypto,
    /// STREAM (0x08-0x0f)
    Stream,
    /// CONNECTION_CLOSE (0x1c-0x1d)
    ConnectionClose,
    /// 其他帧类型
    Other(u64),
}

impl FrameType {
    /// 从类型值解析
    pub fn from_type_value(value: u64) -> Self {
        // TODO: 解析帧类型
        todo!("解析帧类型")
    }

    /// 是否是 ACK 触发帧
    pub fn is_ack_eliciting(&self) -> bool {
        // TODO: 判断是否需要 ACK
        // PADDING, ACK 不需要 ACK
        todo!("是否触发 ACK")
    }
}

/// 变长整数编码 (QUIC Variable-Length Integer)
///
/// QUIC 使用变长整数来节省空间
pub struct VarInt;

impl VarInt {
    /// 解码变长整数
    ///
    /// 编码格式:
    /// - 2MSB = 00: 6-bit value (1 byte, max 63)
    /// - 2MSB = 01: 14-bit value (2 bytes, max 16383)
    /// - 2MSB = 10: 30-bit value (4 bytes, max 1073741823)
    /// - 2MSB = 11: 62-bit value (8 bytes)
    pub fn decode(data: &[u8]) -> Option<(u64, usize)> {
        // TODO: 解码变长整数
        todo!("解码变长整数")
    }

    /// 编码变长整数
    pub fn encode(value: u64) -> Vec<u8> {
        // TODO: 编码变长整数
        todo!("编码变长整数")
    }

    /// 计算编码所需字节数
    pub fn encoded_size(value: u64) -> usize {
        // TODO: 计算所需字节数
        todo!("计算编码长度")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_basics_version() {
        assert!(Version::V1.is_valid());
        assert!(Version::V2.is_valid());
        assert!(Version::NEGOTIATION.is_negotiation());
    }

    #[test]
    fn test_quic_basics_connection_id() {
        let cid = ConnectionId::new(vec![1, 2, 3, 4]).unwrap();
        assert_eq!(cid.len(), 4);

        // 太长应该失败
        let long_data = vec![0u8; 21];
        assert!(ConnectionId::new(long_data).is_none());

        let empty = ConnectionId::empty();
        assert!(empty.is_empty());
    }

    #[test]
    fn test_quic_basics_packet_type() {
        // 长头 Initial: 11000000
        assert_eq!(
            PacketType::from_first_byte(0xc0, Some(Version::V1)),
            Some(PacketType::Initial)
        );

        // 长头 Handshake: 11100000
        assert_eq!(
            PacketType::from_first_byte(0xe0, Some(Version::V1)),
            Some(PacketType::Handshake)
        );

        // 短头: 01000000
        assert_eq!(
            PacketType::from_first_byte(0x40, Some(Version::V1)),
            Some(PacketType::OneRtt)
        );
    }

    #[test]
    fn test_quic_basics_varint() {
        // 1 字节: 0-63
        let (val, len) = VarInt::decode(&[0x25]).unwrap();
        assert_eq!(val, 37);
        assert_eq!(len, 1);

        // 2 字节: 64-16383
        let (val, len) = VarInt::decode(&[0x7b, 0xbd]).unwrap();
        assert_eq!(val, 15293);
        assert_eq!(len, 2);

        // 编码测试
        assert_eq!(VarInt::encode(37), vec![0x25]);
        assert_eq!(VarInt::encoded_size(37), 1);
        assert_eq!(VarInt::encoded_size(15293), 2);
    }

    #[test]
    fn test_quic_basics_frame_type() {
        assert_eq!(FrameType::from_type_value(0x00), FrameType::Padding);
        assert_eq!(FrameType::from_type_value(0x06), FrameType::Crypto);

        assert!(!FrameType::Padding.is_ack_eliciting());
        assert!(FrameType::Ping.is_ack_eliciting());
        assert!(FrameType::Stream.is_ack_eliciting());
    }
}
