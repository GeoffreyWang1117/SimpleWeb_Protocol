//! # 练习 02: DNS 响应
//!
//! DNS 响应包含查询的答案、权威记录和附加记录。
//!
//! ## 资源记录结构
//!
//! ```text
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                                               |
//! /                      NAME                     /
//! /                                               /
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                      TYPE                     |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                     CLASS                     |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                      TTL                      |
//! |                                               |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! |                   RDLENGTH                    |
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! /                     RDATA                     /
//! /                                               /
//! +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test dns_response
//! ```

use super::ex00_header::DnsHeader;
use super::ex01_question::{DnsClass, DnsQuestion, DnsType};

/// DNS 资源记录
#[derive(Debug, Clone)]
pub struct DnsResourceRecord {
    /// 域名
    pub name: String,
    /// 记录类型
    pub rtype: DnsType,
    /// 记录类
    pub rclass: DnsClass,
    /// 生存时间（秒）
    pub ttl: u32,
    /// 记录数据
    pub rdata: DnsRData,
}

/// DNS 记录数据
#[derive(Debug, Clone)]
pub enum DnsRData {
    /// A 记录 - IPv4 地址
    A([u8; 4]),
    /// AAAA 记录 - IPv6 地址
    AAAA([u8; 16]),
    /// CNAME 记录 - 规范名称
    CName(String),
    /// MX 记录 - 邮件交换
    MX { preference: u16, exchange: String },
    /// NS 记录 - 名称服务器
    NS(String),
    /// TXT 记录 - 文本
    TXT(String),
    /// 未知类型
    Unknown(Vec<u8>),
}

impl DnsResourceRecord {
    /// 从字节切片解析资源记录
    ///
    /// # 参数
    /// * `data` - 完整的 DNS 消息
    /// * `offset` - 记录开始的偏移量
    ///
    /// # 返回值
    /// (DnsResourceRecord, 消耗的字节数)
    pub fn parse(data: &[u8], offset: usize) -> Option<(Self, usize)> {
        // TODO: 解析资源记录
        //
        // 步骤:
        // 1. 解码域名（可能是压缩的）
        // 2. 读取 TYPE (2 bytes)
        // 3. 读取 CLASS (2 bytes)
        // 4. 读取 TTL (4 bytes)
        // 5. 读取 RDLENGTH (2 bytes)
        // 6. 根据 TYPE 解析 RDATA
        todo!("解析资源记录")
    }

    /// 解析 RDATA
    fn parse_rdata(rtype: DnsType, data: &[u8], offset: usize, rdlength: u16) -> Option<DnsRData> {
        // TODO: 根据类型解析数据
        //
        // A: 4 字节 IPv4 地址
        // AAAA: 16 字节 IPv6 地址
        // CNAME/NS: 压缩域名
        // MX: 2 字节优先级 + 域名
        // TXT: 长度前缀的文本
        todo!("解析 RDATA")
    }
}

/// DNS 响应消息
#[derive(Debug, Clone)]
pub struct DnsResponse {
    /// 头部
    pub header: DnsHeader,
    /// 问题部分
    pub questions: Vec<DnsQuestion>,
    /// 回答部分
    pub answers: Vec<DnsResourceRecord>,
    /// 授权部分
    pub authorities: Vec<DnsResourceRecord>,
    /// 附加部分
    pub additionals: Vec<DnsResourceRecord>,
}

