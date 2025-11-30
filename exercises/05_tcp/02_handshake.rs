//! # 练习 02: TCP 三次握手
//!
//! TCP 连接建立需要经过三次握手（Three-Way Handshake）。
//!
//! ## 握手过程
//!
//! ```text
//! Client                                  Server
//!   |                                        |
//!   |  -------- SYN (seq=x) ------------>   |
//!   |                                        |
//!   |  <-- SYN-ACK (seq=y, ack=x+1) ------  |
//!   |                                        |
//!   |  -------- ACK (ack=y+1) ----------->  |
//!   |                                        |
//!   |            连接建立完成                 |
//! ```
//!
//! ## 握手步骤
//!
//! 1. **SYN**: 客户端发送 SYN，携带初始序列号 (ISN)
//! 2. **SYN-ACK**: 服务器响应 SYN-ACK，携带自己的 ISN，并确认客户端的 SYN
//! 3. **ACK**: 客户端发送 ACK，确认服务器的 SYN
//!
//! ## 为什么是三次？
//!
//! - 确保双方都能发送和接收
//! - 同步序列号
//! - 防止历史连接的干扰
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test tcp_handshake
//! ```

use super::ex00_header::TcpHeader;
use super::ex01_flags::TcpFlags;

/// 握手状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    /// 初始状态
    Closed,
    /// 已发送 SYN，等待 SYN-ACK
    SynSent,
    /// 已接收 SYN，已发送 SYN-ACK，等待 ACK
    SynReceived,
    /// 连接已建立
    Established,
    /// 握手失败
    Failed,
}

/// 客户端握手状态机
#[derive(Debug)]
pub struct ClientHandshake {
    /// 当前状态
    pub state: HandshakeState,
    /// 客户端初始序列号
    pub client_isn: u32,
    /// 服务器初始序列号
    pub server_isn: Option<u32>,
    /// 本地端口
    pub local_port: u16,
    /// 远程端口
    pub remote_port: u16,
}

impl ClientHandshake {
    /// 创建新的客户端握手
    pub fn new(local_port: u16, remote_port: u16, isn: u32) -> Self {
        // TODO: 初始化客户端握手状态
        // 状态应该是 Closed
        todo!("创建客户端握手")
    }

    /// 生成 SYN 段
    ///
    /// 开始握手过程
    pub fn create_syn(&mut self) -> TcpHeader {
        // TODO: 创建 SYN 段
        //
        // 步骤:
        // 1. 检查当前状态是否为 Closed
        // 2. 创建 TCP 头部，设置 SYN 标志
        // 3. 序列号设置为 client_isn
        // 4. 更新状态为 SynSent
        todo!("创建 SYN 段")
    }

    /// 处理收到的 SYN-ACK 段
    ///
    /// # 参数
    /// * `header` - 收到的 TCP 头部
    ///
    /// # 返回值
    /// 如果 SYN-ACK 有效，返回 Some(ACK 头部)；否则返回 None
    pub fn handle_syn_ack(&mut self, header: &TcpHeader) -> Option<TcpHeader> {
        // TODO: 处理 SYN-ACK
        //
        // 验证:
        // 1. 当前状态是 SynSent
        // 2. 收到的是 SYN+ACK 标志
        // 3. 确认号 == client_isn + 1
        //
        // 如果验证通过:
        // 1. 保存服务器 ISN
        // 2. 创建 ACK 段
        // 3. 更新状态为 Established
        todo!("处理 SYN-ACK")
    }

    /// 检查握手是否完成
    pub fn is_established(&self) -> bool {
        // TODO: 检查状态是否为 Established
        todo!("检查是否已建立连接")
    }
}

/// 服务器端握手状态机
#[derive(Debug)]
pub struct ServerHandshake {
    /// 当前状态
    pub state: HandshakeState,
    /// 服务器初始序列号
    pub server_isn: u32,
    /// 客户端初始序列号
    pub client_isn: Option<u32>,
    /// 本地端口
    pub local_port: u16,
}

impl ServerHandshake {
    /// 创建新的服务器端握手
    pub fn new(local_port: u16, isn: u32) -> Self {
        // TODO: 初始化服务器握手状态
        todo!("创建服务器握手")
    }

    /// 处理收到的 SYN 段
    ///
    /// # 参数
    /// * `header` - 收到的 TCP 头部
    ///
    /// # 返回值
    /// 如果 SYN 有效，返回 Some(SYN-ACK 头部)；否则返回 None
    pub fn handle_syn(&mut self, header: &TcpHeader) -> Option<TcpHeader> {
        // TODO: 处理 SYN
        //
        // 验证:
        // 1. 当前状态是 Closed
        // 2. 收到的是 SYN 标志（不带 ACK）
        //
        // 如果验证通过:
        // 1. 保存客户端 ISN
        // 2. 创建 SYN-ACK 段
        // 3. 更新状态为 SynReceived
        todo!("处理 SYN")
    }

