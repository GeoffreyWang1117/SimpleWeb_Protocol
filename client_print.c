#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080

int main() {
    int sock = 0;
    struct sockaddr_in serv_addr;
    char *syn_msg = "SYN";
    char buffer[1024] = {0};
    char *ack_msg = "SYN-ACK";
    char *final_ack = "ACK";

    // 创建socket文件描述符
    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        printf("Socket creation error\n");
        return -1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);

    // 将IP地址转换为二进制形式
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        printf("Invalid address/ Address not supported \n");
        return -1;
    }

    // 连接服务器
    if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
        printf("Connection Failed \n");
        return -1;
    }

    // 第一次握手：发送SYN
    send(sock, syn_msg, strlen(syn_msg), 0);
    printf("Client sent: %s\n", syn_msg);

    // 第二次握手：接收SYN-ACK
    read(sock, buffer, 1024);
    printf("Client received: %s\n", buffer);

    if (strcmp(buffer, ack_msg) == 0) {
        // 第三次握手：发送ACK
        send(sock, final_ack, strlen(final_ack), 0);
        printf("Client sent: %s\n", final_ack);
    } else {
        printf("Handshake failed.\n");
    }

    close(sock);
    return 0;
}
