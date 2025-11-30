//! # 练习 03: TCP 状态机
//!
//! TCP 连接有 11 种状态，状态之间的转换由发送和接收的段决定。
//!
//! ## TCP 状态
//!
//! ```text
//!                              +---------+ ---------\      active OPEN
//!                              |  CLOSED |            \    -----------
//!                              +---------+<---------\   \   create TCB
//!                                |     ^              \   \  snd SYN
//!                   passive OPEN |     |   CLOSE        \   \
//!                   ------------ |     | ----------       \   \
//!                    create TCB  |     | delete TCB         \   \
//!                                V     |                      \   \
//!                              +---------+            CLOSE    |    \
//!                              |  LISTEN |          ---------- |     |
//!                              +---------+          delete TCB |     |
//!                   rcv SYN      |     |     SEND              |     |
//!                  -----------   |     |    -------            |     V
//! +---------+      snd SYN,ACK  /       \   snd SYN          +---------+
//! |         |<-----------------           ------------------>|         |
//! |   SYN   |                    rcv SYN                     |   SYN   |
//! |   RCVD  |<-----------------------------------------------|   SENT  |
//! |         |                    snd ACK                     |         |
//! |         |------------------           -------------------|         |
//! +---------+   rcv ACK of SYN  \       /  rcv SYN,ACK       +---------+
//!      |           ----------    |     |   -----------
//!      |               x         |     |     snd ACK
//!      |                         V     V
//!      |  CLOSE                +---------+
//!      | -------               |  ESTAB  |
//!      | snd FIN               +---------+
//!      |                 CLOSE    |     |    rcv FIN
//!      V                -------   |     |    -------
//! +---------+          snd FIN   /       \   snd ACK         +---------+
//! |  FIN    |<------------------           ----------------->|  CLOSE  |
//! | WAIT-1  |------------------                              |   WAIT  |
//! +---------+          rcv FIN  \                            +---------+
//!      | rcv ACK of FIN -------   |                            CLOSE  |
//!      | --------------   x       V                           ------- |
//!      V        snd ACK +---------+                           snd FIN V
//! +---------+           |CLOSING  |                          +---------+
//! |FIN      |           +---------+                          | LAST-ACK|
//! |WAIT-2   |               | rcv ACK of FIN                 +---------+
//! +---------+               | --------------                       |
//!      |                    V       x                              |
//!      |              +---------+                                  |
//!      |              |TIME WAIT|                                  |
//!      |              +---------+                                  |
//!      |                   | Timeout=2MSL                          |
//!      |                   | --------------                        |
//!      V                   V        x                              V
//!                        +---------+
//!                        | CLOSED  |
//!                        +---------+
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test tcp_state
//! ```

use super::ex01_flags::TcpFlags;

/// TCP 连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    /// 关闭状态，没有连接
    Closed,
    /// 监听状态，等待连接请求
    Listen,
    /// 已发送 SYN，等待匹配的连接请求
    SynSent,
    /// 已收到并发送 SYN，等待 ACK
    SynReceived,
    /// 连接已建立，可以传输数据
    Established,
    /// 已发送 FIN，等待 ACK 或 FIN
    FinWait1,
    /// 已收到对方的 FIN ACK，等待对方的 FIN
    FinWait2,
    /// 双方同时关闭，等待 ACK
    Closing,
    /// 等待足够的时间确保远端收到 ACK
    TimeWait,
    /// 等待远端的连接终止请求
    CloseWait,
    /// 已发送 FIN，等待最终 ACK
    LastAck,
}

impl TcpState {
    /// 检查是否可以发送数据
    pub fn can_send(&self) -> bool {
        // TODO: 返回在此状态下是否可以发送数据
        // 只有 Established 和 CloseWait 状态可以发送
        todo!("检查是否可以发送")
    }

    /// 检查是否可以接收数据
    pub fn can_receive(&self) -> bool {
        // TODO: 返回在此状态下是否可以接收数据
        // Established, FinWait1, FinWait2 可以接收
        todo!("检查是否可以接收")
    }

    /// 检查连接是否已关闭
    pub fn is_closed(&self) -> bool {
        // TODO: 检查是否为 Closed 状态
        todo!("检查是否已关闭")
    }

    /// 检查连接是否已建立
    pub fn is_established(&self) -> bool {
        // TODO: 检查是否为 Established 状态
        todo!("检查是否已建立")
    }

    /// 返回状态名称
    pub fn name(&self) -> &'static str {
        // TODO: 返回状态的字符串名称
        todo!("获取状态名称")
    }
}

/// TCP 状态机事件
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpEvent {
    /// 应用程序执行主动打开
    ActiveOpen,
    /// 应用程序执行被动打开（监听）
    PassiveOpen,
    /// 收到 SYN 段
    RecvSyn,
    /// 收到 SYN+ACK 段
    RecvSynAck,
    /// 收到 ACK 段
    RecvAck,
    /// 收到 FIN 段
    RecvFin,
    /// 收到 RST 段
    RecvRst,
    /// 应用程序关闭连接
    Close,
    /// 超时事件
    Timeout,
}

/// TCP 状态机
#[derive(Debug)]
pub struct TcpStateMachine {
    /// 当前状态
    state: TcpState,
}

impl TcpStateMachine {
    /// 创建新的状态机（初始状态为 Closed）
    pub fn new() -> Self {
        // TODO: 创建初始状态为 Closed 的状态机
        todo!("创建状态机")
    }

    /// 获取当前状态
    pub fn state(&self) -> TcpState {
        self.state
    }

