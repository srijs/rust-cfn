use indexmap::IndexMap;
use serde::Deserialize;

use ::Resource;

/// Specifies the stack resources and their properties, such as an Amazon Elastic Compute Cloud instance or an Amazon Simple Storage Service bucket.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Resources(IndexMap<String, ResourceInner>);

impl Resources {
    /// Get the resource identified by the logical id, if it exists.
    pub fn get<R: Resource>(&self, name: &str) -> Option<R> {
        self.0.get(name).and_then(|inner| {
            if inner.tag == R::TYPE {
                R::Properties::deserialize(&inner.properties).ok().map(|properties| {
                    properties.into()
                })
            } else {
                None
            }
        })
    }

    /// Checks if a resource with the provided logical id exists.
    pub fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Insert a resource with the provided logical id.
    pub fn set<R: Resource>(&mut self, name: &str, resource: R) {
        let inner = ResourceInner {
            tag: R::TYPE.to_owned(),
            properties: ::serde_json::to_value(resource.properties()).unwrap()
        };
        self.0.insert(name.to_owned(), inner);
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
