//! Types for the `Greengrass` service.

/// The [`AWS::Greengrass::ConnectorDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectorDefinition {
    properties: ConnectorDefinitionProperties
}

/// Properties for the `ConnectorDefinition` resource.
#[derive(Debug, Default)]
pub struct ConnectorDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinition.html#cfn-greengrass-connectordefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::connector_definition::ConnectorDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinition.html#cfn-greengrass-connectordefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinition.html#cfn-greengrass-connectordefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ConnectorDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::connector_definition::ConnectorDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectorDefinition {
    type Properties = ConnectorDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::ConnectorDefinition";
    fn properties(&self) -> &ConnectorDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectorDefinition {}

impl From<ConnectorDefinitionProperties> for ConnectorDefinition {
    fn from(properties: ConnectorDefinitionProperties) -> ConnectorDefinition {
        ConnectorDefinition { properties }
    }
}

/// The [`AWS::Greengrass::ConnectorDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectorDefinitionVersion {
    properties: ConnectorDefinitionVersionProperties
}

/// Properties for the `ConnectorDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct ConnectorDefinitionVersionProperties {
    /// Property [`ConnectorDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinitionversion.html#cfn-greengrass-connectordefinitionversion-connectordefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_definition_id: ::Value<String>,
    /// Property [`Connectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinitionversion.html#cfn-greengrass-connectordefinitionversion-connectors).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connectors: ::ValueList<self::connector_definition_version::Connector>,
}

