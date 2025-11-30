//! # 练习 03: 连接迁移
//!
//! QUIC 支持连接迁移，允许连接在网络变化时保持活跃。
//!
//! ## TCP vs QUIC 连接标识
//!
//! ```text
//! TCP 连接: (源IP, 源端口, 目标IP, 目标端口)
//!
//! 问题: IP 变化 = 连接断开
//!
//! QUIC 连接: Connection ID
//!
//! 优势: IP 变化 ≠ 连接断开
//! ```
//!
//! ## 连接迁移场景
//!
//! 1. **WiFi → 蜂窝**: 从 WiFi 切换到 4G/5G
//! 2. **蜂窝 → WiFi**: 反向切换
//! 3. **基站切换**: 移动过程中的切换
//! 4. **NAT 重绑定**: NAT 分配新端口
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test quic_migration
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

/// 连接 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub Vec<u8>);

impl ConnectionId {
    /// 创建随机连接 ID
    pub fn random(len: usize) -> Self {
        // TODO: 生成随机连接 ID
        todo!("生成随机 ID")
    }

    /// 长度
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 连接 ID 管理器
///
/// 每端可以提供多个连接 ID 给对端使用
pub struct ConnectionIdManager {
    /// 本地连接 ID 列表
    local_cids: Vec<LocalConnectionId>,
    /// 对端提供的连接 ID 列表
    peer_cids: Vec<PeerConnectionId>,
    /// 已退休的序列号
    retired_before: u64,
    /// 最大活跃连接 ID 数量
    active_cid_limit: u64,
}

/// 本地连接 ID
#[derive(Debug, Clone)]
pub struct LocalConnectionId {
    /// 连接 ID
    pub cid: ConnectionId,
    /// 序列号
    pub sequence: u64,
    /// 状态重置令牌
    pub stateless_reset_token: [u8; 16],
    /// 是否已发送给对端
    pub sent: bool,
    /// 是否已退休
    pub retired: bool,
}

/// 对端连接 ID
#[derive(Debug, Clone)]
pub struct PeerConnectionId {
    /// 连接 ID
    pub cid: ConnectionId,
    /// 序列号
    pub sequence: u64,
    /// 状态重置令牌
    pub stateless_reset_token: Option<[u8; 16]>,
    /// 是否正在使用
    pub in_use: bool,
    /// 关联的路径
    pub path_id: Option<u32>,
}

impl ConnectionIdManager {
    pub fn new(active_cid_limit: u64) -> Self {
        Self {
            local_cids: Vec::new(),
            peer_cids: Vec::new(),
            retired_before: 0,
            active_cid_limit,
        }
    }

    /// 生成新的本地连接 ID
    pub fn generate_local_cid(&mut self) -> Option<LocalConnectionId> {
        // TODO: 生成新的本地连接 ID
        //
        // 1. 检查是否超过限制
        // 2. 分配序列号
        // 3. 生成随机 CID 和重置令牌
        todo!("生成本地 CID")
    }

    /// 添加对端的连接 ID
    pub fn add_peer_cid(&mut self, cid: ConnectionId, sequence: u64, token: Option<[u8; 16]>) -> Result<(), MigrationError> {
        // TODO: 添加对端 CID
        //
        // 验证:
        // - 序列号递增
        // - 未超过限制
        todo!("添加对端 CID")
    }

    /// 获取可用的对端连接 ID
    pub fn get_available_peer_cid(&mut self) -> Option<&PeerConnectionId> {
        // TODO: 获取一个未使用的对端 CID
        todo!("获取可用 CID")
    }

    /// 退休连接 ID
    pub fn retire_cid(&mut self, sequence: u64) -> Result<(), MigrationError> {
        // TODO: 退休指定序列号的连接 ID
        todo!("退休 CID")
    }

    /// 处理 NEW_CONNECTION_ID 帧
    pub fn handle_new_connection_id(&mut self, frame: &NewConnectionIdFrame) -> Result<(), MigrationError> {
        // TODO: 处理新连接 ID
        todo!("处理新 CID")
    }

