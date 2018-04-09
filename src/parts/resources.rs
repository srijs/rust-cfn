use indexmap::IndexMap;
use serde::Deserialize;

use ::Resource;

/// Specifies the stack resources and their properties, such as an Amazon Elastic Compute Cloud instance or an Amazon Simple Storage Service bucket.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Resources(IndexMap<String, ResourceInner>);

impl Resources {
    /// Get the resource identified by the logical id.
    ///
    /// If the resource does not exist, or has a different type,
    /// an error is returned.
    pub fn get<R: Resource>(&self, id: &str) -> Result<R, ::Error> {
        self.0.get(id)
            .ok_or_else(|| ::Error::new(::ErrorKind::NotFound,
                format_args!("resource with logical id {} not found", id)))
            .and_then(|inner| {
                if inner.tag == R::TYPE {
                    R::Properties::deserialize(&inner.properties)
                        .map_err(|err| ::Error::new(::ErrorKind::Serialization, err))
                        .map(|properties| properties.into())
                } else {
                    Err(::Error::new(::ErrorKind::Serialization,
                        format_args!("resource has type {}, expected {}", inner.tag, R::TYPE)))
                }
            })
    }

    /// Checks if a resource with the provided logical id exists.
    pub fn has(&self, id: &str) -> bool {
        self.0.contains_key(id)
    }

    /// Insert a resource with the provided logical id.
    pub fn set<R: Resource>(&mut self, id: &str, resource: R) {
        let inner = ResourceInner {
            tag: R::TYPE.to_owned(),
            properties: ::serde_json::to_value(resource.properties()).unwrap()
        };
        self.0.insert(id.to_owned(), inner);
    }
}

fn empty_object() -> ::serde_json::Value {
    ::serde_json::Value::Object(Default::default())
}

#[derive(Debug, Serialize, Deserialize)]
struct ResourceInner {
    #[serde(rename = "Type")]
    tag: String,
    #[serde(rename = "Properties", default = "empty_object")]
    properties: ::serde_json::Value
}
