//! Types for the `Connect` service.

/// The [`AWS::Connect::ContactFlow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlow {
    properties: ContactFlowProperties
}

/// Properties for the `ContactFlow` resource.
#[derive(Debug, Default)]
pub struct ContactFlowProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
}

impl ::serde::Serialize for ContactFlowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactFlowProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                    r#type: r#type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlow {
    type Properties = ContactFlowProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlow";
    fn properties(&self) -> &ContactFlowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlow {}

impl From<ContactFlowProperties> for ContactFlow {
    fn from(properties: ContactFlowProperties) -> ContactFlow {
        ContactFlow { properties }
    }
}

/// The [`AWS::Connect::ContactFlowModule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlowModule {
    properties: ContactFlowModuleProperties
}

/// Properties for the `ContactFlowModule` resource.
#[derive(Debug, Default)]
pub struct ContactFlowModuleProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ContactFlowModuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowModuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowModuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowModuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowModuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactFlowModuleProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlowModule {
    type Properties = ContactFlowModuleProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlowModule";
    fn properties(&self) -> &ContactFlowModuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowModuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlowModule {}

impl From<ContactFlowModuleProperties> for ContactFlowModule {
    fn from(properties: ContactFlowModuleProperties) -> ContactFlowModule {
        ContactFlowModule { properties }
    }
}

/// The [`AWS::Connect::HoursOfOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html) resource type.
#[derive(Debug, Default)]
pub struct HoursOfOperation {
    properties: HoursOfOperationProperties
}

/// Properties for the `HoursOfOperation` resource.
#[derive(Debug, Default)]
pub struct HoursOfOperationProperties {
    /// Property [`Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config: ::ValueList<self::hours_of_operation::HoursOfOperationConfig>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-timezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_zone: ::Value<String>,
}

impl ::serde::Serialize for HoursOfOperationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Config", &self.config)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", &self.time_zone)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HoursOfOperationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HoursOfOperationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HoursOfOperationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config: Option<::ValueList<self::hours_of_operation::HoursOfOperationConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_zone: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Config" => {
                            config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeZone" => {
                            time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HoursOfOperationProperties {
                    config: config.ok_or(::serde::de::Error::missing_field("Config"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    time_zone: time_zone.ok_or(::serde::de::Error::missing_field("TimeZone"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HoursOfOperation {
    type Properties = HoursOfOperationProperties;
    const TYPE: &'static str = "AWS::Connect::HoursOfOperation";
    fn properties(&self) -> &HoursOfOperationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HoursOfOperationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HoursOfOperation {}

impl From<HoursOfOperationProperties> for HoursOfOperation {
    fn from(properties: HoursOfOperationProperties) -> HoursOfOperation {
        HoursOfOperation { properties }
    }
}

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

/// The [`AWS::Connect::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`DirectoryUserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-directoryuserid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub directory_user_id: Option<::Value<String>>,
    /// Property [`HierarchyGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-hierarchygrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hierarchy_group_arn: Option<::Value<String>>,
    /// Property [`IdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-identityinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_info: Option<::Value<self::user::UserIdentityInfo>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-password).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-phoneconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub phone_config: ::Value<self::user::UserPhoneConfig>,
    /// Property [`RoutingProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-routingprofilearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_profile_arn: ::Value<String>,
    /// Property [`SecurityProfileArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-securityprofilearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_profile_arns: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-username).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref directory_user_id) = self.directory_user_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryUserId", directory_user_id)?;
        }
        if let Some(ref hierarchy_group_arn) = self.hierarchy_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HierarchyGroupArn", hierarchy_group_arn)?;
        }
        if let Some(ref identity_info) = self.identity_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityInfo", identity_info)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", &self.phone_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingProfileArn", &self.routing_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileArns", &self.security_profile_arns)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut directory_user_id: Option<::Value<String>> = None;
                let mut hierarchy_group_arn: Option<::Value<String>> = None;
                let mut identity_info: Option<::Value<self::user::UserIdentityInfo>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut phone_config: Option<::Value<self::user::UserPhoneConfig>> = None;
                let mut routing_profile_arn: Option<::Value<String>> = None;
                let mut security_profile_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DirectoryUserId" => {
                            directory_user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HierarchyGroupArn" => {
                            hierarchy_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityInfo" => {
                            identity_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PhoneConfig" => {
                            phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingProfileArn" => {
                            routing_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileArns" => {
                            security_profile_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    directory_user_id: directory_user_id,
                    hierarchy_group_arn: hierarchy_group_arn,
                    identity_info: identity_info,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    password: password,
                    phone_config: phone_config.ok_or(::serde::de::Error::missing_field("PhoneConfig"))?,
                    routing_profile_arn: routing_profile_arn.ok_or(::serde::de::Error::missing_field("RoutingProfileArn"))?,
                    security_profile_arns: security_profile_arns.ok_or(::serde::de::Error::missing_field("SecurityProfileArns"))?,
                    tags: tags,
                    username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::Connect::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::Connect::UserHierarchyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html) resource type.
#[derive(Debug, Default)]
pub struct UserHierarchyGroup {
    properties: UserHierarchyGroupProperties
}

/// Properties for the `UserHierarchyGroup` resource.
#[derive(Debug, Default)]
pub struct UserHierarchyGroupProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ParentGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-parentgrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_group_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for UserHierarchyGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref parent_group_arn) = self.parent_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentGroupArn", parent_group_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserHierarchyGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserHierarchyGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserHierarchyGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserHierarchyGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parent_group_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentGroupArn" => {
                            parent_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserHierarchyGroupProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parent_group_arn: parent_group_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserHierarchyGroup {
    type Properties = UserHierarchyGroupProperties;
    const TYPE: &'static str = "AWS::Connect::UserHierarchyGroup";
    fn properties(&self) -> &UserHierarchyGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserHierarchyGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserHierarchyGroup {}

impl From<UserHierarchyGroupProperties> for UserHierarchyGroup {
    fn from(properties: UserHierarchyGroupProperties) -> UserHierarchyGroup {
        UserHierarchyGroup { properties }
    }
}

pub mod hours_of_operation {
    //! Property types for the `HoursOfOperation` resource.

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationConfig {
        /// Property [`Day`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-day).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day: ::Value<String>,
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: ::Value<HoursOfOperationTimeSlice>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: ::Value<HoursOfOperationTimeSlice>,
    }

    impl ::codec::SerializeValue for HoursOfOperationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Day", &self.day)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day: Option<::Value<String>> = None;
                    let mut end_time: Option<::Value<HoursOfOperationTimeSlice>> = None;
                    let mut start_time: Option<::Value<HoursOfOperationTimeSlice>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Day" => {
                                day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationConfig {
                        day: day.ok_or(::serde::de::Error::missing_field("Day"))?,
                        end_time: end_time.ok_or(::serde::de::Error::missing_field("EndTime"))?,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationTimeSlice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationTimeSlice {
        /// Property [`Hours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-hours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hours: ::Value<u32>,
        /// Property [`Minutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-minutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for HoursOfOperationTimeSlice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hours", &self.hours)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Minutes", &self.minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationTimeSlice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationTimeSlice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationTimeSlice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationTimeSlice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hours: Option<::Value<u32>> = None;
                    let mut minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hours" => {
                                hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Minutes" => {
                                minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationTimeSlice {
                        hours: hours.ok_or(::serde::de::Error::missing_field("Hours"))?,
                        minutes: minutes.ok_or(::serde::de::Error::missing_field("Minutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
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

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::Connect::User.UserIdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct UserIdentityInfo {
        /// Property [`Email`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-email).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email: Option<::Value<String>>,
        /// Property [`FirstName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-firstname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_name: Option<::Value<String>>,
        /// Property [`LastName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-lastname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserIdentityInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email) = self.email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", email)?;
            }
            if let Some(ref first_name) = self.first_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstName", first_name)?;
            }
            if let Some(ref last_name) = self.last_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastName", last_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserIdentityInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserIdentityInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserIdentityInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserIdentityInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email: Option<::Value<String>> = None;
                    let mut first_name: Option<::Value<String>> = None;
                    let mut last_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Email" => {
                                email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstName" => {
                                first_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastName" => {
                                last_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserIdentityInfo {
                        email: email,
                        first_name: first_name,
                        last_name: last_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::User.UserPhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserPhoneConfig {
        /// Property [`AfterContactWorkTimeLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-aftercontactworktimelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_contact_work_time_limit: Option<::Value<u32>>,
        /// Property [`AutoAccept`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-autoaccept).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_accept: Option<::Value<bool>>,
        /// Property [`DeskPhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-deskphonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desk_phone_number: Option<::Value<String>>,
        /// Property [`PhoneType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-phonetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserPhoneConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_contact_work_time_limit) = self.after_contact_work_time_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterContactWorkTimeLimit", after_contact_work_time_limit)?;
            }
            if let Some(ref auto_accept) = self.auto_accept {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAccept", auto_accept)?;
            }
            if let Some(ref desk_phone_number) = self.desk_phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeskPhoneNumber", desk_phone_number)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneType", &self.phone_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserPhoneConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPhoneConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserPhoneConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserPhoneConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_contact_work_time_limit: Option<::Value<u32>> = None;
                    let mut auto_accept: Option<::Value<bool>> = None;
                    let mut desk_phone_number: Option<::Value<String>> = None;
                    let mut phone_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterContactWorkTimeLimit" => {
                                after_contact_work_time_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoAccept" => {
                                auto_accept = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeskPhoneNumber" => {
                                desk_phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneType" => {
                                phone_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserPhoneConfig {
                        after_contact_work_time_limit: after_contact_work_time_limit,
                        auto_accept: auto_accept,
                        desk_phone_number: desk_phone_number,
                        phone_type: phone_type.ok_or(::serde::de::Error::missing_field("PhoneType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
