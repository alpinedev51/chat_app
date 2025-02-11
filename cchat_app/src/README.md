# Socket Programming

## Socket

A **socket** is one endpoint of a two way communication link between two programs running on the network. The socket mechanism provides a means of inter-process communication (IPC) by establishing named contact points between which the communication take place.

The socket provides bidirectional FIFO Communication facility over the network. A socket connecting to the network is created at each end of the communication. Each socket has a specific address. This address is composed of an IP address and a port number.

Sockets are generally employed in client-server applications. The server creates a socket, attaches it to a network port address then waits for the client to contact it. THe client creates a socket and then attempts to connect to the server socket. When the connection is established, transfer of data takes place.

## Binding

Binding a socket is the process of associating a socket with a specific IP address and port number.

# Server

# Client

A TCP client is a program that:

1. Establishes a connection to a TCP server using the server's IP address and port.

2. Sends data to the server and/or receives data from it.

3. Gracefully disconnects after the interaction.

# Common

