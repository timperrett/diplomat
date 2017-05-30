extern crate clap;
extern crate consul;

use clap::{Arg, App, SubCommand};
use consul::{Client};

fn main() {
    let app = App::new("diplomat")
        .version("0.1")
        .about("Provides the Envoy v2 API as a gRPC service and CLI application.")
        .author("Timothy Perrett")
        .subcommand(SubCommand::with_name("sds")
                        .about("given a service name, resolve the IPs providing that service"));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("sds", Some(_)) => {

            let client = Client::new("http://127.0.0.1:8500");
            let ips = client.catalog.get_nodes("consul".to_string()).unwrap();
            println!("{:?}", ips);

        }
        _ => {}
    }
}
