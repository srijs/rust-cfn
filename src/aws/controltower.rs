//! Types for the `ControlTower` service.

/// The [`AWS::ControlTower::EnabledControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html) resource type.
#[derive(Debug, Default)]
pub struct EnabledControl {
    properties: EnabledControlProperties
}

/// Properties for the `EnabledControl` resource.
#[derive(Debug, Default)]
pub struct EnabledControlProperties {
    /// Property [`ControlIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html#cfn-controltower-enabledcontrol-controlidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub control_identifier: ::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html#cfn-controltower-enabledcontrol-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueList<self::enabled_control::EnabledControlParameter>>,
    /// Property [`TargetIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html#cfn-controltower-enabledcontrol-targetidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_identifier: ::Value<String>,
}

impl ::serde::Serialize for EnabledControlProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlIdentifier", &self.control_identifier)?;
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetIdentifier", &self.target_identifier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnabledControlProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnabledControlProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnabledControlProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnabledControlProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut control_identifier: Option<::Value<String>> = None;
                let mut parameters: Option<::ValueList<self::enabled_control::EnabledControlParameter>> = None;
                let mut target_identifier: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ControlIdentifier" => {
                            control_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetIdentifier" => {
                            target_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnabledControlProperties {
                    control_identifier: control_identifier.ok_or(::serde::de::Error::missing_field("ControlIdentifier"))?,
                    parameters: parameters,
                    target_identifier: target_identifier.ok_or(::serde::de::Error::missing_field("TargetIdentifier"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnabledControl {
    type Properties = EnabledControlProperties;
    const TYPE: &'static str = "AWS::ControlTower::EnabledControl";
    fn properties(&self) -> &EnabledControlProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnabledControlProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnabledControl {}

impl From<EnabledControlProperties> for EnabledControl {
    fn from(properties: EnabledControlProperties) -> EnabledControl {
        EnabledControl { properties }
    }
}

/// The [`AWS::ControlTower::LandingZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-landingzone.html) resource type.
#[derive(Debug, Default)]
pub struct LandingZone {
    properties: LandingZoneProperties
}

/// Properties for the `LandingZone` resource.
#[derive(Debug, Default)]
pub struct LandingZoneProperties {
    /// Property [`Manifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-landingzone.html#cfn-controltower-landingzone-manifest).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manifest: ::Value<::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-landingzone.html#cfn-controltower-landingzone-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-landingzone.html#cfn-controltower-landingzone-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: ::Value<String>,
}

impl ::serde::Serialize for LandingZoneProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Manifest", &self.manifest)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LandingZoneProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LandingZoneProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LandingZoneProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LandingZoneProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut manifest: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Manifest" => {
                            manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LandingZoneProperties {
                    manifest: manifest.ok_or(::serde::de::Error::missing_field("Manifest"))?,
                    tags: tags,
                    version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LandingZone {
    type Properties = LandingZoneProperties;
    const TYPE: &'static str = "AWS::ControlTower::LandingZone";
    fn properties(&self) -> &LandingZoneProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LandingZoneProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LandingZone {}

impl From<LandingZoneProperties> for LandingZone {
    fn from(properties: LandingZoneProperties) -> LandingZone {
        LandingZone { properties }
    }
}

pub mod enabled_control {
    //! Property types for the `EnabledControl` resource.

    /// The [`AWS::ControlTower::EnabledControl.EnabledControlParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-controltower-enabledcontrol-enabledcontrolparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct EnabledControlParameter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-controltower-enabledcontrol-enabledcontrolparameter.html#cfn-controltower-enabledcontrol-enabledcontrolparameter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-controltower-enabledcontrol-enabledcontrolparameter.html#cfn-controltower-enabledcontrol-enabledcontrolparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<::json::Value>,
    }

    impl ::codec::SerializeValue for EnabledControlParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnabledControlParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnabledControlParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnabledControlParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnabledControlParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnabledControlParameter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
