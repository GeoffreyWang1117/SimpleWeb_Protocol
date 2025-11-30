//! # 练习 03: 拥塞控制
//!
//! 拥塞控制防止网络过载，是 TCP 的核心机制。
//!
//! ## 拥塞 vs 流量控制
//!
//! - **流量控制**: 防止发送方淹没接收方
//! - **拥塞控制**: 防止发送方淹没网络
//!
//! ## TCP 拥塞控制阶段
//!
//! ```text
//!          |
//! cwnd     |      /-------------- 拥塞避免
//!          |     /
//!          |    /
//!          |   /
//!          |  /
//!          | / 慢启动
//!          |/
//!          +-------------------------->
//!                     time
//! ```
//!
//! ## 主要算法
//!
//! 1. **慢启动 (Slow Start)**: 指数增长
//! 2. **拥塞避免 (Congestion Avoidance)**: 线性增长
//! 3. **快速重传 (Fast Retransmit)**: 3个重复ACK触发重传
//! 4. **快速恢复 (Fast Recovery)**: 避免重新慢启动
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test reliability_congestion
//! ```

/// 拥塞控制状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionState {
    /// 慢启动
    SlowStart,
    /// 拥塞避免
    CongestionAvoidance,
    /// 快速恢复
    FastRecovery,
}

/// 拥塞控制器 (TCP Reno 风格)
#[derive(Debug)]
pub struct CongestionController {
    /// 拥塞窗口大小（字节）
    cwnd: u32,
    /// 慢启动阈值
    ssthresh: u32,
    /// 当前状态
    state: CongestionState,
    /// 最大段大小
    mss: u32,
    /// 重复 ACK 计数
    dup_ack_count: u32,
}

impl CongestionController {
    /// 创建拥塞控制器
    ///
    /// # 参数
    /// * `mss` - 最大段大小（通常 1460 字节）
    pub fn new(mss: u32) -> Self {
        // TODO: 初始化拥塞控制器
        // 初始 cwnd = 1 MSS（或更现代的 10 MSS）
        // 初始 ssthresh = 65535
        todo!("创建拥塞控制器")
    }

    /// 获取当前拥塞窗口大小
    pub fn cwnd(&self) -> u32 {
        self.cwnd
    }

    /// 获取当前状态
    pub fn state(&self) -> CongestionState {
        self.state
    }

    /// 收到新的 ACK
    ///
    /// 根据当前状态更新 cwnd
    pub fn on_ack(&mut self) {
        // TODO: 处理 ACK
        //
        // 慢启动: cwnd += mss（指数增长）
        // 拥塞避免: cwnd += mss * mss / cwnd（线性增长）
        // 快速恢复: cwnd += mss
        //
        // 如果在慢启动中 cwnd >= ssthresh，转换到拥塞避免
        todo!("处理 ACK")
    }

    /// 收到重复 ACK
    ///
    /// 3 个重复 ACK 触发快速重传和快速恢复
    pub fn on_dup_ack(&mut self) -> bool {
        // TODO: 处理重复 ACK
        //
        // 增加 dup_ack_count
        // 如果达到 3:
        //   - ssthresh = cwnd / 2
        //   - cwnd = ssthresh + 3 * mss
        //   - 进入快速恢复
        //   - 返回 true（表示应该快速重传）
        todo!("处理重复 ACK")
    }

    /// 超时事件
    ///
    /// 超时表示严重拥塞，需要大幅减小窗口
    pub fn on_timeout(&mut self) {
        // TODO: 处理超时
        //
        // ssthresh = cwnd / 2
        // cwnd = 1 MSS
        // 回到慢启动
        // 重置 dup_ack_count
        todo!("处理超时")
    }

    /// 快速恢复完成（收到新数据的 ACK）
    pub fn exit_fast_recovery(&mut self) {
        // TODO: 退出快速恢复
        //
        // cwnd = ssthresh
        // 进入拥塞避免
        // 重置 dup_ack_count
        todo!("退出快速恢复")
    }
}

/// TCP CUBIC 拥塞控制
///
/// 现代 Linux 默认使用的拥塞控制算法
#[derive(Debug)]
pub struct CubicController {
    /// 拥塞窗口
    cwnd: f64,
    /// 上次拥塞事件时的窗口大小
    wmax: f64,
    /// 上次拥塞事件的时间
    epoch_start: Option<std::time::Instant>,
    /// CUBIC 参数
    c: f64,
    /// beta (乘法减少因子)
    beta: f64,
    /// MSS
    mss: u32,
}

impl CubicController {
    pub fn new(mss: u32) -> Self {
        // TODO: 创建 CUBIC 控制器
        // 默认 C = 0.4, beta = 0.7
        todo!("创建 CUBIC 控制器")
    }

