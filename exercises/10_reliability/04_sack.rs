//! # 练习 04: 选择性确认 (SACK)
//!
//! SACK (Selective Acknowledgment) 允许接收方精确告知发送方哪些数据已收到。
//!
//! ## 为什么需要 SACK？
//!
//! 传统累积 ACK 的问题:
//! ```text
//! 发送: [1][2][3][4][5]
//! 接收: [1][_][3][4][5]  (2丢失)
//! ACK:  只能说 "收到了1"
//!
//! 发送方不知道3,4,5已收到，可能重传所有
//! ```
//!
//! 使用 SACK:
//! ```text
//! 发送: [1][2][3][4][5]
//! 接收: [1][_][3][4][5]  (2丢失)
//! SACK: "收到了1, 还有3-5"
//!
//! 发送方只需重传2
//! ```
//!
//! ## TCP SACK 选项 (RFC 2018)
//!
//! ```text
//! Kind: 5
//! Length: 可变 (2 + 8*n)
//!
//! +--------+--------+
//! | Kind=5 | Length |
//! +--------+--------+--------+--------+
//! |       Left Edge of 1st Block      |
//! +--------+--------+--------+--------+
//! |       Right Edge of 1st Block     |
//! +--------+--------+--------+--------+
//! |       ...                         |
//! +--------+--------+--------+--------+
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test reliability_sack
//! ```

use std::collections::BTreeSet;

/// SACK 块
///
/// 表示一个连续已接收数据的范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SackBlock {
    /// 左边界（包含）
    pub left: u64,
    /// 右边界（不包含）
    pub right: u64,
}

impl SackBlock {
    /// 创建 SACK 块
    pub fn new(left: u64, right: u64) -> Self {
        assert!(left < right, "Invalid SACK block");
        Self { left, right }
    }

    /// 块的大小
    pub fn size(&self) -> u64 {
        self.right - self.left
    }

    /// 是否包含序列号
    pub fn contains(&self, seq: u64) -> bool {
        seq >= self.left && seq < self.right
    }

    /// 是否与另一个块相邻或重叠
    pub fn adjacent_or_overlapping(&self, other: &SackBlock) -> bool {
        // TODO: 检查两个块是否相邻或重叠
        // 可以合并的条件: self.right >= other.left && self.left <= other.right
        todo!("检查相邻或重叠")
    }

    /// 合并两个块
    pub fn merge(&self, other: &SackBlock) -> Option<SackBlock> {
        // TODO: 如果可以合并，返回合并后的块
        todo!("合并块")
    }
}

impl PartialOrd for SackBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SackBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.left.cmp(&other.left)
    }
}

/// SACK 信息
#[derive(Debug, Clone, Default)]
pub struct SackInfo {
    /// 累积 ACK（所有小于此值的数据都已收到）
    pub cumulative_ack: u64,
    /// SACK 块列表（按左边界排序）
    pub blocks: Vec<SackBlock>,
}

impl SackInfo {
    pub fn new(cumulative_ack: u64) -> Self {
        Self {
            cumulative_ack,
            blocks: Vec::new(),
        }
    }

    /// 添加一个 SACK 块
    pub fn add_block(&mut self, block: SackBlock) {
        // TODO: 添加块并合并重叠的块
        // 注意: SACK 块应该在累积 ACK 之后
        todo!("添加 SACK 块")
    }

    /// 检查序列号是否已被确认
    pub fn is_acked(&self, seq: u64) -> bool {
        // TODO: 检查序列号是否在累积 ACK 之前或在任意 SACK 块中
        todo!("检查确认状态")
    }

    /// 获取丢失的区间
    pub fn get_gaps(&self) -> Vec<(u64, u64)> {
        // TODO: 返回累积 ACK 和 SACK 块之间的间隙
        todo!("获取间隙")
    }

    /// 更新累积 ACK
    pub fn update_cumulative(&mut self) {
        // TODO: 如果第一个 SACK 块与累积 ACK 相邻，更新累积 ACK
        todo!("更新累积 ACK")
    }
}

/// 接收端 SACK 跟踪器
pub struct SackReceiver {
    /// 已接收的序列号集合
    received: BTreeSet<u64>,
    /// 累积 ACK
    cumulative_ack: u64,
    /// 初始序列号
    initial_seq: u64,
}

impl SackReceiver {
    pub fn new(initial_seq: u64) -> Self {
        Self {
            received: BTreeSet::new(),
            cumulative_ack: initial_seq,
            initial_seq,
        }
    }

    /// 接收数据段
    pub fn receive(&mut self, seq: u64, len: u64) {
        // TODO: 记录接收到的数据
        //
        // 1. 将 [seq, seq+len) 范围的序列号标记为已接收
        // 2. 更新累积 ACK（如果可能）
        todo!("接收数据")
    }

    /// 生成 SACK 信息
    pub fn generate_sack(&self) -> SackInfo {
        // TODO: 生成当前的 SACK 信息
        //
        // 1. 设置累积 ACK
        // 2. 找出所有不连续的已接收块
        // 3. 最多返回 4 个 SACK 块（TCP 限制）
        todo!("生成 SACK")
    }

    /// 检查是否有间隙
    pub fn has_gaps(&self) -> bool {
        // TODO: 检查累积 ACK 之后是否有已接收但不连续的数据
        todo!("检查间隙")
    }
}

