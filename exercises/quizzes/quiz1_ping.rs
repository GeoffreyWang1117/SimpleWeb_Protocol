//! Quiz 1: Simple Ping Tool
//!
//! Build a complete ping utility that:
//! - Constructs ICMP Echo Request packets
//! - Calculates proper checksums
//! - Parses ICMP Echo Reply packets
//! - Measures round-trip time (RTT)
//!
//! This quiz combines: binary encoding, IP headers, ICMP protocol, checksum calculation

use std::time::{Duration, Instant};

/// ICMP message types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IcmpType {
    EchoReply = 0,
    EchoRequest = 8,
    DestinationUnreachable = 3,
    TimeExceeded = 11,
}

impl IcmpType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(IcmpType::EchoReply),
            8 => Some(IcmpType::EchoRequest),
            3 => Some(IcmpType::DestinationUnreachable),
            11 => Some(IcmpType::TimeExceeded),
            _ => None,
        }
    }
}

/// ICMP Echo packet structure
#[derive(Debug, Clone)]
pub struct IcmpEcho {
    pub icmp_type: IcmpType,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16,
    pub payload: Vec<u8>,
}

impl IcmpEcho {
    /// Create a new Echo Request packet
    pub fn new_request(identifier: u16, sequence: u16, payload: Vec<u8>) -> Self {
        let mut packet = IcmpEcho {
            icmp_type: IcmpType::EchoRequest,
            code: 0,
            checksum: 0,
            identifier,
            sequence,
            payload,
        };
        packet.checksum = packet.calculate_checksum();
        packet
    }

    /// Calculate ICMP checksum (one's complement of one's complement sum)
    ///
    /// The checksum is calculated over the entire ICMP message.
    /// Steps:
    /// 1. Set checksum field to 0
    /// 2. Sum all 16-bit words
    /// 3. Add carry bits back to the sum
    /// 4. Take one's complement
    pub fn calculate_checksum(&self) -> u16 {
        let bytes = self.to_bytes_without_checksum();

        let mut sum: u32 = 0;

        // TODO: Sum all 16-bit words in the packet
        // Hint: Iterate over pairs of bytes, combine them into u16, and add to sum
        // Handle odd-length data by padding with zero
        todo!("Sum all 16-bit words");

        // TODO: Fold 32-bit sum to 16 bits (add carry)
        // Hint: Keep adding the high 16 bits to low 16 bits until no carry
        todo!("Fold to 16 bits");

        // TODO: Return one's complement
        todo!("Return one's complement")
    }

    fn to_bytes_without_checksum(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());
        bytes.push(self.icmp_type as u8);
        bytes.push(self.code);
        bytes.push(0); // checksum placeholder
        bytes.push(0);
        bytes.extend_from_slice(&self.identifier.to_be_bytes());
        bytes.extend_from_slice(&self.sequence.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    /// Serialize the ICMP packet to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize the complete ICMP Echo packet
        // Format: [type: 1][code: 1][checksum: 2][identifier: 2][sequence: 2][payload: N]
        // All multi-byte fields are in network byte order (big-endian)
        todo!("Serialize ICMP packet to bytes")
    }

    /// Parse an ICMP packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 8 {
            return Err("Packet too short");
        }

        // TODO: Parse the ICMP packet
        // 1. Extract type (byte 0) and convert to IcmpType
        // 2. Extract code (byte 1)
        // 3. Extract checksum (bytes 2-3, big-endian)
        // 4. Extract identifier (bytes 4-5, big-endian)
        // 5. Extract sequence (bytes 6-7, big-endian)
        // 6. Extract payload (remaining bytes)
        todo!("Parse ICMP packet from bytes")
    }

    /// Verify the checksum of a received packet
    pub fn verify_checksum(&self) -> bool {
        // TODO: Verify that the checksum is valid
        // Hint: Calculate checksum over entire packet (including checksum field)
        // If valid, result should be 0xFFFF or 0x0000
        todo!("Verify checksum")
    }
}

/// Ping statistics tracker
#[derive(Debug, Default)]
pub struct PingStats {
    pub packets_sent: u32,
    pub packets_received: u32,
    pub min_rtt: Option<Duration>,
    pub max_rtt: Option<Duration>,
    pub total_rtt: Duration,
}

impl PingStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a sent packet
    pub fn record_sent(&mut self) {
        // TODO: Increment packets_sent counter
        todo!("Record sent packet")
    }

    /// Record a received reply with its RTT
    pub fn record_received(&mut self, rtt: Duration) {
        // TODO: Update statistics
        // 1. Increment packets_received
        // 2. Update min_rtt if this is smaller (or first packet)
        // 3. Update max_rtt if this is larger (or first packet)
        // 4. Add to total_rtt
        todo!("Record received packet")
    }

    /// Calculate packet loss percentage
    pub fn packet_loss_percent(&self) -> f64 {
        // TODO: Calculate and return packet loss as percentage
        // Hint: (sent - received) / sent * 100
        todo!("Calculate packet loss")
    }

    /// Calculate average RTT
    pub fn average_rtt(&self) -> Option<Duration> {
        // TODO: Calculate average RTT
        // Return None if no packets received
        todo!("Calculate average RTT")
    }

    /// Generate summary string (like real ping output)
    pub fn summary(&self) -> String {
        // TODO: Generate summary string in format:
        // "X packets transmitted, Y received, Z% packet loss"
        // "rtt min/avg/max = A/B/C ms"
        todo!("Generate summary")
    }
}

