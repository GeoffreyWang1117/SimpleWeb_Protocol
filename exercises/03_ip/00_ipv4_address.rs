//! # 练习 00: IPv4 地址
//!
//! IPv4 地址是一个 32 位（4 字节）的标识符，通常用点分十进制表示。
//!
//! ## 地址格式
//!
//! ```text
//! 二进制:    11000000.10101000.00000001.00000001
//! 十进制:    192.168.1.1
//! 十六进制:  0xC0A80101
//! ```
//!
//! ## 特殊地址
//!
//! - `0.0.0.0`: 未指定地址
//! - `127.0.0.1`: 本地回环地址
//! - `255.255.255.255`: 广播地址
//! - `10.x.x.x`, `172.16-31.x.x`, `192.168.x.x`: 私有地址
//!
//! ## 地址分类（传统分类，现已不常用）
//!
//! - A 类: 0.0.0.0 - 127.255.255.255 (第一位为 0)
//! - B 类: 128.0.0.0 - 191.255.255.255 (前两位为 10)
//! - C 类: 192.0.0.0 - 223.255.255.255 (前三位为 110)
//! - D 类: 224.0.0.0 - 239.255.255.255 (前四位为 1110，多播)
//! - E 类: 240.0.0.0 - 255.255.255.255 (前四位为 1111，保留)
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ip_ipv4_address
//! ```

/// IPv4 地址结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Address {
    /// 4 字节的 IPv4 地址
    pub octets: [u8; 4],
}

