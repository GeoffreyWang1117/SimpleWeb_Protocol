//! # 练习 01: QUIC 流复用
//!
//! QUIC 原生支持多路复用，解决了 TCP 的队头阻塞问题。
//!
//! ## TCP vs QUIC 流
//!
//! ```text
//! TCP (HTTP/2):
//! Stream 1: [1][2][3][ ][ ][6]  <- 丢包阻塞整个连接
//! Stream 2: [等待...]
//! Stream 3: [等待...]
//!
//! QUIC:
//! Stream 1: [1][2][3][ ][ ][6]  <- 只影响 Stream 1
//! Stream 2: [1][2][3][4][5][6]  <- 正常传输
//! Stream 3: [1][2][3][4][5][6]  <- 正常传输
//! ```
//!
//! ## 流 ID
//!
//! QUIC 流 ID 编码了发起方和类型:
//!
//! | 低2位 | 发起方 | 类型 |
//! |-------|--------|------|
//! | 0x0 | 客户端 | 双向 |
//! | 0x1 | 服务端 | 双向 |
//! | 0x2 | 客户端 | 单向 |
//! | 0x3 | 服务端 | 单向 |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test quic_streams
//! ```

use std::collections::{BTreeMap, VecDeque};

/// 流发起方
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamInitiator {
    /// 客户端发起
    Client,
    /// 服务端发起
    Server,
}

/// 流类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    /// 双向流
    Bidirectional,
    /// 单向流
    Unidirectional,
}

/// 流 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct StreamId(pub u64);

impl StreamId {
    /// 创建流 ID
    pub fn new(initiator: StreamInitiator, stream_type: StreamType, index: u64) -> Self {
        // TODO: 根据发起方、类型和索引创建流 ID
        //
        // 流 ID = (index << 2) | type_bits
        //
        // type_bits:
        // - 0b00: 客户端双向
        // - 0b01: 服务端双向
        // - 0b10: 客户端单向
        // - 0b11: 服务端单向
        todo!("创建流 ID")
    }

    /// 获取发起方
    pub fn initiator(&self) -> StreamInitiator {
        // TODO: 从流 ID 解析发起方
        // bit 0: 0=客户端, 1=服务端
        todo!("获取发起方")
    }

    /// 获取流类型
    pub fn stream_type(&self) -> StreamType {
        // TODO: 从流 ID 解析类型
        // bit 1: 0=双向, 1=单向
        todo!("获取流类型")
    }

    /// 获取流索引
    pub fn index(&self) -> u64 {
        // TODO: 获取流索引
        // index = stream_id >> 2
        todo!("获取索引")
    }

    /// 是否是客户端发起的
    pub fn is_client_initiated(&self) -> bool {
        self.initiator() == StreamInitiator::Client
    }

    /// 是否是双向流
    pub fn is_bidirectional(&self) -> bool {
        self.stream_type() == StreamType::Bidirectional
    }
}

/// 流状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    /// 准备发送（初始状态）
    Ready,
    /// 发送中
    Send,
    /// 已发送 FIN
    DataSent,
    /// 接收中
    Recv,
    /// 已接收所有数据
    DataRecvd,
    /// 已关闭
    Closed,
    /// 重置
    Reset,
}

/// 流帧（STREAM frame）
///
/// ```text
/// STREAM Frame:
/// +-+-+-+-+-+-+-+-+
/// |0 0 0 0 1|O|L|F|  类型 (0x08-0x0f)
/// +-+-+-+-+-+-+-+-+
/// |   Stream ID   |  变长整数
/// +-+-+-+-+-+-+-+-+
/// |  [Offset]     |  变长整数 (如果 O=1)
/// +-+-+-+-+-+-+-+-+
/// |  [Length]     |  变长整数 (如果 L=1)
/// +-+-+-+-+-+-+-+-+
/// |  Stream Data  |
/// +-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone)]
pub struct StreamFrame {
    /// 流 ID
    pub stream_id: StreamId,
    /// 偏移量
    pub offset: u64,
    /// 数据
    pub data: Vec<u8>,
    /// 是否是最后一帧
    pub fin: bool,
}

impl StreamFrame {
    /// 创建流帧
    pub fn new(stream_id: StreamId, offset: u64, data: Vec<u8>, fin: bool) -> Self {
        Self {
            stream_id,
            offset,
            data,
            fin,
        }
    }

    /// 解析流帧
    pub fn parse(frame_type: u8, data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 STREAM 帧
        //
        // frame_type 低 3 位:
        // - bit 0 (F): FIN 位
        // - bit 1 (L): 有 Length 字段
        // - bit 2 (O): 有 Offset 字段
        todo!("解析流帧")
    }