    /// 处理收到的 ACK 段
    ///
    /// # 参数
    /// * `header` - 收到的 TCP 头部
    ///
    /// # 返回值
    /// 如果 ACK 有效返回 true
    pub fn handle_ack(&mut self, header: &TcpHeader) -> bool {
        // TODO: 处理 ACK
        //
        // 验证:
        // 1. 当前状态是 SynReceived
        // 2. 收到的是 ACK 标志
        // 3. 确认号 == server_isn + 1
        //
        // 如果验证通过:
        // 1. 更新状态为 Established
        // 2. 返回 true
        todo!("处理 ACK")
    }
}

/// 模拟完整的三次握手过程
///
/// 用于测试和学习目的
pub fn simulate_handshake(
    client_isn: u32,
    server_isn: u32,
    client_port: u16,
    server_port: u16,
) -> Result<(ClientHandshake, ServerHandshake), &'static str> {
    // TODO: 模拟三次握手
    //
    // 步骤:
    // 1. 创建客户端和服务器握手状态机
    // 2. 客户端创建 SYN
    // 3. 服务器处理 SYN，返回 SYN-ACK
    // 4. 客户端处理 SYN-ACK，返回 ACK
    // 5. 服务器处理 ACK
    // 6. 验证双方都进入 Established 状态
    todo!("模拟三次握手")
}

/// 计算期望的确认号
///
/// 在三次握手中:
/// - 对于 SYN 的确认: ack = seq + 1
/// - 对于数据的确认: ack = seq + data_len
pub fn expected_ack_num(seq: u32, data_len: u32, is_syn_or_fin: bool) -> u32 {
    // TODO: 计算期望的确认号
    // SYN 和 FIN 各占用一个序列号
    todo!("计算确认号")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_handshake_client_syn() {
        let mut client = ClientHandshake::new(12345, 80, 1000);

        assert_eq!(client.state, HandshakeState::Closed);

        let syn = client.create_syn();

        assert_eq!(client.state, HandshakeState::SynSent);
        assert!(syn.flags.syn);
        assert!(!syn.flags.ack);
        assert_eq!(syn.seq_num, 1000);
    }

    #[test]
    fn test_tcp_handshake_server_syn_ack() {
        let mut server = ServerHandshake::new(80, 2000);

        // 模拟收到的 SYN
        let mut syn = TcpHeader::new(12345, 80, 1000);
        syn.flags.syn = true;

        let syn_ack = server.handle_syn(&syn).unwrap();

        assert_eq!(server.state, HandshakeState::SynReceived);
        assert!(syn_ack.flags.syn);
        assert!(syn_ack.flags.ack);
        assert_eq!(syn_ack.seq_num, 2000);
        assert_eq!(syn_ack.ack_num, 1001); // client_isn + 1
    }

    #[test]
    fn test_tcp_handshake_client_ack() {
        let mut client = ClientHandshake::new(12345, 80, 1000);
        let _ = client.create_syn();

        // 模拟收到的 SYN-ACK
        let mut syn_ack = TcpHeader::new(80, 12345, 2000);
        syn_ack.flags.syn = true;
        syn_ack.flags.ack = true;
        syn_ack.ack_num = 1001;

        let ack = client.handle_syn_ack(&syn_ack).unwrap();

        assert!(client.is_established());
        assert!(ack.flags.ack);
        assert!(!ack.flags.syn);
        assert_eq!(ack.ack_num, 2001); // server_isn + 1
    }

    #[test]
    fn test_tcp_handshake_complete() {
        let result = simulate_handshake(1000, 2000, 12345, 80);
        let (client, server) = result.unwrap();

        assert!(client.is_established());
        assert_eq!(server.state, HandshakeState::Established);
        assert_eq!(client.server_isn, Some(2000));
        assert_eq!(server.client_isn, Some(1000));
    }

    #[test]
    fn test_tcp_handshake_invalid_ack() {
        let mut client = ClientHandshake::new(12345, 80, 1000);
        let _ = client.create_syn();

        // 错误的确认号
        let mut syn_ack = TcpHeader::new(80, 12345, 2000);
        syn_ack.flags.syn = true;
        syn_ack.flags.ack = true;
        syn_ack.ack_num = 9999; // 错误的确认号

        let result = client.handle_syn_ack(&syn_ack);
        assert!(result.is_none());
        assert_eq!(client.state, HandshakeState::SynSent); // 状态不变
    }

    #[test]
    fn test_expected_ack_num() {
        // SYN 的确认
        assert_eq!(expected_ack_num(1000, 0, true), 1001);

        // 数据的确认
        assert_eq!(expected_ack_num(1000, 100, false), 1100);

        // FIN 的确认
        assert_eq!(expected_ack_num(1000, 0, true), 1001);
    }
}
