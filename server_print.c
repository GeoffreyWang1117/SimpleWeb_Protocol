#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080

int main() {
    int server_fd, new_socket;
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    char buffer[1024] = {0};
    char *ack_msg = "SYN-ACK";
    char *final_ack = "ACK";

    // 创建socket文件描述符
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // 绑定地址和端口
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(PORT);

    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("bind failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    // 监听连接
    if (listen(server_fd, 3) < 0) {
        perror("listen failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    printf("Server is waiting for a connection...\n");

    // 接受客户端连接
    if ((new_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t*)&addrlen)) < 0) {
        perror("accept failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    // 第一次握手：收到SYN
    read(new_socket, buffer, 1024);
    printf("Server received: %s\n", buffer);

    // 第二次握手：发送SYN-ACK
    send(new_socket, ack_msg, strlen(ack_msg), 0);
    printf("Server sent: %s\n", ack_msg);

    // 第三次握手：收到ACK
    read(new_socket, buffer, 1024);
    printf("Server received: %s\n", buffer);

    if (strcmp(buffer, final_ack) == 0) {
        printf("Three-way handshake completed successfully.\n");
    } else {
        printf("Handshake failed.\n");
    }

    close(new_socket);
    close(server_fd);
    return 0;
}
