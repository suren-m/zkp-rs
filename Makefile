REG := ghcr.io
OWNER := suren-m
REPO := zkp-rs
SERVER_TAG := $(REG)/$(OWNER)/zkp-server:latest
CLIENT_TAG := $(REG)/$(OWNER)/zkp-client:latest

install-all: install-server \
	install-client 

uninstall-all: uninstall-server \
	uninstall-client 

build-images: build-image-server \
	build-image-client

push-images: push-image-server \
	push-image-client

install-server:
	cargo install --path server

install-client:
	cargo install --path client

uninstall-server:
	cargo uninstall -p server

uninstall-client:
	cargo uninstall -p client

run-tests:
	cargo test

# Set context at root level to import common lib dependency as well
build-image-server:
	docker build . -f ./server/Dockerfile -t $(SERVER_TAG)

build-image-client:
	docker build . -f ./client/Dockerfile -t $(CLIENT_TAG)

push-image-server:
	docker push $(SERVER_TAG)

push-image-client:
	docker push $(CLIENT_TAG)	