    /// CUBIC 窗口计算
    ///
    /// W(t) = C * (t - K)³ + Wmax
    /// K = ³√(Wmax * beta / C)
    pub fn calculate_cwnd(&self, t: f64) -> f64 {
        // TODO: 计算 CUBIC 窗口
        todo!("计算 CUBIC 窗口")
    }

    /// 收到 ACK
    pub fn on_ack(&mut self) {
        // TODO: 更新 CUBIC 窗口
        todo!("CUBIC on_ack")
    }

    /// 丢包事件
    pub fn on_loss(&mut self) {
        // TODO: 处理丢包
        // wmax = cwnd
        // cwnd = cwnd * beta
        todo!("CUBIC on_loss")
    }

    pub fn cwnd(&self) -> u32 {
        self.cwnd as u32
    }
}

/// BBR 拥塞控制
///
/// Google 开发的基于带宽和延迟的拥塞控制
#[derive(Debug)]
pub struct BbrController {
    /// 估计的瓶颈带宽
    btl_bw: f64,
    /// 估计的最小 RTT
    min_rtt: Option<std::time::Duration>,
    /// 当前模式
    mode: BbrMode,
    /// pacing 增益
    pacing_gain: f64,
    /// cwnd 增益
    cwnd_gain: f64,
}

/// BBR 模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BbrMode {
    /// 启动阶段，探测带宽
    Startup,
    /// 排空阶段
    Drain,
    /// 带宽探测
    ProbeBW,
    /// RTT 探测
    ProbeRTT,
}

impl BbrController {
    pub fn new() -> Self {
        // TODO: 创建 BBR 控制器
        todo!("创建 BBR 控制器")
    }

    /// 更新带宽估计
    pub fn update_bandwidth(&mut self, delivered: u64, interval: std::time::Duration) {
        // TODO: 更新带宽估计
        // btl_bw = max(btl_bw, delivered / interval)
        todo!("更新带宽")
    }

    /// 更新 RTT 估计
    pub fn update_rtt(&mut self, rtt: std::time::Duration) {
        // TODO: 更新最小 RTT
        todo!("更新 RTT")
    }

    /// 计算发送速率
    pub fn pacing_rate(&self) -> f64 {
        // TODO: pacing_rate = btl_bw * pacing_gain
        todo!("计算发送速率")
    }

    /// 计算拥塞窗口
    pub fn cwnd(&self) -> u32 {
        // TODO: cwnd = btl_bw * min_rtt * cwnd_gain
        todo!("计算拥塞窗口")
    }

    pub fn mode(&self) -> BbrMode {
        self.mode
    }
}

impl Default for BbrController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_congestion_init() {
        let cc = CongestionController::new(1460);
        assert_eq!(cc.state(), CongestionState::SlowStart);
        assert!(cc.cwnd() >= 1460);
    }

    #[test]
    fn test_reliability_congestion_slow_start() {
        let mut cc = CongestionController::new(1000);
        let initial = cc.cwnd();

        cc.on_ack();
        // 慢启动：每个 ACK 增加 1 MSS
        assert_eq!(cc.cwnd(), initial + 1000);

        cc.on_ack();
        assert_eq!(cc.cwnd(), initial + 2000);
    }

    #[test]
    fn test_reliability_congestion_transition() {
        let mut cc = CongestionController::new(1000);

        // 增长到超过 ssthresh
        for _ in 0..100 {
            cc.on_ack();
            if cc.state() == CongestionState::CongestionAvoidance {
                break;
            }
        }

        assert_eq!(cc.state(), CongestionState::CongestionAvoidance);
    }

    #[test]
    fn test_reliability_congestion_timeout() {
        let mut cc = CongestionController::new(1000);

        // 先增大窗口
        for _ in 0..10 {
            cc.on_ack();
        }
        let cwnd_before = cc.cwnd();

        cc.on_timeout();

        // 超时后回到慢启动，cwnd = 1 MSS
        assert_eq!(cc.state(), CongestionState::SlowStart);
        assert_eq!(cc.cwnd(), 1000);
        assert!(cc.cwnd() < cwnd_before);
    }

    #[test]
    fn test_reliability_congestion_fast_retransmit() {
        let mut cc = CongestionController::new(1000);

        // 先增大窗口
        for _ in 0..10 {
            cc.on_ack();
        }

        // 3 个重复 ACK
        assert!(!cc.on_dup_ack());
        assert!(!cc.on_dup_ack());
        assert!(cc.on_dup_ack()); // 第 3 个触发快速重传

        assert_eq!(cc.state(), CongestionState::FastRecovery);
    }

    #[test]
    fn test_reliability_congestion_cubic() {
        let mut cubic = CubicController::new(1460);

        // 模拟 ACK
        for _ in 0..10 {
            cubic.on_ack();
        }

        let cwnd_before = cubic.cwnd();

        // 丢包
        cubic.on_loss();

        // cwnd 应该减小
        assert!(cubic.cwnd() < cwnd_before);
    }
}
