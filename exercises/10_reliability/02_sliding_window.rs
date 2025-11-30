//! # 练习 02: 滑动窗口
//!
//! 滑动窗口是实现高效可靠传输的核心机制。
//!
//! ## 窗口的作用
//!
//! 1. **流量控制**: 防止发送方发送过快
//! 2. **提高效率**: 允许多个未确认的包
//! 3. **拥塞控制**: 根据网络状况调整窗口
//!
//! ## 发送窗口
//!
//! ```text
//! 序列号:  0  1  2  3  4  5  6  7  8  9  10 11 12
//!         [已确认][  发送未确认  ][可发送][不可发送]
//!                ^                ^
//!              base            next_seq
//!         |<----- 窗口大小 ----->|
//! ```
//!
//! ## 接收窗口
//!
//! ```text
//! 序列号:  0  1  2  3  4  5  6  7  8  9  10 11 12
//!         [已接收][等待接收][可接收][不可接收]
//!                ^
//!              base
//!         |<-- 窗口 -->|
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test reliability_window
//! ```

use std::collections::{HashMap, VecDeque};

/// 发送窗口
#[derive(Debug)]
pub struct SendWindow {
    /// 窗口大小
    size: usize,
    /// 已确认的最后一个序列号 + 1
    base: u32,
    /// 下一个要发送的序列号
    next_seq: u32,
    /// 窗口内的数据
    buffer: VecDeque<Option<Vec<u8>>>,
}

impl SendWindow {
    /// 创建发送窗口
    pub fn new(size: usize) -> Self {
        // TODO: 创建发送窗口
        todo!("创建发送窗口")
    }

    /// 获取窗口大小
    pub fn size(&self) -> usize {
        self.size
    }

    /// 获取可用的窗口空间
    pub fn available(&self) -> usize {
        // TODO: 计算可以发送多少个包
        // window_size - (next_seq - base)
        todo!("计算可用窗口")
    }

    /// 检查序列号是否在窗口内
    pub fn in_window(&self, seq: u32) -> bool {
        // TODO: 检查 seq 是否在 [base, base + size) 范围内
        todo!("检查窗口范围")
    }

    /// 发送数据，返回分配的序列号
    pub fn send(&mut self, data: Vec<u8>) -> Option<u32> {
        // TODO: 发送数据
        // 1. 检查窗口是否有空间
        // 2. 分配序列号
        // 3. 存储数据
        // 4. 返回序列号
        todo!("发送数据")
    }

    /// 处理确认
    ///
    /// # 参数
    /// * `seq` - 确认的序列号（累积确认）
    pub fn ack(&mut self, seq: u32) {
        // TODO: 处理累积确认
        // 移动 base 到 seq + 1
        // 清除已确认的数据
        todo!("处理确认")
    }

    /// 获取指定序列号的数据（用于重传）
    pub fn get(&self, seq: u32) -> Option<&Vec<u8>> {
        // TODO: 获取数据
        todo!("获取数据")
    }

    /// 获取所有未确认的数据
    pub fn unacked(&self) -> Vec<(u32, &Vec<u8>)> {
        // TODO: 返回所有未确认的数据
        todo!("获取未确认数据")
    }
}

/// 接收窗口
#[derive(Debug)]
pub struct RecvWindow {
    /// 窗口大小
    size: usize,
    /// 期望的下一个序列号
    base: u32,
    /// 已接收但乱序的数据
    buffer: HashMap<u32, Vec<u8>>,
}

impl RecvWindow {
    /// 创建接收窗口
    pub fn new(size: usize) -> Self {
        // TODO: 创建接收窗口
        todo!("创建接收窗口")
    }

    /// 检查序列号是否在接收窗口内
    pub fn in_window(&self, seq: u32) -> bool {
        // TODO: 检查序列号是否可接收
        todo!("检查接收窗口")
    }

    /// 接收数据
    ///
    /// # 返回值
    /// (要发送的 ACK, 是否有连续数据可以交付)
    pub fn receive(&mut self, seq: u32, data: Vec<u8>) -> (u32, bool) {
        // TODO: 处理接收
        // 1. 检查是否在窗口内
        // 2. 如果是期望的序列号，交付并检查后续的乱序数据
        // 3. 如果不是，缓存乱序数据
        // 4. 返回 ACK（期望的下一个序列号 - 1）
        todo!("接收数据")
    }

    /// 获取可以交付的连续数据
    pub fn deliver(&mut self) -> Vec<Vec<u8>> {
        // TODO: 返回所有可以交付的数据
        // 按顺序返回从 base 开始的连续数据
        todo!("交付数据")
    }

    /// 获取期望的下一个序列号
    pub fn expected_seq(&self) -> u32 {
        self.base
    }

    /// 获取已缓存的乱序包数量
    pub fn buffered_count(&self) -> usize {
        self.buffer.len()
    }
}

/// 选择性重传接收窗口
///
/// 与 Go-Back-N 不同，选择性重传可以缓存乱序的包
#[derive(Debug)]
pub struct SelectiveRecvWindow {
    size: usize,
    base: u32,
    /// 接收状态：Some = 已收到，None = 未收到
    received: VecDeque<Option<Vec<u8>>>,
}

