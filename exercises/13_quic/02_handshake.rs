//! # 练习 02: QUIC 握手
//!
//! QUIC 握手集成了 TLS 1.3，实现 0-RTT/1-RTT 连接建立。
//!
//! ## TCP+TLS vs QUIC 握手
//!
//! ```text
//! TCP + TLS 1.3:
//!
//! 客户端              服务端
//!    |                  |
//!    |---- SYN -------->|  RTT 1
//!    |<--- SYN-ACK -----|
//!    |---- ACK -------->|  RTT 2
//!    |---- ClientHello->|  RTT 3
//!    |<-- ServerHello --|
//!    |<-- Finished -----|
//!    |---- Finished --->|
//!    |---- Data ------->|  开始传输
//!
//! QUIC 1-RTT:
//!
//! 客户端              服务端
//!    |                  |
//!    |-- Initial ------>|  RTT 1
//!    |   (ClientHello)  |
//!    |<-- Initial ------|
//!    |   (ServerHello)  |
//!    |<-- Handshake ----|
//!    |   (Finished)     |
//!    |-- Handshake ---->|
//!    |   (Finished)     |
//!    |-- 1-RTT Data --->|  开始传输
//!
//! QUIC 0-RTT (重连):
//!
//! 客户端              服务端
//!    |                  |
//!    |-- Initial ------>|  立即
//!    |   (ClientHello)  |
//!    |-- 0-RTT Data --->|  携带早期数据!
//!    |<-- Initial ------|
//!    |   ...            |
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test quic_handshake
//! ```

use std::time::{Duration, Instant};

/// 握手状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    /// 初始状态
    Initial,
    /// 已发送 ClientHello
    ClientHelloSent,
    /// 已接收 ClientHello
    ClientHelloReceived,
    /// 已发送 ServerHello
    ServerHelloSent,
    /// 已接收 ServerHello
    ServerHelloReceived,
    /// 握手完成
    Complete,
    /// 握手失败
    Failed,
}

/// 加密级别
///
/// QUIC 在不同阶段使用不同的密钥
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionLevel {
    /// Initial（使用 QUIC 版本特定的密钥）
    Initial,
    /// 0-RTT（使用 PSK 派生的密钥）
    ZeroRtt,
    /// Handshake（使用握手派生的密钥）
    Handshake,
    /// 1-RTT（使用会话密钥）
    OneRtt,
}

impl EncryptionLevel {
    /// 获取下一个加密级别
    pub fn next(&self) -> Option<Self> {
        // TODO: 返回下一个加密级别
        //
        // Initial -> Handshake -> OneRtt
        // ZeroRtt -> OneRtt
        todo!("下一个加密级别")
    }

    /// 该级别是否可以携带应用数据
    pub fn can_send_data(&self) -> bool {
        // TODO: 判断是否可以发送应用数据
        // 只有 ZeroRtt 和 OneRtt 可以
        todo!("可否发送数据")
    }
}

/// 传输参数
///
/// 在握手期间交换的参数
#[derive(Debug, Clone)]
pub struct TransportParameters {
    /// 原始目标连接 ID
    pub original_destination_connection_id: Option<Vec<u8>>,
    /// 最大空闲超时
    pub max_idle_timeout: Duration,
    /// 最大 UDP 载荷大小
    pub max_udp_payload_size: u64,
    /// 初始最大数据量
    pub initial_max_data: u64,
    /// 初始最大流数据量（双向）
    pub initial_max_stream_data_bidi_local: u64,
    /// 初始最大流数据量（双向远程）
    pub initial_max_stream_data_bidi_remote: u64,
    /// 初始最大流数据量（单向）
    pub initial_max_stream_data_uni: u64,
    /// 初始最大双向流数量
    pub initial_max_streams_bidi: u64,
    /// 初始最大单向流数量
    pub initial_max_streams_uni: u64,
    /// ACK 延迟指数
    pub ack_delay_exponent: u64,
    /// 最大 ACK 延迟
    pub max_ack_delay: Duration,
    /// 禁用主动迁移
    pub disable_active_migration: bool,
    /// 首选地址
    pub preferred_address: Option<PreferredAddress>,
    /// 活动连接 ID 限制
    pub active_connection_id_limit: u64,
    /// 初始源连接 ID
    pub initial_source_connection_id: Option<Vec<u8>>,
    /// 重试源连接 ID
    pub retry_source_connection_id: Option<Vec<u8>>,
}

