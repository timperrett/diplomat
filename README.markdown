# Diplomat

[![Build Status](https://travis-ci.org/timperrett/diplomat.svg?branch=master)](https://travis-ci.org/timperrett/diplomat)

Highly experimental gRPC server providing the also experimental [v2 envoy-api](https://github.com/envoyproxy/data-plane-api), with information backed by [Consul](https://github.com/hashicorp/consul)

This project is implemented wth [Rust](https://www.rust-lang.org/)

### Setup

On OSX install protobuf with Homebrew:

```
brew install protobuf
```

On Ubuntu, `protobuf-compiler` package can be installed:

```
apt-get install protobuf-compiler
```

Once you have that installed, you can run:

```
make setup
```

If you would like to make a release build, use:

```
make release
```

By default all builds are devel builds and contains symbols
