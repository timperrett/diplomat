#![allow(unused_imports)]
#![allow(dead_code)]

mod api;
mod server;
mod config;

#[macro_use]
extern crate log;
extern crate clap;
extern crate consul;
extern crate grpcio;
extern crate futures;
extern crate protobuf;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use clap::{Arg, App, SubCommand};
use consul::Client as ConsulClient;

fn main() {
    let app = App::new("diplomat")
        .version("0.1")
        .about("Provides the Envoy v2 API as a gRPC service and CLI application.")
        .author("Timothy Perrett")
        .subcommand(SubCommand::with_name("sds")
            .about("given a service name, resolve the IPs providing that service"));

    let matches = app.get_matches();

    // let eds = api::eds::LbEndpoint::new();

    let consul = ConsulClient::new("http://127.0.0.1:8500");

    match matches.subcommand() {
        ("eds", Some(_)) => {
            let ips = consul.catalog.get_nodes("consul".to_string()).unwrap();
            println!("{:?}", ips);
        },
        ("serve", Some(_)) => {
            ::server::start();
        },
        _ => {}
    }
}