/// 发送端 SACK 处理器
pub struct SackSender {
    /// 发送缓冲区中每个段的状态
    segments: Vec<SegmentStatus>,
    /// 基础序列号
    base_seq: u64,
}

/// 数据段状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentStatus {
    /// 已发送，等待确认
    Sent,
    /// 已被 SACK 确认
    SackAcked,
    /// 已被累积 ACK 确认
    Acked,
    /// 标记为丢失，需要重传
    Lost,
    /// 已重传
    Retransmitted,
}

impl SackSender {
    pub fn new(base_seq: u64) -> Self {
        Self {
            segments: Vec::new(),
            base_seq,
        }
    }

    /// 记录发送的段
    pub fn record_send(&mut self, seq: u64, _len: u64) {
        // TODO: 记录已发送的段
        todo!("记录发送")
    }

    /// 处理收到的 SACK 信息
    pub fn process_sack(&mut self, sack: &SackInfo) {
        // TODO: 更新段状态
        //
        // 1. 将小于 cumulative_ack 的段标记为 Acked
        // 2. 将在 SACK 块中的段标记为 SackAcked
        // 3. 将间隙中已发送的段标记为 Lost
        todo!("处理 SACK")
    }

    /// 获取需要重传的段
    pub fn get_segments_to_retransmit(&self) -> Vec<u64> {
        // TODO: 返回标记为 Lost 的段的序列号
        todo!("获取重传段")
    }

    /// 快速重传检测
    ///
    /// 如果一个段被 SACK 跳过 3 次，标记为丢失
    pub fn detect_loss_by_sack(&mut self, threshold: usize) {
        // TODO: 实现基于 SACK 的丢包检测
        todo!("SACK 丢包检测")
    }
}

/// 编码 TCP SACK 选项
pub fn encode_tcp_sack_option(sack: &SackInfo) -> Vec<u8> {
    // TODO: 编码为 TCP 选项格式
    //
    // Kind(1) + Length(1) + N * (Left(4) + Right(4))
    // 最多 4 个块
    todo!("编码 TCP SACK")
}

/// 解码 TCP SACK 选项
pub fn decode_tcp_sack_option(data: &[u8]) -> Option<Vec<SackBlock>> {
    // TODO: 从 TCP 选项解码 SACK 块
    todo!("解码 TCP SACK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_sack_block() {
        let block = SackBlock::new(100, 200);
        assert_eq!(block.size(), 100);
        assert!(block.contains(100));
        assert!(block.contains(199));
        assert!(!block.contains(200));
    }

    #[test]
    fn test_reliability_sack_merge() {
        let a = SackBlock::new(100, 200);
        let b = SackBlock::new(200, 300);
        let merged = a.merge(&b).unwrap();
        assert_eq!(merged.left, 100);
        assert_eq!(merged.right, 300);

        // 重叠块
        let c = SackBlock::new(150, 250);
        let merged2 = a.merge(&c).unwrap();
        assert_eq!(merged2.left, 100);
        assert_eq!(merged2.right, 250);

        // 不相邻
        let d = SackBlock::new(300, 400);
        assert!(a.merge(&d).is_none());
    }

    #[test]
    fn test_reliability_sack_info() {
        let mut sack = SackInfo::new(100);
        sack.add_block(SackBlock::new(200, 300));
        sack.add_block(SackBlock::new(400, 500));

        assert!(sack.is_acked(50));   // 在累积 ACK 之前
        assert!(!sack.is_acked(150)); // 在间隙中
        assert!(sack.is_acked(250));  // 在第一个 SACK 块中
        assert!(!sack.is_acked(350)); // 在间隙中
        assert!(sack.is_acked(450));  // 在第二个 SACK 块中

        let gaps = sack.get_gaps();
        assert_eq!(gaps, vec![(100, 200), (300, 400)]);
    }

    #[test]
    fn test_reliability_sack_receiver() {
        let mut receiver = SackReceiver::new(0);

        // 正常接收
        receiver.receive(0, 100);
        assert_eq!(receiver.cumulative_ack, 100);
        assert!(!receiver.has_gaps());

        // 乱序接收
        receiver.receive(200, 100);
        assert_eq!(receiver.cumulative_ack, 100);
        assert!(receiver.has_gaps());

        // 填补间隙
        receiver.receive(100, 100);
        assert_eq!(receiver.cumulative_ack, 300);
        assert!(!receiver.has_gaps());
    }

    #[test]
    fn test_reliability_sack_generate() {
        let mut receiver = SackReceiver::new(0);
        receiver.receive(0, 100);
        receiver.receive(200, 100);
        receiver.receive(400, 100);

        let sack = receiver.generate_sack();
        assert_eq!(sack.cumulative_ack, 100);
        assert_eq!(sack.blocks.len(), 2);
        assert_eq!(sack.blocks[0].left, 200);
        assert_eq!(sack.blocks[1].left, 400);
    }

    #[test]
    fn test_reliability_sack_tcp_encode() {
        let mut sack = SackInfo::new(1000);
        sack.add_block(SackBlock::new(2000, 3000));

        let encoded = encode_tcp_sack_option(&sack);
        assert_eq!(encoded[0], 5); // Kind
        assert_eq!(encoded[1], 10); // Length: 2 + 8

        let decoded = decode_tcp_sack_option(&encoded).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].left, 2000);
        assert_eq!(decoded[0].right, 3000);
    }
}
