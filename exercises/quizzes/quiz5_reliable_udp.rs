//! Quiz 5: Reliable UDP Transport
//!
//! Build a reliable data transfer protocol over UDP that:
//! - Implements sequence numbers and acknowledgments
//! - Handles packet loss with retransmission
//! - Provides flow control with sliding window
//! - Implements congestion control (simple AIMD)
//! - Supports ordered and unordered delivery
//! - Handles connection establishment and teardown
//!
//! This quiz combines: UDP, reliability, flow control, congestion control, state machine

use std::collections::{BTreeMap, VecDeque};
use std::time::{Duration, Instant};

/// Packet types for our reliable UDP protocol
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketType {
    Syn = 0,      // Connection request
    SynAck = 1,   // Connection accepted
    Ack = 2,      // Acknowledgment only
    Data = 3,     // Data packet (requires ACK)
    DataAck = 4,  // Data with piggybacked ACK
    Fin = 5,      // Connection close request
    FinAck = 6,   // Connection close acknowledged
    Reset = 7,    // Connection reset
}

impl PacketType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PacketType::Syn),
            1 => Some(PacketType::SynAck),
            2 => Some(PacketType::Ack),
            3 => Some(PacketType::Data),
            4 => Some(PacketType::DataAck),
            5 => Some(PacketType::Fin),
            6 => Some(PacketType::FinAck),
            7 => Some(PacketType::Reset),
            _ => None,
        }
    }
}

/// Packet header (16 bytes)
#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub flags: u8,
    pub seq_num: u32,      // Sequence number
    pub ack_num: u32,      // Acknowledgment number
    pub window: u16,       // Receive window size
    pub checksum: u16,     // Packet checksum
}

impl PacketHeader {
    pub const SIZE: usize = 16;

    /// Serialize header to bytes
    pub fn to_bytes(&self) -> [u8; 16] {
        // TODO: Serialize header to 16 bytes
        // Format: [type:1][flags:1][seq:4][ack:4][window:2][checksum:2][reserved:2]
        todo!("Serialize packet header")
    }

    /// Parse header from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        // TODO: Parse header from 16 bytes
        todo!("Parse packet header")
    }
}

/// Complete packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: PacketType, seq_num: u32, payload: Vec<u8>) -> Self {
        Self {
            header: PacketHeader {
                packet_type,
                flags: 0,
                seq_num,
                ack_num: 0,
                window: 0,
                checksum: 0,
            },
            payload,
        }
    }

    /// Create ACK packet
    pub fn ack(ack_num: u32, window: u16) -> Self {
        Self {
            header: PacketHeader {
                packet_type: PacketType::Ack,
                flags: 0,
                seq_num: 0,
                ack_num,
                window,
                checksum: 0,
            },
            payload: Vec::new(),
        }
    }

    /// Serialize packet
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize packet (header + payload)
        // Calculate checksum over entire packet
        todo!("Serialize packet")
    }

    /// Parse packet
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        // TODO: Parse packet and verify checksum
        todo!("Parse packet")
    }

    /// Calculate packet checksum (similar to Internet checksum)
    pub fn calculate_checksum(data: &[u8]) -> u16 {
        // TODO: Calculate one's complement checksum
        todo!("Calculate checksum")
    }
}

/// Retransmission timer with exponential backoff
#[derive(Debug)]
pub struct RetransmitTimer {
    rto: Duration,          // Retransmission timeout
    srtt: Duration,         // Smoothed RTT
    rttvar: Duration,       // RTT variance
    min_rto: Duration,
    max_rto: Duration,
}

impl RetransmitTimer {
    pub fn new() -> Self {
        Self {
            rto: Duration::from_millis(1000),
            srtt: Duration::from_millis(0),
            rttvar: Duration::from_millis(0),
            min_rto: Duration::from_millis(200),
            max_rto: Duration::from_secs(60),
        }
    }

    /// Update RTO based on measured RTT (RFC 6298 algorithm)
    pub fn update_rtt(&mut self, rtt: Duration) {
        // TODO: Implement RFC 6298 RTO calculation
        // First measurement: SRTT = RTT, RTTVAR = RTT/2
        // Subsequent: RTTVAR = (1-beta)*RTTVAR + beta*|SRTT-RTT|
        //             SRTT = (1-alpha)*SRTT + alpha*RTT
        //             RTO = SRTT + max(G, K*RTTVAR)
        // Where alpha=1/8, beta=1/4, K=4, G=clock granularity

        todo!("Update RTO from RTT measurement")
    }

    /// Get current RTO
    pub fn rto(&self) -> Duration {
        self.rto
    }

    /// Double the RTO (exponential backoff)
    pub fn backoff(&mut self) {
        // TODO: Double RTO up to max_rto
        todo!("Exponential backoff")
    }

