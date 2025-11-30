//! # 练习 00: TCP 头部
//!
//! TCP 头部比 UDP 复杂得多，包含了连接管理和可靠传输所需的信息。
//!
//! ## TCP 头部结构 (RFC 793)
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |          Source Port          |       Destination Port        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                        Sequence Number                        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                    Acknowledgment Number                      |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |  Data |       |C|E|U|A|P|R|S|F|                               |
//! | Offset| Rsrvd |W|C|R|C|S|S|Y|I|            Window             |
//! |       |       |R|E|G|K|H|T|N|N|                               |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |           Checksum            |         Urgent Pointer        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                    Options                    |    Padding    |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                             data                              |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//! ## 字段说明
//!
//! - Source/Dest Port (16 bits each): 端口号
//! - Sequence Number (32 bits): 序列号
//! - Acknowledgment Number (32 bits): 确认号
//! - Data Offset (4 bits): 头部长度，以 4 字节为单位
//! - Reserved (4 bits): 保留位
//! - Flags (8 bits): 控制标志
//! - Window (16 bits): 接收窗口大小
//! - Checksum (16 bits): 校验和
//! - Urgent Pointer (16 bits): 紧急指针
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test tcp_header
//! ```

use super::ex01_flags::TcpFlags;

/// TCP 头部结构
#[derive(Debug, Clone)]
pub struct TcpHeader {
    /// 源端口
    pub src_port: u16,
    /// 目标端口
    pub dst_port: u16,
    /// 序列号
    pub seq_num: u32,
    /// 确认号
    pub ack_num: u32,
    /// 数据偏移（头部长度，以 4 字节为单位）
    pub data_offset: u8,
    /// 标志位
    pub flags: TcpFlags,
    /// 窗口大小
    pub window: u16,
    /// 校验和
    pub checksum: u16,
    /// 紧急指针
    pub urgent_ptr: u16,
    /// 选项
    pub options: Vec<u8>,
}

impl TcpHeader {
    /// TCP 头部最小长度
    pub const MIN_HEADER_LEN: usize = 20;

    /// 创建一个基本的 TCP 头部（无选项）
    pub fn new(src_port: u16, dst_port: u16, seq_num: u32) -> Self {
        // TODO: 创建基本 TCP 头部
        //
        // 默认值:
        // - ack_num: 0
        // - data_offset: 5 (20 字节，无选项)
        // - flags: 空
        // - window: 65535 (最大值)
        // - checksum: 0
        // - urgent_ptr: 0
        // - options: 空
        todo!("创建 TCP 头部")
    }

    /// 从字节切片解析 TCP 头部
    ///
    /// # 返回值
    /// 解析成功返回 Some((TcpHeader, header_len))，失败返回 None
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 TCP 头部
        //
        // 步骤:
        // 1. 检查数据至少 20 字节
        // 2. 提取各字段
        // 3. 从 data_offset 计算头部长度
        // 4. 提取选项
        //
        // 字节偏移:
        // 0-1: src_port
        // 2-3: dst_port
        // 4-7: seq_num
        // 8-11: ack_num
        // 12: data_offset(4) + reserved(4)
        // 13: flags
        // 14-15: window
        // 16-17: checksum
        // 18-19: urgent_ptr
        // 20+: options
        todo!("解析 TCP 头部")
    }

    /// 将 TCP 头部序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 TCP 头部
        todo!("序列化 TCP 头部")
    }

    /// 计算头部长度（字节）
    pub fn header_len(&self) -> usize {
        // TODO: data_offset * 4
        todo!("计算头部长度")
    }

    /// 计算并设置校验和（需要 IP 伪头部）
    pub fn compute_checksum(&mut self, src_ip: [u8; 4], dst_ip: [u8; 4], data: &[u8]) {
        // TODO: 计算 TCP 校验和
        //
        // 与 UDP 类似，需要包含伪头部
        // 伪头部: 源IP + 目标IP + 0 + 协议(6) + TCP长度
        todo!("计算 TCP 校验和")
    }
}

/// TCP 选项类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpOption {
    /// 结束选项列表
    EndOfList,
    /// 无操作（用于填充）
    Nop,
    /// 最大段大小
    MaxSegmentSize(u16),
    /// 窗口缩放因子
    WindowScale(u8),
    /// 选择确认允许
    SackPermitted,
    /// 时间戳
    Timestamps { ts_val: u32, ts_ecr: u32 },
    /// 未知选项
    Unknown { kind: u8, data: Vec<u8> },
}

