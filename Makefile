
PROGRAM_NAME ?= consort

all: setup compile

setup:
	cargo install protobuf && \
	cargo install grpc-compiler

compile: proto
	cargo build -v

clean:
	rm -rf vendor

proto: vendor
	mkdir -p src/envoy && \
	cd vendor/envoy-api && \
	protoc --rust_out ../../src/envoy api/*.proto && \
	protoc --rust-grpc_out ../../src/envoy api/*.proto

vendor: clean
	mkdir -p vendor && \
	git clone git@github.com:lyft/envoy-api.git vendor/envoy-api && \
	git clone git@github.com:googleapis/googleapis.git vendor/googleapis && \
	ln -s `pwd`/vendor/googleapis/google `pwd`/vendor/envoy-api