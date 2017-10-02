# Diplomat

[![Build Status](https://travis-ci.org/timperrett/diplomat.svg?branch=master)](https://travis-ci.org/timperrett/diplomat)

Highly experimental gRPC server providing the also experimental [v2 envoy-api](https://github.com/envoyproxy/data-plane-api), with information backed by [Consul](https://github.com/hashicorp/consul)

This project is implemented wth [Rust](https://www.rust-lang.org/)

## Usage

Diplomat is shipped as a standalone native binary and is configured via a TOML configuration file. Each xDS action can be invoked on the command line for debugging purposes, but to run the service in producution you should use sometihing like:

```
$ diplomat --config /path/to/config.toml serve
```

Using the individual xDS APIs is just as simple, with the additional params passed directly on the command line as arguments:

```
$ diplomat --config /path/to/config.toml eds --service-name foo
```

## Development

As gRPC relies on [Protocol Buffers](https://developers.google.com/protocol-buffers/), you will need to ensure that you have the `protoc` tooling avaialble on your development machine.

#### Installing Protobuf

On OSX install protobuf with Homebrew:

```
$ brew install protobuf
```

On Ubuntu, a few additional steps are needed. Sadly, using `apt-get install protobuf-compiler` does not provide a recent enough version of the `protoc` compiller. The Envoy `.proto` files require at **least version 3** of the compiller.

```
# download and unzip the needed tools
$ curl -OL https://github.com/google/protobuf/releases/download/v3.4.0/protoc-3.4.0-linux-x86_64.zip
$ unzip protoc-3.4.0-linux-x86_64.zip -d protoc3
# move the binary onto your $PATH
$ mv protoc3/bin/* /usr/local/bin && chmod 775 /usr/local/bin/protoc
# move the common proto includes onto the include path
$ mv protoc3/include/* /usr/local/include/
```

#### Fetching dependencies

Once you have the compiller installed, there is a one-time operation that is needed to fetch the various upstream dependencies for the version of protocol buffers. This has been encoded into the projects make file, so you simply need to run:

```
$ make proto
```

This will fetch to github.com and move all the gubbins which was downloaded into a local `vendor` folder. If for some reason you need to update these files over time, simply run:

```
$ make clean-full && make proto
```

#### Development Builds

```
# produce a development build (do not use in production)
$ cargo build

# run compilation whenever the source tree changes (useful for dev feedback)
$ cargo watch
```

#### Release Builds

Release builds are automatically built for this project and posted [on Github Releases](https://github.com/timperrett/diplomat/releases) for Linux amd64 machines. If you need to build your own release-class binaray for another platform, simply use:

```
$ make release
```

By default all builds are devel builds and contains symbols, so making a dedicated release is important.
