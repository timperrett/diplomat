
PROGRAM_NAME ?= diplomat

all: setup compile

compile: proto
	cargo build -v

release: proto
	cargo build -v --release && \
	mv target/release/diplomat diplomat && \
	tar cvzf diplomat.tar.gz diplomat

run: compile
	./target/debug/diplomat --help

clean:
	rm -rf target

clean-full:
	rm -rf vendor && \
	rm -rf target

setup: vendor
	cargo install protobuf || echo "[info] protobuf crate already installed..." && \
	cargo install grpcio-compiler || echo "[info] grpcio-compiler crate already installed..."

proto: setup
	mkdir -p `pwd`/src/api && \
	cd `pwd`/vendor/envoy-api && \
	protoc \
		--rust_out=../../src/api \
		--grpc_out=../../src/api \
		--plugin=protoc-gen-grpc=`which grpc_rust_plugin` api/*.proto && \
	cd ../../ && \
	scripts/generate-api-mod

vendor:
	mkdir -p vendor && \
	git clone https://github.com/envoyproxy/data-plane-api.git vendor/envoy-api || echo "[warn] unable to clone data-plane-api" && \
	git clone https://github.com/googleapis/googleapis.git vendor/googleapis || echo "[warn] unable to clone googleapis" && \
	ln -s `pwd`/vendor/googleapis/google `pwd`/vendor/envoy-api || echo "[warn] unable to link to vendor/envoy-api" && \
	echo "[info] successfully updated the vendor dependencies..."

consul:
	consul agent -ui -server -advertise 127.0.0.1 -dev -data-dir target
