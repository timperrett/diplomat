#![allow(non_snake_case)]
#![allow(unused_doc_comment)]

pub mod catalog;
pub mod errors;
pub mod kv;

mod request;

use std::time::Duration;
use reqwest::Client as HttpClient;
use consul::errors::Result;
use consul::errors::ResultExt;

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config: config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub wait_time: Option<Duration>,
}

// use self::request::ResultExt;

impl Config {
    pub fn new() -> Result<Config> {
        HttpClient::new()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| {
                Config {
                    address: String::from("http://localhost:8500"),
                    datacenter: None,
                    http_client: client,
                    wait_time: None,
                }
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