    /// Reset RTO after successful transmission
    pub fn reset(&mut self) {
        // TODO: Reset RTO to initial value
        todo!("Reset RTO")
    }
}

/// Sliding window sender
pub struct SendWindow {
    base: u32,              // Oldest unacked sequence
    next_seq: u32,          // Next sequence to use
    window_size: u32,       // Current window size
    max_window: u32,        // Maximum window size
    unacked: BTreeMap<u32, (Packet, Instant)>,  // Unacked packets with send time
    retransmit_timer: RetransmitTimer,
}

impl SendWindow {
    pub fn new(initial_window: u32) -> Self {
        Self {
            base: 0,
            next_seq: 0,
            window_size: initial_window,
            max_window: 65535,
            unacked: BTreeMap::new(),
            retransmit_timer: RetransmitTimer::new(),
        }
    }

    /// Check if we can send more data
    pub fn can_send(&self) -> bool {
        // TODO: Check if window allows more data
        // next_seq - base < window_size
        todo!("Check if can send")
    }

    /// Get number of bytes in flight
    pub fn bytes_in_flight(&self) -> u32 {
        // TODO: Calculate bytes currently in flight
        todo!("Calculate bytes in flight")
    }

    /// Send a packet (add to unacked)
    pub fn send(&mut self, packet: Packet) -> u32 {
        // TODO: Add packet to unacked, advance next_seq
        // Return the sequence number used
        todo!("Send packet")
    }

    /// Process acknowledgment
    pub fn process_ack(&mut self, ack_num: u32) -> Vec<u32> {
        // TODO: Cumulative ACK processing
        // 1. Remove all packets with seq < ack_num
        // 2. Update RTT estimate for acked packets
        // 3. Advance base
        // 4. Return list of newly acked sequence numbers

        todo!("Process ACK")
    }

    /// Get packets that need retransmission
    pub fn get_retransmits(&mut self) -> Vec<Packet> {
        // TODO: Find packets that have timed out
        // Update their send time and apply backoff

        todo!("Get packets for retransmission")
    }

    /// Update window size (from receiver advertisement)
    pub fn update_window(&mut self, window: u16) {
        // TODO: Update window size from receiver
        todo!("Update window")
    }
}

/// Sliding window receiver
pub struct RecvWindow {
    next_expected: u32,     // Next expected sequence number
    window_size: u32,       // Receive window size
    buffer: BTreeMap<u32, Packet>,  // Out-of-order packets
    delivered: VecDeque<Vec<u8>>,   // Ready to deliver
}

impl RecvWindow {
    pub fn new(window_size: u32) -> Self {
        Self {
            next_expected: 0,
            window_size,
            buffer: BTreeMap::new(),
            delivered: VecDeque::new(),
        }
    }

    /// Receive a packet
    pub fn receive(&mut self, packet: Packet) -> Option<u32> {
        // TODO: Process received packet
        // 1. Check if in window range
        // 2. If expected seq, deliver and check buffer for consecutive packets
        // 3. If future seq, buffer it
        // 4. Return ACK number to send

        todo!("Receive packet")
    }

    /// Get available window size
    pub fn available_window(&self) -> u16 {
        // TODO: Calculate available window
        todo!("Calculate available window")
    }

    /// Get delivered data
    pub fn read(&mut self) -> Option<Vec<u8>> {
        // TODO: Return next delivered payload
        todo!("Read delivered data")
    }

    /// Check for gaps in received data (for SACK)
    pub fn get_sack_blocks(&self) -> Vec<(u32, u32)> {
        // TODO: Return list of (start, end) for received ranges
        // Used for selective acknowledgment

        todo!("Get SACK blocks")
    }
}

/// Connection states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

/// Connection state machine
pub struct Connection {
    state: ConnectionState,
    local_seq: u32,
    remote_seq: u32,
    send_window: SendWindow,
    recv_window: RecvWindow,
    send_buffer: VecDeque<u8>,
    mss: usize,  // Maximum segment size
}

