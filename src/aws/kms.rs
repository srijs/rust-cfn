//! Types for the `KMS` service.

/// The [`AWS::KMS::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug)]
pub struct AliasProperties {
    /// Property `AliasName`.
    pub alias_name: ::Value<String>,
    /// Property `TargetKeyId`.
    pub target_key_id: ::Value<String>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasName", &self.alias_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetKeyId", &self.target_key_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alias_name = None;
                let mut target_key_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AliasName" => {
                            alias_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetKeyId" => {
                            target_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    alias_name: alias_name.ok_or(::serde::de::Error::missing_field("AliasName"))?,
                    target_key_id: target_key_id.ok_or(::serde::de::Error::missing_field("TargetKeyId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alias {
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
#[derive(Debug)]
pub struct KeyProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EnableKeyRotation`.
    pub enable_key_rotation: Option<::Value<bool>>,
    /// Property `Enabled`.
    pub enabled: Option<::Value<bool>>,
    /// Property `KeyPolicy`.
    pub key_policy: ::Value<::json::Value>,
    /// Property `KeyUsage`.
    pub key_usage: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for KeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableKeyRotation", &self.enable_key_rotation)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPolicy", &self.key_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", &self.key_usage)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut enable_key_rotation = None;
                let mut enabled = None;
                let mut key_policy = None;
                let mut key_usage = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableKeyRotation" => {
                            enable_key_rotation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Enabled" => {
                            enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeyPolicy" => {
                            key_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeyUsage" => {
                            key_usage = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(KeyProperties {
                    description: description,
                    enable_key_rotation: enable_key_rotation,
                    enabled: enabled,
                    key_policy: key_policy.ok_or(::serde::de::Error::missing_field("KeyPolicy"))?,
                    key_usage: key_usage,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Key {
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