impl TcpOption {
    /// 解析单个 TCP 选项
    ///
    /// # 返回值
    /// Some((TcpOption, 消耗的字节数))
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 TCP 选项
        //
        // 选项格式:
        // - Kind 0 (EOL): 单字节
        // - Kind 1 (NOP): 单字节
        // - 其他: Kind(1) + Length(1) + Data(Length-2)
        //
        // 常见选项:
        // - Kind 2: MSS (长度 4)
        // - Kind 3: Window Scale (长度 3)
        // - Kind 4: SACK Permitted (长度 2)
        // - Kind 8: Timestamps (长度 10)
        todo!("解析 TCP 选项")
    }

    /// 将选项序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 TCP 选项
        todo!("序列化 TCP 选项")
    }
}

/// 解析所有 TCP 选项
pub fn parse_tcp_options(data: &[u8]) -> Vec<TcpOption> {
    // TODO: 解析选项列表
    //
    // 循环解析直到:
    // - 遇到 EOL
    // - 数据用尽
    // - 解析失败
    todo!("解析 TCP 选项列表")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_header_new() {
        let header = TcpHeader::new(12345, 80, 1000);

        assert_eq!(header.src_port, 12345);
        assert_eq!(header.dst_port, 80);
        assert_eq!(header.seq_num, 1000);
        assert_eq!(header.data_offset, 5);
        assert_eq!(header.header_len(), 20);
    }

    #[test]
    fn test_tcp_header_parse() {
        // 一个简单的 TCP 头部
        let data = [
            0x30, 0x39, // src_port = 12345
            0x00, 0x50, // dst_port = 80
            0x00, 0x00, 0x03, 0xe8, // seq_num = 1000
            0x00, 0x00, 0x00, 0x00, // ack_num = 0
            0x50, // data_offset=5, reserved=0
            0x02, // flags = SYN
            0xff, 0xff, // window = 65535
            0x00, 0x00, // checksum
            0x00, 0x00, // urgent_ptr
        ];

        let (header, len) = TcpHeader::parse(&data).unwrap();

        assert_eq!(len, 20);
        assert_eq!(header.src_port, 12345);
        assert_eq!(header.dst_port, 80);
        assert_eq!(header.seq_num, 1000);
        assert!(header.flags.syn);
        assert!(!header.flags.ack);
    }

    #[test]
    fn test_tcp_header_to_bytes() {
        let mut header = TcpHeader::new(1234, 5678, 100);
        header.flags.syn = true;

        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), 20);

        // 验证端口
        assert_eq!((bytes[0] as u16) << 8 | bytes[1] as u16, 1234);
        assert_eq!((bytes[2] as u16) << 8 | bytes[3] as u16, 5678);
    }

    #[test]
    fn test_tcp_option_mss() {
        let data = [0x02, 0x04, 0x05, 0xb4]; // MSS = 1460

        let (opt, len) = TcpOption::parse(&data).unwrap();
        assert_eq!(len, 4);

        if let TcpOption::MaxSegmentSize(mss) = opt {
            assert_eq!(mss, 1460);
        } else {
            panic!("Expected MaxSegmentSize");
        }
    }

    #[test]
    fn test_tcp_option_window_scale() {
        let data = [0x03, 0x03, 0x07]; // Window Scale = 7

        let (opt, len) = TcpOption::parse(&data).unwrap();
        assert_eq!(len, 3);

        if let TcpOption::WindowScale(scale) = opt {
            assert_eq!(scale, 7);
        } else {
            panic!("Expected WindowScale");
        }
    }

    #[test]
    fn test_tcp_option_nop() {
        let data = [0x01];

        let (opt, len) = TcpOption::parse(&data).unwrap();
        assert_eq!(len, 1);
        assert_eq!(opt, TcpOption::Nop);
    }

    #[test]
    fn test_parse_tcp_options() {
        // NOP + MSS(1460) + NOP + Window Scale(7) + EOL
        let data = [
            0x01, // NOP
            0x02, 0x04, 0x05, 0xb4, // MSS = 1460
            0x01, // NOP
            0x03, 0x03, 0x07, // Window Scale = 7
            0x00, // EOL
        ];

        let options = parse_tcp_options(&data);

        assert_eq!(options.len(), 5);
        assert_eq!(options[0], TcpOption::Nop);
        assert_eq!(options[1], TcpOption::MaxSegmentSize(1460));
        assert_eq!(options[2], TcpOption::Nop);
        assert_eq!(options[3], TcpOption::WindowScale(7));
        assert_eq!(options[4], TcpOption::EndOfList);
    }
}
