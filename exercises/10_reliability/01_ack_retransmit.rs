//! # 练习 01: 确认与重传
//!
//! 确认（ACK）和重传是保证可靠传输的核心机制。
//!
//! ## 基本原理
//!
//! ```text
//! Sender                          Receiver
//!   |                                 |
//!   | -------- Data(seq=1) -------->  |
//!   |                                 |
//!   | <-------- ACK(1) -------------- |
//!   |                                 |
//!   | -------- Data(seq=2) ----X      |  (丢失)
//!   |                                 |
//!   | -------- (timeout) ---------->  |
//!   |                                 |
//!   | -------- Data(seq=2) -------->  |  (重传)
//!   |                                 |
//!   | <-------- ACK(2) -------------- |
//! ```
//!
//! ## 重传策略
//!
//! 1. **固定超时**: 简单但不够灵活
//! 2. **自适应超时**: 根据 RTT 动态调整
//! 3. **指数退避**: 每次重传增加超时时间
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test reliability_ack
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 序列号类型
pub type SeqNum = u32;

/// 数据包状态
#[derive(Debug, Clone)]
pub struct PacketState {
    /// 数据内容
    pub data: Vec<u8>,
    /// 发送时间
    pub sent_at: Instant,
    /// 重传次数
    pub retransmit_count: u32,
    /// 当前超时时间
    pub timeout: Duration,
}

/// 重传配置
#[derive(Debug, Clone)]
pub struct RetransmitConfig {
    /// 初始超时时间
    pub initial_timeout: Duration,
    /// 最大超时时间
    pub max_timeout: Duration,
    /// 最大重传次数
    pub max_retries: u32,
    /// 超时倍增因子（指数退避）
    pub backoff_factor: f64,
}

impl Default for RetransmitConfig {
    fn default() -> Self {
        Self {
            initial_timeout: Duration::from_millis(200),
            max_timeout: Duration::from_secs(60),
            max_retries: 5,
            backoff_factor: 2.0,
        }
    }
}

/// 简单的 ARQ (Automatic Repeat reQuest) 发送器
///
/// 实现停止等待协议的发送端
#[derive(Debug)]
pub struct StopAndWaitSender {
    config: RetransmitConfig,
    /// 下一个要使用的序列号
    next_seq: SeqNum,
    /// 当前未确认的包
    unacked: Option<(SeqNum, PacketState)>,
}

impl StopAndWaitSender {
    pub fn new(config: RetransmitConfig) -> Self {
        // TODO: 创建发送器
        todo!("创建停止等待发送器")
    }

    /// 发送数据
    ///
    /// # 返回值
    /// (序列号, 要发送的数据)
    pub fn send(&mut self, data: Vec<u8>) -> Result<(SeqNum, Vec<u8>), &'static str> {
        // TODO: 发送数据
        // 1. 检查是否有未确认的包
        // 2. 分配序列号
        // 3. 记录包状态
        // 4. 返回要发送的内容
        todo!("发送数据")
    }

    /// 处理确认
    ///
    /// # 返回值
    /// true 如果确认有效
    pub fn ack_received(&mut self, seq: SeqNum) -> bool {
        // TODO: 处理 ACK
        // 如果 seq 匹配当前未确认的包，清除它
        todo!("处理确认")
    }

    /// 检查是否需要重传
    ///
    /// # 返回值
    /// 如果需要重传，返回 Some((seq, data))
    pub fn check_retransmit(&mut self) -> Option<(SeqNum, Vec<u8>)> {
        // TODO: 检查超时和重传
        // 1. 检查是否有未确认的包
        // 2. 检查是否超时
        // 3. 如果超时，增加重传计数
        // 4. 应用指数退避
        // 5. 返回要重传的数据
        todo!("检查重传")
    }

    /// 是否可以发送新数据
    pub fn can_send(&self) -> bool {
        self.unacked.is_none()
    }

    /// 获取当前重传次数
    pub fn retransmit_count(&self) -> u32 {
        self.unacked.as_ref().map(|(_, s)| s.retransmit_count).unwrap_or(0)
    }
}

