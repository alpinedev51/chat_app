# cchat_app
Chat application implemented in C using TCP/UDP for learning about socket programming and concurrency.

# Overview

Features:

1. TCP/UDP Support
2. Server-Client Architecture
3. Basic CLI
4. Concurrency

# TCP

TCP (Transmission Control Protocol) is one of the main protocols of the TCP/IP suite. It lies between the Application and Network layers which are used in providing reliable delivery services. TCP ensures reliable and efficient data transmission over the internet.

TCP is a connection-oriented protocol for communications that helps in the exchange of messages between different devices over a network. The Internet Protocol (IP), which establishes the technique for sending data packets between computers, works with TCP.

The position of TCP is at the transport layer of the OSI model. TCP also helps in ensuring that information is transmitted accurately by establishing a virtual connection between the sender and receiver.

# IP

Internet Protocol (IP) is a method that is useful for sending data from one device to another from all over the internet. It is a set of rules governing how data is sent and received over the internet. It is responsible for addressing and routing packets of data so they can travel from the sender to the correct destination across multiple networks. Every device contains a unique IP address that helps it communicate and exchange data across other devices present on the internet.

# TCP Further

TCP model breaks down that data into small bundles and afterward reassembles the bundles into the original message on the opposite end to make sure that each message reaches its target location intact. Sending the information in little bundles of information makes it simpler to maintain efficiency as opposed to sending everything in one go.

After a particular message is broken down into bundles, these bundles may travel along multiple routes if one route is jammed but the destination remains the same.

**Example**: When a user requests a web page on the internet, the server processes that request and sends back an HTML page to that user. The server makes use of a protocol called the HTTP protocol. The HTTP then requests the TCP layer to set the required connection and send the HTML file.

The TCP breaks the data into small packets and forwards it toward the Internet Protocol (IP) layer. The packets are then sent to the destination through different routes.

The TCP layer in the user's system waits for the transmission to get finished and acknowledges once all packets have been received.

## Features of TCP/IP

Some of the most prominent features of TCP:

- Segment Numbering System: TCP keeps track of the segments being transmitted or received by assigning numbers to each and every single one of them. A specific Byte Number is assigned to data bytes that are to be transferred while segments are assigned sequence numbers. Acknowledgement Numbers are assigned to received segments.

- Connection Oriented: It means sender and receiver are connected to each other till the completion of the process. The order of the data is maintained i.e. order remains same before and after transmission.

- Full Duplex: In TCP data can be transmitted from receiver to the sender or vice-versa at the same time. It increases efficiency of data flow between sender and receiver.

- Flow Control: Flow control limits the rate at which a sender transfers data. This is done to ensure reliable delivery. The receiver continually hints to the sender on how much data can be received (using a sliding window).

- Error Control: TCP implements an error control mechanism for reliable data transfer. Error control is byte-oriented. Segments are checked for error detection. Error Control includes - Corrupted Segment * Lost Segment Management, Out-of-order segments, Duplicate segments, etc.

- Congestion Control: TCP takes into account the level of congestion in the network. Congestion level is determined by the amount of data sent by a sender.

# RFC 9293 - TCP

## Functional Specification

### Header Format

TCP segments are sent as internet diagrams. The Internet Protocol (IP) header carries several information fields, including the source and destination host addresses. A TCP header follows the IP headers, supplying information specific to TCP. This division allows for the existence of host-level protocols other than TCP. In the early development of the Internet suite of protocols, the IP header fields had been a part of TCP.

A TCP header, followed by any user data in the segment, is formatted as follows, using the style from [66]:

 0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |          Source Port          |       Destination Port        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                        Sequence Number                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Acknowledgment Number                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |  Data |       |C|E|U|A|P|R|S|F|                               |
   | Offset| Rsrvd |W|C|R|C|S|S|Y|I|            Window             |
   |       |       |R|E|G|K|H|T|N|N|                               |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |           Checksum            |         Urgent Pointer        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                           [Options]                           |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                                                               :
   :                             Data                              :
   :                                                               |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

          Note that one tick mark represents one bit position.
Figure 1: TCP Header Format

where:
- Source Port: 16 bits
- Destination Port: 16 bits
- Sequence Number: 32 bits
  - The sequence number of the first data octet in this segment (except when the SYN flag is set). If SYN is set, the sequence number is the initial sequence number (ISN) and the first data octet is ISN+1.
- Acknowledgement Number: 32 bits
  - If the ACK control bit is set, this field contains the value of the next sequence number the sender of the segment is expecting to receive. Once a connection is established, this is always sent.
- Data Offset (DOffset): 4 bits
  - The number of 32-bit words in the TCP header. This indicates where the data begins. The TCP header (even one including options) is an integer multiple of 32 bits long.
- Reserved (Rsrvd): 4 bits
  - A set of control bits reserved for future use. Must be zero in generated segments and must be ignored in received segments if the corresponding future features are not implemented by the sending or receiving host.
- Control bits: THe control bits are also known as "flags". Assignment is managed by IANA from the "TCP Header Flags" registry. The currently assigned control bits are CWR, ECE, URG, ACK, PSH, RST, SYN, and FIN.
  - CWR: 1 bit
    - Congestion Window Reduced.
  - ECE: 1bit
    - ECN-Echo.
  - URG: 1 bit
    - Urgent pointer field is significant.
  - ACK: 1 bit
    - Acknowledgement field is significant.
  - PSH: 1 bit
    - Push function.
  - RST: 1 bit
    - Reset the connection.
  - SYN: 1 bit
    - Synchronize sequence numbers.
  - FIN: 1 bit
    - No more data from sender.
  - Window: 16 bits
    - The number of data octets beginning with the one indicated in the acknowledgement field that the sender of this segment is willing to accept. The value is shifted when the window scaling extension is used.
  - Checksum: 16 bits
    - The checksum field is the 16-bit ones' complement of the ones' complement sum of all 16-bit words in the header and text.

# Docker

`docker build -t cchat_app .`

`docker run -it --rm -v $(pwd):/src cchat_app`

