/// The [`AWS::KMS::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html) resource.
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="AliasName")]
    pub alias_name: String,
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

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::KMS::Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html) resource.
pub struct Key {
    properties: KeyProperties
}

/// Properties for the `Key` resource.
#[derive(Serialize, Deserialize)]
pub struct KeyProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EnableKeyRotation")]
    pub enable_key_rotation: bool,
    #[serde(rename="Enabled")]
    pub enabled: bool,
    #[serde(rename="KeyPolicy")]
    pub key_policy: ::json::Value,
    #[serde(rename="KeyUsage")]
    pub key_usage: String,
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

impl From<KeyProperties> for Key {
    fn from(properties: KeyProperties) -> Key {
        Key { properties }
    }
}

