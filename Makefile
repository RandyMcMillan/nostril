LD_LIBRARY_PATH=/usr/local/lib
export LD_LIBRARY_PATH

CFLAGS = -Wall -Og
OBJS = sha256.o nostril.o aes.o base64.o
HEADERS = hex.h random.h config.h sha256.h

-: all
	bash -c "git submodule update --init"

all: websocat nostril secp256k1

config.h: configurator
	./configurator > $@

configurator: configurator.c
	$(CC) $< -o $@

clean:
	rm -f nostril *.o

tags: fake
	ctags *.c *.h
	
	

websocat:
	git clone https://github.com/vi/websocat.git websc || git -C websc reset --hard
	mkdir -p /usr/local/bin
	cd websc && cargo install --path=. && install -v target/release/websocat /usr/local/bin

nostril: $(HEADERS) $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -lsecp256k1 -o $@

.PHONY: secp256k1
secp256k1:
	git clone --depth 1 https://github.com/bitcoin-core/secp256k1.git || git -C secp256k1 reset --hard
	cd secp256k1/ && ./autogen.sh && ./configure --prefix=/usr/local --with-gnu-ld --enable-module-extrakeys --enable-module-ecdh --enable-module-schnorrsig --enable-examples && make && make install
	cd secp256k1 ./libtool  --finish $(PREFIX)/lib

install: nostril
	mkdir -p $(PREFIX)/bin
	mkdir -p $(PREFIX)/lib
	make install -C secp256k1 && install secp256k1/schnorr_example $(PREFIX)/bin/schnorr-key
	cp nostril $(PREFIX)/bin
	cp scripts/* $(PREFIX)/bin

test:
	nostril --pow 16 --envelope --sec 1a03a2b6ce40340f012043e6d9e717950076b757a708cb6e9ac3d2e3bbe5fb1a --tag nostril test --content test | websocat wss://relay.damus.io

.PHONY: fake
