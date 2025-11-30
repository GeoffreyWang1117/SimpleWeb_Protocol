//! # 练习 01: TCP 标志位
//!
//! TCP 头部包含 8 个控制标志位，用于管理连接和数据传输。
//!
//! ## 标志位说明
//!
//! ```text
//! 位:  7   6   5   4   3   2   1   0
//!     CWR ECE URG ACK PSH RST SYN FIN
//! ```
//!
//! | 标志 | 名称 | 说明 |
//! |------|------|------|
//! | FIN  | Finish | 发送方完成发送 |
//! | SYN  | Synchronize | 同步序列号，用于建立连接 |
//! | RST  | Reset | 重置连接 |
//! | PSH  | Push | 推送数据，不缓冲 |
//! | ACK  | Acknowledgment | 确认号有效 |
//! | URG  | Urgent | 紧急指针有效 |
//! | ECE  | ECN-Echo | ECN 回显 |
//! | CWR  | Congestion Window Reduced | 拥塞窗口减少 |
//!
//! ## 常见组合
//!
//! - `SYN`: 连接请求
//! - `SYN+ACK`: 连接响应
//! - `ACK`: 普通确认
//! - `FIN+ACK`: 连接终止
//! - `RST`: 强制断开
//! - `PSH+ACK`: 推送数据
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test tcp_flags
//! ```

/// TCP 标志位结构
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TcpFlags {
    /// FIN - 发送方完成发送
    pub fin: bool,
    /// SYN - 同步序列号
    pub syn: bool,
    /// RST - 重置连接
    pub rst: bool,
    /// PSH - 推送数据
    pub psh: bool,
    /// ACK - 确认号有效
    pub ack: bool,
    /// URG - 紧急指针有效
    pub urg: bool,
    /// ECE - ECN 回显
    pub ece: bool,
    /// CWR - 拥塞窗口减少
    pub cwr: bool,
}

impl TcpFlags {
    /// 创建空标志（所有位都是 false）
    pub fn empty() -> Self {
        // TODO: 返回所有标志都为 false 的结构
        todo!("创建空标志")
    }

    /// 从字节创建标志
    ///
    /// 位布局: CWR ECE URG ACK PSH RST SYN FIN
    ///         7   6   5   4   3   2   1   0
    pub fn from_byte(byte: u8) -> Self {
        // TODO: 从字节解析标志
        // 提示: 使用位与操作检查每一位
        todo!("从字节创建标志")
    }

    /// 将标志转换为字节
    pub fn to_byte(&self) -> u8 {
        // TODO: 将标志组合为字节
        // 提示: 使用位或操作
        todo!("转换为字节")
    }

    /// 创建 SYN 标志
    pub fn syn() -> Self {
        // TODO: 返回只设置了 SYN 的标志
        todo!("创建 SYN 标志")
    }

    /// 创建 SYN+ACK 标志
    pub fn syn_ack() -> Self {
        // TODO: 返回设置了 SYN 和 ACK 的标志
        todo!("创建 SYN+ACK 标志")
    }

    /// 创建 ACK 标志
    pub fn ack_only() -> Self {
        // TODO: 返回只设置了 ACK 的标志
        todo!("创建 ACK 标志")
    }

    /// 创建 FIN+ACK 标志
    pub fn fin_ack() -> Self {
        // TODO: 返回设置了 FIN 和 ACK 的标志
        todo!("创建 FIN+ACK 标志")
    }

    /// 创建 RST 标志
    pub fn rst_only() -> Self {
        // TODO: 返回只设置了 RST 的标志
        todo!("创建 RST 标志")
    }

    /// 创建 PSH+ACK 标志
    pub fn psh_ack() -> Self {
        // TODO: 返回设置了 PSH 和 ACK 的标志
        todo!("创建 PSH+ACK 标志")
    }

    /// 检查是否是 SYN 包（只有 SYN，没有 ACK）
    pub fn is_syn_only(&self) -> bool {
        // TODO: 检查 SYN=1 且 ACK=0
        todo!("检查 SYN 包")
    }

    /// 检查是否是 SYN-ACK 包
    pub fn is_syn_ack(&self) -> bool {
        // TODO: 检查 SYN=1 且 ACK=1
        todo!("检查 SYN-ACK 包")
    }

    /// 检查是否是纯 ACK 包（只有 ACK，没有数据标志）
    pub fn is_ack_only(&self) -> bool {
        // TODO: 检查只有 ACK 被设置
        todo!("检查纯 ACK 包")
    }

    /// 检查是否是 FIN 包
    pub fn has_fin(&self) -> bool {
        // TODO: 检查 FIN 位
        todo!("检查 FIN 包")
    }

