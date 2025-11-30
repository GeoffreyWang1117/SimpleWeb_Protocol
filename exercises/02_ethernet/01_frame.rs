//! # 练习 01: 以太网帧
//!
//! 以太网帧是数据链路层的协议数据单元（PDU）。
//!
//! ## 以太网帧结构（IEEE 802.3）
//!
//! ```text
//! ┌────────────────┬───────────────┬───────────┬─────────────────┬─────┐
//! │ Destination MAC│   Source MAC  │ EtherType │     Payload     │ FCS │
//! │    (6 bytes)   │   (6 bytes)   │ (2 bytes) │  (46-1500 bytes)│(4B) │
//! └────────────────┴───────────────┴───────────┴─────────────────┴─────┘
//! ```
//!
//! ## 常见 EtherType 值
//!
//! - `0x0800`: IPv4
//! - `0x0806`: ARP
//! - `0x86DD`: IPv6
//! - `0x8100`: VLAN 标签 (802.1Q)
//!
//! ## 注意
//!
//! - 前导码和帧起始符通常由网卡硬件处理，不在软件层面处理
//! - FCS（帧检验序列）通常也由网卡处理
//! - 最小帧长度为 64 字节（包括 FCS），不足时需要填充
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ethernet_frame
//! ```

use super::ex00_mac_address::MacAddress;

/// 常见的 EtherType 值
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtherType {
    /// IPv4 协议 (0x0800)
    IPv4,
    /// ARP 协议 (0x0806)
    Arp,
    /// IPv6 协议 (0x86DD)
    IPv6,
    /// VLAN 标签 (0x8100)
    Vlan,
    /// 未知类型
    Unknown(u16),
}

impl EtherType {
    /// 从 u16 值创建 EtherType
    ///
    /// # 示例
    /// ```
    /// assert_eq!(EtherType::from_u16(0x0800), EtherType::IPv4);
    /// ```
    pub fn from_u16(value: u16) -> Self {
        // TODO: 根据值返回对应的 EtherType
        // 提示: 使用 match 表达式
        todo!("从 u16 创建 EtherType")
    }

    /// 将 EtherType 转换为 u16 值
    ///
    /// # 示例
    /// ```
    /// assert_eq!(EtherType::IPv4.to_u16(), 0x0800);
    /// ```
    pub fn to_u16(&self) -> u16 {
        // TODO: 返回 EtherType 对应的 u16 值
        todo!("将 EtherType 转换为 u16")
    }
}

