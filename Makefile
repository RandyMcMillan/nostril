ifneq ($(wildcard /usr/local/bin),)
       PREFIX := /usr/local/bin
else
       PREFIX := /usr/bin
endif
PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin:/sbin" ldconfig -n /usr/local/lib

CFLAGS = -Wall -Og
OBJS = sha256.o nostril.o aes.o base64.o
HEADERS = hex.h random.h config.h sha256.h

all: nostril

%.o: %.c config.h
	@echo "cc $<"
	@$(CC) $(CFLAGS) -c $< -o $@

nostril: $(HEADERS) $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -lsecp256k1 -o $@ 

.PHONY: secp256k1
secp256k1:
	git clone --depth 1 https://github.com/bitcoin-core/secp256k1.git || true
	cd secp256k1 && ./autogen.sh && ./configure --with-gnu-ld --enable-module-extrakeys --enable-module-ecdh --enable-module-schnorrsig --enable-examples && make install && cd ..
	install secp256k1/include/* /usr/local/include

install: nostril
	mkdir -p $(PREFIX)/bin
	cp nostril $(PREFIX)
	cp scripts/* $(PREFIX)/

config.h: configurator                                                          
	./configurator > $@                                                     

configurator: configurator.c                                                    
	$(CC) $< -o $@

clean:
	rm -f nostril *.o

tags: fake
	ctags *.c *.h

.PHONY: fake
