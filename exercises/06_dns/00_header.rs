//! # 练习 00: DNS 头部
//!
//! DNS 消息头部固定 12 字节。
//!
//! ## DNS 头部结构 (RFC 1035)
//!
//! ```text
//!                                 1  1  1  1  1  1
//!   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                      ID                       |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                    QDCOUNT                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                    ANCOUNT                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                    NSCOUNT                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                    ARCOUNT                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! ```
//!
//! ## 字段说明
//!
//! - ID: 查询标识符
//! - QR: 查询(0)/响应(1)
//! - OPCODE: 操作类型
//! - AA: 权威回答
//! - TC: 截断标志
//! - RD: 期望递归
//! - RA: 递归可用
//! - RCODE: 响应码
//! - QDCOUNT: 问题数量
//! - ANCOUNT: 回答数量
//! - NSCOUNT: 授权数量
//! - ARCOUNT: 附加数量
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test dns_header
//! ```

/// DNS 头部固定长度
pub const DNS_HEADER_LEN: usize = 12;

/// DNS 操作码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsOpcode {
    /// 标准查询
    Query = 0,
    /// 反向查询
    IQuery = 1,
    /// 服务器状态请求
    Status = 2,
    /// 其他
    Other(u8),
}

impl DnsOpcode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => DnsOpcode::Query,
            1 => DnsOpcode::IQuery,
            2 => DnsOpcode::Status,
            n => DnsOpcode::Other(n),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            DnsOpcode::Query => 0,
            DnsOpcode::IQuery => 1,
            DnsOpcode::Status => 2,
            DnsOpcode::Other(n) => *n,
        }
    }
}

/// DNS 响应码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsRcode {
    /// 无错误
    NoError = 0,
    /// 格式错误
    FormErr = 1,
    /// 服务器失败
    ServFail = 2,
    /// 名称错误（域名不存在）
    NxDomain = 3,
    /// 未实现
    NotImp = 4,
    /// 拒绝
    Refused = 5,
    /// 其他
    Other(u8),
}

impl DnsRcode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => DnsRcode::NoError,
            1 => DnsRcode::FormErr,
            2 => DnsRcode::ServFail,
            3 => DnsRcode::NxDomain,
            4 => DnsRcode::NotImp,
            5 => DnsRcode::Refused,
            n => DnsRcode::Other(n),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            DnsRcode::NoError => 0,
            DnsRcode::FormErr => 1,
            DnsRcode::ServFail => 2,
            DnsRcode::NxDomain => 3,
            DnsRcode::NotImp => 4,
            DnsRcode::Refused => 5,
            DnsRcode::Other(n) => *n,
        }
    }
}

/// DNS 头部标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DnsFlags {
    /// 查询/响应标志 (0=查询, 1=响应)
    pub qr: bool,
    /// 操作码
    pub opcode: DnsOpcode,
    /// 权威回答
    pub aa: bool,
    /// 截断
    pub tc: bool,
    /// 期望递归
    pub rd: bool,
    /// 递归可用
    pub ra: bool,
    /// 响应码
    pub rcode: DnsRcode,
}

impl DnsFlags {
    /// 创建查询标志
    pub fn query() -> Self {
        // TODO: 创建标准查询标志
        // qr=0, opcode=Query, rd=1 (期望递归)
        todo!("创建查询标志")
    }

    /// 创建响应标志
    pub fn response(rcode: DnsRcode) -> Self {
        // TODO: 创建响应标志
        // qr=1, 其他根据参数设置
        todo!("创建响应标志")
    }

    /// 从两个字节解析标志
    ///
    /// 字节布局:
    /// 第一个字节: QR(1) + Opcode(4) + AA(1) + TC(1) + RD(1)
    /// 第二个字节: RA(1) + Z(3) + RCODE(4)
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        // TODO: 从字节解析标志
        todo!("从字节解析标志")
    }

    /// 将标志转换为两个字节
    pub fn to_bytes(&self) -> [u8; 2] {
        // TODO: 将标志转换为字节
        todo!("转换为字节")
    }
}

/// DNS 头部结构
#[derive(Debug, Clone)]
pub struct DnsHeader {
    /// 查询 ID
    pub id: u16,
    /// 标志
    pub flags: DnsFlags,
    /// 问题数量
    pub qd_count: u16,
    /// 回答数量
    pub an_count: u16,
    /// 授权数量
    pub ns_count: u16,
    /// 附加数量
    pub ar_count: u16,
}

impl DnsHeader {
    /// 创建新的查询头部
    pub fn new_query(id: u16) -> Self {
        // TODO: 创建查询头部
        // qd_count=1, 其他=0
        todo!("创建查询头部")
    }

    /// 从字节切片解析头部
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 DNS 头部
        //
        // 字节偏移:
        // 0-1: ID
        // 2-3: Flags
        // 4-5: QDCOUNT
        // 6-7: ANCOUNT
        // 8-9: NSCOUNT
        // 10-11: ARCOUNT
        todo!("解析 DNS 头部")
    }

    /// 将头部序列化为字节
    pub fn to_bytes(&self) -> [u8; 12] {
        // TODO: 序列化 DNS 头部
        todo!("序列化 DNS 头部")
    }

    /// 检查是否为查询
    pub fn is_query(&self) -> bool {
        !self.flags.qr
    }

    /// 检查是否为响应
    pub fn is_response(&self) -> bool {
        self.flags.qr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_header_flags_query() {
        let flags = DnsFlags::query();
        assert!(!flags.qr);
        assert_eq!(flags.opcode, DnsOpcode::Query);
        assert!(flags.rd);
    }

    #[test]
    fn test_dns_header_flags_bytes() {
        let flags = DnsFlags::query();
        let bytes = flags.to_bytes();

        let parsed = DnsFlags::from_bytes(bytes);
        assert_eq!(parsed.qr, flags.qr);
        assert_eq!(parsed.opcode, flags.opcode);
        assert_eq!(parsed.rd, flags.rd);
    }

    #[test]
    fn test_dns_header_new_query() {
        let header = DnsHeader::new_query(0x1234);

        assert_eq!(header.id, 0x1234);
        assert!(header.is_query());
        assert_eq!(header.qd_count, 1);
        assert_eq!(header.an_count, 0);
    }

    #[test]
    fn test_dns_header_parse() {
        let data = [
            0x12, 0x34, // ID
            0x01, 0x00, // Flags: RD=1
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x00, // ANCOUNT = 0
            0x00, 0x00, // NSCOUNT = 0
            0x00, 0x00, // ARCOUNT = 0
        ];

        let header = DnsHeader::parse(&data).unwrap();

        assert_eq!(header.id, 0x1234);
        assert!(header.is_query());
        assert!(header.flags.rd);
        assert_eq!(header.qd_count, 1);
    }

    #[test]
    fn test_dns_header_to_bytes() {
        let header = DnsHeader::new_query(0xABCD);
        let bytes = header.to_bytes();

        assert_eq!(bytes.len(), 12);
        assert_eq!(bytes[0], 0xAB);
        assert_eq!(bytes[1], 0xCD);
    }

    #[test]
    fn test_dns_header_roundtrip() {
        let original = DnsHeader::new_query(0x1234);
        let bytes = original.to_bytes();
        let parsed = DnsHeader::parse(&bytes).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.qd_count, original.qd_count);
    }
}
