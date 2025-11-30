//! # 练习 00: 心跳机制
//!
//! 心跳用于检测连接是否存活，以及保持连接不被中间设备（如 NAT、防火墙）关闭。
//!
//! ## 心跳的作用
//!
//! 1. **检测死连接**: 对端进程崩溃、网络断开
//! 2. **保持连接**: 防止 NAT 表项过期
//! 3. **测量延迟**: 计算 RTT
//!
//! ## 心跳策略
//!
//! ### 1. 应用层心跳
//! - 自定义 Ping/Pong 消息
//! - 灵活可控
//! - 需要应用实现
//!
//! ### 2. TCP Keepalive
//! - 操作系统实现
//! - 间隔通常很长（默认2小时）
//! - 配置不够灵活
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test reliability_heartbeat
//! ```

use std::time::{Duration, Instant};

/// 心跳配置
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    /// 心跳发送间隔
    pub interval: Duration,
    /// 超时时间（未收到响应）
    pub timeout: Duration,
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        // TODO: 返回默认配置
        // 建议: interval=30s, timeout=5s, max_retries=3
        todo!("默认心跳配置")
    }
}

impl HeartbeatConfig {
    pub fn new(interval: Duration, timeout: Duration, max_retries: u32) -> Self {
        Self {
            interval,
            timeout,
            max_retries,
        }
    }

    /// 快速心跳配置（用于实时应用）
    pub fn fast() -> Self {
        Self::new(Duration::from_secs(5), Duration::from_secs(2), 3)
    }

    /// 慢速心跳配置（用于长连接）
    pub fn slow() -> Self {
        Self::new(Duration::from_secs(60), Duration::from_secs(10), 3)
    }
}

/// 心跳状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatState {
    /// 正常运行
    Active,
    /// 等待 Pong 响应
    WaitingForPong,
    /// 连接超时
    TimedOut,
}

/// 心跳跟踪器
///
/// 跟踪心跳状态和计时
#[derive(Debug)]
pub struct HeartbeatTracker {
    config: HeartbeatConfig,
    state: HeartbeatState,
    /// 上次发送 Ping 的时间
    last_ping_sent: Option<Instant>,
    /// 上次收到 Pong 的时间
    last_pong_received: Option<Instant>,
    /// 上次收到任何数据的时间
    last_activity: Instant,
    /// 当前重试次数
    retry_count: u32,
    /// RTT 采样
    rtt_samples: Vec<Duration>,
}

impl HeartbeatTracker {
    /// 创建心跳跟踪器
    pub fn new(config: HeartbeatConfig) -> Self {
        // TODO: 初始化跟踪器
        todo!("创建心跳跟踪器")
    }

    /// 获取当前状态
    pub fn state(&self) -> HeartbeatState {
        self.state
    }

    /// 检查是否需要发送心跳
    ///
    /// 如果距离上次活动超过 interval，返回 true
    pub fn should_send_ping(&self) -> bool {
        // TODO: 检查是否需要发送 Ping
        // 条件: 状态是 Active 且 距离上次活动 >= interval
        todo!("检查是否需要发送 Ping")
    }

    /// 记录发送了 Ping
    pub fn ping_sent(&mut self) {
        // TODO: 记录 Ping 发送时间
        // 更新状态为 WaitingForPong
        todo!("记录 Ping 发送")
    }

    /// 记录收到了 Pong
    ///
    /// # 返回值
    /// 本次心跳的 RTT
    pub fn pong_received(&mut self) -> Option<Duration> {
        // TODO: 处理 Pong
        // 1. 计算 RTT
        // 2. 更新状态为 Active
        // 3. 重置重试计数
        // 4. 记录 RTT 样本
        todo!("处理 Pong")
    }

    /// 记录收到了任何数据（可以替代心跳）
    pub fn activity(&mut self) {
        // TODO: 更新活动时间
        // 收到任何数据都表示连接存活
        todo!("记录活动")
    }

    /// 检查是否超时
    ///
    /// 如果发送了 Ping 但超过 timeout 未收到 Pong
    pub fn check_timeout(&mut self) -> bool {
        // TODO: 检查超时
        // 如果在 WaitingForPong 状态且超过 timeout
        // 增加重试计数，如果超过 max_retries，标记为 TimedOut
        todo!("检查超时")
    }

    /// 获取平均 RTT
    pub fn average_rtt(&self) -> Option<Duration> {
        // TODO: 计算平均 RTT
        todo!("计算平均 RTT")
    }

    /// 获取连接空闲时间
    pub fn idle_time(&self) -> Duration {
        self.last_activity.elapsed()
    }

    /// 重置跟踪器
    pub fn reset(&mut self) {
        // TODO: 重置所有状态
        todo!("重置跟踪器")
    }
}

/// 心跳消息类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeartbeatMessage {
    /// Ping 请求，包含时间戳
    Ping { timestamp: u64 },
    /// Pong 响应，包含原始时间戳
    Pong { timestamp: u64 },
}

