//! Quiz 4: DNS Query Client
//!
//! Build a complete DNS client that:
//! - Constructs DNS query packets
//! - Parses DNS response packets
//! - Supports A, AAAA, CNAME, MX, TXT record types
//! - Handles DNS message compression
//! - Implements caching with TTL
//!
//! This quiz combines: UDP protocol, binary encoding, domain names, caching
//!
//! Reference: RFC 1035

use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::{Duration, Instant};

/// DNS Record Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordType {
    A = 1,        // IPv4 address
    NS = 2,       // Name server
    CNAME = 5,    // Canonical name
    MX = 15,      // Mail exchange
    TXT = 16,     // Text record
    AAAA = 28,    // IPv6 address
}

impl RecordType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(RecordType::A),
            2 => Some(RecordType::NS),
            5 => Some(RecordType::CNAME),
            15 => Some(RecordType::MX),
            16 => Some(RecordType::TXT),
            28 => Some(RecordType::AAAA),
            _ => None,
        }
    }
}

/// DNS Query/Response class
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordClass {
    IN = 1,  // Internet
}

/// DNS Header flags
#[derive(Debug, Clone, Default)]
pub struct DnsFlags {
    pub qr: bool,           // Query (0) or Response (1)
    pub opcode: u8,         // 0 = standard query
    pub aa: bool,           // Authoritative answer
    pub tc: bool,           // Truncated
    pub rd: bool,           // Recursion desired
    pub ra: bool,           // Recursion available
    pub rcode: u8,          // Response code (0 = no error)
}

impl DnsFlags {
    /// Pack flags into u16
    pub fn to_u16(&self) -> u16 {
        // TODO: Pack flags into 16-bit value
        // Bit layout (from MSB):
        // QR(1) | OPCODE(4) | AA(1) | TC(1) | RD(1) | RA(1) | Z(3) | RCODE(4)
        todo!("Pack DNS flags")
    }

    /// Unpack flags from u16
    pub fn from_u16(value: u16) -> Self {
        // TODO: Unpack 16-bit value into flags
        todo!("Unpack DNS flags")
    }
}

/// DNS Header (12 bytes)
#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: DnsFlags,
    pub qdcount: u16,   // Question count
    pub ancount: u16,   // Answer count
    pub nscount: u16,   // Authority count
    pub arcount: u16,   // Additional count
}

impl DnsHeader {
    pub fn new_query(id: u16) -> Self {
        Self {
            id,
            flags: DnsFlags {
                qr: false,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,  // Request recursion
                ra: false,
                rcode: 0,
            },
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    /// Serialize header to bytes (12 bytes)
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize DNS header to 12 bytes
        // Format: ID(2) + FLAGS(2) + QDCOUNT(2) + ANCOUNT(2) + NSCOUNT(2) + ARCOUNT(2)
        todo!("Serialize DNS header")
    }

    /// Parse header from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        // TODO: Parse DNS header from 12 bytes
        todo!("Parse DNS header")
    }
}

/// DNS Question
#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: RecordType,
    pub qclass: RecordClass,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: RecordType) -> Self {
        Self {
            name,
            qtype,
            qclass: RecordClass::IN,
        }
    }

    /// Encode domain name to DNS wire format
    ///
    /// "www.example.com" -> [3]www[7]example[3]com[0]
    pub fn encode_name(name: &str) -> Vec<u8> {
        // TODO: Encode domain name to DNS format
        // Each label is prefixed with its length
        // End with null byte
        todo!("Encode domain name")
    }

    /// Serialize question to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize question section
        // Format: NAME + QTYPE(2) + QCLASS(2)
        todo!("Serialize question")
    }
}

/// DNS Resource Record data
#[derive(Debug, Clone)]
pub enum RecordData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    NS(String),
    MX { priority: u16, exchange: String },
    TXT(String),
    Unknown(Vec<u8>),
}

/// DNS Resource Record
#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: RecordType,
    pub class: RecordClass,
    pub ttl: u32,
    pub data: RecordData,
}

