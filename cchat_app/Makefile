CC = gcc
CFLAGS = -Wall -Wextra -g
SRC = src/
CLIENT_DIR = client/
SERVER_DIR = server/
OBJ = obj/
BIN = bin/

all: server client

server: $(SRC)$(SERVER_DIR)server.c
	$(CC) $(CFLAGS) -o $(BIN)server.out $(SRC)$(SERVER_DIR)server.c

client: $(SRC)$(CLIENT_DIR)client.c
	$(CC) $(CFLAGS) -o $(BIN)client.out $(SRC)$(CLIENT_DIR)client.c

clean:
	rm -rf $(BIN)* $(OBJ).o

.PHONY: all clean