impl HeartbeatMessage {
    /// 创建 Ping 消息
    pub fn ping() -> Self {
        // TODO: 创建带当前时间戳的 Ping
        todo!("创建 Ping")
    }

    /// 创建对应的 Pong 响应
    pub fn pong_for(ping: &HeartbeatMessage) -> Option<Self> {
        // TODO: 从 Ping 创建 Pong
        todo!("创建 Pong")
    }

    /// 序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化心跳消息
        // 格式: type(1) + timestamp(8)
        todo!("序列化心跳消息")
    }

    /// 从字节解析
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        // TODO: 解析心跳消息
        todo!("解析心跳消息")
    }

    /// 计算 RTT（仅对 Pong 有效）
    pub fn calculate_rtt(&self) -> Option<Duration> {
        // TODO: 计算 RTT
        // 当前时间 - timestamp
        todo!("计算 RTT")
    }
}

/// TCP Keepalive 配置
///
/// 用于配置系统级 TCP keepalive
#[derive(Debug, Clone)]
pub struct TcpKeepaliveConfig {
    /// 空闲多久后开始发送 keepalive
    pub idle: Duration,
    /// keepalive 探测间隔
    pub interval: Duration,
    /// 最大探测次数
    pub count: u32,
}

impl TcpKeepaliveConfig {
    /// 应用配置到 socket
    ///
    /// 注意: 这需要使用平台特定的 socket 选项
    #[cfg(target_os = "linux")]
    pub fn apply_to_socket(&self, _socket: &std::net::TcpStream) -> std::io::Result<()> {
        // TODO: 在 Linux 上设置 TCP keepalive
        // 使用 setsockopt: SO_KEEPALIVE, TCP_KEEPIDLE, TCP_KEEPINTVL, TCP_KEEPCNT
        todo!("设置 TCP keepalive")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_heartbeat_config() {
        let config = HeartbeatConfig::default();
        assert!(config.interval > Duration::ZERO);
        assert!(config.timeout < config.interval);
    }

    #[test]
    fn test_reliability_heartbeat_tracker_new() {
        let config = HeartbeatConfig::fast();
        let tracker = HeartbeatTracker::new(config);
        assert_eq!(tracker.state(), HeartbeatState::Active);
    }

    #[test]
    fn test_reliability_heartbeat_should_send() {
        let config = HeartbeatConfig::new(
            Duration::from_millis(100),
            Duration::from_millis(50),
            3,
        );
        let tracker = HeartbeatTracker::new(config);

        // 刚创建，不需要发送
        assert!(!tracker.should_send_ping());

        // 等待后应该需要发送
        std::thread::sleep(Duration::from_millis(150));
        assert!(tracker.should_send_ping());
    }

    #[test]
    fn test_reliability_heartbeat_ping_pong() {
        let config = HeartbeatConfig::fast();
        let mut tracker = HeartbeatTracker::new(config);

        tracker.ping_sent();
        assert_eq!(tracker.state(), HeartbeatState::WaitingForPong);

        std::thread::sleep(Duration::from_millis(10));

        let rtt = tracker.pong_received();
        assert!(rtt.is_some());
        assert!(rtt.unwrap() >= Duration::from_millis(10));
        assert_eq!(tracker.state(), HeartbeatState::Active);
    }

    #[test]
    fn test_reliability_heartbeat_timeout() {
        let config = HeartbeatConfig::new(
            Duration::from_millis(100),
            Duration::from_millis(10),
            2,
        );
        let mut tracker = HeartbeatTracker::new(config);

        tracker.ping_sent();
        std::thread::sleep(Duration::from_millis(20));

        // 第一次超时
        assert!(tracker.check_timeout());
        assert_eq!(tracker.state(), HeartbeatState::WaitingForPong);

        // 模拟重发
        tracker.ping_sent();
        std::thread::sleep(Duration::from_millis(20));

        // 第二次超时
        assert!(tracker.check_timeout());
        assert_eq!(tracker.state(), HeartbeatState::WaitingForPong);

        // 模拟再次重发
        tracker.ping_sent();
        std::thread::sleep(Duration::from_millis(20));

        // 第三次超时，超过 max_retries
        assert!(tracker.check_timeout());
        assert_eq!(tracker.state(), HeartbeatState::TimedOut);
    }

    #[test]
    fn test_reliability_heartbeat_message() {
        let ping = HeartbeatMessage::ping();
        if let HeartbeatMessage::Ping { timestamp } = ping {
            assert!(timestamp > 0);
        } else {
            panic!("Expected Ping");
        }

        let pong = HeartbeatMessage::pong_for(&ping).unwrap();
        if let HeartbeatMessage::Pong { .. } = pong {
            // OK
        } else {
            panic!("Expected Pong");
        }
    }

    #[test]
    fn test_reliability_heartbeat_message_serialize() {
        let ping = HeartbeatMessage::ping();
        let bytes = ping.to_bytes();
        let parsed = HeartbeatMessage::from_bytes(&bytes).unwrap();
        assert_eq!(ping, parsed);
    }
}
