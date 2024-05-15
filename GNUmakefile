default: all
ext/secp256k1/include/secp256k1.h:
	git checkout ext
	git checkout ext/secp256k1

ext/secp256k1/configure: ext/secp256k1/include/secp256k1.h
	@cd ext/secp256k1; \
	@./autogen.sh

ext/secp256k1/Makefile: ext/secp256k1/configure
	@cd ext/secp256k1; \
	@./configure \
        --disable-shared \
        --enable-module-ecdh \
        --enable-module-schnorrsig \
        --enable-module-extrakeys

ext/secp256k1/.libs/libsecp256k1.a: ext/secp256k1/Makefile
	@cd ext/secp256k1; \
	make -j libsecp256k1.la

libsecp256k1.a: ext/secp256k1/.libs/libsecp256k1.a
	cp $< $@
-include Makefile
