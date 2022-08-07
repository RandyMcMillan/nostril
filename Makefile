uname_S := $(shell sh -c 'uname -s 2>/dev/null || echo not')
uname_M := $(shell sh -c 'uname -m 2>/dev/null || echo not')
uname_O := $(shell sh -c 'uname -o 2>/dev/null || echo not')
uname_R := $(shell sh -c 'uname -r 2>/dev/null || echo not')
uname_P := $(shell sh -c 'uname -p 2>/dev/null || echo not')
uname_V := $(shell sh -c 'uname -v 2>/dev/null || echo not')

-:
	@echo $(uname_S)
	@echo $(uname_M)
	@echo $(uname_O)
	@echo $(uname_R)
	@echo $(uname_P)
	#@echo $(uname_V)
	
websocat:
	git clone https://github.com/vi/websocat.git websc || true
	mkdir -p /usr/local/bin
	cd websc && cargo install --path=. && install -v target/release/websocat /usr/local/bin

ifeq ($(uname_S),Linux)
	WEBSOCAT=:https://github.com/vi/websocat/releases/download/v1.10.0/websocat.$(uname_M)-unknown-linux-musl
endif
ifeq ($(uname_S),Darwin)
	WEBSOCAT=:https://github.com/vi/websocat/releases/download/v1.10.0/websocat.$(uname_M)-apple-darwin
endif
ifeq ($(uname_S),FreeBSD)
	WEBSOCAT=:https://github.com/vi/websocat/releases/download/v1.10.0/websocat.$(uname_M)-unknown-freebsd
endif
ifeq ($(uname_S),OpenBSD)
endif
ifeq ($(uname_S),NetBSD)
endif
	@echo $(WEBSOCAT)



# ifneq ($(wildcard /usr/local/bin),)
       PREFIX := /usr/local
# else
#       PREFIX := /usr
# endif

LD_LIBRARY_PATH=/usr/local/lib
export LD_LIBRARY_PATH

CFLAGS = -Wall -Og
OBJS = sha256.o nostril.o aes.o base64.o
HEADERS = hex.h random.h config.h sha256.h

all: install websocat nostril secp256k1

%.o: %.c config.h
	@echo "cc $<"
	@$(CC) $(CFLAGS) -c $< -o $@

nostril: $(HEADERS) $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -lsecp256k1 -o $@

.PHONY: secp256k1
secp256k1:
	git clone --depth 1 https://github.com/bitcoin-core/secp256k1.git || true
	cd secp256k1/ && ./autogen.sh && ./configure --prefix=/usr/local --with-gnu-ld --enable-module-extrakeys --enable-module-ecdh --enable-module-schnorrsig --enable-examples && make && make install
	cd secp256k1 ./libtool  --finish $(PREFIX)/lib

install: nostril
	mkdir -p $(PREFIX)/bin
	mkdir -p $(PREFIX)/lib
	wget -qO websocat https://github.com/vi/websocat/releases/download/v1.10.0/websocat.x86_64-unknown-linux-musl
	chmod +x websocat
	make install -C secp256k1 && install secp256k1/schnorr_example $(PREFIX)/bin/schnorr-key
	cp nostril $(PREFIX)/bin
	cp scripts/* $(PREFIX)/bin


config.h: configurator
	./configurator > $@

configurator: configurator.c
	$(CC) $< -o $@

clean:
	rm -f nostril *.o

tags: fake
	ctags *.c *.h
test:
	nostril --pow 16 --envelope --sec 1a03a2b6ce40340f012043e6d9e717950076b757a708cb6e9ac3d2e3bbe5fb1a --tag nostril test --content test | ./websocat wss://relay.damus.io

.PHONY: fake
https://github.com/vi/websocat/releases/download/v1.10.0/websocat.aarch64-unknown-linux-musl