/// 停止等待协议的接收端
#[derive(Debug)]
pub struct StopAndWaitReceiver {
    /// 期望的下一个序列号
    expected_seq: SeqNum,
}

impl StopAndWaitReceiver {
    pub fn new() -> Self {
        // TODO: 创建接收器
        todo!("创建停止等待接收器")
    }

    /// 处理收到的数据
    ///
    /// # 返回值
    /// (要发送的 ACK 序列号, 是否是新数据)
    pub fn receive(&mut self, seq: SeqNum, _data: &[u8]) -> (SeqNum, bool) {
        // TODO: 处理接收
        // 1. 检查序列号是否是期望的
        // 2. 如果是，更新期望序列号，返回 (seq, true)
        // 3. 如果不是（重复包），返回 (expected_seq - 1, false)
        todo!("处理接收")
    }
}

impl Default for StopAndWaitReceiver {
    fn default() -> Self {
        Self::new()
    }
}

/// Go-Back-N 发送器
///
/// 允许发送多个未确认的包（滑动窗口）
#[derive(Debug)]
pub struct GoBackNSender {
    config: RetransmitConfig,
    /// 窗口大小
    window_size: usize,
    /// 下一个要发送的序列号
    next_seq: SeqNum,
    /// 最小未确认的序列号
    base: SeqNum,
    /// 未确认的包
    unacked: HashMap<SeqNum, PacketState>,
}

impl GoBackNSender {
    pub fn new(config: RetransmitConfig, window_size: usize) -> Self {
        // TODO: 创建 Go-Back-N 发送器
        todo!("创建 Go-Back-N 发送器")
    }

    /// 发送数据
    pub fn send(&mut self, data: Vec<u8>) -> Result<(SeqNum, Vec<u8>), &'static str> {
        // TODO: 发送数据
        // 检查窗口是否已满
        todo!("发送数据")
    }

    /// 处理累积确认
    ///
    /// Go-Back-N 使用累积确认：ACK(n) 表示序列号 <= n 的包都已收到
    pub fn ack_received(&mut self, seq: SeqNum) {
        // TODO: 处理累积确认
        // 移除所有序列号 <= seq 的包
        todo!("处理累积确认")
    }

    /// 检查超时并返回需要重传的所有包
    pub fn check_retransmit(&mut self) -> Vec<(SeqNum, Vec<u8>)> {
        // TODO: Go-Back-N 重传
        // 如果最老的包超时，重传从 base 开始的所有包
        todo!("检查重传")
    }

    /// 可以发送多少个包
    pub fn available_window(&self) -> usize {
        self.window_size - self.unacked.len()
    }
}

/// 自适应超时计算器 (Jacobson's Algorithm)
///
/// TCP 使用的 RTT 估计和超时计算算法
#[derive(Debug)]
pub struct AdaptiveTimeout {
    /// 平滑 RTT 估计值
    srtt: Option<Duration>,
    /// RTT 变化估计值
    rttvar: Option<Duration>,
    /// 计算出的重传超时
    rto: Duration,
    /// 最小 RTO
    min_rto: Duration,
    /// 最大 RTO
    max_rto: Duration,
}

impl AdaptiveTimeout {
    pub fn new() -> Self {
        // TODO: 创建自适应超时计算器
        // 初始 RTO 通常是 1 秒
        todo!("创建自适应超时")
    }

    /// 更新 RTT 样本
    ///
    /// 使用 Jacobson's Algorithm:
    /// - SRTT = (1 - α) * SRTT + α * RTT，α = 1/8
    /// - RTTVAR = (1 - β) * RTTVAR + β * |SRTT - RTT|，β = 1/4
    /// - RTO = SRTT + 4 * RTTVAR
    pub fn update(&mut self, rtt: Duration) {
        // TODO: 更新 RTT 估计
        todo!("更新 RTT 样本")
    }