    /// 处理 RETIRE_CONNECTION_ID 帧
    pub fn handle_retire_connection_id(&mut self, sequence: u64) -> Result<(), MigrationError> {
        // TODO: 处理退休请求
        todo!("处理退休请求")
    }
}

/// NEW_CONNECTION_ID 帧
#[derive(Debug, Clone)]
pub struct NewConnectionIdFrame {
    /// 序列号
    pub sequence: u64,
    /// 退休此序列号之前的所有 CID
    pub retire_prior_to: u64,
    /// 连接 ID
    pub connection_id: ConnectionId,
    /// 状态重置令牌
    pub stateless_reset_token: [u8; 16],
}

impl NewConnectionIdFrame {
    /// 解析帧
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 NEW_CONNECTION_ID 帧
        //
        // 格式:
        // +-+-+-+-+-+-+-+-+
        // | Type (0x18)   |
        // +-+-+-+-+-+-+-+-+
        // | Sequence (i)  |
        // +-+-+-+-+-+-+-+-+
        // | Retire (i)    |
        // +-+-+-+-+-+-+-+-+
        // | Length (8)    |
        // +-+-+-+-+-+-+-+-+
        // | Connection ID |
        // +-+-+-+-+-+-+-+-+
        // | Reset Token   |  16 bytes
        // +-+-+-+-+-+-+-+-+
        todo!("解析帧")
    }

    /// 序列化帧
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化帧
        todo!("序列化帧")
    }
}

/// 路径状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathState {
    /// 等待验证
    Pending,
    /// 验证中
    Validating,
    /// 已验证
    Validated,
    /// 活跃
    Active,
    /// 已关闭
    Closed,
}

/// 网络路径
#[derive(Debug, Clone)]
pub struct Path {
    /// 路径 ID
    pub id: u32,
    /// 本地地址
    pub local_addr: SocketAddr,
    /// 远程地址
    pub remote_addr: SocketAddr,
    /// 使用的连接 ID
    pub connection_id: ConnectionId,
    /// 路径状态
    pub state: PathState,
    /// 路径 RTT
    pub rtt: Option<Duration>,
    /// 路径挑战数据
    pub challenge_data: Option<[u8; 8]>,
    /// 上次活动时间
    pub last_activity: Instant,
}

impl Path {
    /// 创建新路径
    pub fn new(id: u32, local: SocketAddr, remote: SocketAddr, cid: ConnectionId) -> Self {
        Self {
            id,
            local_addr: local,
            remote_addr: remote,
            connection_id: cid,
            state: PathState::Pending,
            rtt: None,
            challenge_data: None,
            last_activity: Instant::now(),
        }
    }

    /// 开始路径验证
    pub fn start_validation(&mut self) -> PathChallengeFrame {
        // TODO: 生成路径挑战
        //
        // 1. 生成 8 字节随机数据
        // 2. 保存用于验证响应
        // 3. 返回 PATH_CHALLENGE 帧
        todo!("开始验证")
    }

    /// 验证路径响应
    pub fn validate_response(&mut self, data: &[u8; 8]) -> bool {
        // TODO: 验证 PATH_RESPONSE
        //
        // 检查数据是否匹配
        todo!("验证响应")
    }

    /// 标记为活跃
    pub fn activate(&mut self) {
        self.state = PathState::Active;
        self.last_activity = Instant::now();
    }
}

/// PATH_CHALLENGE 帧
#[derive(Debug, Clone)]
pub struct PathChallengeFrame {
    pub data: [u8; 8],
}

/// PATH_RESPONSE 帧
#[derive(Debug, Clone)]
pub struct PathResponseFrame {
    pub data: [u8; 8],
}

/// 连接迁移器
pub struct ConnectionMigrator {
    /// 所有路径
    paths: HashMap<u32, Path>,
    /// 活跃路径 ID
    active_path: Option<u32>,
    /// 连接 ID 管理器
    cid_manager: ConnectionIdManager,
    /// 下一个路径 ID
    next_path_id: u32,
    /// 是否禁用主动迁移
    migration_disabled: bool,
}

impl ConnectionMigrator {
    pub fn new(cid_manager: ConnectionIdManager, migration_disabled: bool) -> Self {
        Self {
            paths: HashMap::new(),
            active_path: None,
            cid_manager,
            next_path_id: 0,
            migration_disabled,
        }
    }

    /// 添加初始路径
    pub fn add_initial_path(&mut self, local: SocketAddr, remote: SocketAddr, cid: ConnectionId) -> u32 {
        // TODO: 添加初始路径
        // 初始路径不需要验证
        todo!("添加初始路径")
    }

    /// 开始迁移到新地址
    pub fn initiate_migration(&mut self, new_local: SocketAddr) -> Result<MigrationAction, MigrationError> {
        // TODO: 发起迁移
        //
        // 1. 检查是否允许迁移
        // 2. 获取新的连接 ID
        // 3. 创建新路径
        // 4. 开始路径验证
        todo!("发起迁移")
    }

    /// 处理来自新地址的数据包
    pub fn handle_packet_from_new_addr(&mut self, new_remote: SocketAddr) -> Result<MigrationAction, MigrationError> {
        // TODO: 处理对端迁移
        //
        // 1. 检查是否是已知路径
        // 2. 如果是新路径，需要验证
        todo!("处理新地址")
    }

