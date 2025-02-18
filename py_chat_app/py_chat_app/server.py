'''
CLI parameters:
- Listening port
- Password
'''
import argparse
import socket
import threading
import sys
from datetime import datetime, timedelta

class ClientThread:
    def __init__(self, client_socket, host, port, address, clients, passcode):
        '''
        - client_socket: client socket created by server
        - host: hostname for server
        - port: port server parent socket is bound to
        - address: client IPv4 address
        - passcode given by client
        '''
        self.client_socket = client_socket
        self.host = host
        self.port = port
        self.address = address
        self.clients = clients
        self.passcode = passcode

        # client username in chat
        self.username = None

        # chat shortcuts
        self.shortcuts = {
            ':)': '[Feeling Joyful]',
            ':(': '[Feeling Unhappy]'
        }

    # handle connection
    def run(self):
        try:
            # receive client's passcode attempt
            auth_msg = self.client_socket.recv(1024).decode().strip()
            # authenticate client
            if auth_msg != self.passcode:
                self.client_socket.send("Incorrect passcode".encode())
                self.client_socket.close()
                return
            else:
                self.client_socket.send(f"Connected to {self.host} on port {self.port}".encode())

            # receive and set client username
            self.username = self.client_socket.recv(1024).decode().strip()

            join_message = f"{self.username} joined the chatroom"
            print(join_message)
            sys.stdout.flush()
            # send message to all other active and authenticated connected clients
            self.broadcast_message(join_message)

            # insert client to dictionary mapping client username -> client socket
            # NOTE: multiple clients can have same username. switch this to client address -> client socket
            self.clients[self.username] = self.client_socket

            # persistent connection loop
            flag = True
            while flag:
                message = self.client_socket.recv(1024).decode()
                if not message:
                    break
                flag = self.handle_message(message)

        except Exception as e:
            print(f"Error handling client {self.address}: {e}")
        finally:
            if self.username and self.username in self.clients:
                del self.clients[self.username]
                self.broadcast_message(f"{self.username} left the chatroom")
            self.client_socket.close()

    def handle_message(self, message):
        try:
            match message.split(" ", 2):
                case [':Exit']:
                    print(f"{self.username} left the chatroom")
                    return False
                case [shortcut] if shortcut in self.shortcuts:
                    shortcut_message = f"{self.username}: {self.shortcuts[message]}"
                    print(shortcut_message)
                    sys.stdout.flush()
                    self.broadcast_message(shortcut_message)
                    return True # Finished handling
                case [':mytime']:
                    time = datetime.now().strftime("%Y %b %d %H:%M:%S %a")
                    time_message = f"{self.username}: {time}"
                    print(time_message)
                    sys.stdout.flush()
                    self.broadcast_message(time_message)
                    return True
                case [':+1hr']:
                    time = (datetime.now() + timedelta(hours=1)).strftime("%Y %b %d %H:%M:%S %a")
                    time_message = f"{self.username}: {time}"
                    print(time_message)
                    sys.stdout.flush()
                    self.broadcast_message(time_message)
                    return True
                case [':dm', dm_username, dm_message]:
                    self.handle_dm(dm_username, dm_message)
                    print(f"{self.username} to {dm_username}: {dm_message}")
                    sys.stdout.flush()
                    return True
                case _:
                    print(f"{self.username}: {message}")
                    sys.stdout.flush()
                    self.broadcast_message(f"{self.username}: {message}")
                    return True
        except Exception as e:
            print(f"Error handling message: {e}")

    def broadcast_message(self, message):
        for username, client in self.clients.items():
            try:
                if username != self.username:
                    client.send(message.encode())
            except Exception as e:
                print(f"Error sending message to {username}: {e}")

    def handle_dm(self, dm_username, dm_message):
        try:
            message = f"{self.username}: {dm_message}"
            self.clients[dm_username].send(message.encode())
        except Exception as e:
            print(f"Error handling dm to {dm_username}: {e}")

def parse_args():
    parser = argparse.ArgumentParser(
        prog='Server',
        description='Server for chat room',
        formatter_class=argparse.RawTextHelpFormatter
    )

    parser.add_argument('-start', action='store_const', const=True, required=True, help='Flag to start server')
    parser.add_argument('-port', type=int, required=True, help='Port number to listen on')
    parser.add_argument('-passcode', type=str, required=True, help='Passcode for connecting to the server')

    args = parser.parse_args()

    if args.port < 0 or args.port > 65535:
        parser.error("Port must be in range 0-65535")
    if len(args.passcode) > 5:
        parser.error("Passcode length must be no longer than 5 characters")

    return args

if __name__ == '__main__':
    arguments = parse_args()
    port = arguments.port
    passcode = arguments.passcode
    host = '127.0.0.1'
    active_clients = {}

    parentfd = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    parentfd.bind((host, port))

    parentfd.listen(5)

    print(f"Server started on port {port}. Accepting connections")
    sys.stdout.flush()

    while True:
        try:
            (clientfd, address) = parentfd.accept()
            ct = ClientThread(clientfd, host, port, address, active_clients, passcode)
            thread = threading.Thread(target=ct.run)
            thread.daemon = True
            thread.start()
        except Exception as e:
            print(f"Error accepting connection: {e}")

