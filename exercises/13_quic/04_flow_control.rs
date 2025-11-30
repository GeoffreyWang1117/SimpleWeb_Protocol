//! # 练习 04: QUIC 流量控制
//!
//! QUIC 提供两级流量控制：连接级和流级。
//!
//! ## 为什么需要两级流量控制？
//!
//! - **连接级**: 限制整个连接的数据量，防止内存耗尽
//! - **流级**: 限制单个流的数据量，实现公平性
//!
//! ## 流量控制帧
//!
//! | 帧类型 | 用途 |
//! |--------|------|
//! | MAX_DATA | 连接级流量控制 |
//! | MAX_STREAM_DATA | 流级流量控制 |
//! | DATA_BLOCKED | 通知对方被阻塞 |
//! | STREAM_DATA_BLOCKED | 通知对方流被阻塞 |
//!
//! ## 工作流程
//!
//! ```text
//! 发送方                    接收方
//!   |                         |
//!   |---- STREAM (数据) ----->|
//!   |                         | (处理数据，缓冲区有空间)
//!   |<--- MAX_STREAM_DATA ----|  (增加允许的偏移量)
//!   |---- STREAM (更多) ----->|
//!   |                         |
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test quic_flow_control
//! ```

/// 流量控制限制
#[derive(Debug, Clone, Copy)]
pub struct FlowControlLimit {
    /// 当前已发送/接收的字节数
    pub offset: u64,
    /// 允许的最大偏移量
    pub max_offset: u64,
}

impl FlowControlLimit {
    pub fn new(initial_max: u64) -> Self {
        Self {
            offset: 0,
            max_offset: initial_max,
        }
    }

    /// 可以发送/接收的字节数
    pub fn available(&self) -> u64 {
        // TODO: 计算剩余可用空间
        todo!("计算可用空间")
    }

    /// 是否被阻塞
    pub fn is_blocked(&self) -> bool {
        self.offset >= self.max_offset
    }

    /// 消耗配额
    pub fn consume(&mut self, bytes: u64) -> Result<(), FlowControlError> {
        // TODO: 消耗流量控制配额
        // 如果超出限制，返回错误
        todo!("消耗配额")
    }

    /// 更新最大偏移量
    pub fn update_max(&mut self, new_max: u64) {
        // TODO: 更新最大偏移量
        // 只能增加，不能减少
        todo!("更新限制")
    }

    /// 计算需要的新配额（用于发送 BLOCKED 帧）
    pub fn needed(&self, want_to_send: u64) -> u64 {
        // TODO: 计算需要多少额外配额
        todo!("计算所需配额")
    }
}

/// 流量控制错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowControlError {
    /// 超出流量控制限制
    LimitExceeded,
    /// 超出流级限制
    StreamLimitExceeded,
    /// 超出连接级限制
    ConnectionLimitExceeded,
}

/// MAX_DATA 帧
#[derive(Debug, Clone, Copy)]
pub struct MaxDataFrame {
    /// 新的连接级最大数据量
    pub max_data: u64,
}

impl MaxDataFrame {
    pub fn new(max_data: u64) -> Self {
        Self { max_data }
    }

    /// 编码帧
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码 MAX_DATA 帧
        // 帧类型 (0x10) + max_data (varint)
        todo!("编码 MAX_DATA")
    }

    /// 解码帧
    pub fn decode(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解码 MAX_DATA 帧
        todo!("解码 MAX_DATA")
    }
}

/// MAX_STREAM_DATA 帧
#[derive(Debug, Clone, Copy)]
pub struct MaxStreamDataFrame {
    /// 流 ID
    pub stream_id: u64,
    /// 新的流级最大数据量
    pub max_stream_data: u64,
}

impl MaxStreamDataFrame {
    pub fn new(stream_id: u64, max_stream_data: u64) -> Self {
        Self {
            stream_id,
            max_stream_data,
        }
    }

    /// 编码帧
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码 MAX_STREAM_DATA 帧
        // 帧类型 (0x11) + stream_id (varint) + max_stream_data (varint)
        todo!("编码 MAX_STREAM_DATA")
    }

    /// 解码帧
    pub fn decode(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解码 MAX_STREAM_DATA 帧
        todo!("解码 MAX_STREAM_DATA")
    }
}

/// DATA_BLOCKED 帧
#[derive(Debug, Clone, Copy)]
pub struct DataBlockedFrame {
    /// 被阻塞时的数据限制
    pub limit: u64,
}

impl DataBlockedFrame {
    pub fn new(limit: u64) -> Self {
        Self { limit }
    }

    /// 编码帧
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码 DATA_BLOCKED 帧
        // 帧类型 (0x14) + limit (varint)
        todo!("编码 DATA_BLOCKED")
    }
}

