#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]

//! This crate provides type-safe representations for AWS CloudFormation templates,
//! resources and properties.

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[macro_use] mod codec;
mod types;
mod parts;
pub mod aws;

pub use types::*;
pub use parts::*;

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
    resources: Resources,
    #[serde(rename = "Outputs", default)]
    outputs: Outputs
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

    /// Get a reference to the outputs defined in the template.
    pub fn outputs(&self) -> &Outputs {
        &self.outputs
    }

    /// Get a mutable reference to the outputs defined in the template.
    pub fn outputs_mut(&mut self) -> &mut Outputs {
        &mut self.outputs
    }
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