impl Default for TransportParameters {
    fn default() -> Self {
        Self {
            original_destination_connection_id: None,
            max_idle_timeout: Duration::from_secs(30),
            max_udp_payload_size: 65527,
            initial_max_data: 10 * 1024 * 1024,           // 10 MB
            initial_max_stream_data_bidi_local: 1024 * 1024, // 1 MB
            initial_max_stream_data_bidi_remote: 1024 * 1024,
            initial_max_stream_data_uni: 1024 * 1024,
            initial_max_streams_bidi: 100,
            initial_max_streams_uni: 100,
            ack_delay_exponent: 3,
            max_ack_delay: Duration::from_millis(25),
            disable_active_migration: false,
            preferred_address: None,
            active_connection_id_limit: 2,
            initial_source_connection_id: None,
            retry_source_connection_id: None,
        }
    }
}

impl TransportParameters {
    /// 编码传输参数
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码传输参数为 TLV 格式
        //
        // 格式:
        // +-+-+-+-+-+-+-+-+
        // | Param ID (i)  |
        // +-+-+-+-+-+-+-+-+
        // | Param Len (i) |
        // +-+-+-+-+-+-+-+-+
        // | Param Value   |
        // +-+-+-+-+-+-+-+-+
        // | ...           |
        todo!("编码传输参数")
    }

    /// 解码传输参数
    pub fn decode(data: &[u8]) -> Option<Self> {
        // TODO: 解码传输参数
        todo!("解码传输参数")
    }
}

/// 首选地址
#[derive(Debug, Clone)]
pub struct PreferredAddress {
    pub ipv4_address: Option<std::net::SocketAddrV4>,
    pub ipv6_address: Option<std::net::SocketAddrV6>,
    pub connection_id: Vec<u8>,
    pub stateless_reset_token: [u8; 16],
}

/// CRYPTO 帧
///
/// 用于传输 TLS 握手数据
#[derive(Debug, Clone)]
pub struct CryptoFrame {
    /// 偏移量
    pub offset: u64,
    /// 数据
    pub data: Vec<u8>,
}

impl CryptoFrame {
    /// 解析 CRYPTO 帧
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 CRYPTO 帧
        //
        // 格式:
        // +-+-+-+-+-+-+-+-+
        // | Type (0x06)   |
        // +-+-+-+-+-+-+-+-+
        // | Offset (i)    |
        // +-+-+-+-+-+-+-+-+
        // | Length (i)    |
        // +-+-+-+-+-+-+-+-+
        // | Crypto Data   |
        // +-+-+-+-+-+-+-+-+
        todo!("解析 CRYPTO 帧")
    }

    /// 序列化 CRYPTO 帧
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 CRYPTO 帧
        todo!("序列化 CRYPTO 帧")
    }
}

/// 握手器
pub struct Handshaker {
    /// 是否是客户端
    is_client: bool,
    /// 当前状态
    state: HandshakeState,
    /// 本地传输参数
    local_params: TransportParameters,
    /// 对端传输参数
    peer_params: Option<TransportParameters>,
    /// 握手开始时间
    start_time: Option<Instant>,
    /// 是否支持 0-RTT
    zero_rtt_enabled: bool,
    /// 是否有 0-RTT 密钥
    has_zero_rtt_keys: bool,
}

impl Handshaker {
    /// 创建客户端握手器
    pub fn client(params: TransportParameters) -> Self {
        // TODO: 创建客户端握手器
        todo!("创建客户端握手器")
    }

    /// 创建服务端握手器
    pub fn server(params: TransportParameters) -> Self {
        // TODO: 创建服务端握手器
        todo!("创建服务端握手器")
    }

    /// 开始握手
    pub fn start(&mut self) -> Vec<CryptoFrame> {
        // TODO: 开始握手
        //
        // 客户端: 生成 ClientHello
        // 服务端: 等待 ClientHello
        todo!("开始握手")
    }

    /// 处理收到的 CRYPTO 数据
    pub fn process(&mut self, level: EncryptionLevel, data: &[u8]) -> HandshakeResult {
        // TODO: 处理握手数据
        //
        // 根据当前状态和收到的消息推进状态机
        todo!("处理握手数据")
    }

    /// 是否握手完成
    pub fn is_complete(&self) -> bool {
        self.state == HandshakeState::Complete
    }

    /// 是否可以发送 0-RTT 数据
    pub fn can_send_zero_rtt(&self) -> bool {
        // TODO: 判断是否可以发送 0-RTT
        todo!("检查 0-RTT")
    }

    /// 获取当前加密级别
    pub fn current_level(&self) -> EncryptionLevel {
        // TODO: 根据状态返回当前加密级别
        todo!("当前加密级别")
    }

