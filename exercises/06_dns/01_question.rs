//! # 练习 01: DNS 问题部分
//!
//! DNS 问题部分包含要查询的域名和类型。
//!
//! ## 域名编码
//!
//! 域名使用"长度前缀"编码，每个标签前有一个长度字节，以零字节结尾。
//!
//! ```text
//! www.example.com 编码为:
//! 03 w w w 07 e x a m p l e 03 c o m 00
//! |  |     |  |             |  |     |
//! |  标签   |  标签          |  标签  结束符
//! 长度      长度             长度
//! ```
//!
//! ## 问题结构
//!
//! ```text
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                                               |
//! /                     QNAME                     /
//! /                                               /
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                     QTYPE                     |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                     QCLASS                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test dns_question
//! ```

/// DNS 查询类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsType {
    /// 主机地址 (IPv4)
    A = 1,
    /// 名称服务器
    NS = 2,
    /// 规范名称
    CNAME = 5,
    /// 起始授权
    SOA = 6,
    /// 邮件交换
    MX = 15,
    /// 文本记录
    TXT = 16,
    /// IPv6 地址
    AAAA = 28,
    /// 服务定位
    SRV = 33,
    /// 所有类型
    ALL = 255,
    /// 其他
    Other(u16),
}

impl DnsType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => DnsType::A,
            2 => DnsType::NS,
            5 => DnsType::CNAME,
            6 => DnsType::SOA,
            15 => DnsType::MX,
            16 => DnsType::TXT,
            28 => DnsType::AAAA,
            33 => DnsType::SRV,
            255 => DnsType::ALL,
            n => DnsType::Other(n),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            DnsType::A => 1,
            DnsType::NS => 2,
            DnsType::CNAME => 5,
            DnsType::SOA => 6,
            DnsType::MX => 15,
            DnsType::TXT => 16,
            DnsType::AAAA => 28,
            DnsType::SRV => 33,
            DnsType::ALL => 255,
            DnsType::Other(n) => *n,
        }
    }
}

/// DNS 查询类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsClass {
    /// Internet
    IN = 1,
    /// 其他
    Other(u16),
}

impl DnsClass {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => DnsClass::IN,
            n => DnsClass::Other(n),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            DnsClass::IN => 1,
            DnsClass::Other(n) => *n,
        }
    }
}

/// DNS 问题结构
#[derive(Debug, Clone)]
pub struct DnsQuestion {
    /// 查询的域名
    pub name: String,
    /// 查询类型
    pub qtype: DnsType,
    /// 查询类
    pub qclass: DnsClass,
}

impl DnsQuestion {
    /// 创建新的 DNS 问题
    pub fn new(name: &str, qtype: DnsType) -> Self {
        // TODO: 创建 DNS 问题
        // qclass 默认为 IN
        todo!("创建 DNS 问题")
    }

    /// 将域名编码为 DNS 格式
    ///
    /// # 示例
    /// ```
    /// let encoded = encode_domain_name("www.example.com");
    /// // 结果: [3, 'w', 'w', 'w', 7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm', 0]
    /// ```
    pub fn encode_domain_name(name: &str) -> Vec<u8> {
        // TODO: 编码域名
        //
        // 步骤:
        // 1. 按点分割域名
        // 2. 对每个标签：添加长度字节 + 标签内容
        // 3. 添加结束符 0
        todo!("编码域名")
    }

    /// 从 DNS 格式解码域名
    ///
    /// # 参数
    /// * `data` - 完整的 DNS 消息（用于处理压缩）
    /// * `offset` - 域名开始的偏移量
    ///
    /// # 返回值
    /// (域名, 消耗的字节数)
    pub fn decode_domain_name(data: &[u8], offset: usize) -> Option<(String, usize)> {
        // TODO: 解码域名
        //
        // 需要处理两种情况:
        // 1. 普通标签: 第一个字节是长度 (0-63)
        // 2. 压缩指针: 前两位是 11，后 14 位是偏移量
        //
        // 压缩指针格式: 11xx xxxx xxxx xxxx
        //              高6位      低8位
        todo!("解码域名")
    }

