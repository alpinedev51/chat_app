#include<sys/types.h>  // Contains definitions for data types used in system calls
#include<sys/socket.h> // Provides the socket API (socket(), bind(), listen(), accept(), etc.)
#include<netinet/in.h> // Defines structures for storing IP addresses and ports
#include<stdio.h>      // For input/output functions like printf() and fgets()
#include<string.h>     // For memory and string manipulation functions like bzero()
#include<unistd.h>     // For close() function to close sockets

int main() {
	// Buffers to store messages
	char buffer[1024];
	char response[1024];

	// File descriptors for the listening socket and the communication socket
	int listen_fd, comm_fd;

	// The struct sockaddr_in is defined in the <netinet/in.h> header file. This structure is specifically designed to hold an IPv4 address and port number for use with sockets.
	// struct sockaddr_in {
	//     sa_family_t    sin_family;  // Address family (e.g., AF_INET for IPv4)
	//     in_port_t      sin_port;    // Port number in network byte order
	//     struct in_addr sin_addr;    // IPv4 address (struct holding a single address)
	//     unsigned char  sin_zero[8]; // Padding to match the size of sockaddr (not used)
	// };
	struct sockaddr_in servaddr;

	// Step 1: Create a socket
	// int socket(int domain, int type, int protocol);
	// The 'socket()' function creates a new socket and returns its file descriptor.
	// Arguments:
	// - AF_INET: Use IPv4 addresses.
	// - SOCK_STREAM: Use TCP.
	// - 0: Default protocol (TCP for SOCK_STREAM)
	listen_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (listen_fd < 0) {
		perror("Error creating socket");
		return 1;
	}
	printf("Socket created successfully.\n");

	// Step 2: Clear the server address structure
	// The 'bzero()' function sets all bytes in the memory to zero.
	bzero(&servaddr, sizeof(servaddr));

	// Step 3: Configure the server address structure
	servaddr.sin_family = AF_INET; // IPv4 address family
	servaddr.sin_addr.s_addr = htonl(INADDR_ANY); // Bind to any available network interface
	servaddr.sin_port = htons(21000); // Convert port number (21000) to network byte order
	

	// Step 4: Bind the socket to the specified address and port
	// int bind(int socket, const struct sockaddr *address, socklen_t address_len);
	// The 'bind()' function associates the socket with the address and port.
	if (bind(listen_fd, (struct sockaddr*)&servaddr, sizeof(servaddr)) < 0) {
		perror("Error binding socket");
		return 1;
	}
	printf("Socket bound to port 21000.\n");

	// Step 5: Put the socket into listening mode
	// int listen(int socket, int backlog);
	// The 'listen()' function marks the socket as a passive socket, i.e., it will accept incoming connections.
	// The second argument specifies the maximum number of pending connections (backlog).
	if (listen(listen_fd, 5) < 0) {
		perror("Error listening on socket");
		return 1;
	}
	printf("Server is listening for incoming connections...\n");
	
	// Step 6: Communication loop
	// This loop handles communication with the connected client.
	while (1) {
		printf("Waiting for a client to connect...\n");
		// Step 7: Accept a client connection
		// int accept(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len);
		// The 'accept()' function waits for a client to connect.
		// It creates a new socket (comm_fd) for communication with the client.
		comm_fd = accept(listen_fd, (struct sockaddr*)NULL, NULL);
		if (comm_fd < 0) {
			perror("Error accepting connection");
			continue;
		}
		printf("Client connected.\n");
		
		while (1) {
			bzero(buffer, sizeof(buffer));
			bzero(response, sizeof(response));

			// Receive a message from the client
			// ssize_t recv(int socket, void *buffer, size_t length, int flags);
			// The 'recv()' function reads data from the socket into the buffer.
			int bytes_received = recv(comm_fd, buffer, sizeof(buffer) - 1, 0);
			if (bytes_received < 0) {
				perror("Error receiving data");
			} else if (bytes_received == 0) {
				printf("Client disconnected.\n");
				break;
			}
			
			// Null-terminate the received message
			buffer[bytes_received] = '\0';
			printf("Client: %s\n", buffer);

			// Prompt the server user to respond
			printf("Your response: ");
			fgets(response, sizeof(response), stdin);

			// Send the response to the client
			// The 'send()' function writes data from the buffer to the socket.
			int bytes_sent = send(comm_fd, response, strlen(response), 0);
			if (bytes_sent < 0) {
				perror("Error sending data");
				break;
			}
		}
		// Step 8: Close the communication socket
		close(comm_fd);
		printf("Client connection closed.\n");
	}
	// Step 9: Close the listening socket
	close(listen_fd);
	printf("Server shut down.\n");
	return 0;
}

