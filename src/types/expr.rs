use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

use super::Value;

#[derive(Debug)]
/// AWS CloudFormation provides several built-in functions that help you manage your stacks.
/// Use intrinsic functions in your templates to assign values to properties that are not available until runtime. 
pub enum Expr {
    /// Append a set of values into a single value, separated by the specified delimiter.
    /// If a delimiter is the empty string, the set of values are concatenated with no delimiter. 
    Join {
        /// The value you want to occur between fragments.
        /// The delimiter will occur between fragments only.
        /// It will not terminate the final value. 
        delimiter: String,
        /// The list of values you want combined.
        values: Vec<Value<String>>
    }
}

#[derive(Serialize)]
enum SerializeExpr<'a> {
    #[serde(rename = "Fn::Join")]
    Join((&'a str, &'a [Value<String>]))
}

impl SerializeValue for Expr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            &Expr::Join { ref delimiter, ref values } =>
                SerializeExpr::Join((delimiter, values))
        };
        proxy.serialize(s)
    }
}

#[derive(Deserialize)]
enum DeserializeExpr {
    #[serde(rename = "Fn::Join")]
    Join((String, Vec<Value<String>>))
}

impl DeserializeValue for Expr {
    fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Expr, D::Error> {
        match Deserialize::deserialize(d)? {
            DeserializeExpr::Join((delimiter, values)) =>
                Ok(Expr::Join { delimiter, values })
        }
    }
}