    /// 检查是否是 RST 包
    pub fn has_rst(&self) -> bool {
        // TODO: 检查 RST 位
        todo!("检查 RST 包")
    }

    /// 返回标志的字符串表示
    ///
    /// 格式: "[SYN,ACK,...]"
    pub fn to_string(&self) -> String {
        // TODO: 生成标志的字符串表示
        // 例如: "[SYN]", "[SYN,ACK]", "[FIN,ACK]"
        todo!("格式化标志")
    }
}

/// 根据标志判断 TCP 段类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpSegmentType {
    /// SYN 段（连接请求）
    Syn,
    /// SYN-ACK 段（连接响应）
    SynAck,
    /// ACK 段（普通确认或数据）
    Ack,
    /// FIN 段（连接终止）
    Fin,
    /// FIN-ACK 段
    FinAck,
    /// RST 段（连接重置）
    Rst,
    /// 其他
    Other,
}

impl TcpSegmentType {
    /// 从标志判断段类型
    pub fn from_flags(flags: &TcpFlags) -> Self {
        // TODO: 根据标志判断段类型
        //
        // 判断顺序很重要:
        // 1. RST 优先级最高
        // 2. SYN+ACK
        // 3. SYN
        // 4. FIN+ACK
        // 5. FIN
        // 6. 纯 ACK
        // 7. 其他
        todo!("从标志判断段类型")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_flags_empty() {
        let flags = TcpFlags::empty();
        assert!(!flags.fin);
        assert!(!flags.syn);
        assert!(!flags.rst);
        assert!(!flags.psh);
        assert!(!flags.ack);
        assert!(!flags.urg);
    }

    #[test]
    fn test_tcp_flags_from_byte() {
        // SYN = 0x02
        let flags = TcpFlags::from_byte(0x02);
        assert!(flags.syn);
        assert!(!flags.ack);
        assert!(!flags.fin);

        // SYN+ACK = 0x12
        let flags = TcpFlags::from_byte(0x12);
        assert!(flags.syn);
        assert!(flags.ack);

        // FIN+ACK = 0x11
        let flags = TcpFlags::from_byte(0x11);
        assert!(flags.fin);
        assert!(flags.ack);

        // 全部标志 = 0xFF
        let flags = TcpFlags::from_byte(0xFF);
        assert!(flags.fin);
        assert!(flags.syn);
        assert!(flags.rst);
        assert!(flags.psh);
        assert!(flags.ack);
        assert!(flags.urg);
        assert!(flags.ece);
        assert!(flags.cwr);
    }

    #[test]
    fn test_tcp_flags_to_byte() {
        assert_eq!(TcpFlags::syn().to_byte(), 0x02);
        assert_eq!(TcpFlags::syn_ack().to_byte(), 0x12);
        assert_eq!(TcpFlags::ack_only().to_byte(), 0x10);
        assert_eq!(TcpFlags::fin_ack().to_byte(), 0x11);
        assert_eq!(TcpFlags::rst_only().to_byte(), 0x04);
        assert_eq!(TcpFlags::psh_ack().to_byte(), 0x18);
    }

    #[test]
    fn test_tcp_flags_checks() {
        assert!(TcpFlags::syn().is_syn_only());
        assert!(!TcpFlags::syn_ack().is_syn_only());

        assert!(TcpFlags::syn_ack().is_syn_ack());
        assert!(!TcpFlags::syn().is_syn_ack());

        assert!(TcpFlags::ack_only().is_ack_only());
        assert!(!TcpFlags::syn_ack().is_ack_only());

        assert!(TcpFlags::fin_ack().has_fin());
        assert!(TcpFlags::rst_only().has_rst());
    }

    #[test]
    fn test_tcp_flags_to_string() {
        assert!(TcpFlags::syn().to_string().contains("SYN"));
        assert!(!TcpFlags::syn().to_string().contains("ACK"));

        let syn_ack_str = TcpFlags::syn_ack().to_string();
        assert!(syn_ack_str.contains("SYN"));
        assert!(syn_ack_str.contains("ACK"));
    }

    #[test]
    fn test_tcp_segment_type() {
        assert_eq!(TcpSegmentType::from_flags(&TcpFlags::syn()), TcpSegmentType::Syn);
        assert_eq!(TcpSegmentType::from_flags(&TcpFlags::syn_ack()), TcpSegmentType::SynAck);
        assert_eq!(TcpSegmentType::from_flags(&TcpFlags::ack_only()), TcpSegmentType::Ack);
        assert_eq!(TcpSegmentType::from_flags(&TcpFlags::fin_ack()), TcpSegmentType::FinAck);
        assert_eq!(TcpSegmentType::from_flags(&TcpFlags::rst_only()), TcpSegmentType::Rst);
    }
}
