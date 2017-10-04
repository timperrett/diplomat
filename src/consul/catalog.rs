use std::collections::HashMap;
use super::{Client, QueryMeta, QueryOptions};
use super::errors::Result;
use super::request::{get,get_vec};

pub struct CatalogDeregistration {
    pub Node: String,
    pub Datacenter: String,
    pub CheckID: String,
    pub ServiceID: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CatalogNode {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceAddress: String,
    pub ServicePort: i32,
    pub ServiceEnableTagOverride: bool,
}

pub trait Catalog {
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    fn list_nodes_for(&self, &str, Option<&QueryOptions>) -> Result<(Vec<CatalogNode>, QueryMeta)>;

    //fn deregister(&self, &CatalogDeregistration, &WriteOptions) -> Result<((), WriteMeta)>;
}

impl Catalog for Client {
    /// implements https://www.consul.io/api/catalog.html#list-datacenters
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
    }

    /// implements https://www.consul.io/api/catalog.html#list-nodes-for-service
    fn list_nodes_for(&self, service: &str, q: Option<&QueryOptions>) -> Result<(Vec<CatalogNode>, QueryMeta)> {
        get_vec(
            format!("/v1/catalog/service/{}", service).as_str(),
            &self.config,
            HashMap::new(),
            q,
        )
    }
}
