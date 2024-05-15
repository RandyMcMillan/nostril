
CFLAGS = -Wall -O2 -Iext/secp256k1/include
OBJS = sha256.o nostril.o aes.o base64.o
HEADERS = hex.h random.h config.h sha256.h ext/secp256k1/include/secp256k1.h
PREFIX ?= /usr/local
ARS = libsecp256k1.a

SUBMODULES = ext/secp256k1

default: all
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
help:## 	print verbose help
	@echo ''
	@echo 'Usage: make [TARGET] [EXTRA_ARGUMENTS]'
	@echo ''
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':   ' |  sed -e 's/^/ /' ## verbose help ideas
	@sed -n 's/^##  //p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
	@echo ""
	@echo "Useful Commands:"
	@echo ""

all: libsecp256k1.a nostril docs## 	nostril docs

docs: doc/nostril.1## 	docs
doc/nostril.1: README.md## 	doc/nostril.1
	@scdoc < $^ > $@ || help2man ./nostril > doc/nostril.1

version: nostril.c## 	version
	@grep '^#define VERSION' $< | sed -En 's,.*"([^"]+)".*,\1,p' > $@

dist: docs version## 	dist
	@mkdir -p dist
	git ls-files --recurse-submodules | $(shell which gtar || which tar) --transform 's/^/nostril-$(shell cat version)\//' -T- -caf dist/nostril-$(shell cat version).tar.gz
	@ls -dt dist/* | head -n1 | xargs echo "tgz "
	cd dist;\
	sha256sum *.tar.gz > SHA256SUMS.txt;\
	gpg -u $(shell gpg --list-signatures --with-colons | grep 'sig' | grep 'E616FA7221A1613E5B99206297966C06BB06757B' | head -n 1 | cut -d':' -f5) --sign --armor --detach-sig --output SHA256SUMS.txt.asc SHA256SUMS.txt
	cp CHANGELOG dist/CHANGELOG.txt
	#rsync -avzP dist/ charon:/www/cdn.jb55.com/tarballs/nostril/


%.o: %.c $(HEADERS)
	@echo "cc $<"
	@$(CC) $(CFLAGS) -c $< -o $@

nostril: $(HEADERS) $(OBJS)## 	nostril
	@$(CC) $(CFLAGS) $(OBJS) $(ARS) -o $@ || $(MAKE) $(ARS)

install: all## 	install
	@mkdir -p $(PREFIX)/bin || true
	@install -m644 doc/nostril.1 $(PREFIX)/share/man/man1/nostril.1 || true
	@install -m755 nostril $(PREFIX)/bin/nostril || true
	@install -m755 nostril-query $(PREFIX)/bin/nostril-query || true
	@$(shell which nostril)

config.h: configurator## 	config.h
	./configurator > $@

configurator: configurator.c## 	configurator
	$(CC) $< -o $@

clean:## 	clean
	rm -f nostril *.o *.a
	rm -rf ext/secp256k1/.lib

tags: fake
	ctags *.c *.h

.PHONY:docs doc/nostril.1 fake nostril version