impl Ipv4Address {
    /// 从 4 个字节创建 IPv4 地址
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv4Address::new(192, 168, 1, 1);
    /// ```
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        // TODO: 创建 IPv4 地址
        todo!("创建 IPv4 地址")
    }

    /// 从字节数组创建 IPv4 地址
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        // TODO: 从字节数组创建地址
        todo!("从字节数组创建地址")
    }

    /// 创建回环地址 (127.0.0.1)
    pub fn loopback() -> Self {
        // TODO: 返回回环地址
        todo!("创建回环地址")
    }

    /// 创建未指定地址 (0.0.0.0)
    pub fn unspecified() -> Self {
        // TODO: 返回未指定地址
        todo!("创建未指定地址")
    }

    /// 创建广播地址 (255.255.255.255)
    pub fn broadcast() -> Self {
        // TODO: 返回广播地址
        todo!("创建广播地址")
    }

    /// 检查是否为回环地址 (127.x.x.x)
    pub fn is_loopback(&self) -> bool {
        // TODO: 检查第一个字节是否为 127
        todo!("检查回环地址")
    }

    /// 检查是否为私有地址
    ///
    /// 私有地址范围:
    /// - 10.0.0.0/8 (10.0.0.0 - 10.255.255.255)
    /// - 172.16.0.0/12 (172.16.0.0 - 172.31.255.255)
    /// - 192.168.0.0/16 (192.168.0.0 - 192.168.255.255)
    pub fn is_private(&self) -> bool {
        // TODO: 检查是否在私有地址范围内
        // 提示: 检查第一个字节，有些还需要检查第二个字节
        todo!("检查私有地址")
    }

    /// 检查是否为广播地址 (255.255.255.255)
    pub fn is_broadcast(&self) -> bool {
        // TODO: 检查是否所有字节都是 255
        todo!("检查广播地址")
    }

    /// 检查是否为多播地址 (224.0.0.0 - 239.255.255.255)
    pub fn is_multicast(&self) -> bool {
        // TODO: 检查第一个字节是否在 224-239 范围内
        todo!("检查多播地址")
    }

    /// 将 IPv4 地址转换为 u32（大端序）
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv4Address::new(192, 168, 1, 1);
    /// assert_eq!(addr.to_u32(), 0xC0A80101);
    /// ```
    pub fn to_u32(&self) -> u32 {
        // TODO: 将 4 个字节组合成 u32（大端序）
        todo!("转换为 u32")
    }

    /// 从 u32 创建 IPv4 地址（大端序）
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv4Address::from_u32(0xC0A80101);
    /// assert_eq!(addr.octets, [192, 168, 1, 1]);
    /// ```
    pub fn from_u32(value: u32) -> Self {
        // TODO: 从 u32 提取 4 个字节
        todo!("从 u32 创建地址")
    }

    /// 从点分十进制字符串解析 IPv4 地址
    ///
    /// # 参数
    /// * `s` - 格式为 "A.B.C.D" 的字符串
    ///
    /// # 返回值
    /// 解析成功返回 Some(Ipv4Address)，失败返回 None
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv4Address::parse("192.168.1.1").unwrap();
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        // TODO: 解析点分十进制字符串
        //
        // 步骤:
        // 1. 按点分割字符串
        // 2. 检查是否有 4 个部分
        // 3. 将每个部分解析为 u8
        // 4. 返回地址
        todo!("解析 IPv4 地址")
    }

    /// 将 IPv4 地址格式化为点分十进制字符串
    pub fn to_string(&self) -> String {
        // TODO: 格式化为 "A.B.C.D"
        todo!("格式化 IPv4 地址")
    }

    /// 应用子网掩码，获取网络地址
    ///
    /// # 参数
    /// * `mask` - 子网掩码
    ///
    /// # 示例
    /// ```
    /// let addr = Ipv4Address::parse("192.168.1.100").unwrap();
    /// let mask = Ipv4Address::parse("255.255.255.0").unwrap();
    /// let network = addr.network_address(&mask);
    /// assert_eq!(network.to_string(), "192.168.1.0");
    /// ```
    pub fn network_address(&self, mask: &Ipv4Address) -> Self {
        // TODO: 对每个字节进行位与操作
        todo!("计算网络地址")
    }

    /// 从 CIDR 前缀长度创建子网掩码
    ///
    /// # 参数
    /// * `prefix_len` - 前缀长度 (0-32)
    ///
    /// # 示例
    /// ```
    /// let mask = Ipv4Address::mask_from_prefix(24);
    /// assert_eq!(mask.to_string(), "255.255.255.0");
    /// ```
    pub fn mask_from_prefix(prefix_len: u8) -> Self {
        // TODO: 根据前缀长度生成子网掩码
        // 提示: 前缀长度表示从高位开始连续的 1 的个数
        // 例如: 24 -> 11111111.11111111.11111111.00000000
        todo!("从前缀创建掩码")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_ipv4_address_new() {
        let addr = Ipv4Address::new(192, 168, 1, 1);
        assert_eq!(addr.octets, [192, 168, 1, 1]);
    }

    #[test]
    fn test_ip_ipv4_address_special() {
        assert_eq!(Ipv4Address::loopback().octets, [127, 0, 0, 1]);
        assert_eq!(Ipv4Address::unspecified().octets, [0, 0, 0, 0]);
        assert_eq!(Ipv4Address::broadcast().octets, [255, 255, 255, 255]);
    }

    #[test]
    fn test_ip_ipv4_address_is_loopback() {
        assert!(Ipv4Address::new(127, 0, 0, 1).is_loopback());
        assert!(Ipv4Address::new(127, 255, 255, 255).is_loopback());
        assert!(!Ipv4Address::new(128, 0, 0, 1).is_loopback());
    }

    #[test]
    fn test_ip_ipv4_address_is_private() {
        // 10.0.0.0/8
        assert!(Ipv4Address::new(10, 0, 0, 1).is_private());
        assert!(Ipv4Address::new(10, 255, 255, 255).is_private());

        // 172.16.0.0/12
        assert!(Ipv4Address::new(172, 16, 0, 1).is_private());
        assert!(Ipv4Address::new(172, 31, 255, 255).is_private());
        assert!(!Ipv4Address::new(172, 32, 0, 1).is_private());

        // 192.168.0.0/16
        assert!(Ipv4Address::new(192, 168, 0, 1).is_private());
        assert!(Ipv4Address::new(192, 168, 255, 255).is_private());

        // 公网地址
        assert!(!Ipv4Address::new(8, 8, 8, 8).is_private());
    }

    #[test]
    fn test_ip_ipv4_address_is_broadcast() {
        assert!(Ipv4Address::broadcast().is_broadcast());
        assert!(!Ipv4Address::new(255, 255, 255, 0).is_broadcast());
    }

    #[test]
    fn test_ip_ipv4_address_is_multicast() {
        assert!(Ipv4Address::new(224, 0, 0, 1).is_multicast());
        assert!(Ipv4Address::new(239, 255, 255, 255).is_multicast());
        assert!(!Ipv4Address::new(223, 255, 255, 255).is_multicast());
        assert!(!Ipv4Address::new(240, 0, 0, 0).is_multicast());
    }

    #[test]
    fn test_ip_ipv4_address_u32_conversion() {
        let addr = Ipv4Address::new(192, 168, 1, 1);
        assert_eq!(addr.to_u32(), 0xC0A80101);

        let addr2 = Ipv4Address::from_u32(0xC0A80101);
        assert_eq!(addr2.octets, [192, 168, 1, 1]);
    }

    #[test]
    fn test_ip_ipv4_address_parse() {
        let addr = Ipv4Address::parse("192.168.1.1").unwrap();
        assert_eq!(addr.octets, [192, 168, 1, 1]);

        assert!(Ipv4Address::parse("invalid").is_none());
        assert!(Ipv4Address::parse("256.0.0.1").is_none());
        assert!(Ipv4Address::parse("1.2.3").is_none());
    }

    #[test]
    fn test_ip_ipv4_address_to_string() {
        let addr = Ipv4Address::new(192, 168, 1, 1);
        assert_eq!(addr.to_string(), "192.168.1.1");
    }

    #[test]
    fn test_ip_ipv4_address_network() {
        let addr = Ipv4Address::parse("192.168.1.100").unwrap();
        let mask = Ipv4Address::parse("255.255.255.0").unwrap();
        let network = addr.network_address(&mask);
        assert_eq!(network.to_string(), "192.168.1.0");
    }

    #[test]
    fn test_ip_ipv4_address_mask_from_prefix() {
        assert_eq!(Ipv4Address::mask_from_prefix(0).octets, [0, 0, 0, 0]);
        assert_eq!(Ipv4Address::mask_from_prefix(8).octets, [255, 0, 0, 0]);
        assert_eq!(Ipv4Address::mask_from_prefix(16).octets, [255, 255, 0, 0]);
        assert_eq!(Ipv4Address::mask_from_prefix(24).octets, [255, 255, 255, 0]);
        assert_eq!(Ipv4Address::mask_from_prefix(32).octets, [255, 255, 255, 255]);
        assert_eq!(Ipv4Address::mask_from_prefix(25).octets, [255, 255, 255, 128]);
    }
}