/// DNS Message parser (handles compression)
pub struct DnsParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> DnsParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Read u16 from current position
    fn read_u16(&mut self) -> Result<u16, &'static str> {
        if self.pos + 2 > self.data.len() {
            return Err("Unexpected end of data");
        }
        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    /// Read u32 from current position
    fn read_u32(&mut self) -> Result<u32, &'static str> {
        if self.pos + 4 > self.data.len() {
            return Err("Unexpected end of data");
        }
        let value = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    /// Read domain name with compression support
    ///
    /// DNS compression uses pointers (first two bits = 11)
    /// Pointer format: 11xxxxxx xxxxxxxx (14-bit offset)
    pub fn read_name(&mut self) -> Result<String, &'static str> {
        // TODO: Read domain name, handling compression
        // 1. Read length byte
        // 2. If top 2 bits are 11, it's a pointer - follow it
        // 3. Otherwise, read 'length' bytes as label
        // 4. Repeat until length is 0
        // 5. Join labels with '.'
        //
        // Important: Track position correctly with pointers

        todo!("Read domain name with compression")
    }

    /// Read name at a specific offset (for following pointers)
    fn read_name_at(&self, offset: usize) -> Result<String, &'static str> {
        // TODO: Read name at offset (recursive helper)
        todo!("Read name at offset")
    }

    /// Parse DNS header
    pub fn parse_header(&mut self) -> Result<DnsHeader, &'static str> {
        DnsHeader::from_bytes(&self.data[..12]).map(|h| {
            self.pos = 12;
            h
        })
    }

    /// Parse DNS question
    pub fn parse_question(&mut self) -> Result<DnsQuestion, &'static str> {
        // TODO: Parse question section
        // Read: NAME + QTYPE(2) + QCLASS(2)
        todo!("Parse question")
    }

    /// Parse DNS resource record
    pub fn parse_record(&mut self) -> Result<DnsRecord, &'static str> {
        // TODO: Parse resource record
        // Read: NAME + TYPE(2) + CLASS(2) + TTL(4) + RDLENGTH(2) + RDATA
        // Parse RDATA based on type

        todo!("Parse resource record")
    }

    /// Parse A record data (4 bytes -> IPv4)
    fn parse_a_record(&mut self, rdlength: u16) -> Result<RecordData, &'static str> {
        // TODO: Parse A record (IPv4 address)
        todo!("Parse A record")
    }

    /// Parse AAAA record data (16 bytes -> IPv6)
    fn parse_aaaa_record(&mut self, rdlength: u16) -> Result<RecordData, &'static str> {
        // TODO: Parse AAAA record (IPv6 address)
        todo!("Parse AAAA record")
    }

    /// Parse CNAME/NS record data (domain name)
    fn parse_name_record(&mut self) -> Result<RecordData, &'static str> {
        // TODO: Parse CNAME or NS record
        todo!("Parse CNAME/NS record")
    }

    /// Parse MX record data (priority + exchange)
    fn parse_mx_record(&mut self) -> Result<RecordData, &'static str> {
        // TODO: Parse MX record (preference + exchange)
        todo!("Parse MX record")
    }

    /// Parse TXT record data
    fn parse_txt_record(&mut self, rdlength: u16) -> Result<RecordData, &'static str> {
        // TODO: Parse TXT record (one or more character strings)
        todo!("Parse TXT record")
    }
}

/// Complete DNS Response
#[derive(Debug, Clone)]
pub struct DnsResponse {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsResponse {
    /// Parse complete DNS response
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        // TODO: Parse complete DNS response
        // 1. Parse header
        // 2. Parse questions (qdcount times)
        // 3. Parse answers (ancount times)
        // 4. Parse authorities (nscount times)
        // 5. Parse additionals (arcount times)

        todo!("Parse DNS response")
    }

    /// Check if response indicates an error
    pub fn is_error(&self) -> bool {
        self.header.flags.rcode != 0
    }

