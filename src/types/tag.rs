/// Key-value pair that can be used to identify and organise resources.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

impl Tag {
    /// Create a tag from a key-value pair .
    pub fn new<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) -> Tag {
        Tag {
            key: key.into(),
            value: value.into()
        }
    }
}
