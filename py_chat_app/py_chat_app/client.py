'''
CLI parameters:
- The server's IP address
- Listening port
- Username
- Password (all clients should use the same password)
'''
import argparse
import socket
import threading
import sys

def receive_messages(sock):
    while True:
        try:
            message = sock.recv(1024).decode()
            if not message:
                break
            print(f"{message}")
            sys.stdout.flush()
        except Exception as e:
            print(f"Error receiving message: {e}")
            break

def send_messages(sock):
    while True:
        try:
            message = input()
            if message == ':Exit':
                sock.send(message.encode())
                break
            sock.send(message.encode())
        except KeyboardInterrupt:
            print("\nDisconnecting...")
            sock.send(':Exit'.encode())
            break
        except Exception as e:
            print(f"Error sending message: {e}")
            break

def parse_args():
    parser = argparse.ArgumentParser(
        prog='Client',
        description='Client for chat room',
        formatter_class=argparse.RawTextHelpFormatter
    )
    parser.add_argument('-join', action='store_const', const=True, required=True, help='Flag to inititate connection to the chat server.')
    parser.add_argument('-host', type=str, required=True, help='Hostname or IP address (IPv4) of the chat server')
    parser.add_argument('-port', type=int, required=True, help='Port number of the chat server')
    parser.add_argument('-username', type=str, required=True, help='Username of client for chat room')
    parser.add_argument('-passcode', type=str, required=True, help='Passcode for connecting to the chat room')

    args = parser.parse_args()

    if args.port < 0 or args.port > 65535:
        parser.error("Port must be in range 0-65535")
    if args.username == "":
        parser.error("Invalid username")
    if args.host == "":
        parser.error("Invalid hostname")
    if len(args.host) > 15:
        parser.error("Invalid hostname (make sure not to include whitespace)")

    return args

if __name__ == '__main__':
    args = parse_args()
    host = args.host
    port = args.port
    username = args.username
    passcode = args.passcode

    sockfd = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        sockfd.connect((host, port))

        sockfd.send(passcode.encode())

        response = sockfd.recv(1024).decode()

        print(response)

        if "Incorrect passcode" == response:
            sockfd.close()
            exit(1)

        sockfd.send(username.encode())

        recv_thread = threading.Thread(target=receive_messages, args=(sockfd,))
        recv_thread.daemon = True
        recv_thread.start()

        send_thread = threading.Thread(target=send_messages, args=(sockfd,))
        send_thread.daemon = True
        send_thread.start()

        recv_thread.join()
        send_thread.join()


    except Exception as e:
        print(f"Connection error: {e}")
    finally:
        sockfd.close()