    /// Get error description
    pub fn error_description(&self) -> Option<&'static str> {
        match self.header.flags.rcode {
            0 => None,
            1 => Some("Format error"),
            2 => Some("Server failure"),
            3 => Some("Name error (NXDOMAIN)"),
            4 => Some("Not implemented"),
            5 => Some("Refused"),
            _ => Some("Unknown error"),
        }
    }

    /// Get all A records
    pub fn a_records(&self) -> Vec<Ipv4Addr> {
        // TODO: Extract all A record addresses
        todo!("Get A records")
    }

    /// Get all AAAA records
    pub fn aaaa_records(&self) -> Vec<Ipv6Addr> {
        // TODO: Extract all AAAA record addresses
        todo!("Get AAAA records")
    }
}

/// DNS Query builder
pub struct DnsQuery {
    id: u16,
    questions: Vec<DnsQuestion>,
}

impl DnsQuery {
    pub fn new() -> Self {
        Self {
            id: rand_id(),
            questions: Vec::new(),
        }
    }

    pub fn with_id(id: u16) -> Self {
        Self {
            id,
            questions: Vec::new(),
        }
    }

    /// Add a question
    pub fn question(mut self, name: &str, qtype: RecordType) -> Self {
        self.questions.push(DnsQuestion::new(name.to_string(), qtype));
        self
    }

    /// Build the DNS query packet
    pub fn build(&self) -> Vec<u8> {
        // TODO: Build complete DNS query packet
        // 1. Create header with question count
        // 2. Serialize header
        // 3. Serialize each question
        todo!("Build DNS query")
    }

    /// Get the query ID
    pub fn id(&self) -> u16 {
        self.id
    }
}

/// Generate a random query ID
fn rand_id() -> u16 {
    // Simple random ID generation
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (now & 0xFFFF) as u16
}

/// DNS Cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    records: Vec<DnsRecord>,
    expires_at: Instant,
}

/// DNS Cache with TTL support
pub struct DnsCache {
    entries: HashMap<(String, RecordType), CacheEntry>,
    max_entries: usize,
}

impl DnsCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
        }
    }

    /// Cache DNS records
    pub fn insert(&mut self, name: &str, rtype: RecordType, records: Vec<DnsRecord>) {
        // TODO: Insert records into cache
        // 1. Find minimum TTL from records
        // 2. Calculate expiration time
        // 3. Evict oldest entries if at capacity
        // 4. Store the entry

        todo!("Insert into DNS cache")
    }

    /// Look up cached records
    pub fn get(&mut self, name: &str, rtype: RecordType) -> Option<Vec<DnsRecord>> {
        // TODO: Look up cached records
        // 1. Check if entry exists
        // 2. Check if entry is expired
        // 3. Remove if expired, return None
        // 4. Return records if valid

        todo!("Get from DNS cache")
    }

    /// Remove expired entries
    pub fn cleanup(&mut self) {
        // TODO: Remove all expired entries
        todo!("Cleanup expired entries")
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        // TODO: Calculate cache statistics
        todo!("Get cache stats")
    }
}

#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
}

/// Complete DNS Client
pub struct DnsClient {
    server: String,
    cache: DnsCache,
    timeout: Duration,
}

impl DnsClient {
    pub fn new(server: &str) -> Self {
        Self {
            server: server.to_string(),
            cache: DnsCache::new(1000),
            timeout: Duration::from_secs(5),
        }
    }

    /// Resolve a domain name to IPv4 addresses
    pub fn resolve_a(&mut self, name: &str) -> Result<Vec<Ipv4Addr>, &'static str> {
        // TODO: Resolve A records (with caching)
        // 1. Check cache first
        // 2. If not cached, build and send query
        // 3. Parse response
        // 4. Cache results
        // 5. Return addresses