    /// 处理事件，返回新状态和需要发送的标志
    ///
    /// # 参数
    /// * `event` - 事件
    ///
    /// # 返回值
    /// 需要发送的 TCP 标志（如果有的话）
    pub fn handle_event(&mut self, event: TcpEvent) -> Option<TcpFlags> {
        // TODO: 实现状态转换
        //
        // 主要转换:
        // - Closed + ActiveOpen -> SynSent (发送 SYN)
        // - Closed + PassiveOpen -> Listen
        // - Listen + RecvSyn -> SynReceived (发送 SYN+ACK)
        // - SynSent + RecvSynAck -> Established (发送 ACK)
        // - SynReceived + RecvAck -> Established
        // - Established + RecvFin -> CloseWait (发送 ACK)
        // - Established + Close -> FinWait1 (发送 FIN)
        // - FinWait1 + RecvAck -> FinWait2
        // - FinWait1 + RecvFin -> Closing (发送 ACK)
        // - FinWait2 + RecvFin -> TimeWait (发送 ACK)
        // - Closing + RecvAck -> TimeWait
        // - CloseWait + Close -> LastAck (发送 FIN)
        // - LastAck + RecvAck -> Closed
        // - TimeWait + Timeout -> Closed
        // - 任何状态 + RecvRst -> Closed
        todo!("处理事件")
    }

    /// 重置状态机
    pub fn reset(&mut self) {
        // TODO: 将状态重置为 Closed
        todo!("重置状态机")
    }
}

impl Default for TcpStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// 模拟完整的连接建立和关闭过程
///
/// # 返回值
/// 状态转换序列
pub fn simulate_connection_lifecycle() -> Vec<(TcpEvent, TcpState)> {
    // TODO: 模拟连接生命周期
    //
    // 1. 主动打开 (Closed -> SynSent)
    // 2. 收到 SYN-ACK (SynSent -> Established)
    // 3. ... 数据传输 ...
    // 4. 关闭 (Established -> FinWait1)
    // 5. 收到 ACK (FinWait1 -> FinWait2)
    // 6. 收到 FIN (FinWait2 -> TimeWait)
    // 7. 超时 (TimeWait -> Closed)
    todo!("模拟连接生命周期")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_state_can_send() {
        assert!(TcpState::Established.can_send());
        assert!(TcpState::CloseWait.can_send());
        assert!(!TcpState::Closed.can_send());
        assert!(!TcpState::Listen.can_send());
        assert!(!TcpState::FinWait1.can_send());
    }

    #[test]
    fn test_tcp_state_can_receive() {
        assert!(TcpState::Established.can_receive());
        assert!(TcpState::FinWait1.can_receive());
        assert!(TcpState::FinWait2.can_receive());
        assert!(!TcpState::Closed.can_receive());
        assert!(!TcpState::CloseWait.can_receive());
    }

    #[test]
    fn test_tcp_state_machine_initial() {
        let sm = TcpStateMachine::new();
        assert_eq!(sm.state(), TcpState::Closed);
    }

    #[test]
    fn test_tcp_state_machine_active_open() {
        let mut sm = TcpStateMachine::new();

        let flags = sm.handle_event(TcpEvent::ActiveOpen);

        assert_eq!(sm.state(), TcpState::SynSent);
        assert!(flags.is_some());
        assert!(flags.unwrap().syn);
    }

    #[test]
    fn test_tcp_state_machine_passive_open() {
        let mut sm = TcpStateMachine::new();

        let flags = sm.handle_event(TcpEvent::PassiveOpen);

        assert_eq!(sm.state(), TcpState::Listen);
        assert!(flags.is_none());
    }

    #[test]
    fn test_tcp_state_machine_handshake() {
        let mut sm = TcpStateMachine::new();

        // 主动打开
        sm.handle_event(TcpEvent::ActiveOpen);
        assert_eq!(sm.state(), TcpState::SynSent);

        // 收到 SYN-ACK
        let flags = sm.handle_event(TcpEvent::RecvSynAck);
        assert_eq!(sm.state(), TcpState::Established);
        assert!(flags.unwrap().ack);
    }

    #[test]
    fn test_tcp_state_machine_close() {
        let mut sm = TcpStateMachine::new();

        // 建立连接
        sm.handle_event(TcpEvent::ActiveOpen);
        sm.handle_event(TcpEvent::RecvSynAck);
        assert_eq!(sm.state(), TcpState::Established);

        // 关闭连接
        let flags = sm.handle_event(TcpEvent::Close);
        assert_eq!(sm.state(), TcpState::FinWait1);
        assert!(flags.unwrap().fin);

        // 收到 ACK
        sm.handle_event(TcpEvent::RecvAck);
        assert_eq!(sm.state(), TcpState::FinWait2);

        // 收到 FIN
        let flags = sm.handle_event(TcpEvent::RecvFin);
        assert_eq!(sm.state(), TcpState::TimeWait);
        assert!(flags.unwrap().ack);

        // 超时
        sm.handle_event(TcpEvent::Timeout);
        assert_eq!(sm.state(), TcpState::Closed);
    }

    #[test]
    fn test_tcp_state_machine_rst() {
        let mut sm = TcpStateMachine::new();

        // 建立连接
        sm.handle_event(TcpEvent::ActiveOpen);
        sm.handle_event(TcpEvent::RecvSynAck);

        // 收到 RST 应该立即关闭
        sm.handle_event(TcpEvent::RecvRst);
        assert_eq!(sm.state(), TcpState::Closed);
    }

    #[test]
    fn test_tcp_state_machine_lifecycle() {
        let transitions = simulate_connection_lifecycle();

        assert!(!transitions.is_empty());

        // 应该从 Closed 开始
        // 最终回到 Closed
        let last_state = transitions.last().unwrap().1;
        assert_eq!(last_state, TcpState::Closed);
    }
}
