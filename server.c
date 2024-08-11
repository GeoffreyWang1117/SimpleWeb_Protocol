#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 8080

int main() {
    int server_sock, new_sock;
    struct sockaddr_in addr;
    socklen_t addr_size = sizeof(addr);

    // Create socket
    server_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (server_sock == -1) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    // Set up address and port number
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(PORT);

    // Bind socket to address and port number
    if (bind(server_sock, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }

    // Listen for incoming connections
    listen(server_sock, 3);

    printf("Server listening on port %d\\n", PORT);

    while (1) {
        // Accept connection request from client
        addr_size = sizeof(addr);
        new_sock = accept(server_sock, (struct sockaddr *)&addr, &addr_size);
        if (new_sock < 0) {
            perror("Accept failed");
            exit(EXIT_FAILURE);
        }

        printf("Connection accepted\\n");

        // Close connection when done
        close(new_sock);
    }

    return 0;
}