impl Connection {
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Closed,
            local_seq: rand_seq(),
            remote_seq: 0,
            send_window: SendWindow::new(1),
            recv_window: RecvWindow::new(65535),
            send_buffer: VecDeque::new(),
            mss: 1400,
        }
    }

    /// Initiate connection (client side)
    pub fn connect(&mut self) -> Packet {
        // TODO: Create SYN packet and transition to SynSent
        todo!("Initiate connection")
    }

    /// Accept connection (server side)
    pub fn listen(&mut self) {
        // TODO: Transition to Listen state
        todo!("Start listening")
    }

    /// Process incoming packet and generate response
    pub fn process(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: State machine processing
        // Handle packet based on current state and packet type
        // Return response packet if needed

        match self.state {
            ConnectionState::Closed => self.process_closed(packet),
            ConnectionState::Listen => self.process_listen(packet),
            ConnectionState::SynSent => self.process_syn_sent(packet),
            ConnectionState::SynReceived => self.process_syn_received(packet),
            ConnectionState::Established => self.process_established(packet),
            ConnectionState::FinWait1 => self.process_fin_wait1(packet),
            ConnectionState::FinWait2 => self.process_fin_wait2(packet),
            ConnectionState::CloseWait => self.process_close_wait(packet),
            ConnectionState::Closing => self.process_closing(packet),
            ConnectionState::LastAck => self.process_last_ack(packet),
            ConnectionState::TimeWait => self.process_time_wait(packet),
        }
    }

    fn process_closed(&mut self, _packet: Packet) -> Option<Packet> {
        // TODO: Handle packet in Closed state (usually RST)
        todo!("Process in Closed state")
    }

    fn process_listen(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle SYN in Listen state
        // Send SYN-ACK and transition to SynReceived
        todo!("Process in Listen state")
    }

    fn process_syn_sent(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle SYN-ACK in SynSent state
        // Send ACK and transition to Established
        todo!("Process in SynSent state")
    }

    fn process_syn_received(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle ACK in SynReceived state
        // Transition to Established
        todo!("Process in SynReceived state")
    }

    fn process_established(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle Data/ACK/FIN in Established state
        todo!("Process in Established state")
    }

    fn process_fin_wait1(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle ACK/FIN in FinWait1 state
        todo!("Process in FinWait1 state")
    }

    fn process_fin_wait2(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle FIN in FinWait2 state
        todo!("Process in FinWait2 state")
    }

    fn process_close_wait(&mut self, _packet: Packet) -> Option<Packet> {
        // TODO: Application should close, send FIN
        todo!("Process in CloseWait state")
    }

    fn process_closing(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle ACK in Closing state
        todo!("Process in Closing state")
    }

    fn process_last_ack(&mut self, packet: Packet) -> Option<Packet> {
        // TODO: Handle ACK in LastAck state
        todo!("Process in LastAck state")
    }

    fn process_time_wait(&mut self, _packet: Packet) -> Option<Packet> {
        // TODO: Handle in TimeWait state (usually respond to retransmitted FINs)
        todo!("Process in TimeWait state")
    }

    /// Queue data for sending
    pub fn send(&mut self, data: &[u8]) -> Result<(), &'static str> {
        // TODO: Add data to send buffer
        // Return error if not in Established state
        todo!("Queue data for sending")
    }

    /// Get next packet to transmit
    pub fn get_next_packet(&mut self) -> Option<Packet> {
        // TODO: Create data packet from send buffer
        // Respect MSS and window constraints
        todo!("Get next packet to transmit")
    }

    /// Initiate connection close
    pub fn close(&mut self) -> Option<Packet> {
        // TODO: Send FIN and transition to appropriate state
        todo!("Initiate close")
    }

    /// Get current connection state
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Read received data
    pub fn read(&mut self) -> Option<Vec<u8>> {
        self.recv_window.read()
    }
}

/// Congestion control (simplified AIMD)
pub struct CongestionController {
    cwnd: u32,           // Congestion window (bytes)
    ssthresh: u32,       // Slow start threshold
    mss: u32,            // Maximum segment size
    dup_ack_count: u32,  // Duplicate ACK counter
    in_fast_recovery: bool,
}

impl CongestionController {
    pub fn new(mss: u32) -> Self {
        Self {
            cwnd: mss,           // Start with 1 MSS
            ssthresh: 65535,
            mss,
            dup_ack_count: 0,
            in_fast_recovery: false,
        }
    }

    /// Get current congestion window
    pub fn window(&self) -> u32 {
        self.cwnd
    }

    /// Called when ACK received for new data
    pub fn on_ack(&mut self) {
        // TODO: Increase cwnd
        // Slow start: cwnd += MSS (exponential growth)
        // Congestion avoidance: cwnd += MSS * MSS / cwnd (linear growth)
        todo!("Handle ACK")
    }

    /// Called when duplicate ACK received
    pub fn on_dup_ack(&mut self) -> bool {
        // TODO: Track duplicate ACKs
        // Return true if fast retransmit should be triggered (3 dup ACKs)
        todo!("Handle duplicate ACK")
    }

    /// Called on timeout (packet loss detected)
    pub fn on_timeout(&mut self) {
        // TODO: Multiplicative decrease
        // ssthresh = max(cwnd/2, 2*MSS)
        // cwnd = MSS
        // Exit fast recovery
        todo!("Handle timeout")
    }

