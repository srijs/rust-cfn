//! Types for the `KMS` service.

/// The [`AWS::KMS::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AliasProperties {
    /// Property `AliasName`.
    #[serde(rename="AliasName")]
    pub alias_name: String,
    /// Property `TargetKeyId`.
    #[serde(rename="TargetKeyId")]
    pub target_key_id: String,
}

impl<'a> ::Resource<'a> for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::KMS::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::KMS::Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html) resource type.
#[derive(Debug)]
pub struct Key {
    properties: KeyProperties
}

/// Properties for the `Key` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `EnableKeyRotation`.
    #[serde(rename="EnableKeyRotation")]
    pub enable_key_rotation: bool,
    /// Property `Enabled`.
    #[serde(rename="Enabled")]
    pub enabled: bool,
    /// Property `KeyPolicy`.
    #[serde(rename="KeyPolicy")]
    pub key_policy: ::json::Value,
    /// Property `KeyUsage`.
    #[serde(rename="KeyUsage")]
    pub key_usage: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for Key {
    type Properties = KeyProperties;
    const TYPE: &'static str = "AWS::KMS::Key";
    fn properties(&self) -> &KeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Key {}

impl From<KeyProperties> for Key {
    fn from(properties: KeyProperties) -> Key {
        Key { properties }
    }
}
