#![allow(unused_imports)]
#![allow(dead_code)]

mod api;
mod server;
mod config;

#[cfg(test)]
mod config_test;

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
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  let app = App::new("diplomat")
    .version(VERSION)
    .about("Provides the Envoy v2 API as a gRPC service and CLI application.")
    .author("Timothy Perrett")
    .arg(Arg::with_name("c")
      .long("config")
      .value_name("config")
      .help("Path to the configuration for diplomat")
      .required(false)
      .takes_value(true))
    .subcommand(SubCommand::with_name("sds")
      .about("given a service name, resolve the IPs providing that service"))
      .arg(Arg::with_name("service-name")
        .long("service-name")
        .value_name("service-name")
        .required(true)
        .takes_value(true));

  // TIM: Not sure if cloning here is going to cause problems,
  // but given this is once at the edge of the world it probally isn't
  // too much of a big deal.
  let matches = app.clone().get_matches();

  let config_path: &str = matches.value_of("consul-addr").unwrap_or("diplomat.toml");
  let config = config::load(config_path.to_string());
  let consul = ConsulClient::new("http://127.0.0.1:8500");

  match matches.subcommand() {
    ("eds", Some(_)) => {
        let ips = consul.catalog.get_nodes("consul".to_string()).unwrap();
        println!("{:?}", ips);
    },
    ("serve", Some(_)) => {
        ::server::start();
    },
    _ => {
      let _ = app.clone().print_help();
      println!("");
    }
  }
}