    /// 将问题序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化问题
        //
        // 格式: QNAME + QTYPE(2) + QCLASS(2)
        todo!("序列化问题")
    }

    /// 从字节切片解析问题
    ///
    /// # 参数
    /// * `data` - 完整的 DNS 消息
    /// * `offset` - 问题开始的偏移量
    ///
    /// # 返回值
    /// (DnsQuestion, 消耗的字节数)
    pub fn parse(data: &[u8], offset: usize) -> Option<(Self, usize)> {
        // TODO: 解析问题
        //
        // 步骤:
        // 1. 解码域名
        // 2. 读取 QTYPE
        // 3. 读取 QCLASS
        todo!("解析问题")
    }
}

/// 构建完整的 DNS 查询消息
///
/// # 参数
/// * `id` - 查询 ID
/// * `domain` - 要查询的域名
/// * `qtype` - 查询类型
pub fn build_dns_query(id: u16, domain: &str, qtype: DnsType) -> Vec<u8> {
    // TODO: 构建 DNS 查询
    //
    // 步骤:
    // 1. 创建头部
    // 2. 创建问题
    // 3. 序列化头部 + 问题
    todo!("构建 DNS 查询")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_question_encode_name() {
        let encoded = DnsQuestion::encode_domain_name("www.example.com");

        assert_eq!(encoded[0], 3); // "www" 长度
        assert_eq!(&encoded[1..4], b"www");
        assert_eq!(encoded[4], 7); // "example" 长度
        assert_eq!(&encoded[5..12], b"example");
        assert_eq!(encoded[12], 3); // "com" 长度
        assert_eq!(&encoded[13..16], b"com");
        assert_eq!(encoded[16], 0); // 结束符
    }

    #[test]
    fn test_dns_question_encode_single_label() {
        let encoded = DnsQuestion::encode_domain_name("localhost");

        assert_eq!(encoded[0], 9);
        assert_eq!(&encoded[1..10], b"localhost");
        assert_eq!(encoded[10], 0);
    }

    #[test]
    fn test_dns_question_decode_name() {
        let data = [
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];

        let (name, len) = DnsQuestion::decode_domain_name(&data, 0).unwrap();

        assert_eq!(name, "www.example.com");
        assert_eq!(len, 17);
    }

    #[test]
    fn test_dns_question_new() {
        let question = DnsQuestion::new("example.com", DnsType::A);

        assert_eq!(question.name, "example.com");
        assert_eq!(question.qtype, DnsType::A);
        assert_eq!(question.qclass, DnsClass::IN);
    }

    #[test]
    fn test_dns_question_to_bytes() {
        let question = DnsQuestion::new("test.com", DnsType::A);
        let bytes = question.to_bytes();

        // 域名: 4 + 'test' + 3 + 'com' + 0 = 10 字节
        // QTYPE: 2 字节
        // QCLASS: 2 字节
        // 总共: 14 字节
        assert_eq!(bytes.len(), 14);

        // 最后 4 字节是 QTYPE 和 QCLASS
        assert_eq!(bytes[bytes.len() - 4], 0);
        assert_eq!(bytes[bytes.len() - 3], 1); // A = 1
        assert_eq!(bytes[bytes.len() - 2], 0);
        assert_eq!(bytes[bytes.len() - 1], 1); // IN = 1
    }

    #[test]
    fn test_dns_question_parse() {
        let data = [
            4, b't', b'e', b's', b't', 3, b'c', b'o', b'm', 0, // name
            0, 1, // QTYPE = A
            0, 1, // QCLASS = IN
        ];

        let (question, len) = DnsQuestion::parse(&data, 0).unwrap();

        assert_eq!(question.name, "test.com");
        assert_eq!(question.qtype, DnsType::A);
        assert_eq!(question.qclass, DnsClass::IN);
        assert_eq!(len, 14);
    }

    #[test]
    fn test_build_dns_query() {
        let query = build_dns_query(0x1234, "example.com", DnsType::A);

        // 头部 12 字节 + 问题
        assert!(query.len() >= 12);

        // 验证 ID
        assert_eq!(query[0], 0x12);
        assert_eq!(query[1], 0x34);

        // 验证 QDCOUNT = 1
        assert_eq!(query[4], 0);
        assert_eq!(query[5], 1);
    }
}