    /// 处理 PATH_CHALLENGE
    pub fn handle_path_challenge(&mut self, challenge: PathChallengeFrame, from: SocketAddr) -> PathResponseFrame {
        // TODO: 响应路径挑战
        PathResponseFrame {
            data: challenge.data,
        }
    }

    /// 处理 PATH_RESPONSE
    pub fn handle_path_response(&mut self, response: PathResponseFrame) -> Result<(), MigrationError> {
        // TODO: 处理路径响应
        //
        // 找到匹配的路径并验证
        todo!("处理路径响应")
    }

    /// 获取活跃路径
    pub fn active_path(&self) -> Option<&Path> {
        self.active_path.and_then(|id| self.paths.get(&id))
    }

    /// 切换到指定路径
    pub fn switch_to_path(&mut self, path_id: u32) -> Result<(), MigrationError> {
        // TODO: 切换活跃路径
        todo!("切换路径")
    }
}

/// 迁移动作
#[derive(Debug)]
pub enum MigrationAction {
    /// 发送路径挑战
    SendChallenge(PathChallengeFrame, u32),
    /// 等待响应
    WaitResponse,
    /// 迁移完成
    Complete,
}

/// 迁移错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationError {
    /// 迁移被禁用
    Disabled,
    /// 没有可用的连接 ID
    NoAvailableCid,
    /// 路径验证失败
    ValidationFailed,
    /// 无效序列号
    InvalidSequence,
    /// 超过限制
    LimitExceeded,
    /// 路径不存在
    PathNotFound,
}

/// 状态重置
///
/// 当无法正常关闭连接时，可以发送状态重置
pub struct StatelessReset;

impl StatelessReset {
    /// 生成状态重置令牌
    pub fn generate_token(static_key: &[u8], connection_id: &ConnectionId) -> [u8; 16] {
        // TODO: 生成状态重置令牌
        //
        // 令牌 = HMAC(static_key, connection_id)[..16]
        todo!("生成令牌")
    }

    /// 验证状态重置
    pub fn verify(packet: &[u8], expected_token: &[u8; 16]) -> bool {
        // TODO: 验证状态重置包
        //
        // 最后 16 字节应该匹配令牌
        todo!("验证重置")
    }

    /// 生成状态重置包
    pub fn create_reset_packet(token: [u8; 16]) -> Vec<u8> {
        // TODO: 创建状态重置包
        //
        // 格式:
        // - 随机字节（至少 21 字节）
        // - 状态重置令牌（16 字节）
        //
        // 看起来像随机数据，但对端可以识别
        todo!("创建重置包")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn make_addr(port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
    }

    #[test]
    fn test_quic_migration_connection_id() {
        let cid = ConnectionId::random(8);
        assert_eq!(cid.len(), 8);
    }

    #[test]
    fn test_quic_migration_cid_manager() {
        let mut manager = ConnectionIdManager::new(4);

        // 生成本地 CID
        let local_cid = manager.generate_local_cid().unwrap();
        assert_eq!(local_cid.sequence, 0);

        // 添加对端 CID
        manager
            .add_peer_cid(ConnectionId::random(8), 0, Some([0u8; 16]))
            .unwrap();

        let peer_cid = manager.get_available_peer_cid().unwrap();
        assert_eq!(peer_cid.sequence, 0);
    }

    #[test]
    fn test_quic_migration_path() {
        let local = make_addr(12345);
        let remote = make_addr(443);
        let cid = ConnectionId::random(8);

        let mut path = Path::new(0, local, remote, cid);
        assert_eq!(path.state, PathState::Pending);

        let challenge = path.start_validation();
        assert_eq!(path.state, PathState::Validating);

        let valid = path.validate_response(&challenge.data);
        assert!(valid);
        assert_eq!(path.state, PathState::Validated);
    }

    #[test]
    fn test_quic_migration_migrator() {
        let cid_manager = ConnectionIdManager::new(4);
        let mut migrator = ConnectionMigrator::new(cid_manager, false);

        let local = make_addr(12345);
        let remote = make_addr(443);
        let cid = ConnectionId::random(8);

        // 添加初始路径
        let path_id = migrator.add_initial_path(local, remote, cid);
        assert_eq!(path_id, 0);

        let path = migrator.active_path().unwrap();
        assert_eq!(path.state, PathState::Active);
    }

    #[test]
    fn test_quic_migration_stateless_reset() {
        let static_key = b"secret_key_12345";
        let cid = ConnectionId::random(8);

        let token = StatelessReset::generate_token(static_key, &cid);
        let packet = StatelessReset::create_reset_packet(token);

        assert!(packet.len() >= 21 + 16);
        assert!(StatelessReset::verify(&packet, &token));
    }
}