/// STREAM_DATA_BLOCKED 帧
#[derive(Debug, Clone, Copy)]
pub struct StreamDataBlockedFrame {
    /// 流 ID
    pub stream_id: u64,
    /// 被阻塞时的流数据限制
    pub limit: u64,
}

impl StreamDataBlockedFrame {
    pub fn new(stream_id: u64, limit: u64) -> Self {
        Self { stream_id, limit }
    }

    /// 编码帧
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码 STREAM_DATA_BLOCKED 帧
        // 帧类型 (0x15) + stream_id (varint) + limit (varint)
        todo!("编码 STREAM_DATA_BLOCKED")
    }
}

/// 发送端流量控制器
pub struct SendFlowController {
    /// 连接级限制
    connection_limit: FlowControlLimit,
    /// 各流的限制
    stream_limits: std::collections::HashMap<u64, FlowControlLimit>,
    /// 默认流级初始限制
    default_stream_limit: u64,
}

impl SendFlowController {
    pub fn new(connection_max: u64, stream_max: u64) -> Self {
        Self {
            connection_limit: FlowControlLimit::new(connection_max),
            stream_limits: std::collections::HashMap::new(),
            default_stream_limit: stream_max,
        }
    }

    /// 检查是否可以发送指定字节数
    pub fn can_send(&self, stream_id: u64, bytes: u64) -> bool {
        // TODO: 检查连接级和流级限制
        todo!("检查是否可发送")
    }

    /// 获取可发送的最大字节数
    pub fn available_for_stream(&self, stream_id: u64) -> u64 {
        // TODO: 返回 min(连接级可用, 流级可用)
        todo!("获取可发送量")
    }

    /// 记录发送的数据
    pub fn on_send(&mut self, stream_id: u64, bytes: u64) -> Result<(), FlowControlError> {
        // TODO: 更新连接级和流级偏移量
        todo!("记录发送")
    }

    /// 处理 MAX_DATA 帧
    pub fn on_max_data(&mut self, frame: &MaxDataFrame) {
        // TODO: 更新连接级限制
        todo!("处理 MAX_DATA")
    }

    /// 处理 MAX_STREAM_DATA 帧
    pub fn on_max_stream_data(&mut self, frame: &MaxStreamDataFrame) {
        // TODO: 更新流级限制
        todo!("处理 MAX_STREAM_DATA")
    }

    /// 是否被连接级阻塞
    pub fn is_connection_blocked(&self) -> bool {
        self.connection_limit.is_blocked()
    }

    /// 是否被流级阻塞
    pub fn is_stream_blocked(&self, stream_id: u64) -> bool {
        // TODO: 检查流是否被阻塞
        todo!("检查流阻塞")
    }

    /// 生成 DATA_BLOCKED 帧（如果需要）
    pub fn generate_blocked_frame(&self) -> Option<DataBlockedFrame> {
        // TODO: 如果被连接级阻塞，生成帧
        todo!("生成 DATA_BLOCKED")
    }
}

/// 接收端流量控制器
pub struct RecvFlowController {
    /// 连接级限制
    connection_limit: FlowControlLimit,
    /// 连接级已消费（读取）的字节数
    connection_consumed: u64,
    /// 各流的限制
    stream_limits: std::collections::HashMap<u64, StreamRecvLimit>,
    /// 默认流级初始限制
    default_stream_limit: u64,
    /// 自动更新阈值（当已消费超过此比例时发送更新）
    auto_update_threshold: f64,
}

/// 流接收限制
#[derive(Debug, Clone)]
pub struct StreamRecvLimit {
    /// 流量控制限制
    pub limit: FlowControlLimit,
    /// 已消费（应用层读取）的字节数
    pub consumed: u64,
}

impl RecvFlowController {
    pub fn new(connection_max: u64, stream_max: u64) -> Self {
        Self {
            connection_limit: FlowControlLimit::new(connection_max),
            connection_consumed: 0,
            stream_limits: std::collections::HashMap::new(),
            default_stream_limit: stream_max,
            auto_update_threshold: 0.5,
        }
    }

    /// 检查是否允许接收数据
    pub fn check_receive(&self, stream_id: u64, offset: u64, len: u64) -> Result<(), FlowControlError> {
        // TODO: 检查是否超出连接级或流级限制
        todo!("检查接收")
    }

    /// 记录接收到的数据
    pub fn on_receive(&mut self, stream_id: u64, offset: u64, len: u64) -> Result<(), FlowControlError> {
        // TODO: 更新接收偏移量
        todo!("记录接收")
    }

    /// 记录应用层消费的数据
    pub fn on_consume(&mut self, stream_id: u64, bytes: u64) {
        // TODO: 更新消费量，可能触发流量控制更新
        todo!("记录消费")
    }

