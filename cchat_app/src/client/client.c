#include <stdio.h>      // For I/O operations
#include <stdlib.h>     // For memory management and exit handling
#include <string.h>     // For string operations (e.g, bzero, memcpy)
#include <unistd.h>     // For system calls like close
#include <arpa/inet.h>  // For Internet operations (e.g., inet_pton)
#include <sys/types.h>  // For socket functions like size_t, ssize_t
#include <sys/socket.h> // For socket functions like socket, connect, send, recv
#include <netdb.h>

#define BUFFER_SIZE 1024

int main() {
	int client_fd;				// Socket file descriptor
	struct sockaddr_in server_address;	// Server address structure
	char buffer[BUFFER_SIZE];		// Buffer to hold messages
	char message[BUFFER_SIZE];		// Buffer to send messages

	const char *server_port_str = getenv("SERVER_PORT");
	const char *server_ip = getenv("SERVER_ADDRESS");

	if (server_port_str == NULL) {
		printf("Using default port of 21000.\n");
		server_port_str = "21000";
	}

	int server_port = atoi(server_port_str);
	if (server_port == 0) {
		fprintf(stderr, "Invalid port number: %s\n", server_port_str);
		exit(EXIT_FAILURE);
	}

	// Use a configurable server IP or hostname (defaulting to "localhost" if not specified)
	if (!server_ip) {
		server_ip = "localhost";
	}

	// Step 1: Create a socket
	client_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (client_fd < 0) {
		perror("Socket creation failed");
		exit(EXIT_FAILURE);
	}
	printf("Socket created successfully.\n");

	// Step 2: Define the server's address
	memset(&server_address, 0, sizeof(server_address));	// Zero out server_address structure
	server_address.sin_family = AF_INET;			// IPv4
	server_address.sin_port = htons(server_port);			// Convert port to network byte order

	struct addrinfo hints, *res;
	memset(&hints, 0, sizeof(hints));
	hints.ai_family = AF_INET;
	hints.ai_socktype = SOCK_STREAM;

	int status = getaddrinfo(server_ip, server_port_str, &hints, &res);
	if (status != 0) {
		perror("getaddrinfo failed");
		close(client_fd);
		exit(EXIT_FAILURE);
	}

	memcpy(&server_address, res->ai_addr, sizeof(server_address));

	// Convert server IP from text to binary format
	// if (inet_pton(AF_INET, server_ip, &server_address.sin_addr) <= 0) {
	//	perror("Invalid address or address not supported");
	//	close(client_fd);
	//	exit(EXIT_FAILURE);
	//}

	// Step 3: Connect to the server
	if (connect(client_fd, (struct sockaddr*)&server_address, sizeof(server_address)) < 0) {
		perror("Connection failed");
		close(client_fd);
		exit(EXIT_FAILURE);
	}
	printf("Connected to the server at 127.0.0.1:%d.\n", server_port);

	// Step 4: Communicate with the server
	while (1) {
		printf("Enter your message: ");
		fgets(message, BUFFER_SIZE, stdin);  // Read user input
		message[strcspn(message, "\n")] = 0; // Remove the newline character

		// Send the message to the server
		send(client_fd, message, strlen(message), 0);
		printf("Message sent to the server: %s\n", message);

		// If the user types "exit", close the connection
		if (strcmp(message, "exit") == 0) {
			printf("Closing connection.\n");
			break;
		}

		// Receive the server's response
		memset(buffer, 0, BUFFER_SIZE);		// Clear the buffer
		int bytes_received = recv(client_fd, buffer, BUFFER_SIZE, 0);
		if (bytes_received <= 0) {
			perror("Server closed connection or error occurred");
			break;
		}
		printf("Server: %s\n", buffer);
	}

	// Step 5: Close the socket
	close(client_fd);
	printf("Connection closed.\n");

	return 0;
}

