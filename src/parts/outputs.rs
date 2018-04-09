use indexmap::IndexMap;
use serde::Deserialize;

use ::Value;
use ::codec::{SerializeValue, DeserializeValue};

/// Declares output values that you can import into other stacks (to create cross-stack references),
/// return in response (to describe stack calls), or view on the AWS CloudFormation console.
///
/// For example, you can output the S3 bucket name for a stack to make the bucket easier to find.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Outputs(IndexMap<String, ::serde_json::Value>);

impl Outputs {
    /// Get the output identified by the logical id.
    ///
    /// If the output does not exist, or has a different type,
    /// an error is returned.
    pub fn get<T: DeserializeValue>(&self, id: &str) -> Result<Output<T>, ::Error> {
        self.0.get(id)
            .ok_or_else(|| ::Error::new(::ErrorKind::NotFound,
                format_args!("output with logical id {} not found", id)))
            .and_then(|inner| {
                Output::deserialize(inner)
                    .map_err(|err| ::Error::new(::ErrorKind::Serialization, err))
            })
    }

    /// Checks if the output identified by the logical id exists.
    pub fn has(&self, id: &str) -> bool {
        self.0.contains_key(id)
    }

    /// Insert an output with the provided logical id.
    pub fn set<T: SerializeValue>(&mut self, id: &str, output: Output<T>) {
        let inner = ::serde_json::to_value(output).unwrap();
        self.0.insert(id.to_owned(), inner);
    }
}

/// Output value of a CloudFormation template.
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "T: SerializeValue", deserialize = "T: DeserializeValue"))]
pub struct Output<T> {
    /// The value of the output.
    ///
    /// Can include literals, parameter references, pseudo-parameters, a mapping value,
    /// or intrinsic functions. 
    #[serde(rename = "Value")]
    pub value: Value<T>
}
