FROM alpine:latest

RUN apk add --no-cache \
	gcc \
	g++ \
	make \
	gdb \
	valgrind \
	vim \
	git \
	libc-dev

WORKDIR /src

COPY . /src

CMD ["/bin/sh"]