/// 以太网帧结构
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    /// 目标 MAC 地址
    pub dst_mac: MacAddress,
    /// 源 MAC 地址
    pub src_mac: MacAddress,
    /// 以太网类型
    pub ether_type: EtherType,
    /// 载荷数据
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    /// 创建新的以太网帧
    ///
    /// # 参数
    /// * `dst_mac` - 目标 MAC 地址
    /// * `src_mac` - 源 MAC 地址
    /// * `ether_type` - 以太网类型
    /// * `payload` - 载荷数据
    pub fn new(
        dst_mac: MacAddress,
        src_mac: MacAddress,
        ether_type: EtherType,
        payload: Vec<u8>,
    ) -> Self {
        // TODO: 创建以太网帧
        todo!("创建以太网帧")
    }

    /// 从字节切片解析以太网帧
    ///
    /// # 参数
    /// * `data` - 原始帧数据（不包括前导码和 FCS）
    ///
    /// # 返回值
    /// 解析成功返回 Some(EthernetFrame)，失败返回 None
    ///
    /// # 帧格式
    /// ```text
    /// 字节 0-5:   目标 MAC
    /// 字节 6-11:  源 MAC
    /// 字节 12-13: EtherType（大端序）
    /// 字节 14+:   载荷
    /// ```
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析以太网帧
        //
        // 步骤:
        // 1. 检查数据长度是否至少 14 字节（最小帧头长度）
        // 2. 提取目标 MAC（字节 0-5）
        // 3. 提取源 MAC（字节 6-11）
        // 4. 提取 EtherType（字节 12-13，大端序）
        // 5. 剩余部分作为载荷
        //
        // 提示: 使用之前的 parse_mac_from_bytes 函数
        todo!("解析以太网帧")
    }

    /// 将以太网帧序列化为字节
    ///
    /// # 返回值
    /// 帧的字节表示（不包括 FCS）
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 将帧序列化为字节
        //
        // 步骤:
        // 1. 添加目标 MAC（6字节）
        // 2. 添加源 MAC（6字节）
        // 3. 添加 EtherType（2字节，大端序）
        // 4. 添加载荷
        todo!("序列化以太网帧")
    }

    /// 获取帧的总长度
    pub fn len(&self) -> usize {
        // TODO: 返回帧的总长度
        // 帧头(14) + 载荷长度
        todo!("获取帧长度")
    }

    /// 检查帧是否为空（无载荷）
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }

    /// 获取最小帧长度（不包括 FCS）
    ///
    /// 以太网帧最小为 64 字节（包括 4 字节 FCS），
    /// 所以不包括 FCS 时最小为 60 字节。
    pub const MIN_FRAME_SIZE: usize = 60;

    /// 获取需要填充的字节数
    ///
    /// 如果帧长度小于最小值，需要用零填充
    pub fn padding_needed(&self) -> usize {
        // TODO: 计算需要填充的字节数
        // 如果当前长度 >= MIN_FRAME_SIZE，返回 0
        // 否则返回差值
        todo!("计算填充字节数")
    }

    /// 添加填充使帧达到最小长度
    pub fn add_padding(&mut self) {
        // TODO: 在载荷末尾添加零字节，使帧达到最小长度
        todo!("添加填充")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_frame_ether_type() {
        assert_eq!(EtherType::from_u16(0x0800), EtherType::IPv4);
        assert_eq!(EtherType::from_u16(0x0806), EtherType::Arp);
        assert_eq!(EtherType::from_u16(0x86DD), EtherType::IPv6);
        assert_eq!(EtherType::from_u16(0x8100), EtherType::Vlan);
        assert_eq!(EtherType::from_u16(0x9999), EtherType::Unknown(0x9999));

        assert_eq!(EtherType::IPv4.to_u16(), 0x0800);
        assert_eq!(EtherType::Arp.to_u16(), 0x0806);
        assert_eq!(EtherType::IPv6.to_u16(), 0x86DD);
        assert_eq!(EtherType::Unknown(0x1234).to_u16(), 0x1234);
    }

    #[test]
    fn test_ethernet_frame_new() {
        let dst = MacAddress::broadcast();
        let src = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let payload = vec![1, 2, 3, 4, 5];

        let frame = EthernetFrame::new(dst, src, EtherType::IPv4, payload.clone());

        assert!(frame.dst_mac.is_broadcast());
        assert_eq!(frame.src_mac.bytes, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(frame.ether_type, EtherType::IPv4);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn test_ethernet_frame_parse() {
        // 构造一个简单的帧
        let data = [
            // 目标 MAC: FF:FF:FF:FF:FF:FF
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            // 源 MAC: 00:11:22:33:44:55
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            // EtherType: 0x0800 (IPv4)
            0x08, 0x00,
            // Payload
            0x45, 0x00, 0x00, 0x3c,
        ];

        let frame = EthernetFrame::parse(&data).unwrap();
        assert!(frame.dst_mac.is_broadcast());
        assert_eq!(frame.src_mac.bytes, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(frame.ether_type, EtherType::IPv4);
        assert_eq!(frame.payload, vec![0x45, 0x00, 0x00, 0x3c]);

        // 数据太短
        assert!(EthernetFrame::parse(&[0u8; 10]).is_none());
    }

    #[test]
    fn test_ethernet_frame_to_bytes() {
        let dst = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let src = MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        let payload = vec![0x12, 0x34];

        let frame = EthernetFrame::new(dst, src, EtherType::Arp, payload);
        let bytes = frame.to_bytes();

        assert_eq!(
            bytes,
            vec![
                // 目标 MAC
                0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
                // 源 MAC
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                // EtherType (ARP = 0x0806)
                0x08, 0x06,
                // Payload
                0x12, 0x34
            ]
        );
    }

    #[test]
    fn test_ethernet_frame_len() {
        let frame = EthernetFrame::new(
            MacAddress::zero(),
            MacAddress::zero(),
            EtherType::IPv4,
            vec![1, 2, 3, 4, 5],
        );
        // 14 (header) + 5 (payload) = 19
        assert_eq!(frame.len(), 19);
    }

    #[test]
    fn test_ethernet_frame_padding() {
        let mut frame = EthernetFrame::new(
            MacAddress::zero(),
            MacAddress::zero(),
            EtherType::IPv4,
            vec![1, 2, 3, 4, 5],
        );

        // 19 字节，需要填充到 60 字节
        assert_eq!(frame.padding_needed(), 60 - 19);

        frame.add_padding();
        assert_eq!(frame.len(), 60);
        assert_eq!(frame.padding_needed(), 0);
    }
}