/// Ping session manager
pub struct PingSession {
    pub identifier: u16,
    pub sequence: u16,
    pub stats: PingStats,
    pending_requests: std::collections::HashMap<u16, Instant>,
}

impl PingSession {
    pub fn new(identifier: u16) -> Self {
        Self {
            identifier,
            sequence: 0,
            stats: PingStats::new(),
            pending_requests: std::collections::HashMap::new(),
        }
    }

    /// Create the next Echo Request packet
    pub fn create_request(&mut self, payload: Vec<u8>) -> IcmpEcho {
        // TODO: Create an Echo Request packet
        // 1. Use current sequence number
        // 2. Increment sequence for next request
        // 3. Record send time in pending_requests
        // 4. Update stats
        // 5. Return the packet
        todo!("Create Echo Request")
    }

    /// Process a received Echo Reply
    pub fn process_reply(&mut self, reply: &IcmpEcho) -> Result<Duration, &'static str> {
        // TODO: Process Echo Reply
        // 1. Verify it's an Echo Reply
        // 2. Verify identifier matches our session
        // 3. Look up send time from pending_requests
        // 4. Calculate RTT
        // 5. Update stats
        // 6. Return RTT
        todo!("Process Echo Reply")
    }

    /// Handle timeout for a sequence number
    pub fn handle_timeout(&mut self, sequence: u16) {
        // TODO: Remove from pending_requests without updating received count
        // This represents a lost packet
        todo!("Handle timeout")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        let packet = IcmpEcho::new_request(0x1234, 0x0001, vec![0x61, 0x62, 0x63, 0x64]);

        // Verify checksum is non-zero
        assert_ne!(packet.checksum, 0);

        // Verify the packet validates
        assert!(packet.verify_checksum());
    }

    #[test]
    fn test_checksum_known_value() {
        // Test with known values
        // Type=8, Code=0, ID=0x0001, Seq=0x0001, Payload="abcd"
        let packet = IcmpEcho::new_request(0x0001, 0x0001, b"abcd".to_vec());

        // Checksum should be consistent
        let bytes = packet.to_bytes();
        let parsed = IcmpEcho::from_bytes(&bytes).unwrap();
        assert!(parsed.verify_checksum());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = IcmpEcho::new_request(12345, 42, b"Hello, ping!".to_vec());
        let bytes = original.to_bytes();
        let parsed = IcmpEcho::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.icmp_type, IcmpType::EchoRequest);
        assert_eq!(parsed.code, 0);
        assert_eq!(parsed.identifier, 12345);
        assert_eq!(parsed.sequence, 42);
        assert_eq!(parsed.payload, b"Hello, ping!");
        assert!(parsed.verify_checksum());
    }

    #[test]
    fn test_ping_stats() {
        let mut stats = PingStats::new();

        stats.record_sent();
        stats.record_sent();
        stats.record_sent();

        stats.record_received(Duration::from_millis(10));
        stats.record_received(Duration::from_millis(20));

        assert_eq!(stats.packets_sent, 3);
        assert_eq!(stats.packets_received, 2);

        let loss = stats.packet_loss_percent();
        assert!((loss - 33.33).abs() < 0.1);

        let avg = stats.average_rtt().unwrap();
        assert_eq!(avg, Duration::from_millis(15));

        assert_eq!(stats.min_rtt, Some(Duration::from_millis(10)));
        assert_eq!(stats.max_rtt, Some(Duration::from_millis(20)));
    }

    #[test]
    fn test_ping_session() {
        let mut session = PingSession::new(0xABCD);

        let req1 = session.create_request(b"test".to_vec());
        assert_eq!(req1.identifier, 0xABCD);
        assert_eq!(req1.sequence, 0);

        let req2 = session.create_request(b"test".to_vec());
        assert_eq!(req2.sequence, 1);

        assert_eq!(session.stats.packets_sent, 2);
    }

    #[test]
    fn test_ping_session_reply() {
        let mut session = PingSession::new(0xABCD);

        let _req = session.create_request(b"test".to_vec());

        // Simulate receiving reply after some time
        std::thread::sleep(Duration::from_millis(5));

        let reply = IcmpEcho {
            icmp_type: IcmpType::EchoReply,
            code: 0,
            checksum: 0,
            identifier: 0xABCD,
            sequence: 0,
            payload: b"test".to_vec(),
        };

        let rtt = session.process_reply(&reply).unwrap();
        assert!(rtt >= Duration::from_millis(5));
        assert_eq!(session.stats.packets_received, 1);
    }

    #[test]
    fn test_parse_invalid_packet() {
        let short = vec![0, 0, 0];
        assert!(IcmpEcho::from_bytes(&short).is_err());
    }
}
