#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]

//! This crate provides type-safe representations for AWS CloudFormation templates,
//! resources and properties.

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;

use serde::Deserialize;

pub mod aws;

pub mod json {
    //! Types for raw JSON values.
    pub use serde_json::{Value, Number};
}

/// Represents an AWS CloudFormation template.
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Resources")]
    resources: Resources
}

impl Template {
    /// Get the template description as a reference.
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get the template description as a mutable reference.
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    /// Get a reference to the resources defined in the template.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Get a mutable reference to the resources defined in the template.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

/// Specifies the stack resources and their properties, such as an Amazon Elastic Compute Cloud instance or an Amazon Simple Storage Service bucket.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Resources(HashMap<String, ResourceInner>);

impl Resources {
    /// Get the resource identified by the logical id, if it exists.
    pub fn get<'a, R: Resource<'a>>(&'a self, name: &str) -> Option<R> {
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

    /// Checks if the resource identified by the logical id exists.
    pub fn has<'a, R: Resource<'a>>(&'a self, name: &str) -> bool {
        self.0.get(name).map(|inner| inner.tag == R::TYPE).unwrap_or(false)
    }

    /// Insert a resource with the provided logical id.
    pub fn set<'a, R: Resource<'a>>(&mut self, name: &str, resource: R) {
        let inner = ResourceInner {
            tag: R::TYPE.to_owned(),
            properties: serde_json::to_value(resource.properties()).unwrap()
        };
        self.0.insert(name.to_owned(), inner);
    }
}

fn empty_object() -> serde_json::Value {
    serde_json::Value::Object(Default::default())
}

#[derive(Debug, Serialize, Deserialize)]
struct ResourceInner {
    #[serde(rename = "Type")]
    tag: String,
    #[serde(rename = "Properties", default = "empty_object")]
    properties: serde_json::Value
}

/// Trait for stack resources, such as an Amazon Elastic Compute Cloud instance or an Amazon Simple Storage Service bucket.
pub trait Resource<'a>: Sized + private::Sealed {
    /// Uniquely identifies the resource type. 
    const TYPE: &'static str;
    /// Type that represents the set of properties the resource can be configured with.
    type Properties: Into<Self> + private::Properties<'a>;

    /// Get a reference to the properties on the resource.
    fn properties(&self) -> &Self::Properties;
    /// Get a mutable reference to the properties on the resource.
    fn properties_mut(&mut self) -> &mut Self::Properties;
}

mod private {
    pub trait Sealed {}

    pub trait Properties<'a>: ::serde::Serialize + ::serde::de::Deserialize<'a> {}
    impl<'a, P> Properties<'a> for P where P: ::serde::Serialize + ::serde::de::Deserialize<'a> {}
}

/// Set of tags (key-value pairs) that can be used to identify and organise resources.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tags(Vec<Tag>);

impl Tags {
    /// Add a key-value pair to the set of tags.
    pub fn add<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.0.push(Tag {
            key: key.into(),
            value: value.into()
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