        todo!("Resolve A records")
    }

    /// Resolve a domain name to IPv6 addresses
    pub fn resolve_aaaa(&mut self, name: &str) -> Result<Vec<Ipv6Addr>, &'static str> {
        // TODO: Resolve AAAA records
        todo!("Resolve AAAA records")
    }

    /// Get MX records for a domain
    pub fn resolve_mx(&mut self, name: &str) -> Result<Vec<(u16, String)>, &'static str> {
        // TODO: Resolve MX records (sorted by priority)
        todo!("Resolve MX records")
    }

    /// Follow CNAME chain to get final A records
    pub fn resolve_with_cname(&mut self, name: &str) -> Result<(Vec<String>, Vec<Ipv4Addr>), &'static str> {
        // TODO: Resolve with CNAME following
        // Return (cname_chain, final_addresses)
        // Limit CNAME hops to prevent loops
        todo!("Resolve with CNAME chain")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_flags() {
        let flags = DnsFlags {
            qr: true,
            opcode: 0,
            aa: true,
            tc: false,
            rd: true,
            ra: true,
            rcode: 0,
        };

        let packed = flags.to_u16();
        let unpacked = DnsFlags::from_u16(packed);

        assert_eq!(unpacked.qr, flags.qr);
        assert_eq!(unpacked.aa, flags.aa);
        assert_eq!(unpacked.rd, flags.rd);
        assert_eq!(unpacked.ra, flags.ra);
    }

    #[test]
    fn test_encode_name() {
        let encoded = DnsQuestion::encode_name("www.example.com");
        // Should be: [3]www[7]example[3]com[0]
        assert_eq!(encoded[0], 3); // "www" length
        assert_eq!(&encoded[1..4], b"www");
        assert_eq!(encoded[4], 7); // "example" length
        assert_eq!(&encoded[5..12], b"example");
        assert_eq!(encoded[12], 3); // "com" length
        assert_eq!(&encoded[13..16], b"com");
        assert_eq!(encoded[16], 0); // null terminator
    }

    #[test]
    fn test_dns_header_serialization() {
        let header = DnsHeader::new_query(0x1234);
        let bytes = header.to_bytes();

        assert_eq!(bytes.len(), 12);
        assert_eq!(&bytes[0..2], &[0x12, 0x34]); // ID

        let parsed = DnsHeader::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.id, 0x1234);
        assert_eq!(parsed.qdcount, 1);
    }

    #[test]
    fn test_dns_query_build() {
        let query = DnsQuery::with_id(0xABCD)
            .question("example.com", RecordType::A);

        let packet = query.build();

        // Verify header ID
        assert_eq!(&packet[0..2], &[0xAB, 0xCD]);
    }

    #[test]
    fn test_parse_a_response() {
        // Example DNS response for A query
        let response_data = [
            // Header
            0xAB, 0xCD, // ID
            0x81, 0x80, // Flags: response, recursion available
            0x00, 0x01, // QDCOUNT: 1
            0x00, 0x01, // ANCOUNT: 1
            0x00, 0x00, // NSCOUNT: 0
            0x00, 0x00, // ARCOUNT: 0
            // Question
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            0x03, b'c', b'o', b'm', 0x00,
            0x00, 0x01, // Type A
            0x00, 0x01, // Class IN
            // Answer
            0xC0, 0x0C, // Name pointer to question
            0x00, 0x01, // Type A
            0x00, 0x01, // Class IN
            0x00, 0x00, 0x0E, 0x10, // TTL: 3600
            0x00, 0x04, // RDLENGTH: 4
            0x5D, 0xB8, 0xD8, 0x22, // IP: 93.184.216.34
        ];

        let response = DnsResponse::from_bytes(&response_data).unwrap();

        assert_eq!(response.header.id, 0xABCD);
        assert_eq!(response.answers.len(), 1);

        let addrs = response.a_records();
        assert_eq!(addrs.len(), 1);
        assert_eq!(addrs[0], Ipv4Addr::new(93, 184, 216, 34));
    }

    #[test]
    fn test_dns_cache() {
        let mut cache = DnsCache::new(100);

        let record = DnsRecord {
            name: "example.com".to_string(),
            rtype: RecordType::A,
            class: RecordClass::IN,
            ttl: 3600,
            data: RecordData::A(Ipv4Addr::new(93, 184, 216, 34)),
        };

        cache.insert("example.com", RecordType::A, vec![record]);

        let cached = cache.get("example.com", RecordType::A);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);

        // Different type should not be cached
        let not_cached = cache.get("example.com", RecordType::AAAA);
        assert!(not_cached.is_none());
    }

    #[test]
    fn test_record_type_conversion() {
        assert_eq!(RecordType::from_u16(1), Some(RecordType::A));
        assert_eq!(RecordType::from_u16(28), Some(RecordType::AAAA));
        assert_eq!(RecordType::from_u16(999), None);
    }
}
