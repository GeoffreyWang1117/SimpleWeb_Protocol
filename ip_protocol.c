#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <arpa/inet.h>  // 添加这个头文件以使用 htons, htonl, ntohs 函数


// IP 头部结构
struct ip_header {
    uint8_t version_and_length;
    uint8_t type_of_service;
    uint16_t total_length;
    uint16_t identification;
    uint16_t flags_and_fragment_offset;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t checksum;
    uint32_t source_ip;
    uint32_t dest_ip;
};

// 简单的校验和计算函数
uint16_t calculate_checksum(uint16_t *header, int size) {
    uint32_t sum = 0;
    for (int i = 0; i < size / 2; i++) {
        sum += header[i];
        if (sum >> 16) {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
    }
    return ~sum;
}

// 创建 IP 数据包
void create_ip_packet(struct ip_header *ip_packet, uint32_t src_ip, uint32_t dest_ip) {
    ip_packet->version_and_length = (4 << 4) | 5;  // 版本号 (IPv4) 和 头部长度 (5 * 32-bit words)
    ip_packet->type_of_service = 0;
    ip_packet->total_length = htons(sizeof(struct ip_header));
    ip_packet->identification = htons(54321);
    ip_packet->flags_and_fragment_offset = 0;
    ip_packet->ttl = 64;
    ip_packet->protocol = 6;  // TCP
    ip_packet->checksum = 0;
    ip_packet->source_ip = htonl(src_ip);
    ip_packet->dest_ip = htonl(dest_ip);

    // 计算校验和
    ip_packet->checksum = calculate_checksum((uint16_t *)ip_packet, sizeof(struct ip_header));
}

// 解析 IP 数据包
void parse_ip_packet(struct ip_header *ip_packet) {
    printf("IP Version: %d\n", ip_packet->version_and_length >> 4);
    printf("Header Length: %d\n", (ip_packet->version_and_length & 0x0F) * 4);
    printf("Type of Service: %d\n", ip_packet->type_of_service);
    printf("Total Length: %d\n", ntohs(ip_packet->total_length));
    printf("Identification: %d\n", ntohs(ip_packet->identification));
    printf("TTL: %d\n", ip_packet->ttl);
    printf("Protocol: %d\n", ip_packet->protocol);
    printf("Checksum: 0x%x\n", ntohs(ip_packet->checksum));
    printf("Source IP: %d.%d.%d.%d\n", 
           ip_packet->source_ip >> 24 & 0xFF, 
           ip_packet->source_ip >> 16 & 0xFF, 
           ip_packet->source_ip >> 8 & 0xFF, 
           ip_packet->source_ip & 0xFF);
    printf("Destination IP: %d.%d.%d.%d\n", 
           ip_packet->dest_ip >> 24 & 0xFF, 
           ip_packet->dest_ip >> 16 & 0xFF, 
           ip_packet->dest_ip >> 8 & 0xFF, 
           ip_packet->dest_ip & 0xFF);
}

int main() {
    struct ip_header ip_packet;

    // 创建一个IP包
    create_ip_packet(&ip_packet, 0xC0A80001, 0xC0A80002);  // 示例IP地址：192.168.0.1 -> 192.168.0.2

    // 解析并打印IP包内容
    parse_ip_packet(&ip_packet);

    return 0;
}