    /// 是否应该发送 MAX_DATA 更新
    pub fn should_send_max_data(&self) -> bool {
        // TODO: 检查是否应该发送更新
        // 通常在已消费超过限制的一定比例时发送
        todo!("检查是否需要更新")
    }

    /// 是否应该发送 MAX_STREAM_DATA 更新
    pub fn should_send_max_stream_data(&self, stream_id: u64) -> bool {
        // TODO: 检查流是否需要更新
        todo!("检查流是否需要更新")
    }

    /// 生成 MAX_DATA 帧
    pub fn generate_max_data(&mut self) -> Option<MaxDataFrame> {
        // TODO: 生成新的 MAX_DATA 帧
        // 新限制 = 当前限制 + 已消费量
        todo!("生成 MAX_DATA")
    }

    /// 生成 MAX_STREAM_DATA 帧
    pub fn generate_max_stream_data(&mut self, stream_id: u64) -> Option<MaxStreamDataFrame> {
        // TODO: 生成新的 MAX_STREAM_DATA 帧
        todo!("生成 MAX_STREAM_DATA")
    }
}

/// 流量控制配置
#[derive(Debug, Clone)]
pub struct FlowControlConfig {
    /// 初始连接级限制
    pub initial_max_data: u64,
    /// 初始流级限制（双向本地）
    pub initial_max_stream_data_bidi_local: u64,
    /// 初始流级限制（双向远程）
    pub initial_max_stream_data_bidi_remote: u64,
    /// 初始流级限制（单向）
    pub initial_max_stream_data_uni: u64,
    /// 自动更新阈值
    pub auto_update_threshold: f64,
}

impl Default for FlowControlConfig {
    fn default() -> Self {
        Self {
            initial_max_data: 10 * 1024 * 1024,          // 10 MB
            initial_max_stream_data_bidi_local: 1024 * 1024, // 1 MB
            initial_max_stream_data_bidi_remote: 1024 * 1024,
            initial_max_stream_data_uni: 1024 * 1024,
            auto_update_threshold: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_flow_control_limit() {
        let mut limit = FlowControlLimit::new(1000);
        assert_eq!(limit.available(), 1000);
        assert!(!limit.is_blocked());

        limit.consume(500).unwrap();
        assert_eq!(limit.available(), 500);

        limit.consume(500).unwrap();
        assert!(limit.is_blocked());

        // 超出限制应该失败
        assert!(limit.consume(1).is_err());
    }

    #[test]
    fn test_quic_flow_control_update() {
        let mut limit = FlowControlLimit::new(1000);
        limit.consume(1000).unwrap();
        assert!(limit.is_blocked());

        limit.update_max(2000);
        assert!(!limit.is_blocked());
        assert_eq!(limit.available(), 1000);
    }

    #[test]
    fn test_quic_flow_control_sender() {
        let mut controller = SendFlowController::new(10000, 1000);

        // 可以发送
        assert!(controller.can_send(0, 500));
        controller.on_send(0, 500).unwrap();

        // 检查可用量
        assert_eq!(controller.available_for_stream(0), 500); // 受流级限制

        // 流级阻塞
        controller.on_send(0, 500).unwrap();
        assert!(controller.is_stream_blocked(0));
        assert!(!controller.is_connection_blocked());

        // 更新流级限制
        controller.on_max_stream_data(&MaxStreamDataFrame::new(0, 2000));
        assert!(!controller.is_stream_blocked(0));
    }

    #[test]
    fn test_quic_flow_control_receiver() {
        let mut controller = RecvFlowController::new(10000, 1000);

        // 接收数据
        controller.on_receive(0, 0, 500).unwrap();

        // 消费数据
        controller.on_consume(0, 500);

        // 应该可以发送更新
        assert!(controller.should_send_max_stream_data(0));

        let frame = controller.generate_max_stream_data(0).unwrap();
        assert!(frame.max_stream_data > 1000);
    }

    #[test]
    fn test_quic_flow_control_max_data_encode() {
        let frame = MaxDataFrame::new(12345);
        let encoded = frame.encode();

        let (decoded, _) = MaxDataFrame::decode(&encoded).unwrap();
        assert_eq!(decoded.max_data, 12345);
    }

    #[test]
    fn test_quic_flow_control_connection_limit() {
        let mut controller = SendFlowController::new(1000, 10000);

        // 多个流一起消耗连接级配额
        controller.on_send(0, 400).unwrap();
        controller.on_send(4, 400).unwrap();
        controller.on_send(8, 200).unwrap();

        // 连接级被阻塞
        assert!(controller.is_connection_blocked());
        assert!(controller.generate_blocked_frame().is_some());
    }
}
