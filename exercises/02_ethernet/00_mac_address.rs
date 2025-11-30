//! # 练习 00: MAC 地址
//!
//! MAC（Media Access Control）地址是网络接口卡的唯一标识符。
//!
//! ## MAC 地址结构
//!
//! - 长度：6 字节（48 位）
//! - 格式：通常用冒号或连字符分隔的十六进制表示
//!   - 例如：`00:1A:2B:3C:4D:5E` 或 `00-1A-2B-3C-4D-5E`
//!
//! ## MAC 地址分类
//!
//! ```text
//! 第一个字节的结构:
//! ┌───┬───┬───┬───┬───┬───┬───┬───┐
//! │ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │
//! └───┴───┴───┴───┴───┴───┴───┴───┘
//!                               │   └── 单播(0)/多播(1)
//!                               └────── 全局(0)/本地(1)
//! ```
//!
//! ## 特殊地址
//!
//! - 广播地址: `FF:FF:FF:FF:FF:FF`
//! - 零地址: `00:00:00:00:00:00`
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ethernet_mac
//! ```

/// MAC 地址结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddress {
    /// 6 字节的 MAC 地址
    pub bytes: [u8; 6],
}

impl MacAddress {
    /// 从 6 字节数组创建 MAC 地址
    ///
    /// # 示例
    /// ```
    /// let mac = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
    /// ```
    pub fn new(bytes: [u8; 6]) -> Self {
        // TODO: 创建一个新的 MAC 地址
        todo!("创建 MAC 地址")
    }

    /// 创建广播地址 (FF:FF:FF:FF:FF:FF)
    ///
    /// 广播地址用于向局域网内所有设备发送数据。
    pub fn broadcast() -> Self {
        // TODO: 返回广播地址
        todo!("创建广播地址")
    }

    /// 创建零地址 (00:00:00:00:00:00)
    pub fn zero() -> Self {
        // TODO: 返回零地址
        todo!("创建零地址")
    }

    /// 检查是否为广播地址
    pub fn is_broadcast(&self) -> bool {
        // TODO: 检查是否所有字节都是 0xFF
        todo!("检查是否为广播地址")
    }

    /// 检查是否为多播地址
    ///
    /// 多播地址的第一个字节的最低位是 1
    pub fn is_multicast(&self) -> bool {
        // TODO: 检查第一个字节的最低位
        // 提示: self.bytes[0] & 0x01 == 1
        todo!("检查是否为多播地址")
    }

    /// 检查是否为单播地址
    ///
    /// 单播地址的第一个字节的最低位是 0
    pub fn is_unicast(&self) -> bool {
        // TODO: 与 is_multicast 相反
        todo!("检查是否为单播地址")
    }

    /// 检查是否为本地管理地址（Locally Administered）
    ///
    /// 本地地址的第一个字节的第二位是 1
    pub fn is_local(&self) -> bool {
        // TODO: 检查第一个字节的第二位
        // 提示: self.bytes[0] & 0x02 == 2
        todo!("检查是否为本地地址")
    }

    /// 检查是否为全局唯一地址（Universally Administered）
    pub fn is_universal(&self) -> bool {
        // TODO: 与 is_local 相反
        todo!("检查是否为全局地址")
    }

