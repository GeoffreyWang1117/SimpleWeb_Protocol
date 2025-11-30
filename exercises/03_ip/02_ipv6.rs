//! # 练习 02: IPv6 基础
//!
//! IPv6 是 IP 协议的下一代版本，使用 128 位地址。
//!
//! ## IPv6 地址格式
//!
//! - 长度：128 位（16 字节）
//! - 表示：8 组 16 位十六进制数，用冒号分隔
//! - 例如：`2001:0db8:85a3:0000:0000:8a2e:0370:7334`
//!
//! ## 地址简化规则
//!
//! 1. 每组前导零可以省略：`2001:db8:85a3:0:0:8a2e:370:7334`
//! 2. 连续的全零组可以用 `::` 代替（只能使用一次）：`2001:db8:85a3::8a2e:370:7334`
//!
//! ## 特殊地址
//!
//! - `::`: 未指定地址
//! - `::1`: 回环地址
//! - `fe80::/10`: 链路本地地址
//! - `ff00::/8`: 多播地址
//!
//! ## IPv6 头部结构
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |Version| Traffic Class |           Flow Label                  |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |         Payload Length        |  Next Header  |   Hop Limit   |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                                                               |
//! +                                                               +
//! |                         Source Address                        |
//! +                          (128 bits)                           +
//! |                                                               |
//! +                                                               +
//! |                                                               |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                                                               |
//! +                                                               +
//! |                      Destination Address                      |
//! +                          (128 bits)                           +
//! |                                                               |
//! +                                                               +
//! |                                                               |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//! IPv6 头部固定 40 字节，没有选项字段（选项使用扩展头部）。
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ip_ipv6
//! ```

/// IPv6 地址结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Address {
    /// 16 字节的 IPv6 地址
    pub octets: [u8; 16],
}

impl Ipv6Address {
    /// 从 8 个 16 位段创建 IPv6 地址
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv6Address::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
    /// ```
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        // TODO: 将 8 个 u16 转换为 16 字节数组
        // 提示: 每个 u16 转换为 2 个字节（大端序）
        todo!("创建 IPv6 地址")
    }

    /// 从字节数组创建 IPv6 地址
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        // TODO: 从字节数组创建地址
        todo!("从字节数组创建地址")
    }

    /// 创建回环地址 (::1)
    pub fn loopback() -> Self {
        // TODO: 返回 ::1
        // 15 个 0x00 + 1 个 0x01
        todo!("创建回环地址")
    }

    /// 创建未指定地址 (::)
    pub fn unspecified() -> Self {
        // TODO: 返回全零地址
        todo!("创建未指定地址")
    }

    /// 检查是否为回环地址 (::1)
    pub fn is_loopback(&self) -> bool {
        // TODO: 检查前 15 字节为 0，最后一字节为 1
        todo!("检查回环地址")
    }

    /// 检查是否为未指定地址 (::)
    pub fn is_unspecified(&self) -> bool {
        // TODO: 检查所有字节为 0
        todo!("检查未指定地址")
    }

    /// 检查是否为链路本地地址 (fe80::/10)
    ///
    /// 链路本地地址的前 10 位是 1111 1110 10
    pub fn is_link_local(&self) -> bool {
        // TODO: 检查前缀是否为 fe80::/10
        // 第一个字节为 0xfe，第二个字节的前 2 位为 10
        todo!("检查链路本地地址")
    }

    /// 检查是否为多播地址 (ff00::/8)
    pub fn is_multicast(&self) -> bool {
        // TODO: 检查第一个字节是否为 0xff
        todo!("检查多播地址")
    }

    /// 将 IPv6 地址转换为 8 个 u16 段
    pub fn segments(&self) -> [u16; 8] {
        // TODO: 将 16 字节转换为 8 个 u16（大端序）
        todo!("获取段")
    }

    /// 将 IPv6 地址格式化为完整形式（不压缩）
    ///
    /// # 返回值
    /// 格式为 "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx" 的字符串
    pub fn to_string_full(&self) -> String {
        // TODO: 格式化为完整形式
        todo!("格式化为完整形式")
    }

    /// 将 IPv6 地址格式化为压缩形式
    ///
    /// 应用 RFC 5952 的规则：
    /// 1. 省略每组的前导零
    /// 2. 用 :: 替换最长的连续零段
    ///
    /// # 示例
    /// ```
    /// // 2001:0db8:0000:0000:0000:0000:0000:0001 -> 2001:db8::1
    /// ```
    pub fn to_string_compressed(&self) -> String {
        // TODO: 格式化为压缩形式（可选挑战）
        // 这是一个较复杂的练习，可以先返回完整形式
        todo!("格式化为压缩形式")
    }

    /// 从字符串解析 IPv6 地址
    ///
    /// 支持完整形式和压缩形式
    ///
    /// # 示例
    /// ```
    /// let addr1 = Ipv6Address::parse("2001:db8::1").unwrap();
    /// let addr2 = Ipv6Address::parse("::1").unwrap();
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        // TODO: 解析 IPv6 地址字符串
        //
        // 这是一个较复杂的练习，需要处理 :: 压缩
        //
        // 简化版本步骤:
        // 1. 检查是否包含 ::
        // 2. 如果包含，展开为完整的 8 段
        // 3. 将每段解析为 u16
        todo!("解析 IPv6 地址")
    }
}

