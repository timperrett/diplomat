# Consort

[![Build Status](https://travis-ci.org/timperrett/consort.svg?branch=master)](https://travis-ci.org/timperrett/consort)

Highly experimental gRPC server providing the also experimental [v2 envoy-api](https://github.com/lyft/envoy-api), with information backed by [Consul](https://github.com/hashicorp/consul)

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
