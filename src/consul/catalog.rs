use std::collections::HashMap;
use super::{Client, QueryMeta};
use super::errors::Result;
use super::request::get;

pub struct CatalogDeregistration {
    pub Node: String,
    pub Datacenter: String,
    pub CheckID: String,
    pub ServiceID: String,
}

pub trait Catalog {
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    //fn deregister(&self, &CatalogDeregistration, &WriteOptions) -> Result<((), WriteMeta)>;
}

impl Catalog for Client {
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
    }
}