    /// 获取当前 RTO
    pub fn rto(&self) -> Duration {
        self.rto
    }

    /// 重传时的退避
    pub fn backoff(&mut self) {
        // TODO: 将 RTO 翻倍（指数退避）
        todo!("RTO 退避")
    }

    /// 获取平滑 RTT
    pub fn srtt(&self) -> Option<Duration> {
        self.srtt
    }
}

impl Default for AdaptiveTimeout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_ack_stop_and_wait_send() {
        let mut sender = StopAndWaitSender::new(RetransmitConfig::default());

        assert!(sender.can_send());

        let (seq, data) = sender.send(b"Hello".to_vec()).unwrap();
        assert_eq!(seq, 0);
        assert_eq!(data, b"Hello");

        // 不能再发送，等待 ACK
        assert!(!sender.can_send());
        assert!(sender.send(b"World".to_vec()).is_err());
    }

    #[test]
    fn test_reliability_ack_stop_and_wait_ack() {
        let mut sender = StopAndWaitSender::new(RetransmitConfig::default());

        let (seq, _) = sender.send(b"Hello".to_vec()).unwrap();
        assert!(sender.ack_received(seq));
        assert!(sender.can_send());

        // 可以发送下一个
        let (seq2, _) = sender.send(b"World".to_vec()).unwrap();
        assert_eq!(seq2, 1);
    }

    #[test]
    fn test_reliability_ack_stop_and_wait_retransmit() {
        let config = RetransmitConfig {
            initial_timeout: Duration::from_millis(10),
            ..Default::default()
        };
        let mut sender = StopAndWaitSender::new(config);

        let (seq, _) = sender.send(b"Hello".to_vec()).unwrap();

        // 等待超时
        std::thread::sleep(Duration::from_millis(20));

        let retransmit = sender.check_retransmit();
        assert!(retransmit.is_some());
        let (rseq, rdata) = retransmit.unwrap();
        assert_eq!(rseq, seq);
        assert_eq!(rdata, b"Hello");
        assert_eq!(sender.retransmit_count(), 1);
    }

    #[test]
    fn test_reliability_ack_receiver() {
        let mut receiver = StopAndWaitReceiver::new();

        // 正常接收
        let (ack, is_new) = receiver.receive(0, b"Hello");
        assert_eq!(ack, 0);
        assert!(is_new);

        // 重复包
        let (ack, is_new) = receiver.receive(0, b"Hello");
        assert_eq!(ack, 0);
        assert!(!is_new);

        // 下一个包
        let (ack, is_new) = receiver.receive(1, b"World");
        assert_eq!(ack, 1);
        assert!(is_new);
    }

    #[test]
    fn test_reliability_ack_gobackn() {
        let mut sender = GoBackNSender::new(RetransmitConfig::default(), 4);

        // 可以连续发送多个
        sender.send(b"Msg0".to_vec()).unwrap();
        sender.send(b"Msg1".to_vec()).unwrap();
        sender.send(b"Msg2".to_vec()).unwrap();

        assert_eq!(sender.available_window(), 1);

        // 累积确认
        sender.ack_received(1);
        assert_eq!(sender.available_window(), 3); // 确认了 0 和 1
    }

    #[test]
    fn test_reliability_ack_adaptive_timeout() {
        let mut ato = AdaptiveTimeout::new();

        // 初始 RTO
        assert!(ato.rto() >= Duration::from_millis(100));

        // 更新 RTT
        ato.update(Duration::from_millis(100));
        ato.update(Duration::from_millis(120));
        ato.update(Duration::from_millis(80));

        assert!(ato.srtt().is_some());

        // 退避
        let old_rto = ato.rto();
        ato.backoff();
        assert!(ato.rto() > old_rto);
    }
}