    /// 序列化流帧
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化流帧
        todo!("序列化流帧")
    }

    /// 数据结束位置
    pub fn end_offset(&self) -> u64 {
        self.offset + self.data.len() as u64
    }
}

/// 发送流
pub struct SendStream {
    /// 流 ID
    id: StreamId,
    /// 状态
    state: StreamState,
    /// 已确认的偏移量
    acked_offset: u64,
    /// 下一个发送偏移量
    next_offset: u64,
    /// 待发送数据
    pending: VecDeque<u8>,
    /// 是否已发送 FIN
    fin_sent: bool,
    /// 流量控制：最大可发送偏移量
    max_data: u64,
}

impl SendStream {
    pub fn new(id: StreamId, max_data: u64) -> Self {
        Self {
            id,
            state: StreamState::Ready,
            acked_offset: 0,
            next_offset: 0,
            pending: VecDeque::new(),
            fin_sent: false,
            max_data,
        }
    }

    /// 写入数据
    pub fn write(&mut self, data: &[u8]) -> Result<usize, StreamError> {
        // TODO: 将数据加入待发送队列
        // 考虑流量控制限制
        todo!("写入数据")
    }

    /// 获取下一个要发送的帧
    pub fn get_frame(&mut self, max_size: usize) -> Option<StreamFrame> {
        // TODO: 生成下一个 STREAM 帧
        todo!("获取帧")
    }

    /// 确认数据
    pub fn ack(&mut self, offset: u64, len: usize) {
        // TODO: 处理 ACK
        todo!("确认数据")
    }

    /// 关闭发送端（发送 FIN）
    pub fn finish(&mut self) {
        // TODO: 标记流结束
        todo!("完成发送")
    }

    /// 更新流量控制限制
    pub fn update_max_data(&mut self, max_data: u64) {
        if max_data > self.max_data {
            self.max_data = max_data;
        }
    }

    /// 是否可以发送更多数据
    pub fn can_send(&self) -> bool {
        // TODO: 检查是否有数据可发送且未超出流量控制
        todo!("检查可发送")
    }
}

/// 接收流
pub struct RecvStream {
    /// 流 ID
    id: StreamId,
    /// 状态
    state: StreamState,
    /// 已读取的偏移量
    read_offset: u64,
    /// 接收到的数据（按偏移量排序）
    received: BTreeMap<u64, Vec<u8>>,
    /// 最终偏移量（如果收到 FIN）
    final_offset: Option<u64>,
    /// 流量控制：允许的最大偏移量
    max_data: u64,
}

impl RecvStream {
    pub fn new(id: StreamId, max_data: u64) -> Self {
        Self {
            id,
            state: StreamState::Recv,
            read_offset: 0,
            received: BTreeMap::new(),
            final_offset: None,
            max_data,
        }
    }

    /// 处理接收到的流帧
    pub fn receive(&mut self, frame: StreamFrame) -> Result<(), StreamError> {
        // TODO: 处理接收到的数据
        //
        // 1. 检查偏移量是否超出流量控制
        // 2. 存储数据
        // 3. 如果有 FIN，记录最终偏移量
        todo!("接收数据")
    }

    /// 读取可用数据
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, StreamError> {
        // TODO: 读取连续的数据
        // 只有当数据从 read_offset 开始连续时才可读
        todo!("读取数据")
    }

    /// 是否有数据可读
    pub fn is_readable(&self) -> bool {
        // TODO: 检查是否有从 read_offset 开始的连续数据
        todo!("检查可读")
    }

    /// 是否已接收完毕
    pub fn is_finished(&self) -> bool {
        // TODO: 检查是否已接收所有数据（包括 FIN）
        todo!("检查完成")
    }
}

/// 流错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamError {
    /// 流已关闭
    Closed,
    /// 超出流量控制
    FlowControl,
    /// 无效状态
    InvalidState,
    /// 数据超出最终偏移量
    FinalSize,
    /// 没有数据可读
    WouldBlock,
}

/// 流管理器
pub struct StreamManager {
    /// 本端是客户端还是服务端
    is_client: bool,
    /// 双向流
    bidi_streams: BTreeMap<StreamId, (SendStream, RecvStream)>,
    /// 发送单向流
    send_streams: BTreeMap<StreamId, SendStream>,
    /// 接收单向流
    recv_streams: BTreeMap<StreamId, RecvStream>,
    /// 下一个本地双向流索引
    next_bidi_index: u64,
    /// 下一个本地单向流索引
    next_uni_index: u64,
    /// 最大双向流数量
    max_bidi_streams: u64,
    /// 最大单向流数量
    max_uni_streams: u64,
}