impl SelectiveRecvWindow {
    pub fn new(size: usize) -> Self {
        // TODO: 创建选择性重传接收窗口
        todo!("创建选择性重传接收窗口")
    }

    /// 接收数据
    ///
    /// # 返回值
    /// 要发送的 SACK 信息（已接收的序列号列表）
    pub fn receive(&mut self, seq: u32, data: Vec<u8>) -> Vec<u32> {
        // TODO: 处理接收，返回 SACK
        todo!("选择性接收")
    }

    /// 获取可以交付的数据
    pub fn deliver(&mut self) -> Vec<Vec<u8>> {
        // TODO: 交付连续的数据
        todo!("交付数据")
    }

    /// 生成 SACK 块
    ///
    /// 返回已接收的序列号范围列表
    pub fn sack_blocks(&self) -> Vec<(u32, u32)> {
        // TODO: 生成 SACK 块
        // 例如: [(5, 7), (10, 12)] 表示已收到 5-7 和 10-12
        todo!("生成 SACK 块")
    }
}

/// 流量控制窗口
///
/// 根据接收方的能力调整窗口大小
#[derive(Debug)]
pub struct FlowControlWindow {
    /// 发送窗口大小
    send_window: usize,
    /// 接收方通告的窗口大小
    receiver_window: usize,
    /// 已发送但未确认的字节数
    in_flight: usize,
}

impl FlowControlWindow {
    pub fn new(initial_window: usize) -> Self {
        // TODO: 创建流量控制窗口
        todo!("创建流量控制窗口")
    }

    /// 更新接收方窗口大小（从 ACK 中获取）
    pub fn update_receiver_window(&mut self, window: usize) {
        // TODO: 更新窗口
        todo!("更新接收方窗口")
    }

    /// 记录发送的字节数
    pub fn bytes_sent(&mut self, bytes: usize) {
        self.in_flight += bytes;
    }

    /// 记录确认的字节数
    pub fn bytes_acked(&mut self, bytes: usize) {
        self.in_flight = self.in_flight.saturating_sub(bytes);
    }

    /// 可以发送的字节数
    pub fn available(&self) -> usize {
        // TODO: min(send_window, receiver_window) - in_flight
        todo!("计算可发送字节")
    }

    /// 是否可以发送
    pub fn can_send(&self) -> bool {
        self.available() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_window_send_new() {
        let window = SendWindow::new(4);
        assert_eq!(window.size(), 4);
        assert_eq!(window.available(), 4);
    }

    #[test]
    fn test_reliability_window_send() {
        let mut window = SendWindow::new(4);

        let seq0 = window.send(b"A".to_vec()).unwrap();
        assert_eq!(seq0, 0);
        assert_eq!(window.available(), 3);

        let seq1 = window.send(b"B".to_vec()).unwrap();
        assert_eq!(seq1, 1);
        assert_eq!(window.available(), 2);
    }

    #[test]
    fn test_reliability_window_send_full() {
        let mut window = SendWindow::new(2);

        window.send(b"A".to_vec()).unwrap();
        window.send(b"B".to_vec()).unwrap();

        // 窗口已满
        assert!(window.send(b"C".to_vec()).is_none());
    }

    #[test]
    fn test_reliability_window_ack() {
        let mut window = SendWindow::new(4);

        window.send(b"A".to_vec()).unwrap();
        window.send(b"B".to_vec()).unwrap();
        window.send(b"C".to_vec()).unwrap();

        assert_eq!(window.available(), 1);

        // ACK 1（累积确认 0 和 1）
        window.ack(1);
        assert_eq!(window.available(), 3);
    }

    #[test]
    fn test_reliability_window_recv() {
        let mut window = RecvWindow::new(4);

        // 按序接收
        let (ack, new_data) = window.receive(0, b"A".to_vec());
        assert_eq!(ack, 0);
        assert!(new_data);

        // 乱序接收
        let (ack, new_data) = window.receive(2, b"C".to_vec());
        assert_eq!(ack, 0); // 还在等待 1
        assert!(!new_data); // 不能立即交付

        assert_eq!(window.buffered_count(), 1);
    }

    #[test]
    fn test_reliability_window_recv_deliver() {
        let mut window = RecvWindow::new(4);

        window.receive(0, b"A".to_vec());
        window.receive(2, b"C".to_vec()); // 乱序
        window.receive(1, b"B".to_vec()); // 填补空缺

        let delivered = window.deliver();
        assert_eq!(delivered.len(), 3);
        assert_eq!(delivered[0], b"A");
        assert_eq!(delivered[1], b"B");
        assert_eq!(delivered[2], b"C");
    }

    #[test]
    fn test_reliability_window_flow_control() {
        let mut fc = FlowControlWindow::new(1000);

        fc.bytes_sent(300);
        assert_eq!(fc.available(), 700);

        fc.update_receiver_window(500);
        assert_eq!(fc.available(), 200); // min(1000, 500) - 300

        fc.bytes_acked(100);
        assert_eq!(fc.available(), 300);
    }
}
