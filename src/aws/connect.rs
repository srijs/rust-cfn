//! Types for the `Connect` service.

/// The [`AWS::Connect::QuickConnect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html) resource type.
#[derive(Debug, Default)]
pub struct QuickConnect {
    properties: QuickConnectProperties
}

/// Properties for the `QuickConnect` resource.
#[derive(Debug, Default)]
pub struct QuickConnectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-quickconnectconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quick_connect_config: ::Value<self::quick_connect::QuickConnectConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for QuickConnectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectConfig", &self.quick_connect_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QuickConnectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QuickConnectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QuickConnectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut quick_connect_config: Option<::Value<self::quick_connect::QuickConnectConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QuickConnectConfig" => {
                            quick_connect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QuickConnectProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    quick_connect_config: quick_connect_config.ok_or(::serde::de::Error::missing_field("QuickConnectConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for QuickConnect {
    type Properties = QuickConnectProperties;
    const TYPE: &'static str = "AWS::Connect::QuickConnect";
    fn properties(&self) -> &QuickConnectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QuickConnectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for QuickConnect {}

impl From<QuickConnectProperties> for QuickConnect {
    fn from(properties: QuickConnectProperties) -> QuickConnect {
        QuickConnect { properties }
    }
}

pub mod quick_connect {
    //! Property types for the `QuickConnect` resource.

    /// The [`AWS::Connect::QuickConnect.PhoneNumberQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PhoneNumberQuickConnectConfig {
        /// Property [`PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html#cfn-connect-quickconnect-phonenumberquickconnectconfig-phonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number: ::Value<String>,
    }

    impl ::codec::SerializeValue for PhoneNumberQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumber", &self.phone_number)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhoneNumberQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhoneNumberQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhoneNumberQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhoneNumberQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneNumber" => {
                                phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhoneNumberQuickConnectConfig {
                        phone_number: phone_number.ok_or(::serde::de::Error::missing_field("PhoneNumber"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QueueQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueueQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`QueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-queuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueueQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueArn", &self.queue_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueueQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueueQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueueQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut queue_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueArn" => {
                                queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueueQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        queue_arn: queue_arn.ok_or(::serde::de::Error::missing_field("QueueArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QuickConnectConfig {
        /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-phoneconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_config: Option<::Value<PhoneNumberQuickConnectConfig>>,
        /// Property [`QueueConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-queueconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_config: Option<::Value<QueueQuickConnectConfig>>,
        /// Property [`QuickConnectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-quickconnecttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quick_connect_type: ::Value<String>,
        /// Property [`UserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-userconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_config: Option<::Value<UserQuickConnectConfig>>,
    }

    impl ::codec::SerializeValue for QuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref phone_config) = self.phone_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", phone_config)?;
            }
            if let Some(ref queue_config) = self.queue_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueConfig", queue_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectType", &self.quick_connect_type)?;
            if let Some(ref user_config) = self.user_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserConfig", user_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_config: Option<::Value<PhoneNumberQuickConnectConfig>> = None;
                    let mut queue_config: Option<::Value<QueueQuickConnectConfig>> = None;
                    let mut quick_connect_type: Option<::Value<String>> = None;
                    let mut user_config: Option<::Value<UserQuickConnectConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneConfig" => {
                                phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueConfig" => {
                                queue_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuickConnectType" => {
                                quick_connect_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserConfig" => {
                                user_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QuickConnectConfig {
                        phone_config: phone_config,
                        queue_config: queue_config,
                        quick_connect_type: quick_connect_type.ok_or(::serde::de::Error::missing_field("QuickConnectType"))?,
                        user_config: user_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.UserQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`UserArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-userarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserArn", &self.user_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut user_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserArn" => {
                                user_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        user_arn: user_arn.ok_or(::serde::de::Error::missing_field("UserArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