impl DnsResponse {
    /// 从字节切片解析 DNS 响应
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 DNS 响应
        //
        // 步骤:
        // 1. 解析头部 (12 字节)
        // 2. 解析问题部分 (qd_count 个)
        // 3. 解析回答部分 (an_count 个)
        // 4. 解析授权部分 (ns_count 个)
        // 5. 解析附加部分 (ar_count 个)
        todo!("解析 DNS 响应")
    }

    /// 获取所有 A 记录的 IPv4 地址
    pub fn get_a_records(&self) -> Vec<[u8; 4]> {
        // TODO: 从回答中提取 A 记录
        todo!("获取 A 记录")
    }

    /// 获取所有 AAAA 记录的 IPv6 地址
    pub fn get_aaaa_records(&self) -> Vec<[u8; 16]> {
        // TODO: 从回答中提取 AAAA 记录
        todo!("获取 AAAA 记录")
    }

    /// 获取所有 CNAME 记录
    pub fn get_cname_records(&self) -> Vec<String> {
        // TODO: 从回答中提取 CNAME 记录
        todo!("获取 CNAME 记录")
    }

    /// 检查响应是否表示域名不存在
    pub fn is_nxdomain(&self) -> bool {
        // TODO: 检查 RCODE 是否为 NXDOMAIN
        todo!("检查 NXDOMAIN")
    }

    /// 检查响应是否成功
    pub fn is_success(&self) -> bool {
        // TODO: 检查 RCODE 是否为 NoError
        todo!("检查成功")
    }
}

/// 格式化 IPv4 地址
pub fn format_ipv4(addr: [u8; 4]) -> String {
    format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3])
}

/// 格式化 IPv6 地址
pub fn format_ipv6(addr: [u8; 16]) -> String {
    // TODO: 格式化 IPv6 地址
    // 格式: xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx
    todo!("格式化 IPv6 地址")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::ex00_header::DnsRcode;

    #[test]
    fn test_format_ipv4() {
        assert_eq!(format_ipv4([192, 168, 1, 1]), "192.168.1.1");
        assert_eq!(format_ipv4([8, 8, 8, 8]), "8.8.8.8");
    }

    #[test]
    fn test_format_ipv6() {
        let addr = [
            0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70,
            0x73, 0x34,
        ];
        let formatted = format_ipv6(addr);
        // 应该包含 2001 等
        assert!(formatted.contains("2001"));
    }

    #[test]
    fn test_dns_response_parse() {
        // 一个简单的 DNS 响应示例
        let mut data = vec![
            // Header
            0x12, 0x34, // ID
            0x81, 0x80, // Flags: QR=1, RD=1, RA=1
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x01, // ANCOUNT = 1
            0x00, 0x00, // NSCOUNT = 0
            0x00, 0x00, // ARCOUNT = 0
            // Question
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 0x03, b'c', b'o', b'm', 0x00,
            0x00, 0x01, // QTYPE = A
            0x00, 0x01, // QCLASS = IN
            // Answer (使用压缩指针)
            0xc0, 0x0c, // 压缩指针，指向偏移 12
            0x00, 0x01, // TYPE = A
            0x00, 0x01, // CLASS = IN
            0x00, 0x00, 0x01, 0x2c, // TTL = 300
            0x00, 0x04, // RDLENGTH = 4
            0x5d, 0xb8, 0xd8, 0x22, // RDATA = 93.184.216.34
        ];

        let response = DnsResponse::parse(&data).unwrap();

        assert!(response.is_success());
        assert_eq!(response.header.id, 0x1234);
        assert_eq!(response.questions.len(), 1);
        assert_eq!(response.answers.len(), 1);

        let a_records = response.get_a_records();
        assert_eq!(a_records.len(), 1);
        assert_eq!(format_ipv4(a_records[0]), "93.184.216.34");
    }

    #[test]
    fn test_dns_rdata_a() {
        if let DnsRData::A(addr) = DnsRData::A([192, 168, 1, 1]) {
            assert_eq!(format_ipv4(addr), "192.168.1.1");
        }
    }

    #[test]
    fn test_dns_response_nxdomain() {
        // NXDOMAIN 响应
        let data = [
            0x12, 0x34, // ID
            0x81, 0x83, // Flags: QR=1, RCODE=3 (NXDOMAIN)
            0x00, 0x01, // QDCOUNT
            0x00, 0x00, // ANCOUNT
            0x00, 0x00, // NSCOUNT
            0x00, 0x00, // ARCOUNT
            // Question
            0x04, b't', b'e', b's', b't', 0x00, 0x00, 0x01, 0x00, 0x01,
        ];

        let response = DnsResponse::parse(&data).unwrap();
        assert!(response.is_nxdomain());
        assert!(!response.is_success());
    }
}