impl ::serde::Serialize for ConnectorDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorDefinitionId", &self.connector_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Connectors", &self.connectors)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connector_definition_id: Option<::Value<String>> = None;
                let mut connectors: Option<::ValueList<self::connector_definition_version::Connector>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectorDefinitionId" => {
                            connector_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Connectors" => {
                            connectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorDefinitionVersionProperties {
                    connector_definition_id: connector_definition_id.ok_or(::serde::de::Error::missing_field("ConnectorDefinitionId"))?,
                    connectors: connectors.ok_or(::serde::de::Error::missing_field("Connectors"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectorDefinitionVersion {
    type Properties = ConnectorDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::ConnectorDefinitionVersion";
    fn properties(&self) -> &ConnectorDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectorDefinitionVersion {}

impl From<ConnectorDefinitionVersionProperties> for ConnectorDefinitionVersion {
    fn from(properties: ConnectorDefinitionVersionProperties) -> ConnectorDefinitionVersion {
        ConnectorDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::CoreDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinition.html) resource type.
#[derive(Debug, Default)]
pub struct CoreDefinition {
    properties: CoreDefinitionProperties
}

/// Properties for the `CoreDefinition` resource.
#[derive(Debug, Default)]
pub struct CoreDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinition.html#cfn-greengrass-coredefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::core_definition::CoreDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinition.html#cfn-greengrass-coredefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinition.html#cfn-greengrass-coredefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for CoreDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CoreDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CoreDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CoreDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::core_definition::CoreDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CoreDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CoreDefinition {
    type Properties = CoreDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::CoreDefinition";
    fn properties(&self) -> &CoreDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CoreDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CoreDefinition {}

impl From<CoreDefinitionProperties> for CoreDefinition {
    fn from(properties: CoreDefinitionProperties) -> CoreDefinition {
        CoreDefinition { properties }
    }
}

/// The [`AWS::Greengrass::CoreDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct CoreDefinitionVersion {
    properties: CoreDefinitionVersionProperties
}

/// Properties for the `CoreDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct CoreDefinitionVersionProperties {
    /// Property [`CoreDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinitionversion.html#cfn-greengrass-coredefinitionversion-coredefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_definition_id: ::Value<String>,
    /// Property [`Cores`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinitionversion.html#cfn-greengrass-coredefinitionversion-cores).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cores: ::ValueList<self::core_definition_version::Core>,
}

impl ::serde::Serialize for CoreDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreDefinitionId", &self.core_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cores", &self.cores)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CoreDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CoreDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CoreDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut core_definition_id: Option<::Value<String>> = None;
                let mut cores: Option<::ValueList<self::core_definition_version::Core>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CoreDefinitionId" => {
                            core_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cores" => {
                            cores = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CoreDefinitionVersionProperties {
                    core_definition_id: core_definition_id.ok_or(::serde::de::Error::missing_field("CoreDefinitionId"))?,
                    cores: cores.ok_or(::serde::de::Error::missing_field("Cores"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CoreDefinitionVersion {
    type Properties = CoreDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::CoreDefinitionVersion";
    fn properties(&self) -> &CoreDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CoreDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CoreDefinitionVersion {}

impl From<CoreDefinitionVersionProperties> for CoreDefinitionVersion {
    fn from(properties: CoreDefinitionVersionProperties) -> CoreDefinitionVersion {
        CoreDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::DeviceDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinition.html) resource type.
#[derive(Debug, Default)]
pub struct DeviceDefinition {
    properties: DeviceDefinitionProperties
}

/// Properties for the `DeviceDefinition` resource.
#[derive(Debug, Default)]
pub struct DeviceDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinition.html#cfn-greengrass-devicedefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::device_definition::DeviceDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinition.html#cfn-greengrass-devicedefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinition.html#cfn-greengrass-devicedefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for DeviceDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::device_definition::DeviceDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeviceDefinition {
    type Properties = DeviceDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::DeviceDefinition";
    fn properties(&self) -> &DeviceDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeviceDefinition {}

impl From<DeviceDefinitionProperties> for DeviceDefinition {
    fn from(properties: DeviceDefinitionProperties) -> DeviceDefinition {
        DeviceDefinition { properties }
    }
}

/// The [`AWS::Greengrass::DeviceDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct DeviceDefinitionVersion {
    properties: DeviceDefinitionVersionProperties
}

/// Properties for the `DeviceDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct DeviceDefinitionVersionProperties {
    /// Property [`DeviceDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinitionversion.html#cfn-greengrass-devicedefinitionversion-devicedefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_definition_id: ::Value<String>,
    /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinitionversion.html#cfn-greengrass-devicedefinitionversion-devices).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub devices: ::ValueList<self::device_definition_version::Device>,
}

impl ::serde::Serialize for DeviceDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceDefinitionId", &self.device_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", &self.devices)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut device_definition_id: Option<::Value<String>> = None;
                let mut devices: Option<::ValueList<self::device_definition_version::Device>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeviceDefinitionId" => {
                            device_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Devices" => {
                            devices = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceDefinitionVersionProperties {
                    device_definition_id: device_definition_id.ok_or(::serde::de::Error::missing_field("DeviceDefinitionId"))?,
                    devices: devices.ok_or(::serde::de::Error::missing_field("Devices"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeviceDefinitionVersion {
    type Properties = DeviceDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::DeviceDefinitionVersion";
    fn properties(&self) -> &DeviceDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeviceDefinitionVersion {}

impl From<DeviceDefinitionVersionProperties> for DeviceDefinitionVersion {
    fn from(properties: DeviceDefinitionVersionProperties) -> DeviceDefinitionVersion {
        DeviceDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::FunctionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinition.html) resource type.
#[derive(Debug, Default)]
pub struct FunctionDefinition {
    properties: FunctionDefinitionProperties
}

/// Properties for the `FunctionDefinition` resource.
#[derive(Debug, Default)]
pub struct FunctionDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinition.html#cfn-greengrass-functiondefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::function_definition::FunctionDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinition.html#cfn-greengrass-functiondefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinition.html#cfn-greengrass-functiondefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for FunctionDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::function_definition::FunctionDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FunctionDefinition {
    type Properties = FunctionDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::FunctionDefinition";
    fn properties(&self) -> &FunctionDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FunctionDefinition {}

impl From<FunctionDefinitionProperties> for FunctionDefinition {
    fn from(properties: FunctionDefinitionProperties) -> FunctionDefinition {
        FunctionDefinition { properties }
    }
}

/// The [`AWS::Greengrass::FunctionDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct FunctionDefinitionVersion {
    properties: FunctionDefinitionVersionProperties
}

/// Properties for the `FunctionDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct FunctionDefinitionVersionProperties {
    /// Property [`DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinitionversion.html#cfn-greengrass-functiondefinitionversion-defaultconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub default_config: Option<::Value<self::function_definition_version::DefaultConfig>>,
    /// Property [`FunctionDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinitionversion.html#cfn-greengrass-functiondefinitionversion-functiondefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_definition_id: ::Value<String>,
    /// Property [`Functions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinitionversion.html#cfn-greengrass-functiondefinitionversion-functions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub functions: ::ValueList<self::function_definition_version::Function>,
}

impl ::serde::Serialize for FunctionDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref default_config) = self.default_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultConfig", default_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionDefinitionId", &self.function_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Functions", &self.functions)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut default_config: Option<::Value<self::function_definition_version::DefaultConfig>> = None;
                let mut function_definition_id: Option<::Value<String>> = None;
                let mut functions: Option<::ValueList<self::function_definition_version::Function>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultConfig" => {
                            default_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionDefinitionId" => {
                            function_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Functions" => {
                            functions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionDefinitionVersionProperties {
                    default_config: default_config,
                    function_definition_id: function_definition_id.ok_or(::serde::de::Error::missing_field("FunctionDefinitionId"))?,
                    functions: functions.ok_or(::serde::de::Error::missing_field("Functions"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FunctionDefinitionVersion {
    type Properties = FunctionDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::FunctionDefinitionVersion";
    fn properties(&self) -> &FunctionDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FunctionDefinitionVersion {}

impl From<FunctionDefinitionVersionProperties> for FunctionDefinitionVersion {
    fn from(properties: FunctionDefinitionVersionProperties) -> FunctionDefinitionVersion {
        FunctionDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html) resource type.
#[derive(Debug, Default)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug, Default)]
pub struct GroupProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html#cfn-greengrass-group-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::group::GroupVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html#cfn-greengrass-group-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html#cfn-greengrass-group-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html#cfn-greengrass-group-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for GroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::group::GroupVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Group {
    type Properties = GroupProperties;
    const TYPE: &'static str = "AWS::Greengrass::Group";
    fn properties(&self) -> &GroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Group {}

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::Greengrass::GroupVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html) resource type.
#[derive(Debug, Default)]
pub struct GroupVersion {
    properties: GroupVersionProperties
}

/// Properties for the `GroupVersion` resource.
#[derive(Debug, Default)]
pub struct GroupVersionProperties {
    /// Property [`ConnectorDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-connectordefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_definition_version_arn: Option<::Value<String>>,
    /// Property [`CoreDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-coredefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_definition_version_arn: Option<::Value<String>>,
    /// Property [`DeviceDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-devicedefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_definition_version_arn: Option<::Value<String>>,
    /// Property [`FunctionDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-functiondefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_definition_version_arn: Option<::Value<String>>,
    /// Property [`GroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-groupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_id: ::Value<String>,
    /// Property [`LoggerDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-loggerdefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub logger_definition_version_arn: Option<::Value<String>>,
    /// Property [`ResourceDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-resourcedefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_definition_version_arn: Option<::Value<String>>,
    /// Property [`SubscriptionDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html#cfn-greengrass-groupversion-subscriptiondefinitionversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscription_definition_version_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for GroupVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref connector_definition_version_arn) = self.connector_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorDefinitionVersionArn", connector_definition_version_arn)?;
        }
        if let Some(ref core_definition_version_arn) = self.core_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreDefinitionVersionArn", core_definition_version_arn)?;
        }
        if let Some(ref device_definition_version_arn) = self.device_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceDefinitionVersionArn", device_definition_version_arn)?;
        }
        if let Some(ref function_definition_version_arn) = self.function_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionDefinitionVersionArn", function_definition_version_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
        if let Some(ref logger_definition_version_arn) = self.logger_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggerDefinitionVersionArn", logger_definition_version_arn)?;
        }
        if let Some(ref resource_definition_version_arn) = self.resource_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDefinitionVersionArn", resource_definition_version_arn)?;
        }
        if let Some(ref subscription_definition_version_arn) = self.subscription_definition_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionDefinitionVersionArn", subscription_definition_version_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connector_definition_version_arn: Option<::Value<String>> = None;
                let mut core_definition_version_arn: Option<::Value<String>> = None;
                let mut device_definition_version_arn: Option<::Value<String>> = None;
                let mut function_definition_version_arn: Option<::Value<String>> = None;
                let mut group_id: Option<::Value<String>> = None;
                let mut logger_definition_version_arn: Option<::Value<String>> = None;
                let mut resource_definition_version_arn: Option<::Value<String>> = None;
                let mut subscription_definition_version_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectorDefinitionVersionArn" => {
                            connector_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CoreDefinitionVersionArn" => {
                            core_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceDefinitionVersionArn" => {
                            device_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionDefinitionVersionArn" => {
                            function_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupId" => {
                            group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggerDefinitionVersionArn" => {
                            logger_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceDefinitionVersionArn" => {
                            resource_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubscriptionDefinitionVersionArn" => {
                            subscription_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupVersionProperties {
                    connector_definition_version_arn: connector_definition_version_arn,
                    core_definition_version_arn: core_definition_version_arn,
                    device_definition_version_arn: device_definition_version_arn,
                    function_definition_version_arn: function_definition_version_arn,
                    group_id: group_id.ok_or(::serde::de::Error::missing_field("GroupId"))?,
                    logger_definition_version_arn: logger_definition_version_arn,
                    resource_definition_version_arn: resource_definition_version_arn,
                    subscription_definition_version_arn: subscription_definition_version_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GroupVersion {
    type Properties = GroupVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::GroupVersion";
    fn properties(&self) -> &GroupVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GroupVersion {}

impl From<GroupVersionProperties> for GroupVersion {
    fn from(properties: GroupVersionProperties) -> GroupVersion {
        GroupVersion { properties }
    }
}

/// The [`AWS::Greengrass::LoggerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct LoggerDefinition {
    properties: LoggerDefinitionProperties
}

/// Properties for the `LoggerDefinition` resource.
#[derive(Debug, Default)]
pub struct LoggerDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinition.html#cfn-greengrass-loggerdefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::logger_definition::LoggerDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinition.html#cfn-greengrass-loggerdefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinition.html#cfn-greengrass-loggerdefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for LoggerDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoggerDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggerDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoggerDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoggerDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::logger_definition::LoggerDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoggerDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoggerDefinition {
    type Properties = LoggerDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::LoggerDefinition";
    fn properties(&self) -> &LoggerDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoggerDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoggerDefinition {}

impl From<LoggerDefinitionProperties> for LoggerDefinition {
    fn from(properties: LoggerDefinitionProperties) -> LoggerDefinition {
        LoggerDefinition { properties }
    }
}

/// The [`AWS::Greengrass::LoggerDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct LoggerDefinitionVersion {
    properties: LoggerDefinitionVersionProperties
}

/// Properties for the `LoggerDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct LoggerDefinitionVersionProperties {
    /// Property [`LoggerDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinitionversion.html#cfn-greengrass-loggerdefinitionversion-loggerdefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub logger_definition_id: ::Value<String>,
    /// Property [`Loggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinitionversion.html#cfn-greengrass-loggerdefinitionversion-loggers).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub loggers: ::ValueList<self::logger_definition_version::Logger>,
}

impl ::serde::Serialize for LoggerDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggerDefinitionId", &self.logger_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Loggers", &self.loggers)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoggerDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggerDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoggerDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoggerDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut logger_definition_id: Option<::Value<String>> = None;
                let mut loggers: Option<::ValueList<self::logger_definition_version::Logger>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LoggerDefinitionId" => {
                            logger_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Loggers" => {
                            loggers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoggerDefinitionVersionProperties {
                    logger_definition_id: logger_definition_id.ok_or(::serde::de::Error::missing_field("LoggerDefinitionId"))?,
                    loggers: loggers.ok_or(::serde::de::Error::missing_field("Loggers"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoggerDefinitionVersion {
    type Properties = LoggerDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::LoggerDefinitionVersion";
    fn properties(&self) -> &LoggerDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoggerDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoggerDefinitionVersion {}

impl From<LoggerDefinitionVersionProperties> for LoggerDefinitionVersion {
    fn from(properties: LoggerDefinitionVersionProperties) -> LoggerDefinitionVersion {
        LoggerDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::ResourceDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceDefinition {
    properties: ResourceDefinitionProperties
}

/// Properties for the `ResourceDefinition` resource.
#[derive(Debug, Default)]
pub struct ResourceDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinition.html#cfn-greengrass-resourcedefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::resource_definition::ResourceDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinition.html#cfn-greengrass-resourcedefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinition.html#cfn-greengrass-resourcedefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ResourceDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::resource_definition::ResourceDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceDefinition {
    type Properties = ResourceDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::ResourceDefinition";
    fn properties(&self) -> &ResourceDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceDefinition {}

impl From<ResourceDefinitionProperties> for ResourceDefinition {
    fn from(properties: ResourceDefinitionProperties) -> ResourceDefinition {
        ResourceDefinition { properties }
    }
}

/// The [`AWS::Greengrass::ResourceDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceDefinitionVersion {
    properties: ResourceDefinitionVersionProperties
}

/// Properties for the `ResourceDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct ResourceDefinitionVersionProperties {
    /// Property [`ResourceDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinitionversion.html#cfn-greengrass-resourcedefinitionversion-resourcedefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_definition_id: ::Value<String>,
    /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinitionversion.html#cfn-greengrass-resourcedefinitionversion-resources).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resources: ::ValueList<self::resource_definition_version::ResourceInstance>,
}

impl ::serde::Serialize for ResourceDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDefinitionId", &self.resource_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", &self.resources)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_definition_id: Option<::Value<String>> = None;
                let mut resources: Option<::ValueList<self::resource_definition_version::ResourceInstance>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceDefinitionId" => {
                            resource_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resources" => {
                            resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceDefinitionVersionProperties {
                    resource_definition_id: resource_definition_id.ok_or(::serde::de::Error::missing_field("ResourceDefinitionId"))?,
                    resources: resources.ok_or(::serde::de::Error::missing_field("Resources"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceDefinitionVersion {
    type Properties = ResourceDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::ResourceDefinitionVersion";
    fn properties(&self) -> &ResourceDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceDefinitionVersion {}

impl From<ResourceDefinitionVersionProperties> for ResourceDefinitionVersion {
    fn from(properties: ResourceDefinitionVersionProperties) -> ResourceDefinitionVersion {
        ResourceDefinitionVersion { properties }
    }
}

/// The [`AWS::Greengrass::SubscriptionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinition.html) resource type.
#[derive(Debug, Default)]
pub struct SubscriptionDefinition {
    properties: SubscriptionDefinitionProperties
}

/// Properties for the `SubscriptionDefinition` resource.
#[derive(Debug, Default)]
pub struct SubscriptionDefinitionProperties {
    /// Property [`InitialVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinition.html#cfn-greengrass-subscriptiondefinition-initialversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub initial_version: Option<::Value<self::subscription_definition::SubscriptionDefinitionVersion>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinition.html#cfn-greengrass-subscriptiondefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinition.html#cfn-greengrass-subscriptiondefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for SubscriptionDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref initial_version) = self.initial_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVersion", initial_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubscriptionDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubscriptionDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_version: Option<::Value<self::subscription_definition::SubscriptionDefinitionVersion>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InitialVersion" => {
                            initial_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubscriptionDefinitionProperties {
                    initial_version: initial_version,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubscriptionDefinition {
    type Properties = SubscriptionDefinitionProperties;
    const TYPE: &'static str = "AWS::Greengrass::SubscriptionDefinition";
    fn properties(&self) -> &SubscriptionDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubscriptionDefinition {}

impl From<SubscriptionDefinitionProperties> for SubscriptionDefinition {
    fn from(properties: SubscriptionDefinitionProperties) -> SubscriptionDefinition {
        SubscriptionDefinition { properties }
    }
}

/// The [`AWS::Greengrass::SubscriptionDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinitionversion.html) resource type.
#[derive(Debug, Default)]
pub struct SubscriptionDefinitionVersion {
    properties: SubscriptionDefinitionVersionProperties
}

/// Properties for the `SubscriptionDefinitionVersion` resource.
#[derive(Debug, Default)]
pub struct SubscriptionDefinitionVersionProperties {
    /// Property [`SubscriptionDefinitionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinitionversion.html#cfn-greengrass-subscriptiondefinitionversion-subscriptiondefinitionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscription_definition_id: ::Value<String>,
    /// Property [`Subscriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinitionversion.html#cfn-greengrass-subscriptiondefinitionversion-subscriptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscriptions: ::ValueList<self::subscription_definition_version::Subscription>,
}

impl ::serde::Serialize for SubscriptionDefinitionVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionDefinitionId", &self.subscription_definition_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscriptions", &self.subscriptions)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubscriptionDefinitionVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionDefinitionVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionDefinitionVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubscriptionDefinitionVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut subscription_definition_id: Option<::Value<String>> = None;
                let mut subscriptions: Option<::ValueList<self::subscription_definition_version::Subscription>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SubscriptionDefinitionId" => {
                            subscription_definition_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subscriptions" => {
                            subscriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubscriptionDefinitionVersionProperties {
                    subscription_definition_id: subscription_definition_id.ok_or(::serde::de::Error::missing_field("SubscriptionDefinitionId"))?,
                    subscriptions: subscriptions.ok_or(::serde::de::Error::missing_field("Subscriptions"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubscriptionDefinitionVersion {
    type Properties = SubscriptionDefinitionVersionProperties;
    const TYPE: &'static str = "AWS::Greengrass::SubscriptionDefinitionVersion";
    fn properties(&self) -> &SubscriptionDefinitionVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionDefinitionVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubscriptionDefinitionVersion {}

impl From<SubscriptionDefinitionVersionProperties> for SubscriptionDefinitionVersion {
    fn from(properties: SubscriptionDefinitionVersionProperties) -> SubscriptionDefinitionVersion {
        SubscriptionDefinitionVersion { properties }
    }
}

pub mod connector_definition {
    //! Property types for the `ConnectorDefinition` resource.

    /// The [`AWS::Greengrass::ConnectorDefinition.Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connector.html) property type.
    #[derive(Debug, Default)]
    pub struct Connector {
        /// Property [`ConnectorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connector.html#cfn-greengrass-connectordefinition-connector-connectorarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub connector_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connector.html#cfn-greengrass-connectordefinition-connector-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connector.html#cfn-greengrass-connectordefinition-connector-parameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for Connector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorArn", &self.connector_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Connector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Connector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Connector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Connector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorArn" => {
                                connector_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Connector {
                        connector_arn: connector_arn.ok_or(::serde::de::Error::missing_field("ConnectorArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ConnectorDefinition.ConnectorDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connectordefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorDefinitionVersion {
        /// Property [`Connectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connectordefinitionversion.html#cfn-greengrass-connectordefinition-connectordefinitionversion-connectors).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub connectors: ::ValueList<Connector>,
    }

    impl ::codec::SerializeValue for ConnectorDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Connectors", &self.connectors)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connectors: Option<::ValueList<Connector>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Connectors" => {
                                connectors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorDefinitionVersion {
                        connectors: connectors.ok_or(::serde::de::Error::missing_field("Connectors"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod connector_definition_version {
    //! Property types for the `ConnectorDefinitionVersion` resource.

    /// The [`AWS::Greengrass::ConnectorDefinitionVersion.Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinitionversion-connector.html) property type.
    #[derive(Debug, Default)]
    pub struct Connector {
        /// Property [`ConnectorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinitionversion-connector.html#cfn-greengrass-connectordefinitionversion-connector-connectorarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub connector_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinitionversion-connector.html#cfn-greengrass-connectordefinitionversion-connector-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinitionversion-connector.html#cfn-greengrass-connectordefinitionversion-connector-parameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for Connector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorArn", &self.connector_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Connector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Connector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Connector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Connector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorArn" => {
                                connector_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Connector {
                        connector_arn: connector_arn.ok_or(::serde::de::Error::missing_field("ConnectorArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod core_definition {
    //! Property types for the `CoreDefinition` resource.

    /// The [`AWS::Greengrass::CoreDefinition.Core`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html) property type.
    #[derive(Debug, Default)]
    pub struct Core {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html#cfn-greengrass-coredefinition-core-certificatearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html#cfn-greengrass-coredefinition-core-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`SyncShadow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html#cfn-greengrass-coredefinition-core-syncshadow).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sync_shadow: Option<::Value<bool>>,
        /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html#cfn-greengrass-coredefinition-core-thingarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub thing_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Core {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref sync_shadow) = self.sync_shadow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncShadow", sync_shadow)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", &self.thing_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Core {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Core, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Core;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Core")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut sync_shadow: Option<::Value<bool>> = None;
                    let mut thing_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncShadow" => {
                                sync_shadow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingArn" => {
                                thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Core {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        sync_shadow: sync_shadow,
                        thing_arn: thing_arn.ok_or(::serde::de::Error::missing_field("ThingArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::CoreDefinition.CoreDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-coredefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct CoreDefinitionVersion {
        /// Property [`Cores`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-coredefinitionversion.html#cfn-greengrass-coredefinition-coredefinitionversion-cores).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cores: ::ValueList<Core>,
    }

    impl ::codec::SerializeValue for CoreDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cores", &self.cores)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CoreDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CoreDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CoreDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cores: Option<::ValueList<Core>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cores" => {
                                cores = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CoreDefinitionVersion {
                        cores: cores.ok_or(::serde::de::Error::missing_field("Cores"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod core_definition_version {
    //! Property types for the `CoreDefinitionVersion` resource.

    /// The [`AWS::Greengrass::CoreDefinitionVersion.Core`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html) property type.
    #[derive(Debug, Default)]
    pub struct Core {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html#cfn-greengrass-coredefinitionversion-core-certificatearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html#cfn-greengrass-coredefinitionversion-core-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`SyncShadow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html#cfn-greengrass-coredefinitionversion-core-syncshadow).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sync_shadow: Option<::Value<bool>>,
        /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html#cfn-greengrass-coredefinitionversion-core-thingarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub thing_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Core {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref sync_shadow) = self.sync_shadow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncShadow", sync_shadow)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", &self.thing_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Core {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Core, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Core;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Core")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut sync_shadow: Option<::Value<bool>> = None;
                    let mut thing_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncShadow" => {
                                sync_shadow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingArn" => {
                                thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Core {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        sync_shadow: sync_shadow,
                        thing_arn: thing_arn.ok_or(::serde::de::Error::missing_field("ThingArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod device_definition {
    //! Property types for the `DeviceDefinition` resource.

    /// The [`AWS::Greengrass::DeviceDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html) property type.
    #[derive(Debug, Default)]
    pub struct Device {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html#cfn-greengrass-devicedefinition-device-certificatearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html#cfn-greengrass-devicedefinition-device-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`SyncShadow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html#cfn-greengrass-devicedefinition-device-syncshadow).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sync_shadow: Option<::Value<bool>>,
        /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html#cfn-greengrass-devicedefinition-device-thingarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub thing_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref sync_shadow) = self.sync_shadow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncShadow", sync_shadow)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", &self.thing_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Device {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Device, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Device;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Device")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut sync_shadow: Option<::Value<bool>> = None;
                    let mut thing_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncShadow" => {
                                sync_shadow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingArn" => {
                                thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        sync_shadow: sync_shadow,
                        thing_arn: thing_arn.ok_or(::serde::de::Error::missing_field("ThingArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::DeviceDefinition.DeviceDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-devicedefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceDefinitionVersion {
        /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-devicedefinitionversion.html#cfn-greengrass-devicedefinition-devicedefinitionversion-devices).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub devices: ::ValueList<Device>,
    }

    impl ::codec::SerializeValue for DeviceDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", &self.devices)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut devices: Option<::ValueList<Device>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Devices" => {
                                devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceDefinitionVersion {
                        devices: devices.ok_or(::serde::de::Error::missing_field("Devices"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod device_definition_version {
    //! Property types for the `DeviceDefinitionVersion` resource.

    /// The [`AWS::Greengrass::DeviceDefinitionVersion.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html) property type.
    #[derive(Debug, Default)]
    pub struct Device {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html#cfn-greengrass-devicedefinitionversion-device-certificatearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_arn: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html#cfn-greengrass-devicedefinitionversion-device-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`SyncShadow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html#cfn-greengrass-devicedefinitionversion-device-syncshadow).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sync_shadow: Option<::Value<bool>>,
        /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html#cfn-greengrass-devicedefinitionversion-device-thingarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub thing_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref sync_shadow) = self.sync_shadow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncShadow", sync_shadow)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", &self.thing_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Device {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Device, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Device;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Device")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut sync_shadow: Option<::Value<bool>> = None;
                    let mut thing_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncShadow" => {
                                sync_shadow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingArn" => {
                                thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        sync_shadow: sync_shadow,
                        thing_arn: thing_arn.ok_or(::serde::de::Error::missing_field("ThingArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function_definition {
    //! Property types for the `FunctionDefinition` resource.

    /// The [`AWS::Greengrass::FunctionDefinition.DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-defaultconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultConfig {
        /// Property [`Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-defaultconfig.html#cfn-greengrass-functiondefinition-defaultconfig-execution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution: ::Value<Execution>,
    }

    impl ::codec::SerializeValue for DefaultConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Execution", &self.execution)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution: Option<::Value<Execution>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Execution" => {
                                execution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultConfig {
                        execution: execution.ok_or(::serde::de::Error::missing_field("Execution"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
        /// Property [`AccessSysfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html#cfn-greengrass-functiondefinition-environment-accesssysfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_sysfs: Option<::Value<bool>>,
        /// Property [`Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html#cfn-greengrass-functiondefinition-environment-execution).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub execution: Option<::Value<Execution>>,
        /// Property [`ResourceAccessPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html#cfn-greengrass-functiondefinition-environment-resourceaccesspolicies).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_access_policies: Option<::ValueList<ResourceAccessPolicy>>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html#cfn-greengrass-functiondefinition-environment-variables).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub variables: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_sysfs) = self.access_sysfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessSysfs", access_sysfs)?;
            }
            if let Some(ref execution) = self.execution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Execution", execution)?;
            }
            if let Some(ref resource_access_policies) = self.resource_access_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceAccessPolicies", resource_access_policies)?;
            }
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_sysfs: Option<::Value<bool>> = None;
                    let mut execution: Option<::Value<Execution>> = None;
                    let mut resource_access_policies: Option<::ValueList<ResourceAccessPolicy>> = None;
                    let mut variables: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessSysfs" => {
                                access_sysfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Execution" => {
                                execution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceAccessPolicies" => {
                                resource_access_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        access_sysfs: access_sysfs,
                        execution: execution,
                        resource_access_policies: resource_access_policies,
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-execution.html) property type.
    #[derive(Debug, Default)]
    pub struct Execution {
        /// Property [`IsolationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-execution.html#cfn-greengrass-functiondefinition-execution-isolationmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub isolation_mode: Option<::Value<String>>,
        /// Property [`RunAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-execution.html#cfn-greengrass-functiondefinition-execution-runas).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub run_as: Option<::Value<RunAs>>,
    }

    impl ::codec::SerializeValue for Execution {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref isolation_mode) = self.isolation_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsolationMode", isolation_mode)?;
            }
            if let Some(ref run_as) = self.run_as {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunAs", run_as)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Execution {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Execution, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Execution;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Execution")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut isolation_mode: Option<::Value<String>> = None;
                    let mut run_as: Option<::Value<RunAs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsolationMode" => {
                                isolation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunAs" => {
                                run_as = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Execution {
                        isolation_mode: isolation_mode,
                        run_as: run_as,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-function.html) property type.
    #[derive(Debug, Default)]
    pub struct Function {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-function.html#cfn-greengrass-functiondefinition-function-functionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub function_arn: ::Value<String>,
        /// Property [`FunctionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-function.html#cfn-greengrass-functiondefinition-function-functionconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub function_configuration: ::Value<FunctionConfiguration>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-function.html#cfn-greengrass-functiondefinition-function-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
    }

    impl ::codec::SerializeValue for Function {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", &self.function_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionConfiguration", &self.function_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Function {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Function, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Function;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Function")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<::Value<String>> = None;
                    let mut function_configuration: Option<::Value<FunctionConfiguration>> = None;
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionConfiguration" => {
                                function_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Function {
                        function_arn: function_arn.ok_or(::serde::de::Error::missing_field("FunctionArn"))?,
                        function_configuration: function_configuration.ok_or(::serde::de::Error::missing_field("FunctionConfiguration"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.FunctionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionConfiguration {
        /// Property [`EncodingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-encodingtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encoding_type: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::Value<Environment>>,
        /// Property [`ExecArgs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-execargs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub exec_args: Option<::Value<String>>,
        /// Property [`Executable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-executable).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub executable: Option<::Value<String>>,
        /// Property [`MemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-memorysize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory_size: Option<::Value<u32>>,
        /// Property [`Pinned`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-pinned).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pinned: Option<::Value<bool>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html#cfn-greengrass-functiondefinition-functionconfiguration-timeout).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for FunctionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encoding_type) = self.encoding_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncodingType", encoding_type)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref exec_args) = self.exec_args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecArgs", exec_args)?;
            }
            if let Some(ref executable) = self.executable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Executable", executable)?;
            }
            if let Some(ref memory_size) = self.memory_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", memory_size)?;
            }
            if let Some(ref pinned) = self.pinned {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pinned", pinned)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encoding_type: Option<::Value<String>> = None;
                    let mut environment: Option<::Value<Environment>> = None;
                    let mut exec_args: Option<::Value<String>> = None;
                    let mut executable: Option<::Value<String>> = None;
                    let mut memory_size: Option<::Value<u32>> = None;
                    let mut pinned: Option<::Value<bool>> = None;
                    let mut timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncodingType" => {
                                encoding_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecArgs" => {
                                exec_args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Executable" => {
                                executable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemorySize" => {
                                memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pinned" => {
                                pinned = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionConfiguration {
                        encoding_type: encoding_type,
                        environment: environment,
                        exec_args: exec_args,
                        executable: executable,
                        memory_size: memory_size,
                        pinned: pinned,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.FunctionDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functiondefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionDefinitionVersion {
        /// Property [`DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functiondefinitionversion.html#cfn-greengrass-functiondefinition-functiondefinitionversion-defaultconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub default_config: Option<::Value<DefaultConfig>>,
        /// Property [`Functions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functiondefinitionversion.html#cfn-greengrass-functiondefinition-functiondefinitionversion-functions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub functions: ::ValueList<Function>,
    }

    impl ::codec::SerializeValue for FunctionDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_config) = self.default_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultConfig", default_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Functions", &self.functions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_config: Option<::Value<DefaultConfig>> = None;
                    let mut functions: Option<::ValueList<Function>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultConfig" => {
                                default_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Functions" => {
                                functions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionDefinitionVersion {
                        default_config: default_config,
                        functions: functions.ok_or(::serde::de::Error::missing_field("Functions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.ResourceAccessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-resourceaccesspolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceAccessPolicy {
        /// Property [`Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-resourceaccesspolicy.html#cfn-greengrass-functiondefinition-resourceaccesspolicy-permission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permission: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-resourceaccesspolicy.html#cfn-greengrass-functiondefinition-resourceaccesspolicy-resourceid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceAccessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref permission) = self.permission {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", permission)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceAccessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceAccessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceAccessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceAccessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut permission: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceAccessPolicy {
                        permission: permission,
                        resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinition.RunAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-runas.html) property type.
    #[derive(Debug, Default)]
    pub struct RunAs {
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-runas.html#cfn-greengrass-functiondefinition-runas-gid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub gid: Option<::Value<u32>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-runas.html#cfn-greengrass-functiondefinition-runas-uid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub uid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RunAs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref gid) = self.gid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", gid)?;
            }
            if let Some(ref uid) = self.uid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", uid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunAs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunAs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunAs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunAs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gid: Option<::Value<u32>> = None;
                    let mut uid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunAs {
                        gid: gid,
                        uid: uid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function_definition_version {
    //! Property types for the `FunctionDefinitionVersion` resource.

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-defaultconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultConfig {
        /// Property [`Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-defaultconfig.html#cfn-greengrass-functiondefinitionversion-defaultconfig-execution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution: ::Value<Execution>,
    }

    impl ::codec::SerializeValue for DefaultConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Execution", &self.execution)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution: Option<::Value<Execution>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Execution" => {
                                execution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultConfig {
                        execution: execution.ok_or(::serde::de::Error::missing_field("Execution"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
        /// Property [`AccessSysfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html#cfn-greengrass-functiondefinitionversion-environment-accesssysfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_sysfs: Option<::Value<bool>>,
        /// Property [`Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html#cfn-greengrass-functiondefinitionversion-environment-execution).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub execution: Option<::Value<Execution>>,
        /// Property [`ResourceAccessPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html#cfn-greengrass-functiondefinitionversion-environment-resourceaccesspolicies).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_access_policies: Option<::ValueList<ResourceAccessPolicy>>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html#cfn-greengrass-functiondefinitionversion-environment-variables).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub variables: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_sysfs) = self.access_sysfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessSysfs", access_sysfs)?;
            }
            if let Some(ref execution) = self.execution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Execution", execution)?;
            }
            if let Some(ref resource_access_policies) = self.resource_access_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceAccessPolicies", resource_access_policies)?;
            }
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_sysfs: Option<::Value<bool>> = None;
                    let mut execution: Option<::Value<Execution>> = None;
                    let mut resource_access_policies: Option<::ValueList<ResourceAccessPolicy>> = None;
                    let mut variables: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessSysfs" => {
                                access_sysfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Execution" => {
                                execution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceAccessPolicies" => {
                                resource_access_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        access_sysfs: access_sysfs,
                        execution: execution,
                        resource_access_policies: resource_access_policies,
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.Execution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-execution.html) property type.
    #[derive(Debug, Default)]
    pub struct Execution {
        /// Property [`IsolationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-execution.html#cfn-greengrass-functiondefinitionversion-execution-isolationmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub isolation_mode: Option<::Value<String>>,
        /// Property [`RunAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-execution.html#cfn-greengrass-functiondefinitionversion-execution-runas).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub run_as: Option<::Value<RunAs>>,
    }

    impl ::codec::SerializeValue for Execution {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref isolation_mode) = self.isolation_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsolationMode", isolation_mode)?;
            }
            if let Some(ref run_as) = self.run_as {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunAs", run_as)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Execution {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Execution, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Execution;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Execution")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut isolation_mode: Option<::Value<String>> = None;
                    let mut run_as: Option<::Value<RunAs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsolationMode" => {
                                isolation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunAs" => {
                                run_as = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Execution {
                        isolation_mode: isolation_mode,
                        run_as: run_as,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-function.html) property type.
    #[derive(Debug, Default)]
    pub struct Function {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-function.html#cfn-greengrass-functiondefinitionversion-function-functionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub function_arn: ::Value<String>,
        /// Property [`FunctionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-function.html#cfn-greengrass-functiondefinitionversion-function-functionconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub function_configuration: ::Value<FunctionConfiguration>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-function.html#cfn-greengrass-functiondefinitionversion-function-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
    }

    impl ::codec::SerializeValue for Function {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", &self.function_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionConfiguration", &self.function_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Function {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Function, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Function;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Function")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<::Value<String>> = None;
                    let mut function_configuration: Option<::Value<FunctionConfiguration>> = None;
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionConfiguration" => {
                                function_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Function {
                        function_arn: function_arn.ok_or(::serde::de::Error::missing_field("FunctionArn"))?,
                        function_configuration: function_configuration.ok_or(::serde::de::Error::missing_field("FunctionConfiguration"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.FunctionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionConfiguration {
        /// Property [`EncodingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-encodingtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encoding_type: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::Value<Environment>>,
        /// Property [`ExecArgs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-execargs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub exec_args: Option<::Value<String>>,
        /// Property [`Executable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-executable).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub executable: Option<::Value<String>>,
        /// Property [`MemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-memorysize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory_size: Option<::Value<u32>>,
        /// Property [`Pinned`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-pinned).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pinned: Option<::Value<bool>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html#cfn-greengrass-functiondefinitionversion-functionconfiguration-timeout).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for FunctionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encoding_type) = self.encoding_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncodingType", encoding_type)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref exec_args) = self.exec_args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecArgs", exec_args)?;
            }
            if let Some(ref executable) = self.executable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Executable", executable)?;
            }
            if let Some(ref memory_size) = self.memory_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", memory_size)?;
            }
            if let Some(ref pinned) = self.pinned {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pinned", pinned)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encoding_type: Option<::Value<String>> = None;
                    let mut environment: Option<::Value<Environment>> = None;
                    let mut exec_args: Option<::Value<String>> = None;
                    let mut executable: Option<::Value<String>> = None;
                    let mut memory_size: Option<::Value<u32>> = None;
                    let mut pinned: Option<::Value<bool>> = None;
                    let mut timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncodingType" => {
                                encoding_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecArgs" => {
                                exec_args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Executable" => {
                                executable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemorySize" => {
                                memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pinned" => {
                                pinned = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionConfiguration {
                        encoding_type: encoding_type,
                        environment: environment,
                        exec_args: exec_args,
                        executable: executable,
                        memory_size: memory_size,
                        pinned: pinned,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.ResourceAccessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-resourceaccesspolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceAccessPolicy {
        /// Property [`Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-resourceaccesspolicy.html#cfn-greengrass-functiondefinitionversion-resourceaccesspolicy-permission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permission: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-resourceaccesspolicy.html#cfn-greengrass-functiondefinitionversion-resourceaccesspolicy-resourceid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceAccessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref permission) = self.permission {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", permission)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceAccessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceAccessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceAccessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceAccessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut permission: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceAccessPolicy {
                        permission: permission,
                        resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::FunctionDefinitionVersion.RunAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-runas.html) property type.
    #[derive(Debug, Default)]
    pub struct RunAs {
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-runas.html#cfn-greengrass-functiondefinitionversion-runas-gid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub gid: Option<::Value<u32>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-runas.html#cfn-greengrass-functiondefinitionversion-runas-uid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub uid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RunAs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref gid) = self.gid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", gid)?;
            }
            if let Some(ref uid) = self.uid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", uid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunAs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunAs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunAs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunAs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gid: Option<::Value<u32>> = None;
                    let mut uid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunAs {
                        gid: gid,
                        uid: uid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod group {
    //! Property types for the `Group` resource.

    /// The [`AWS::Greengrass::Group.GroupVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html) property type.
    #[derive(Debug, Default)]
    pub struct GroupVersion {
        /// Property [`ConnectorDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-connectordefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub connector_definition_version_arn: Option<::Value<String>>,
        /// Property [`CoreDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-coredefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub core_definition_version_arn: Option<::Value<String>>,
        /// Property [`DeviceDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-devicedefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_definition_version_arn: Option<::Value<String>>,
        /// Property [`FunctionDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-functiondefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub function_definition_version_arn: Option<::Value<String>>,
        /// Property [`LoggerDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-loggerdefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub logger_definition_version_arn: Option<::Value<String>>,
        /// Property [`ResourceDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-resourcedefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_definition_version_arn: Option<::Value<String>>,
        /// Property [`SubscriptionDefinitionVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html#cfn-greengrass-group-groupversion-subscriptiondefinitionversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subscription_definition_version_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GroupVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_definition_version_arn) = self.connector_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorDefinitionVersionArn", connector_definition_version_arn)?;
            }
            if let Some(ref core_definition_version_arn) = self.core_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreDefinitionVersionArn", core_definition_version_arn)?;
            }
            if let Some(ref device_definition_version_arn) = self.device_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceDefinitionVersionArn", device_definition_version_arn)?;
            }
            if let Some(ref function_definition_version_arn) = self.function_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionDefinitionVersionArn", function_definition_version_arn)?;
            }
            if let Some(ref logger_definition_version_arn) = self.logger_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggerDefinitionVersionArn", logger_definition_version_arn)?;
            }
            if let Some(ref resource_definition_version_arn) = self.resource_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDefinitionVersionArn", resource_definition_version_arn)?;
            }
            if let Some(ref subscription_definition_version_arn) = self.subscription_definition_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionDefinitionVersionArn", subscription_definition_version_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GroupVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GroupVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GroupVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_definition_version_arn: Option<::Value<String>> = None;
                    let mut core_definition_version_arn: Option<::Value<String>> = None;
                    let mut device_definition_version_arn: Option<::Value<String>> = None;
                    let mut function_definition_version_arn: Option<::Value<String>> = None;
                    let mut logger_definition_version_arn: Option<::Value<String>> = None;
                    let mut resource_definition_version_arn: Option<::Value<String>> = None;
                    let mut subscription_definition_version_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorDefinitionVersionArn" => {
                                connector_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoreDefinitionVersionArn" => {
                                core_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceDefinitionVersionArn" => {
                                device_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionDefinitionVersionArn" => {
                                function_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggerDefinitionVersionArn" => {
                                logger_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceDefinitionVersionArn" => {
                                resource_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubscriptionDefinitionVersionArn" => {
                                subscription_definition_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GroupVersion {
                        connector_definition_version_arn: connector_definition_version_arn,
                        core_definition_version_arn: core_definition_version_arn,
                        device_definition_version_arn: device_definition_version_arn,
                        function_definition_version_arn: function_definition_version_arn,
                        logger_definition_version_arn: logger_definition_version_arn,
                        resource_definition_version_arn: resource_definition_version_arn,
                        subscription_definition_version_arn: subscription_definition_version_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod logger_definition {
    //! Property types for the `LoggerDefinition` resource.

    /// The [`AWS::Greengrass::LoggerDefinition.Logger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html) property type.
    #[derive(Debug, Default)]
    pub struct Logger {
        /// Property [`Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html#cfn-greengrass-loggerdefinition-logger-component).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html#cfn-greengrass-loggerdefinition-logger-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html#cfn-greengrass-loggerdefinition-logger-level).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub level: ::Value<String>,
        /// Property [`Space`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html#cfn-greengrass-loggerdefinition-logger-space).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub space: Option<::Value<u32>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html#cfn-greengrass-loggerdefinition-logger-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Logger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Component", &self.component)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", &self.level)?;
            if let Some(ref space) = self.space {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Space", space)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut level: Option<::Value<String>> = None;
                    let mut space: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Component" => {
                                component = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Space" => {
                                space = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logger {
                        component: component.ok_or(::serde::de::Error::missing_field("Component"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        level: level.ok_or(::serde::de::Error::missing_field("Level"))?,
                        space: space,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::LoggerDefinition.LoggerDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-loggerdefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggerDefinitionVersion {
        /// Property [`Loggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-loggerdefinitionversion.html#cfn-greengrass-loggerdefinition-loggerdefinitionversion-loggers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub loggers: ::ValueList<Logger>,
    }

    impl ::codec::SerializeValue for LoggerDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Loggers", &self.loggers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggerDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggerDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggerDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggerDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut loggers: Option<::ValueList<Logger>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Loggers" => {
                                loggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggerDefinitionVersion {
                        loggers: loggers.ok_or(::serde::de::Error::missing_field("Loggers"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod logger_definition_version {
    //! Property types for the `LoggerDefinitionVersion` resource.

    /// The [`AWS::Greengrass::LoggerDefinitionVersion.Logger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html) property type.
    #[derive(Debug, Default)]
    pub struct Logger {
        /// Property [`Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html#cfn-greengrass-loggerdefinitionversion-logger-component).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html#cfn-greengrass-loggerdefinitionversion-logger-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html#cfn-greengrass-loggerdefinitionversion-logger-level).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub level: ::Value<String>,
        /// Property [`Space`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html#cfn-greengrass-loggerdefinitionversion-logger-space).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub space: Option<::Value<u32>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html#cfn-greengrass-loggerdefinitionversion-logger-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Logger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Component", &self.component)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", &self.level)?;
            if let Some(ref space) = self.space {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Space", space)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut level: Option<::Value<String>> = None;
                    let mut space: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Component" => {
                                component = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Space" => {
                                space = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logger {
                        component: component.ok_or(::serde::de::Error::missing_field("Component"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        level: level.ok_or(::serde::de::Error::missing_field("Level"))?,
                        space: space,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resource_definition {
    //! Property types for the `ResourceDefinition` resource.

    /// The [`AWS::Greengrass::ResourceDefinition.GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-groupownersetting.html) property type.
    #[derive(Debug, Default)]
    pub struct GroupOwnerSetting {
        /// Property [`AutoAddGroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-groupownersetting.html#cfn-greengrass-resourcedefinition-groupownersetting-autoaddgroupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub auto_add_group_owner: ::Value<bool>,
        /// Property [`GroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-groupownersetting.html#cfn-greengrass-resourcedefinition-groupownersetting-groupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GroupOwnerSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAddGroupOwner", &self.auto_add_group_owner)?;
            if let Some(ref group_owner) = self.group_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwner", group_owner)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GroupOwnerSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupOwnerSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GroupOwnerSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GroupOwnerSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_add_group_owner: Option<::Value<bool>> = None;
                    let mut group_owner: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoAddGroupOwner" => {
                                auto_add_group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupOwner" => {
                                group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GroupOwnerSetting {
                        auto_add_group_owner: auto_add_group_owner.ok_or(::serde::de::Error::missing_field("AutoAddGroupOwner"))?,
                        group_owner: group_owner,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.LocalDeviceResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localdeviceresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalDeviceResourceData {
        /// Property [`GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localdeviceresourcedata.html#cfn-greengrass-resourcedefinition-localdeviceresourcedata-groupownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner_setting: Option<::Value<GroupOwnerSetting>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localdeviceresourcedata.html#cfn-greengrass-resourcedefinition-localdeviceresourcedata-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for LocalDeviceResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group_owner_setting) = self.group_owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwnerSetting", group_owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", &self.source_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalDeviceResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalDeviceResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalDeviceResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalDeviceResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_owner_setting: Option<::Value<GroupOwnerSetting>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupOwnerSetting" => {
                                group_owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalDeviceResourceData {
                        group_owner_setting: group_owner_setting,
                        source_path: source_path.ok_or(::serde::de::Error::missing_field("SourcePath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.LocalVolumeResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localvolumeresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalVolumeResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localvolumeresourcedata.html#cfn-greengrass-resourcedefinition-localvolumeresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localvolumeresourcedata.html#cfn-greengrass-resourcedefinition-localvolumeresourcedata-groupownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner_setting: Option<::Value<GroupOwnerSetting>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localvolumeresourcedata.html#cfn-greengrass-resourcedefinition-localvolumeresourcedata-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for LocalVolumeResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref group_owner_setting) = self.group_owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwnerSetting", group_owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", &self.source_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalVolumeResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalVolumeResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalVolumeResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalVolumeResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut group_owner_setting: Option<::Value<GroupOwnerSetting>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupOwnerSetting" => {
                                group_owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalVolumeResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        group_owner_setting: group_owner_setting,
                        source_path: source_path.ok_or(::serde::de::Error::missing_field("SourcePath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.ResourceDataContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceDataContainer {
        /// Property [`LocalDeviceResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html#cfn-greengrass-resourcedefinition-resourcedatacontainer-localdeviceresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_device_resource_data: Option<::Value<LocalDeviceResourceData>>,
        /// Property [`LocalVolumeResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html#cfn-greengrass-resourcedefinition-resourcedatacontainer-localvolumeresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_volume_resource_data: Option<::Value<LocalVolumeResourceData>>,
        /// Property [`S3MachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html#cfn-greengrass-resourcedefinition-resourcedatacontainer-s3machinelearningmodelresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_machine_learning_model_resource_data: Option<::Value<S3MachineLearningModelResourceData>>,
        /// Property [`SageMakerMachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html#cfn-greengrass-resourcedefinition-resourcedatacontainer-sagemakermachinelearningmodelresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sage_maker_machine_learning_model_resource_data: Option<::Value<SageMakerMachineLearningModelResourceData>>,
        /// Property [`SecretsManagerSecretResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html#cfn-greengrass-resourcedefinition-resourcedatacontainer-secretsmanagersecretresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secrets_manager_secret_resource_data: Option<::Value<SecretsManagerSecretResourceData>>,
    }

    impl ::codec::SerializeValue for ResourceDataContainer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref local_device_resource_data) = self.local_device_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalDeviceResourceData", local_device_resource_data)?;
            }
            if let Some(ref local_volume_resource_data) = self.local_volume_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalVolumeResourceData", local_volume_resource_data)?;
            }
            if let Some(ref s3_machine_learning_model_resource_data) = self.s3_machine_learning_model_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3MachineLearningModelResourceData", s3_machine_learning_model_resource_data)?;
            }
            if let Some(ref sage_maker_machine_learning_model_resource_data) = self.sage_maker_machine_learning_model_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerMachineLearningModelResourceData", sage_maker_machine_learning_model_resource_data)?;
            }
            if let Some(ref secrets_manager_secret_resource_data) = self.secrets_manager_secret_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretResourceData", secrets_manager_secret_resource_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceDataContainer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDataContainer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceDataContainer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceDataContainer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_device_resource_data: Option<::Value<LocalDeviceResourceData>> = None;
                    let mut local_volume_resource_data: Option<::Value<LocalVolumeResourceData>> = None;
                    let mut s3_machine_learning_model_resource_data: Option<::Value<S3MachineLearningModelResourceData>> = None;
                    let mut sage_maker_machine_learning_model_resource_data: Option<::Value<SageMakerMachineLearningModelResourceData>> = None;
                    let mut secrets_manager_secret_resource_data: Option<::Value<SecretsManagerSecretResourceData>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalDeviceResourceData" => {
                                local_device_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalVolumeResourceData" => {
                                local_volume_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3MachineLearningModelResourceData" => {
                                s3_machine_learning_model_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerMachineLearningModelResourceData" => {
                                sage_maker_machine_learning_model_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretResourceData" => {
                                secrets_manager_secret_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceDataContainer {
                        local_device_resource_data: local_device_resource_data,
                        local_volume_resource_data: local_volume_resource_data,
                        s3_machine_learning_model_resource_data: s3_machine_learning_model_resource_data,
                        sage_maker_machine_learning_model_resource_data: sage_maker_machine_learning_model_resource_data,
                        secrets_manager_secret_resource_data: secrets_manager_secret_resource_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.ResourceDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceDefinitionVersion {
        /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedefinitionversion.html#cfn-greengrass-resourcedefinition-resourcedefinitionversion-resources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resources: ::ValueList<ResourceInstance>,
    }

    impl ::codec::SerializeValue for ResourceDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", &self.resources)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resources: Option<::ValueList<ResourceInstance>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Resources" => {
                                resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceDefinitionVersion {
                        resources: resources.ok_or(::serde::de::Error::missing_field("Resources"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.ResourceDownloadOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedownloadownersetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceDownloadOwnerSetting {
        /// Property [`GroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedownloadownersetting.html#cfn-greengrass-resourcedefinition-resourcedownloadownersetting-groupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner: ::Value<String>,
        /// Property [`GroupPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedownloadownersetting.html#cfn-greengrass-resourcedefinition-resourcedownloadownersetting-grouppermission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_permission: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceDownloadOwnerSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwner", &self.group_owner)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupPermission", &self.group_permission)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceDownloadOwnerSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDownloadOwnerSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceDownloadOwnerSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceDownloadOwnerSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_owner: Option<::Value<String>> = None;
                    let mut group_permission: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupOwner" => {
                                group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupPermission" => {
                                group_permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceDownloadOwnerSetting {
                        group_owner: group_owner.ok_or(::serde::de::Error::missing_field("GroupOwner"))?,
                        group_permission: group_permission.ok_or(::serde::de::Error::missing_field("GroupPermission"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.ResourceInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourceinstance.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceInstance {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourceinstance.html#cfn-greengrass-resourcedefinition-resourceinstance-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourceinstance.html#cfn-greengrass-resourcedefinition-resourceinstance-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ResourceDataContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourceinstance.html#cfn-greengrass-resourcedefinition-resourceinstance-resourcedatacontainer).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_data_container: ::Value<ResourceDataContainer>,
    }

    impl ::codec::SerializeValue for ResourceInstance {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDataContainer", &self.resource_data_container)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceInstance {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceInstance, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceInstance;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceInstance")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut resource_data_container: Option<::Value<ResourceDataContainer>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceDataContainer" => {
                                resource_data_container = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceInstance {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        resource_data_container: resource_data_container.ok_or(::serde::de::Error::missing_field("ResourceDataContainer"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.S3MachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-s3machinelearningmodelresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct S3MachineLearningModelResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-s3machinelearningmodelresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`OwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-s3machinelearningmodelresourcedata-ownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_setting: Option<::Value<ResourceDownloadOwnerSetting>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-s3machinelearningmodelresourcedata-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3MachineLearningModelResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref owner_setting) = self.owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerSetting", owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3MachineLearningModelResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3MachineLearningModelResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3MachineLearningModelResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3MachineLearningModelResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut owner_setting: Option<::Value<ResourceDownloadOwnerSetting>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OwnerSetting" => {
                                owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3MachineLearningModelResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        owner_setting: owner_setting,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.SageMakerMachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct SageMakerMachineLearningModelResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`OwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata-ownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_setting: Option<::Value<ResourceDownloadOwnerSetting>>,
        /// Property [`SageMakerJobArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata-sagemakerjobarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sage_maker_job_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SageMakerMachineLearningModelResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref owner_setting) = self.owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerSetting", owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerJobArn", &self.sage_maker_job_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SageMakerMachineLearningModelResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SageMakerMachineLearningModelResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SageMakerMachineLearningModelResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SageMakerMachineLearningModelResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut owner_setting: Option<::Value<ResourceDownloadOwnerSetting>> = None;
                    let mut sage_maker_job_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OwnerSetting" => {
                                owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerJobArn" => {
                                sage_maker_job_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SageMakerMachineLearningModelResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        owner_setting: owner_setting,
                        sage_maker_job_arn: sage_maker_job_arn.ok_or(::serde::de::Error::missing_field("SageMakerJobArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinition.SecretsManagerSecretResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-secretsmanagersecretresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct SecretsManagerSecretResourceData {
        /// Property [`ARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-secretsmanagersecretresourcedata.html#cfn-greengrass-resourcedefinition-secretsmanagersecretresourcedata-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`AdditionalStagingLabelsToDownload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-secretsmanagersecretresourcedata.html#cfn-greengrass-resourcedefinition-secretsmanagersecretresourcedata-additionalstaginglabelstodownload).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub additional_staging_labels_to_download: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SecretsManagerSecretResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ARN", &self.arn)?;
            if let Some(ref additional_staging_labels_to_download) = self.additional_staging_labels_to_download {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalStagingLabelsToDownload", additional_staging_labels_to_download)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecretsManagerSecretResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecretsManagerSecretResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecretsManagerSecretResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecretsManagerSecretResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut additional_staging_labels_to_download: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ARN" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdditionalStagingLabelsToDownload" => {
                                additional_staging_labels_to_download = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecretsManagerSecretResourceData {
                        arn: arn.ok_or(::serde::de::Error::missing_field("ARN"))?,
                        additional_staging_labels_to_download: additional_staging_labels_to_download,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resource_definition_version {
    //! Property types for the `ResourceDefinitionVersion` resource.

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-groupownersetting.html) property type.
    #[derive(Debug, Default)]
    pub struct GroupOwnerSetting {
        /// Property [`AutoAddGroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-groupownersetting.html#cfn-greengrass-resourcedefinitionversion-groupownersetting-autoaddgroupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub auto_add_group_owner: ::Value<bool>,
        /// Property [`GroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-groupownersetting.html#cfn-greengrass-resourcedefinitionversion-groupownersetting-groupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GroupOwnerSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAddGroupOwner", &self.auto_add_group_owner)?;
            if let Some(ref group_owner) = self.group_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwner", group_owner)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GroupOwnerSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupOwnerSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GroupOwnerSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GroupOwnerSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_add_group_owner: Option<::Value<bool>> = None;
                    let mut group_owner: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoAddGroupOwner" => {
                                auto_add_group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupOwner" => {
                                group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GroupOwnerSetting {
                        auto_add_group_owner: auto_add_group_owner.ok_or(::serde::de::Error::missing_field("AutoAddGroupOwner"))?,
                        group_owner: group_owner,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.LocalDeviceResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localdeviceresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalDeviceResourceData {
        /// Property [`GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localdeviceresourcedata.html#cfn-greengrass-resourcedefinitionversion-localdeviceresourcedata-groupownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner_setting: Option<::Value<GroupOwnerSetting>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localdeviceresourcedata.html#cfn-greengrass-resourcedefinitionversion-localdeviceresourcedata-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for LocalDeviceResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group_owner_setting) = self.group_owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwnerSetting", group_owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", &self.source_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalDeviceResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalDeviceResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalDeviceResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalDeviceResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_owner_setting: Option<::Value<GroupOwnerSetting>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupOwnerSetting" => {
                                group_owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalDeviceResourceData {
                        group_owner_setting: group_owner_setting,
                        source_path: source_path.ok_or(::serde::de::Error::missing_field("SourcePath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.LocalVolumeResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localvolumeresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalVolumeResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localvolumeresourcedata.html#cfn-greengrass-resourcedefinitionversion-localvolumeresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`GroupOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localvolumeresourcedata.html#cfn-greengrass-resourcedefinitionversion-localvolumeresourcedata-groupownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner_setting: Option<::Value<GroupOwnerSetting>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localvolumeresourcedata.html#cfn-greengrass-resourcedefinitionversion-localvolumeresourcedata-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for LocalVolumeResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref group_owner_setting) = self.group_owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwnerSetting", group_owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", &self.source_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalVolumeResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalVolumeResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalVolumeResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalVolumeResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut group_owner_setting: Option<::Value<GroupOwnerSetting>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupOwnerSetting" => {
                                group_owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalVolumeResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        group_owner_setting: group_owner_setting,
                        source_path: source_path.ok_or(::serde::de::Error::missing_field("SourcePath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.ResourceDataContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceDataContainer {
        /// Property [`LocalDeviceResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html#cfn-greengrass-resourcedefinitionversion-resourcedatacontainer-localdeviceresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_device_resource_data: Option<::Value<LocalDeviceResourceData>>,
        /// Property [`LocalVolumeResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html#cfn-greengrass-resourcedefinitionversion-resourcedatacontainer-localvolumeresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_volume_resource_data: Option<::Value<LocalVolumeResourceData>>,
        /// Property [`S3MachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html#cfn-greengrass-resourcedefinitionversion-resourcedatacontainer-s3machinelearningmodelresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_machine_learning_model_resource_data: Option<::Value<S3MachineLearningModelResourceData>>,
        /// Property [`SageMakerMachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html#cfn-greengrass-resourcedefinitionversion-resourcedatacontainer-sagemakermachinelearningmodelresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sage_maker_machine_learning_model_resource_data: Option<::Value<SageMakerMachineLearningModelResourceData>>,
        /// Property [`SecretsManagerSecretResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html#cfn-greengrass-resourcedefinitionversion-resourcedatacontainer-secretsmanagersecretresourcedata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secrets_manager_secret_resource_data: Option<::Value<SecretsManagerSecretResourceData>>,
    }

    impl ::codec::SerializeValue for ResourceDataContainer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref local_device_resource_data) = self.local_device_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalDeviceResourceData", local_device_resource_data)?;
            }
            if let Some(ref local_volume_resource_data) = self.local_volume_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalVolumeResourceData", local_volume_resource_data)?;
            }
            if let Some(ref s3_machine_learning_model_resource_data) = self.s3_machine_learning_model_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3MachineLearningModelResourceData", s3_machine_learning_model_resource_data)?;
            }
            if let Some(ref sage_maker_machine_learning_model_resource_data) = self.sage_maker_machine_learning_model_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerMachineLearningModelResourceData", sage_maker_machine_learning_model_resource_data)?;
            }
            if let Some(ref secrets_manager_secret_resource_data) = self.secrets_manager_secret_resource_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretResourceData", secrets_manager_secret_resource_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceDataContainer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDataContainer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceDataContainer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceDataContainer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_device_resource_data: Option<::Value<LocalDeviceResourceData>> = None;
                    let mut local_volume_resource_data: Option<::Value<LocalVolumeResourceData>> = None;
                    let mut s3_machine_learning_model_resource_data: Option<::Value<S3MachineLearningModelResourceData>> = None;
                    let mut sage_maker_machine_learning_model_resource_data: Option<::Value<SageMakerMachineLearningModelResourceData>> = None;
                    let mut secrets_manager_secret_resource_data: Option<::Value<SecretsManagerSecretResourceData>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalDeviceResourceData" => {
                                local_device_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalVolumeResourceData" => {
                                local_volume_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3MachineLearningModelResourceData" => {
                                s3_machine_learning_model_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerMachineLearningModelResourceData" => {
                                sage_maker_machine_learning_model_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretResourceData" => {
                                secrets_manager_secret_resource_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceDataContainer {
                        local_device_resource_data: local_device_resource_data,
                        local_volume_resource_data: local_volume_resource_data,
                        s3_machine_learning_model_resource_data: s3_machine_learning_model_resource_data,
                        sage_maker_machine_learning_model_resource_data: sage_maker_machine_learning_model_resource_data,
                        secrets_manager_secret_resource_data: secrets_manager_secret_resource_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.ResourceDownloadOwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedownloadownersetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceDownloadOwnerSetting {
        /// Property [`GroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedownloadownersetting.html#cfn-greengrass-resourcedefinitionversion-resourcedownloadownersetting-groupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_owner: ::Value<String>,
        /// Property [`GroupPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedownloadownersetting.html#cfn-greengrass-resourcedefinitionversion-resourcedownloadownersetting-grouppermission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_permission: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceDownloadOwnerSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupOwner", &self.group_owner)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupPermission", &self.group_permission)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceDownloadOwnerSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDownloadOwnerSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceDownloadOwnerSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceDownloadOwnerSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_owner: Option<::Value<String>> = None;
                    let mut group_permission: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupOwner" => {
                                group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupPermission" => {
                                group_permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceDownloadOwnerSetting {
                        group_owner: group_owner.ok_or(::serde::de::Error::missing_field("GroupOwner"))?,
                        group_permission: group_permission.ok_or(::serde::de::Error::missing_field("GroupPermission"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.ResourceInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourceinstance.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceInstance {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourceinstance.html#cfn-greengrass-resourcedefinitionversion-resourceinstance-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourceinstance.html#cfn-greengrass-resourcedefinitionversion-resourceinstance-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ResourceDataContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourceinstance.html#cfn-greengrass-resourcedefinitionversion-resourceinstance-resourcedatacontainer).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_data_container: ::Value<ResourceDataContainer>,
    }

    impl ::codec::SerializeValue for ResourceInstance {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDataContainer", &self.resource_data_container)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceInstance {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceInstance, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceInstance;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceInstance")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut resource_data_container: Option<::Value<ResourceDataContainer>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceDataContainer" => {
                                resource_data_container = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceInstance {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        resource_data_container: resource_data_container.ok_or(::serde::de::Error::missing_field("ResourceDataContainer"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.S3MachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct S3MachineLearningModelResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`OwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata-ownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_setting: Option<::Value<ResourceDownloadOwnerSetting>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3MachineLearningModelResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref owner_setting) = self.owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerSetting", owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3MachineLearningModelResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3MachineLearningModelResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3MachineLearningModelResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3MachineLearningModelResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut owner_setting: Option<::Value<ResourceDownloadOwnerSetting>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OwnerSetting" => {
                                owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3MachineLearningModelResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        owner_setting: owner_setting,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.SageMakerMachineLearningModelResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct SageMakerMachineLearningModelResourceData {
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: ::Value<String>,
        /// Property [`OwnerSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata-ownersetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_setting: Option<::Value<ResourceDownloadOwnerSetting>>,
        /// Property [`SageMakerJobArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata.html#cfn-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata-sagemakerjobarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sage_maker_job_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SageMakerMachineLearningModelResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", &self.destination_path)?;
            if let Some(ref owner_setting) = self.owner_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerSetting", owner_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerJobArn", &self.sage_maker_job_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SageMakerMachineLearningModelResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SageMakerMachineLearningModelResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SageMakerMachineLearningModelResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SageMakerMachineLearningModelResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut owner_setting: Option<::Value<ResourceDownloadOwnerSetting>> = None;
                    let mut sage_maker_job_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OwnerSetting" => {
                                owner_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerJobArn" => {
                                sage_maker_job_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SageMakerMachineLearningModelResourceData {
                        destination_path: destination_path.ok_or(::serde::de::Error::missing_field("DestinationPath"))?,
                        owner_setting: owner_setting,
                        sage_maker_job_arn: sage_maker_job_arn.ok_or(::serde::de::Error::missing_field("SageMakerJobArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::ResourceDefinitionVersion.SecretsManagerSecretResourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct SecretsManagerSecretResourceData {
        /// Property [`ARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata.html#cfn-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`AdditionalStagingLabelsToDownload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata.html#cfn-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata-additionalstaginglabelstodownload).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub additional_staging_labels_to_download: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SecretsManagerSecretResourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ARN", &self.arn)?;
            if let Some(ref additional_staging_labels_to_download) = self.additional_staging_labels_to_download {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalStagingLabelsToDownload", additional_staging_labels_to_download)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecretsManagerSecretResourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecretsManagerSecretResourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecretsManagerSecretResourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecretsManagerSecretResourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut additional_staging_labels_to_download: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ARN" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdditionalStagingLabelsToDownload" => {
                                additional_staging_labels_to_download = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecretsManagerSecretResourceData {
                        arn: arn.ok_or(::serde::de::Error::missing_field("ARN"))?,
                        additional_staging_labels_to_download: additional_staging_labels_to_download,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod subscription_definition {
    //! Property types for the `SubscriptionDefinition` resource.

    /// The [`AWS::Greengrass::SubscriptionDefinition.Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscription {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html#cfn-greengrass-subscriptiondefinition-subscription-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html#cfn-greengrass-subscriptiondefinition-subscription-source).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source: ::Value<String>,
        /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html#cfn-greengrass-subscriptiondefinition-subscription-subject).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subject: ::Value<String>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html#cfn-greengrass-subscriptiondefinition-subscription-target).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub target: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", &self.subject)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subscription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subscription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subscription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subscription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut subject: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subject" => {
                                subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscription {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                        subject: subject.ok_or(::serde::de::Error::missing_field("Subject"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Greengrass::SubscriptionDefinition.SubscriptionDefinitionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscriptiondefinitionversion.html) property type.
    #[derive(Debug, Default)]
    pub struct SubscriptionDefinitionVersion {
        /// Property [`Subscriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscriptiondefinitionversion.html#cfn-greengrass-subscriptiondefinition-subscriptiondefinitionversion-subscriptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subscriptions: ::ValueList<Subscription>,
    }

    impl ::codec::SerializeValue for SubscriptionDefinitionVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscriptions", &self.subscriptions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubscriptionDefinitionVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionDefinitionVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubscriptionDefinitionVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubscriptionDefinitionVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subscriptions: Option<::ValueList<Subscription>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Subscriptions" => {
                                subscriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubscriptionDefinitionVersion {
                        subscriptions: subscriptions.ok_or(::serde::de::Error::missing_field("Subscriptions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod subscription_definition_version {
    //! Property types for the `SubscriptionDefinitionVersion` resource.

    /// The [`AWS::Greengrass::SubscriptionDefinitionVersion.Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscription {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html#cfn-greengrass-subscriptiondefinitionversion-subscription-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html#cfn-greengrass-subscriptiondefinitionversion-subscription-source).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source: ::Value<String>,
        /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html#cfn-greengrass-subscriptiondefinitionversion-subscription-subject).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subject: ::Value<String>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html#cfn-greengrass-subscriptiondefinitionversion-subscription-target).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub target: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", &self.subject)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subscription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subscription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subscription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subscription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut subject: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subject" => {
                                subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscription {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                        subject: subject.ok_or(::serde::de::Error::missing_field("Subject"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
