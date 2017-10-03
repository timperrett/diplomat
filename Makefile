
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

setup:
	cargo install --force protobuf && \
	cargo install --force grpcio-compiler && \
	cargo install --force cargo-watch

proto: setup vendor
	mkdir -p src/api && \
	cd vendor/envoy-api && \
	protoc \
		--rust_out=../../src/api \
		--grpc_out=../../src/api \
		--plugin=protoc-gen-grpc=`which grpc_rust_plugin` api/*.proto && \
	cd ../../ && \
	scripts/generate-api-mod

vendor:
	mkdir -p vendor && \
	git clone https://github.com/envoyproxy/data-plane-api.git vendor/envoy-api || echo "" > /dev/null  && \
	git clone https://github.com/googleapis/googleapis.git vendor/googleapis || echo "" > /dev/null && \
	ln -s `pwd`/vendor/googleapis/google `pwd`/vendor/envoy-api || echo "" > /dev/null

consul:
	consul agent -ui -server -advertise 127.0.0.1 -dev -data-dir target
