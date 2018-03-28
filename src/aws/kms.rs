/// The [`AWS::KMS::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="AliasName")]
    pub alias_name: (),
    #[serde(rename="TargetKeyId")]
    pub target_key_id: (),
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
#[derive(Serialize, Deserialize)]
pub struct Key {
    properties: KeyProperties
}

/// Properties for the `Key` resource.
#[derive(Serialize, Deserialize)]
pub struct KeyProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="EnableKeyRotation")]
    pub enable_key_rotation: (),
    #[serde(rename="Enabled")]
    pub enabled: (),
    #[serde(rename="KeyPolicy")]
    pub key_policy: (),
    #[serde(rename="KeyUsage")]
    pub key_usage: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