/// IPv6 头部结构
#[derive(Debug, Clone)]
pub struct Ipv6Header {
    /// 版本（应为 6）
    pub version: u8,
    /// 流量类别
    pub traffic_class: u8,
    /// 流标签
    pub flow_label: u32,
    /// 载荷长度
    pub payload_length: u16,
    /// 下一个头部（协议类型）
    pub next_header: u8,
    /// 跳数限制（类似 IPv4 的 TTL）
    pub hop_limit: u8,
    /// 源地址
    pub src_addr: Ipv6Address,
    /// 目标地址
    pub dst_addr: Ipv6Address,
}

impl Ipv6Header {
    /// IPv6 头部固定长度
    pub const HEADER_LEN: usize = 40;

    /// 创建新的 IPv6 头部
    pub fn new(
        src_addr: Ipv6Address,
        dst_addr: Ipv6Address,
        next_header: u8,
        payload_length: u16,
    ) -> Self {
        // TODO: 创建 IPv6 头部
        //
        // 默认值:
        // - version: 6
        // - traffic_class: 0
        // - flow_label: 0
        // - hop_limit: 64
        todo!("创建 IPv6 头部")
    }

    /// 从字节切片解析 IPv6 头部
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 IPv6 头部
        //
        // 字段偏移:
        // 0: version(4) + traffic_class高4位(4)
        // 1: traffic_class低4位(4) + flow_label高4位(4)
        // 2-3: flow_label低16位
        // 4-5: payload_length
        // 6: next_header
        // 7: hop_limit
        // 8-23: src_addr
        // 24-39: dst_addr
        todo!("解析 IPv6 头部")
    }

    /// 将 IPv6 头部序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 IPv6 头部
        todo!("序列化 IPv6 头部")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_ipv6_address_new() {
        let addr = Ipv6Address::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
        assert_eq!(
            addr.octets,
            [0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70, 0x73, 0x34]
        );
    }

    #[test]
    fn test_ip_ipv6_address_special() {
        let loopback = Ipv6Address::loopback();
        assert!(loopback.is_loopback());

        let unspecified = Ipv6Address::unspecified();
        assert!(unspecified.is_unspecified());
    }

    #[test]
    fn test_ip_ipv6_address_link_local() {
        let link_local = Ipv6Address::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        assert!(link_local.is_link_local());

        let not_link_local = Ipv6Address::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        assert!(!not_link_local.is_link_local());
    }

    #[test]
    fn test_ip_ipv6_address_multicast() {
        let multicast = Ipv6Address::new(0xff02, 0, 0, 0, 0, 0, 0, 1);
        assert!(multicast.is_multicast());

        assert!(!Ipv6Address::loopback().is_multicast());
    }

    #[test]
    fn test_ip_ipv6_address_segments() {
        let addr = Ipv6Address::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
        let segments = addr.segments();
        assert_eq!(segments, [0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334]);
    }

    #[test]
    fn test_ip_ipv6_address_to_string() {
        let addr = Ipv6Address::new(0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001);
        let full = addr.to_string_full();
        assert_eq!(full, "2001:0db8:0000:0000:0000:0000:0000:0001");
    }

    #[test]
    fn test_ip_ipv6_address_parse() {
        // 完整形式
        let addr1 = Ipv6Address::parse("2001:0db8:0000:0000:0000:0000:0000:0001");
        assert!(addr1.is_some());

        // 压缩形式
        let addr2 = Ipv6Address::parse("::1");
        assert!(addr2.is_some());
        assert!(addr2.unwrap().is_loopback());

        // 无效格式
        assert!(Ipv6Address::parse("invalid").is_none());
    }

    #[test]
    fn test_ip_ipv6_header_new() {
        let src = Ipv6Address::loopback();
        let dst = Ipv6Address::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);

        let header = Ipv6Header::new(src, dst, 6, 100); // TCP

        assert_eq!(header.version, 6);
        assert_eq!(header.next_header, 6);
        assert_eq!(header.payload_length, 100);
        assert_eq!(header.hop_limit, 64);
    }

    #[test]
    fn test_ip_ipv6_header_parse() {
        // 构造一个 IPv6 头部
        let mut data = vec![
            0x60, 0x00, 0x00, 0x00, // Version=6, TC=0, Flow=0
            0x00, 0x14, // Payload Length = 20
            0x06, // Next Header = TCP
            0x40, // Hop Limit = 64
        ];
        // 源地址 ::1
        data.extend_from_slice(&[0; 15]);
        data.push(1);
        // 目标地址 ::1
        data.extend_from_slice(&[0; 15]);
        data.push(1);

        let header = Ipv6Header::parse(&data).unwrap();
        assert_eq!(header.version, 6);
        assert_eq!(header.payload_length, 20);
        assert_eq!(header.next_header, 6);
        assert_eq!(header.hop_limit, 64);
        assert!(header.src_addr.is_loopback());
        assert!(header.dst_addr.is_loopback());
    }
}