impl StreamManager {
    pub fn new(is_client: bool) -> Self {
        Self {
            is_client,
            bidi_streams: BTreeMap::new(),
            send_streams: BTreeMap::new(),
            recv_streams: BTreeMap::new(),
            next_bidi_index: 0,
            next_uni_index: 0,
            max_bidi_streams: 100,
            max_uni_streams: 100,
        }
    }

    /// 打开双向流
    pub fn open_bidi(&mut self) -> Result<StreamId, StreamError> {
        // TODO: 打开新的双向流
        todo!("打开双向流")
    }

    /// 打开单向流
    pub fn open_uni(&mut self) -> Result<StreamId, StreamError> {
        // TODO: 打开新的单向流
        todo!("打开单向流")
    }

    /// 处理对端打开的流
    pub fn accept_stream(&mut self, stream_id: StreamId) -> Result<(), StreamError> {
        // TODO: 接受对端打开的流
        todo!("接受流")
    }

    /// 获取发送流
    pub fn get_send(&mut self, stream_id: StreamId) -> Option<&mut SendStream> {
        // TODO: 获取发送流引用
        todo!("获取发送流")
    }

    /// 获取接收流
    pub fn get_recv(&mut self, stream_id: StreamId) -> Option<&mut RecvStream> {
        // TODO: 获取接收流引用
        todo!("获取接收流")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_streams_id() {
        // 客户端双向流 0
        let id = StreamId::new(StreamInitiator::Client, StreamType::Bidirectional, 0);
        assert_eq!(id.0, 0);
        assert_eq!(id.initiator(), StreamInitiator::Client);
        assert_eq!(id.stream_type(), StreamType::Bidirectional);
        assert_eq!(id.index(), 0);

        // 服务端单向流 1
        let id = StreamId::new(StreamInitiator::Server, StreamType::Unidirectional, 1);
        assert_eq!(id.0, 7); // (1 << 2) | 0b11
        assert_eq!(id.initiator(), StreamInitiator::Server);
        assert_eq!(id.stream_type(), StreamType::Unidirectional);
        assert_eq!(id.index(), 1);
    }

    #[test]
    fn test_quic_streams_frame() {
        let frame = StreamFrame::new(StreamId(4), 0, b"Hello".to_vec(), false);
        assert_eq!(frame.stream_id.index(), 1);
        assert_eq!(frame.end_offset(), 5);

        let bytes = frame.to_bytes();
        let (parsed, _) = StreamFrame::parse(bytes[0], &bytes[1..]).unwrap();
        assert_eq!(parsed.stream_id, frame.stream_id);
        assert_eq!(parsed.data, frame.data);
    }

    #[test]
    fn test_quic_streams_send() {
        let id = StreamId::new(StreamInitiator::Client, StreamType::Bidirectional, 0);
        let mut stream = SendStream::new(id, 1000);

        // 写入数据
        let written = stream.write(b"Hello, QUIC!").unwrap();
        assert_eq!(written, 12);

        // 获取帧
        let frame = stream.get_frame(100).unwrap();
        assert_eq!(frame.data, b"Hello, QUIC!");
        assert_eq!(frame.offset, 0);
    }

    #[test]
    fn test_quic_streams_recv() {
        let id = StreamId::new(StreamInitiator::Server, StreamType::Bidirectional, 0);
        let mut stream = RecvStream::new(id, 1000);

        // 接收乱序数据
        stream
            .receive(StreamFrame::new(id, 5, b"World".to_vec(), false))
            .unwrap();
        assert!(!stream.is_readable()); // 缺少 offset 0-5

        stream
            .receive(StreamFrame::new(id, 0, b"Hello".to_vec(), false))
            .unwrap();
        assert!(stream.is_readable()); // 现在连续了

        let mut buf = [0u8; 100];
        let n = stream.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"HelloWorld");
    }

    #[test]
    fn test_quic_streams_manager() {
        let mut manager = StreamManager::new(true); // 客户端

        // 打开双向流
        let id1 = manager.open_bidi().unwrap();
        assert_eq!(id1.0, 0); // 客户端双向流从 0 开始

        let id2 = manager.open_bidi().unwrap();
        assert_eq!(id2.0, 4); // 下一个是 4

        // 打开单向流
        let id3 = manager.open_uni().unwrap();
        assert_eq!(id3.0, 2); // 客户端单向流从 2 开始
    }
}