    /// Called after fast retransmit
    pub fn on_fast_retransmit(&mut self) {
        // TODO: Enter fast recovery
        // ssthresh = max(cwnd/2, 2*MSS)
        // cwnd = ssthresh + 3*MSS
        todo!("Handle fast retransmit")
    }

    /// Called when fast recovery ACK received
    pub fn on_fast_recovery_ack(&mut self, new_data_acked: bool) {
        // TODO: Update cwnd during fast recovery
        // If new data acked: exit fast recovery, cwnd = ssthresh
        // Otherwise: cwnd += MSS (inflate)
        todo!("Handle fast recovery ACK")
    }
}

/// Generate random initial sequence number
fn rand_seq() -> u32 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (now & 0xFFFFFFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_serialization() {
        let packet = Packet::new(PacketType::Data, 12345, b"Hello".to_vec());
        let bytes = packet.to_bytes();
        let parsed = Packet::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.header.packet_type, PacketType::Data);
        assert_eq!(parsed.header.seq_num, 12345);
        assert_eq!(parsed.payload, b"Hello");
    }

    #[test]
    fn test_send_window() {
        let mut window = SendWindow::new(3);

        assert!(window.can_send());

        let p1 = Packet::new(PacketType::Data, 0, vec![1]);
        let p2 = Packet::new(PacketType::Data, 0, vec![2]);
        let p3 = Packet::new(PacketType::Data, 0, vec![3]);

        window.send(p1);
        window.send(p2);
        window.send(p3);

        // Window should be full
        assert!(!window.can_send());

        // Process ACK for first packet
        let acked = window.process_ack(1);
        assert_eq!(acked.len(), 1);
        assert!(window.can_send());
    }

    #[test]
    fn test_recv_window() {
        let mut window = RecvWindow::new(10);

        // Receive in order
        let p1 = Packet::new(PacketType::Data, 0, vec![1]);
        let ack = window.receive(p1);
        assert_eq!(ack, Some(1));

        // Receive out of order
        let p3 = Packet::new(PacketType::Data, 2, vec![3]);
        let ack = window.receive(p3);
        assert_eq!(ack, Some(1)); // Still waiting for seq 1

        // Fill the gap
        let p2 = Packet::new(PacketType::Data, 1, vec![2]);
        let ack = window.receive(p2);
        assert_eq!(ack, Some(3)); // Now have 0, 1, 2
    }

    #[test]
    fn test_recv_window_sack() {
        let mut window = RecvWindow::new(10);

        // Receive with gaps
        window.receive(Packet::new(PacketType::Data, 0, vec![0]));
        window.receive(Packet::new(PacketType::Data, 2, vec![2]));
        window.receive(Packet::new(PacketType::Data, 3, vec![3]));
        window.receive(Packet::new(PacketType::Data, 5, vec![5]));

        let sack = window.get_sack_blocks();
        // Should have blocks for [2-3] and [5-5]
        assert!(sack.len() >= 2);
    }

    #[test]
    fn test_connection_handshake() {
        let mut client = Connection::new();
        let mut server = Connection::new();

        server.listen();

        // Client sends SYN
        let syn = client.connect();
        assert_eq!(client.state(), ConnectionState::SynSent);

        // Server receives SYN, sends SYN-ACK
        let syn_ack = server.process(syn).unwrap();
        assert_eq!(server.state(), ConnectionState::SynReceived);

        // Client receives SYN-ACK, sends ACK
        let ack = client.process(syn_ack).unwrap();
        assert_eq!(client.state(), ConnectionState::Established);

        // Server receives ACK
        server.process(ack);
        assert_eq!(server.state(), ConnectionState::Established);
    }

    #[test]
    fn test_retransmit_timer() {
        let mut timer = RetransmitTimer::new();

        // Update with RTT measurements
        timer.update_rtt(Duration::from_millis(100));
        assert!(timer.rto() >= Duration::from_millis(100));

        // Backoff
        let old_rto = timer.rto();
        timer.backoff();
        assert!(timer.rto() >= old_rto);
    }

    #[test]
    fn test_congestion_control() {
        let mut cc = CongestionController::new(1000);

        // Initial window is 1 MSS
        assert_eq!(cc.window(), 1000);

        // Slow start: exponential growth
        cc.on_ack();
        assert!(cc.window() >= 2000);

        // Timeout: multiplicative decrease
        let window_before = cc.window();
        cc.on_timeout();
        assert!(cc.window() < window_before);
    }

    #[test]
    fn test_fast_retransmit() {
        let mut cc = CongestionController::new(1000);

        // Grow window
        for _ in 0..10 {
            cc.on_ack();
        }

        // 3 duplicate ACKs trigger fast retransmit
        assert!(!cc.on_dup_ack());
        assert!(!cc.on_dup_ack());
        assert!(cc.on_dup_ack()); // Third dup ACK
    }
}