    /// 从冒号分隔的字符串解析 MAC 地址
    ///
    /// # 参数
    /// * `s` - 格式为 "XX:XX:XX:XX:XX:XX" 的字符串
    ///
    /// # 返回值
    /// 解析成功返回 Some(MacAddress)，失败返回 None
    ///
    /// # 示例
    /// ```
    /// let mac = MacAddress::parse("00:1A:2B:3C:4D:5E").unwrap();
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        // TODO: 解析 MAC 地址字符串
        //
        // 步骤:
        // 1. 按冒号分割字符串
        // 2. 检查是否有 6 个部分
        // 3. 将每个部分从十六进制解析为 u8
        // 4. 返回 MacAddress
        //
        // 提示: 使用 split(':')、collect()、u8::from_str_radix()
        todo!("解析 MAC 地址字符串")
    }

    /// 将 MAC 地址格式化为冒号分隔的字符串
    ///
    /// # 返回值
    /// 格式为 "XX:XX:XX:XX:XX:XX" 的字符串（大写）
    ///
    /// # 示例
    /// ```
    /// let mac = MacAddress::new([0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e]);
    /// assert_eq!(mac.to_string(), "00:1A:2B:3C:4D:5E");
    /// ```
    pub fn to_string(&self) -> String {
        // TODO: 格式化 MAC 地址为字符串
        // 提示: 使用 format! 和 {:02X} 格式化每个字节
        todo!("格式化 MAC 地址")
    }
}

/// 从字节切片解析 MAC 地址
///
/// # 参数
/// * `data` - 至少 6 字节的切片
/// * `offset` - 起始偏移量
///
/// # 返回值
/// 解析成功返回 Some(MacAddress)，数据不足返回 None
pub fn parse_mac_from_bytes(data: &[u8], offset: usize) -> Option<MacAddress> {
    // TODO: 从字节切片中提取 6 字节的 MAC 地址
    // 提示:
    // 1. 检查是否有足够的字节
    // 2. 创建 [u8; 6] 数组
    // 3. 返回 MacAddress::new()
    todo!("从字节解析 MAC 地址")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_mac_new() {
        let mac = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert_eq!(mac.bytes, [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
    }

    #[test]
    fn test_ethernet_mac_broadcast() {
        let mac = MacAddress::broadcast();
        assert_eq!(mac.bytes, [0xFF; 6]);
        assert!(mac.is_broadcast());
        assert!(mac.is_multicast());
    }

    #[test]
    fn test_ethernet_mac_zero() {
        let mac = MacAddress::zero();
        assert_eq!(mac.bytes, [0x00; 6]);
    }

    #[test]
    fn test_ethernet_mac_multicast() {
        // 多播地址：第一个字节最低位为 1
        let multicast = MacAddress::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01]);
        assert!(multicast.is_multicast());
        assert!(!multicast.is_unicast());

        // 单播地址：第一个字节最低位为 0
        let unicast = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert!(unicast.is_unicast());
        assert!(!unicast.is_multicast());
    }

    #[test]
    fn test_ethernet_mac_local_universal() {
        // 本地管理地址：第一个字节第二位为 1
        let local = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
        assert!(local.is_local());
        assert!(!local.is_universal());

        // 全局唯一地址：第一个字节第二位为 0
        let universal = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert!(universal.is_universal());
        assert!(!universal.is_local());
    }

    #[test]
    fn test_ethernet_mac_parse() {
        let mac = MacAddress::parse("00:1A:2B:3C:4D:5E").unwrap();
        assert_eq!(mac.bytes, [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);

        // 小写也应该能解析
        let mac2 = MacAddress::parse("aa:bb:cc:dd:ee:ff").unwrap();
        assert_eq!(mac2.bytes, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        // 无效格式返回 None
        assert!(MacAddress::parse("invalid").is_none());
        assert!(MacAddress::parse("00:11:22:33:44").is_none()); // 只有5段
    }

    #[test]
    fn test_ethernet_mac_to_string() {
        let mac = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert_eq!(mac.to_string(), "00:1A:2B:3C:4D:5E");

        let broadcast = MacAddress::broadcast();
        assert_eq!(broadcast.to_string(), "FF:FF:FF:FF:FF:FF");
    }

    #[test]
    fn test_ethernet_mac_parse_from_bytes() {
        let data = [0xAA, 0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0xBB];
        let mac = parse_mac_from_bytes(&data, 1).unwrap();
        assert_eq!(mac.bytes, [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);

        // 数据不足
        assert!(parse_mac_from_bytes(&data, 5).is_none());
    }
}
