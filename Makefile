
PROGRAM_NAME ?= diplomat

all: setup compile

setup:
	cargo install --force protobuf && \
	cargo install --force grpc-compiler && \
	cargo install --force cargo-watch

compile: proto
	cargo build -v

release: proto
	cargo build -v --release

run: compile
	./target/debug/diplomat --help

clean:
	rm -rf vendor && \
	rm -rf target

proto: vendor
	mkdir -p src/envoy && \
	cd vendor/envoy-api && \
	protoc --rust_out ../../src/envoy api/*.proto && \
	protoc --rust-grpc_out ../../src/envoy api/*.proto

vendor: clean
	mkdir -p vendor && \
	git clone https://github.com/envoyproxy/data-plane-api.git vendor/envoy-api && \
	git clone https://github.com/googleapis/googleapis.git vendor/googleapis && \
	ln -s `pwd`/vendor/googleapis/google `pwd`/vendor/envoy-api

consul:
	consul agent -ui -server -advertise 127.0.0.1 -dev -data-dir target