    /// 获取握手延迟
    pub fn handshake_rtt(&self) -> Option<Duration> {
        // TODO: 计算握手 RTT
        todo!("握手延迟")
    }

    /// 获取对端传输参数
    pub fn peer_params(&self) -> Option<&TransportParameters> {
        self.peer_params.as_ref()
    }
}

/// 握手结果
#[derive(Debug)]
pub enum HandshakeResult {
    /// 需要发送数据
    SendData {
        level: EncryptionLevel,
        frames: Vec<CryptoFrame>,
    },
    /// 状态变更
    StateChanged(HandshakeState),
    /// 握手完成
    Complete,
    /// 错误
    Error(HandshakeError),
}

/// 握手错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeError {
    /// 协议错误
    ProtocolViolation,
    /// 参数无效
    InvalidParameter,
    /// TLS 错误
    TlsError(String),
    /// 超时
    Timeout,
}

/// 0-RTT 相关
pub struct ZeroRtt;

impl ZeroRtt {
    /// 解释 0-RTT 的安全风险
    pub fn security_risks() -> Vec<&'static str> {
        // TODO: 列出 0-RTT 的安全风险
        // 提示: 重放攻击
        todo!("0-RTT 风险")
    }

    /// 哪些请求适合 0-RTT？
    pub fn safe_requests() -> Vec<&'static str> {
        // TODO: 列出适合 0-RTT 的请求类型
        // 提示: 幂等请求
        todo!("安全的 0-RTT 请求")
    }

    /// 哪些请求不适合 0-RTT？
    pub fn unsafe_requests() -> Vec<&'static str> {
        // TODO: 列出不适合 0-RTT 的请求类型
        // 提示: 非幂等请求
        todo!("不安全的 0-RTT 请求")
    }
}

/// 地址验证
///
/// QUIC 使用地址验证防止放大攻击
pub struct AddressValidation;

impl AddressValidation {
    /// 什么是放大攻击？
    pub fn amplification_attack_explanation() -> &'static str {
        // TODO: 解释放大攻击
        todo!("放大攻击解释")
    }

    /// QUIC 如何防止放大攻击？
    pub fn mitigation() -> Vec<&'static str> {
        // TODO: 列出防护措施
        // 提示: 3倍限制, Retry 包
        todo!("防护措施")
    }

    /// 什么是 Retry 包？
    pub fn retry_packet_purpose() -> &'static str {
        // TODO: 解释 Retry 包的用途
        todo!("Retry 包用途")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_handshake_encryption_level() {
        assert_eq!(EncryptionLevel::Initial.next(), Some(EncryptionLevel::Handshake));
        assert_eq!(EncryptionLevel::Handshake.next(), Some(EncryptionLevel::OneRtt));
        assert_eq!(EncryptionLevel::OneRtt.next(), None);

        assert!(!EncryptionLevel::Initial.can_send_data());
        assert!(EncryptionLevel::ZeroRtt.can_send_data());
        assert!(EncryptionLevel::OneRtt.can_send_data());
    }

    #[test]
    fn test_quic_handshake_transport_params() {
        let params = TransportParameters::default();

        assert_eq!(params.max_idle_timeout, Duration::from_secs(30));
        assert_eq!(params.initial_max_streams_bidi, 100);

        // 编码和解码
        let encoded = params.encode();
        let decoded = TransportParameters::decode(&encoded).unwrap();

        assert_eq!(decoded.max_idle_timeout, params.max_idle_timeout);
    }

    #[test]
    fn test_quic_handshake_client() {
        let params = TransportParameters::default();
        let mut handshaker = Handshaker::client(params);

        assert!(!handshaker.is_complete());
        assert_eq!(handshaker.current_level(), EncryptionLevel::Initial);

        let frames = handshaker.start();
        assert!(!frames.is_empty()); // 应该有 ClientHello
    }

    #[test]
    fn test_quic_handshake_zero_rtt() {
        let risks = ZeroRtt::security_risks();
        assert!(!risks.is_empty());

        let safe = ZeroRtt::safe_requests();
        let unsafe_reqs = ZeroRtt::unsafe_requests();

        // GET 应该是安全的，POST/DELETE 不安全
        assert!(safe.iter().any(|r| r.contains("GET") || r.contains("idempotent")));
        assert!(unsafe_reqs.iter().any(|r| r.contains("POST") || r.contains("non-idempotent")));
    }

    #[test]
    fn test_quic_handshake_address_validation() {
        let mitigation = AddressValidation::mitigation();
        assert!(!mitigation.is_empty());
    }
}
