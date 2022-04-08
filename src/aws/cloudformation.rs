//! Types for the `CloudFormation` service.

/// The [`AWS::CloudFormation::CustomResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html) resource type.
#[derive(Debug, Default)]
pub struct CustomResource {
    properties: CustomResourceProperties
}

/// Properties for the `CustomResource` resource.
#[derive(Debug, Default)]
pub struct CustomResourceProperties {
    /// Property [`ServiceToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html#cfn-customresource-servicetoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_token: ::Value<String>,
}

impl ::serde::Serialize for CustomResourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceToken", &self.service_token)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomResourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut service_token: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ServiceToken" => {
                            service_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomResourceProperties {
                    service_token: service_token.ok_or(::serde::de::Error::missing_field("ServiceToken"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomResource {
    type Properties = CustomResourceProperties;
    const TYPE: &'static str = "AWS::CloudFormation::CustomResource";
    fn properties(&self) -> &CustomResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomResourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomResource {}

impl From<CustomResourceProperties> for CustomResource {
    fn from(properties: CustomResourceProperties) -> CustomResource {
        CustomResource { properties }
    }
}

/// The [`AWS::CloudFormation::HookDefaultVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookdefaultversion.html) resource type.
#[derive(Debug, Default)]
pub struct HookDefaultVersion {
    properties: HookDefaultVersionProperties
}

/// Properties for the `HookDefaultVersion` resource.
#[derive(Debug, Default)]
pub struct HookDefaultVersionProperties {
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookdefaultversion.html#cfn-cloudformation-hookdefaultversion-typename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_name: Option<::Value<String>>,
    /// Property [`TypeVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookdefaultversion.html#cfn-cloudformation-hookdefaultversion-typeversionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_version_arn: Option<::Value<String>>,
    /// Property [`VersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookdefaultversion.html#cfn-cloudformation-hookdefaultversion-versionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_id: Option<::Value<String>>,
}

impl ::serde::Serialize for HookDefaultVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref type_name) = self.type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
        }
        if let Some(ref type_version_arn) = self.type_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeVersionArn", type_version_arn)?;
        }
        if let Some(ref version_id) = self.version_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionId", version_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HookDefaultVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HookDefaultVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HookDefaultVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HookDefaultVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut type_name: Option<::Value<String>> = None;
                let mut type_version_arn: Option<::Value<String>> = None;
                let mut version_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeVersionArn" => {
                            type_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionId" => {
                            version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HookDefaultVersionProperties {
                    type_name: type_name,
                    type_version_arn: type_version_arn,
                    version_id: version_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HookDefaultVersion {
    type Properties = HookDefaultVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::HookDefaultVersion";
    fn properties(&self) -> &HookDefaultVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HookDefaultVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HookDefaultVersion {}

impl From<HookDefaultVersionProperties> for HookDefaultVersion {
    fn from(properties: HookDefaultVersionProperties) -> HookDefaultVersion {
        HookDefaultVersion { properties }
    }
}

/// The [`AWS::CloudFormation::HookTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html) resource type.
#[derive(Debug, Default)]
pub struct HookTypeConfig {
    properties: HookTypeConfigProperties
}

/// Properties for the `HookTypeConfig` resource.
#[derive(Debug, Default)]
pub struct HookTypeConfigProperties {
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html#cfn-cloudformation-hooktypeconfig-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: ::Value<String>,
    /// Property [`ConfigurationAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html#cfn-cloudformation-hooktypeconfig-configurationalias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration_alias: Option<::Value<String>>,
    /// Property [`TypeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html#cfn-cloudformation-hooktypeconfig-typearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_arn: Option<::Value<String>>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html#cfn-cloudformation-hooktypeconfig-typename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_name: Option<::Value<String>>,
}

impl ::serde::Serialize for HookTypeConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
        if let Some(ref configuration_alias) = self.configuration_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationAlias", configuration_alias)?;
        }
        if let Some(ref type_arn) = self.type_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeArn", type_arn)?;
        }
        if let Some(ref type_name) = self.type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HookTypeConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HookTypeConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HookTypeConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HookTypeConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration: Option<::Value<String>> = None;
                let mut configuration_alias: Option<::Value<String>> = None;
                let mut type_arn: Option<::Value<String>> = None;
                let mut type_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationAlias" => {
                            configuration_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeArn" => {
                            type_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HookTypeConfigProperties {
                    configuration: configuration.ok_or(::serde::de::Error::missing_field("Configuration"))?,
                    configuration_alias: configuration_alias,
                    type_arn: type_arn,
                    type_name: type_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HookTypeConfig {
    type Properties = HookTypeConfigProperties;
    const TYPE: &'static str = "AWS::CloudFormation::HookTypeConfig";
    fn properties(&self) -> &HookTypeConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HookTypeConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HookTypeConfig {}

impl From<HookTypeConfigProperties> for HookTypeConfig {
    fn from(properties: HookTypeConfigProperties) -> HookTypeConfig {
        HookTypeConfig { properties }
    }
}

/// The [`AWS::CloudFormation::HookVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html) resource type.
#[derive(Debug, Default)]
pub struct HookVersion {
    properties: HookVersionProperties
}

/// Properties for the `HookVersion` resource.
#[derive(Debug, Default)]
pub struct HookVersionProperties {
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html#cfn-cloudformation-hookversion-executionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html#cfn-cloudformation-hookversion-loggingconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub logging_config: Option<::Value<self::hook_version::LoggingConfig>>,
    /// Property [`SchemaHandlerPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html#cfn-cloudformation-hookversion-schemahandlerpackage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_handler_package: ::Value<String>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html#cfn-cloudformation-hookversion-typename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name: ::Value<String>,
}

impl ::serde::Serialize for HookVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref logging_config) = self.logging_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfig", logging_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaHandlerPackage", &self.schema_handler_package)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", &self.type_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HookVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HookVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HookVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HookVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut logging_config: Option<::Value<self::hook_version::LoggingConfig>> = None;
                let mut schema_handler_package: Option<::Value<String>> = None;
                let mut type_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfig" => {
                            logging_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaHandlerPackage" => {
                            schema_handler_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HookVersionProperties {
                    execution_role_arn: execution_role_arn,
                    logging_config: logging_config,
                    schema_handler_package: schema_handler_package.ok_or(::serde::de::Error::missing_field("SchemaHandlerPackage"))?,
                    type_name: type_name.ok_or(::serde::de::Error::missing_field("TypeName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HookVersion {
    type Properties = HookVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::HookVersion";
    fn properties(&self) -> &HookVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HookVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HookVersion {}

impl From<HookVersionProperties> for HookVersion {
    fn from(properties: HookVersionProperties) -> HookVersion {
        HookVersion { properties }
    }
}

/// The [`AWS::CloudFormation::Macro`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html) resource type.
#[derive(Debug, Default)]
pub struct Macro {
    properties: MacroProperties
}

/// Properties for the `Macro` resource.
#[derive(Debug, Default)]
pub struct MacroProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html#cfn-cloudformation-macro-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html#cfn-cloudformation-macro-functionname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html#cfn-cloudformation-macro-loggroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_group_name: Option<::Value<String>>,
    /// Property [`LogRoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html#cfn-cloudformation-macro-logrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_role_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html#cfn-cloudformation-macro-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for MacroProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref log_group_name) = self.log_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
        }
        if let Some(ref log_role_arn) = self.log_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogRoleARN", log_role_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MacroProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MacroProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MacroProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MacroProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut log_group_name: Option<::Value<String>> = None;
                let mut log_role_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogGroupName" => {
                            log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogRoleARN" => {
                            log_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MacroProperties {
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    log_group_name: log_group_name,
                    log_role_arn: log_role_arn,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Macro {
    type Properties = MacroProperties;
    const TYPE: &'static str = "AWS::CloudFormation::Macro";
    fn properties(&self) -> &MacroProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MacroProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Macro {}

impl From<MacroProperties> for Macro {
    fn from(properties: MacroProperties) -> Macro {
        Macro { properties }
    }
}

/// The [`AWS::CloudFormation::ModuleDefaultVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduledefaultversion.html) resource type.
#[derive(Debug, Default)]
pub struct ModuleDefaultVersion {
    properties: ModuleDefaultVersionProperties
}

/// Properties for the `ModuleDefaultVersion` resource.
#[derive(Debug, Default)]
pub struct ModuleDefaultVersionProperties {
    /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduledefaultversion.html#cfn-cloudformation-moduledefaultversion-arn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub arn: Option<::Value<String>>,
    /// Property [`ModuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduledefaultversion.html#cfn-cloudformation-moduledefaultversion-modulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub module_name: Option<::Value<String>>,
    /// Property [`VersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduledefaultversion.html#cfn-cloudformation-moduledefaultversion-versionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ModuleDefaultVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref arn) = self.arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
        }
        if let Some(ref module_name) = self.module_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModuleName", module_name)?;
        }
        if let Some(ref version_id) = self.version_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionId", version_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModuleDefaultVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModuleDefaultVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModuleDefaultVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModuleDefaultVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut arn: Option<::Value<String>> = None;
                let mut module_name: Option<::Value<String>> = None;
                let mut version_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Arn" => {
                            arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModuleName" => {
                            module_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionId" => {
                            version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModuleDefaultVersionProperties {
                    arn: arn,
                    module_name: module_name,
                    version_id: version_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModuleDefaultVersion {
    type Properties = ModuleDefaultVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::ModuleDefaultVersion";
    fn properties(&self) -> &ModuleDefaultVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModuleDefaultVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModuleDefaultVersion {}

impl From<ModuleDefaultVersionProperties> for ModuleDefaultVersion {
    fn from(properties: ModuleDefaultVersionProperties) -> ModuleDefaultVersion {
        ModuleDefaultVersion { properties }
    }
}

/// The [`AWS::CloudFormation::ModuleVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduleversion.html) resource type.
#[derive(Debug, Default)]
pub struct ModuleVersion {
    properties: ModuleVersionProperties
}

/// Properties for the `ModuleVersion` resource.
#[derive(Debug, Default)]
pub struct ModuleVersionProperties {
    /// Property [`ModuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduleversion.html#cfn-cloudformation-moduleversion-modulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub module_name: ::Value<String>,
    /// Property [`ModulePackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduleversion.html#cfn-cloudformation-moduleversion-modulepackage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub module_package: ::Value<String>,
}

impl ::serde::Serialize for ModuleVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModuleName", &self.module_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModulePackage", &self.module_package)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModuleVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModuleVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModuleVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModuleVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut module_name: Option<::Value<String>> = None;
                let mut module_package: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ModuleName" => {
                            module_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModulePackage" => {
                            module_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModuleVersionProperties {
                    module_name: module_name.ok_or(::serde::de::Error::missing_field("ModuleName"))?,
                    module_package: module_package.ok_or(::serde::de::Error::missing_field("ModulePackage"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModuleVersion {
    type Properties = ModuleVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::ModuleVersion";
    fn properties(&self) -> &ModuleVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModuleVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModuleVersion {}

impl From<ModuleVersionProperties> for ModuleVersion {
    fn from(properties: ModuleVersionProperties) -> ModuleVersion {
        ModuleVersion { properties }
    }
}

/// The [`AWS::CloudFormation::PublicTypeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html) resource type.
#[derive(Debug, Default)]
pub struct PublicTypeVersion {
    properties: PublicTypeVersionProperties
}

/// Properties for the `PublicTypeVersion` resource.
#[derive(Debug, Default)]
pub struct PublicTypeVersionProperties {
    /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html#cfn-cloudformation-publictypeversion-arn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub arn: Option<::Value<String>>,
    /// Property [`LogDeliveryBucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html#cfn-cloudformation-publictypeversion-logdeliverybucket).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_delivery_bucket: Option<::Value<String>>,
    /// Property [`PublicVersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html#cfn-cloudformation-publictypeversion-publicversionnumber).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub public_version_number: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html#cfn-cloudformation-publictypeversion-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html#cfn-cloudformation-publictypeversion-typename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name: Option<::Value<String>>,
}

impl ::serde::Serialize for PublicTypeVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref arn) = self.arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
        }
        if let Some(ref log_delivery_bucket) = self.log_delivery_bucket {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDeliveryBucket", log_delivery_bucket)?;
        }
        if let Some(ref public_version_number) = self.public_version_number {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicVersionNumber", public_version_number)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        if let Some(ref type_name) = self.type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublicTypeVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicTypeVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublicTypeVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublicTypeVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut arn: Option<::Value<String>> = None;
                let mut log_delivery_bucket: Option<::Value<String>> = None;
                let mut public_version_number: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut type_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Arn" => {
                            arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogDeliveryBucket" => {
                            log_delivery_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicVersionNumber" => {
                            public_version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublicTypeVersionProperties {
                    arn: arn,
                    log_delivery_bucket: log_delivery_bucket,
                    public_version_number: public_version_number,
                    r#type: r#type,
                    type_name: type_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PublicTypeVersion {
    type Properties = PublicTypeVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::PublicTypeVersion";
    fn properties(&self) -> &PublicTypeVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublicTypeVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PublicTypeVersion {}

impl From<PublicTypeVersionProperties> for PublicTypeVersion {
    fn from(properties: PublicTypeVersionProperties) -> PublicTypeVersion {
        PublicTypeVersion { properties }
    }
}

/// The [`AWS::CloudFormation::Publisher`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publisher.html) resource type.
#[derive(Debug, Default)]
pub struct Publisher {
    properties: PublisherProperties
}

/// Properties for the `Publisher` resource.
#[derive(Debug, Default)]
pub struct PublisherProperties {
    /// Property [`AcceptTermsAndConditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publisher.html#cfn-cloudformation-publisher-accepttermsandconditions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accept_terms_and_conditions: ::Value<bool>,
    /// Property [`ConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publisher.html#cfn-cloudformation-publisher-connectionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connection_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for PublisherProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptTermsAndConditions", &self.accept_terms_and_conditions)?;
        if let Some(ref connection_arn) = self.connection_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionArn", connection_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublisherProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublisherProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublisherProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublisherProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_terms_and_conditions: Option<::Value<bool>> = None;
                let mut connection_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptTermsAndConditions" => {
                            accept_terms_and_conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionArn" => {
                            connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublisherProperties {
                    accept_terms_and_conditions: accept_terms_and_conditions.ok_or(::serde::de::Error::missing_field("AcceptTermsAndConditions"))?,
                    connection_arn: connection_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Publisher {
    type Properties = PublisherProperties;
    const TYPE: &'static str = "AWS::CloudFormation::Publisher";
    fn properties(&self) -> &PublisherProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublisherProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Publisher {}

impl From<PublisherProperties> for Publisher {
    fn from(properties: PublisherProperties) -> Publisher {
        Publisher { properties }
    }
}

/// The [`AWS::CloudFormation::ResourceDefaultVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourcedefaultversion.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceDefaultVersion {
    properties: ResourceDefaultVersionProperties
}

/// Properties for the `ResourceDefaultVersion` resource.
#[derive(Debug, Default)]
pub struct ResourceDefaultVersionProperties {
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourcedefaultversion.html#cfn-cloudformation-resourcedefaultversion-typename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_name: Option<::Value<String>>,
    /// Property [`TypeVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourcedefaultversion.html#cfn-cloudformation-resourcedefaultversion-typeversionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_version_arn: Option<::Value<String>>,
    /// Property [`VersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourcedefaultversion.html#cfn-cloudformation-resourcedefaultversion-versionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ResourceDefaultVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref type_name) = self.type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
        }
        if let Some(ref type_version_arn) = self.type_version_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeVersionArn", type_version_arn)?;
        }
        if let Some(ref version_id) = self.version_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionId", version_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceDefaultVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDefaultVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceDefaultVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceDefaultVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut type_name: Option<::Value<String>> = None;
                let mut type_version_arn: Option<::Value<String>> = None;
                let mut version_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeVersionArn" => {
                            type_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionId" => {
                            version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceDefaultVersionProperties {
                    type_name: type_name,
                    type_version_arn: type_version_arn,
                    version_id: version_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceDefaultVersion {
    type Properties = ResourceDefaultVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::ResourceDefaultVersion";
    fn properties(&self) -> &ResourceDefaultVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceDefaultVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceDefaultVersion {}

impl From<ResourceDefaultVersionProperties> for ResourceDefaultVersion {
    fn from(properties: ResourceDefaultVersionProperties) -> ResourceDefaultVersion {
        ResourceDefaultVersion { properties }
    }
}

/// The [`AWS::CloudFormation::ResourceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceVersion {
    properties: ResourceVersionProperties
}

/// Properties for the `ResourceVersion` resource.
#[derive(Debug, Default)]
pub struct ResourceVersionProperties {
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html#cfn-cloudformation-resourceversion-executionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html#cfn-cloudformation-resourceversion-loggingconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub logging_config: Option<::Value<self::resource_version::LoggingConfig>>,
    /// Property [`SchemaHandlerPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html#cfn-cloudformation-resourceversion-schemahandlerpackage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_handler_package: ::Value<String>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html#cfn-cloudformation-resourceversion-typename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name: ::Value<String>,
}

impl ::serde::Serialize for ResourceVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref logging_config) = self.logging_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfig", logging_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaHandlerPackage", &self.schema_handler_package)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", &self.type_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut logging_config: Option<::Value<self::resource_version::LoggingConfig>> = None;
                let mut schema_handler_package: Option<::Value<String>> = None;
                let mut type_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfig" => {
                            logging_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaHandlerPackage" => {
                            schema_handler_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceVersionProperties {
                    execution_role_arn: execution_role_arn,
                    logging_config: logging_config,
                    schema_handler_package: schema_handler_package.ok_or(::serde::de::Error::missing_field("SchemaHandlerPackage"))?,
                    type_name: type_name.ok_or(::serde::de::Error::missing_field("TypeName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceVersion {
    type Properties = ResourceVersionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::ResourceVersion";
    fn properties(&self) -> &ResourceVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceVersion {}

impl From<ResourceVersionProperties> for ResourceVersion {
    fn from(properties: ResourceVersionProperties) -> ResourceVersion {
        ResourceVersion { properties }
    }
}

/// The [`AWS::CloudFormation::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html) resource type.
#[derive(Debug, Default)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Default)]
pub struct StackProperties {
    /// Property [`NotificationARNs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html#cfn-cloudformation-stack-notificationarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_ar_ns: Option<::ValueList<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html#cfn-cloudformation-stack-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueMap<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html#cfn-cloudformation-stack-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html#cfn-cloudformation-stack-templateurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_url: ::Value<String>,
    /// Property [`TimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html#cfn-cloudformation-stack-timeoutinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout_in_minutes: Option<::Value<u32>>,
}

impl ::serde::Serialize for StackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref notification_ar_ns) = self.notification_ar_ns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationARNs", notification_ar_ns)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateURL", &self.template_url)?;
        if let Some(ref timeout_in_minutes) = self.timeout_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMinutes", timeout_in_minutes)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut notification_ar_ns: Option<::ValueList<String>> = None;
                let mut parameters: Option<::ValueMap<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_url: Option<::Value<String>> = None;
                let mut timeout_in_minutes: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "NotificationARNs" => {
                            notification_ar_ns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateURL" => {
                            template_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeoutInMinutes" => {
                            timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackProperties {
                    notification_ar_ns: notification_ar_ns,
                    parameters: parameters,
                    tags: tags,
                    template_url: template_url.ok_or(::serde::de::Error::missing_field("TemplateURL"))?,
                    timeout_in_minutes: timeout_in_minutes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Stack {
    type Properties = StackProperties;
    const TYPE: &'static str = "AWS::CloudFormation::Stack";
    fn properties(&self) -> &StackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stack {}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::CloudFormation::StackSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html) resource type.
#[derive(Debug, Default)]
pub struct StackSet {
    properties: StackSetProperties
}

/// Properties for the `StackSet` resource.
#[derive(Debug, Default)]
pub struct StackSetProperties {
    /// Property [`AdministrationRoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-administrationrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub administration_role_arn: Option<::Value<String>>,
    /// Property [`AutoDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-autodeployment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_deployment: Option<::Value<self::stack_set::AutoDeployment>>,
    /// Property [`CallAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-callas).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub call_as: Option<::Value<String>>,
    /// Property [`Capabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-capabilities).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capabilities: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExecutionRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-executionrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role_name: Option<::Value<String>>,
    /// Property [`ManagedExecution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-managedexecution).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_execution: Option<::Value<::json::Value>>,
    /// Property [`OperationPreferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-operationpreferences).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub operation_preferences: Option<::Value<self::stack_set::OperationPreferences>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueList<self::stack_set::Parameter>>,
    /// Property [`PermissionModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-permissionmodel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub permission_model: ::Value<String>,
    /// Property [`StackInstancesGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-stackinstancesgroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stack_instances_group: Option<::ValueList<self::stack_set::StackInstances>>,
    /// Property [`StackSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-stacksetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_set_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-templatebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_body: Option<::Value<String>>,
    /// Property [`TemplateURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html#cfn-cloudformation-stackset-templateurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_url: Option<::Value<String>>,
}

impl ::serde::Serialize for StackSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref administration_role_arn) = self.administration_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdministrationRoleARN", administration_role_arn)?;
        }
        if let Some(ref auto_deployment) = self.auto_deployment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoDeployment", auto_deployment)?;
        }
        if let Some(ref call_as) = self.call_as {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CallAs", call_as)?;
        }
        if let Some(ref capabilities) = self.capabilities {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capabilities", capabilities)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref execution_role_name) = self.execution_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleName", execution_role_name)?;
        }
        if let Some(ref managed_execution) = self.managed_execution {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedExecution", managed_execution)?;
        }
        if let Some(ref operation_preferences) = self.operation_preferences {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperationPreferences", operation_preferences)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionModel", &self.permission_model)?;
        if let Some(ref stack_instances_group) = self.stack_instances_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackInstancesGroup", stack_instances_group)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetName", &self.stack_set_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_body) = self.template_body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateBody", template_body)?;
        }
        if let Some(ref template_url) = self.template_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateURL", template_url)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut administration_role_arn: Option<::Value<String>> = None;
                let mut auto_deployment: Option<::Value<self::stack_set::AutoDeployment>> = None;
                let mut call_as: Option<::Value<String>> = None;
                let mut capabilities: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut execution_role_name: Option<::Value<String>> = None;
                let mut managed_execution: Option<::Value<::json::Value>> = None;
                let mut operation_preferences: Option<::Value<self::stack_set::OperationPreferences>> = None;
                let mut parameters: Option<::ValueList<self::stack_set::Parameter>> = None;
                let mut permission_model: Option<::Value<String>> = None;
                let mut stack_instances_group: Option<::ValueList<self::stack_set::StackInstances>> = None;
                let mut stack_set_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_body: Option<::Value<String>> = None;
                let mut template_url: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdministrationRoleARN" => {
                            administration_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoDeployment" => {
                            auto_deployment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CallAs" => {
                            call_as = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Capabilities" => {
                            capabilities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleName" => {
                            execution_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedExecution" => {
                            managed_execution = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OperationPreferences" => {
                            operation_preferences = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionModel" => {
                            permission_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackInstancesGroup" => {
                            stack_instances_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackSetName" => {
                            stack_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateBody" => {
                            template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateURL" => {
                            template_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackSetProperties {
                    administration_role_arn: administration_role_arn,
                    auto_deployment: auto_deployment,
                    call_as: call_as,
                    capabilities: capabilities,
                    description: description,
                    execution_role_name: execution_role_name,
                    managed_execution: managed_execution,
                    operation_preferences: operation_preferences,
                    parameters: parameters,
                    permission_model: permission_model.ok_or(::serde::de::Error::missing_field("PermissionModel"))?,
                    stack_instances_group: stack_instances_group,
                    stack_set_name: stack_set_name.ok_or(::serde::de::Error::missing_field("StackSetName"))?,
                    tags: tags,
                    template_body: template_body,
                    template_url: template_url,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StackSet {
    type Properties = StackSetProperties;
    const TYPE: &'static str = "AWS::CloudFormation::StackSet";
    fn properties(&self) -> &StackSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StackSet {}

impl From<StackSetProperties> for StackSet {
    fn from(properties: StackSetProperties) -> StackSet {
        StackSet { properties }
    }
}

/// The [`AWS::CloudFormation::TypeActivation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html) resource type.
#[derive(Debug, Default)]
pub struct TypeActivation {
    properties: TypeActivationProperties
}

/// Properties for the `TypeActivation` resource.
#[derive(Debug, Default)]
pub struct TypeActivationProperties {
    /// Property [`AutoUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-autoupdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_update: Option<::Value<bool>>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-executionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-loggingconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub logging_config: Option<::Value<self::type_activation::LoggingConfig>>,
    /// Property [`MajorVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-majorversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub major_version: Option<::Value<String>>,
    /// Property [`PublicTypeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-publictypearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub public_type_arn: Option<::Value<String>>,
    /// Property [`PublisherId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-publisherid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub publisher_id: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-typename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name: Option<::Value<String>>,
    /// Property [`TypeNameAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-typenamealias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name_alias: Option<::Value<String>>,
    /// Property [`VersionBump`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html#cfn-cloudformation-typeactivation-versionbump).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_bump: Option<::Value<String>>,
}

impl ::serde::Serialize for TypeActivationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_update) = self.auto_update {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoUpdate", auto_update)?;
        }
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref logging_config) = self.logging_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfig", logging_config)?;
        }
        if let Some(ref major_version) = self.major_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MajorVersion", major_version)?;
        }
        if let Some(ref public_type_arn) = self.public_type_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicTypeArn", public_type_arn)?;
        }
        if let Some(ref publisher_id) = self.publisher_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublisherId", publisher_id)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        if let Some(ref type_name) = self.type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
        }
        if let Some(ref type_name_alias) = self.type_name_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeNameAlias", type_name_alias)?;
        }
        if let Some(ref version_bump) = self.version_bump {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionBump", version_bump)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TypeActivationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TypeActivationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TypeActivationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TypeActivationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_update: Option<::Value<bool>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut logging_config: Option<::Value<self::type_activation::LoggingConfig>> = None;
                let mut major_version: Option<::Value<String>> = None;
                let mut public_type_arn: Option<::Value<String>> = None;
                let mut publisher_id: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut type_name: Option<::Value<String>> = None;
                let mut type_name_alias: Option<::Value<String>> = None;
                let mut version_bump: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoUpdate" => {
                            auto_update = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfig" => {
                            logging_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MajorVersion" => {
                            major_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicTypeArn" => {
                            public_type_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublisherId" => {
                            publisher_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeNameAlias" => {
                            type_name_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionBump" => {
                            version_bump = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TypeActivationProperties {
                    auto_update: auto_update,
                    execution_role_arn: execution_role_arn,
                    logging_config: logging_config,
                    major_version: major_version,
                    public_type_arn: public_type_arn,
                    publisher_id: publisher_id,
                    r#type: r#type,
                    type_name: type_name,
                    type_name_alias: type_name_alias,
                    version_bump: version_bump,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TypeActivation {
    type Properties = TypeActivationProperties;
    const TYPE: &'static str = "AWS::CloudFormation::TypeActivation";
    fn properties(&self) -> &TypeActivationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TypeActivationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TypeActivation {}

impl From<TypeActivationProperties> for TypeActivation {
    fn from(properties: TypeActivationProperties) -> TypeActivation {
        TypeActivation { properties }
    }
}

/// The [`AWS::CloudFormation::WaitCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html) resource type.
#[derive(Debug, Default)]
pub struct WaitCondition {
    properties: WaitConditionProperties
}

/// Properties for the `WaitCondition` resource.
#[derive(Debug, Default)]
pub struct WaitConditionProperties {
    /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html#cfn-waitcondition-count).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub count: Option<::Value<u32>>,
    /// Property [`Handle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html#cfn-waitcondition-handle).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub handle: Option<::Value<String>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html#cfn-waitcondition-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<::Value<String>>,
}

impl ::serde::Serialize for WaitConditionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref count) = self.count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
        }
        if let Some(ref handle) = self.handle {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handle", handle)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WaitConditionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WaitConditionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WaitConditionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WaitConditionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut count: Option<::Value<u32>> = None;
                let mut handle: Option<::Value<String>> = None;
                let mut timeout: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Count" => {
                            count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Handle" => {
                            handle = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WaitConditionProperties {
                    count: count,
                    handle: handle,
                    timeout: timeout,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WaitCondition {
    type Properties = WaitConditionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::WaitCondition";
    fn properties(&self) -> &WaitConditionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WaitConditionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WaitCondition {}

impl From<WaitConditionProperties> for WaitCondition {
    fn from(properties: WaitConditionProperties) -> WaitCondition {
        WaitCondition { properties }
    }
}

/// The [`AWS::CloudFormation::WaitConditionHandle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitconditionhandle.html) resource type.
#[derive(Debug, Default)]
pub struct WaitConditionHandle {
    properties: WaitConditionHandleProperties
}

/// Properties for the `WaitConditionHandle` resource.
#[derive(Debug, Default)]
pub struct WaitConditionHandleProperties {
}

impl ::serde::Serialize for WaitConditionHandleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WaitConditionHandleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WaitConditionHandleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WaitConditionHandleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WaitConditionHandleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                Ok(WaitConditionHandleProperties {})
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WaitConditionHandle {
    type Properties = WaitConditionHandleProperties;
    const TYPE: &'static str = "AWS::CloudFormation::WaitConditionHandle";
    fn properties(&self) -> &WaitConditionHandleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WaitConditionHandleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WaitConditionHandle {}

impl From<WaitConditionHandleProperties> for WaitConditionHandle {
    fn from(properties: WaitConditionHandleProperties) -> WaitConditionHandle {
        WaitConditionHandle { properties }
    }
}

pub mod hook_version {
    //! Property types for the `HookVersion` resource.

    /// The [`AWS::CloudFormation::HookVersion.LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-hookversion-loggingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfig {
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-hookversion-loggingconfig.html#cfn-cloudformation-hookversion-loggingconfig-loggroupname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-hookversion-loggingconfig.html#cfn-cloudformation-hookversion-loggingconfig-logrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_role_arn) = self.log_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogRoleArn", log_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogRoleArn" => {
                                log_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfig {
                        log_group_name: log_group_name,
                        log_role_arn: log_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resource_version {
    //! Property types for the `ResourceVersion` resource.

    /// The [`AWS::CloudFormation::ResourceVersion.LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-resourceversion-loggingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfig {
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-resourceversion-loggingconfig.html#cfn-cloudformation-resourceversion-loggingconfig-loggroupname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-resourceversion-loggingconfig.html#cfn-cloudformation-resourceversion-loggingconfig-logrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_role_arn) = self.log_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogRoleArn", log_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogRoleArn" => {
                                log_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfig {
                        log_group_name: log_group_name,
                        log_role_arn: log_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod stack_set {
    //! Property types for the `StackSet` resource.

    /// The [`AWS::CloudFormation::StackSet.AutoDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-autodeployment.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoDeployment {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-autodeployment.html#cfn-cloudformation-stackset-autodeployment-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`RetainStacksOnAccountRemoval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-autodeployment.html#cfn-cloudformation-stackset-autodeployment-retainstacksonaccountremoval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retain_stacks_on_account_removal: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AutoDeployment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref retain_stacks_on_account_removal) = self.retain_stacks_on_account_removal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainStacksOnAccountRemoval", retain_stacks_on_account_removal)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoDeployment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoDeployment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoDeployment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoDeployment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut retain_stacks_on_account_removal: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetainStacksOnAccountRemoval" => {
                                retain_stacks_on_account_removal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoDeployment {
                        enabled: enabled,
                        retain_stacks_on_account_removal: retain_stacks_on_account_removal,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFormation::StackSet.DeploymentTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-deploymenttargets.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentTargets {
        /// Property [`Accounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-deploymenttargets.html#cfn-cloudformation-stackset-deploymenttargets-accounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accounts: Option<::ValueList<String>>,
        /// Property [`OrganizationalUnitIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-deploymenttargets.html#cfn-cloudformation-stackset-deploymenttargets-organizationalunitids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organizational_unit_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DeploymentTargets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accounts) = self.accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Accounts", accounts)?;
            }
            if let Some(ref organizational_unit_ids) = self.organizational_unit_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitIds", organizational_unit_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentTargets {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentTargets, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentTargets;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentTargets")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accounts: Option<::ValueList<String>> = None;
                    let mut organizational_unit_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Accounts" => {
                                accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitIds" => {
                                organizational_unit_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentTargets {
                        accounts: accounts,
                        organizational_unit_ids: organizational_unit_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFormation::StackSet.OperationPreferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html) property type.
    #[derive(Debug, Default)]
    pub struct OperationPreferences {
        /// Property [`FailureToleranceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-failuretolerancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_tolerance_count: Option<::Value<u32>>,
        /// Property [`FailureTolerancePercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-failuretolerancepercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_tolerance_percentage: Option<::Value<u32>>,
        /// Property [`MaxConcurrentCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-maxconcurrentcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_concurrent_count: Option<::Value<u32>>,
        /// Property [`MaxConcurrentPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-maxconcurrentpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_concurrent_percentage: Option<::Value<u32>>,
        /// Property [`RegionConcurrencyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-regionconcurrencytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_concurrency_type: Option<::Value<String>>,
        /// Property [`RegionOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html#cfn-cloudformation-stackset-operationpreferences-regionorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_order: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for OperationPreferences {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_tolerance_count) = self.failure_tolerance_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureToleranceCount", failure_tolerance_count)?;
            }
            if let Some(ref failure_tolerance_percentage) = self.failure_tolerance_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureTolerancePercentage", failure_tolerance_percentage)?;
            }
            if let Some(ref max_concurrent_count) = self.max_concurrent_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentCount", max_concurrent_count)?;
            }
            if let Some(ref max_concurrent_percentage) = self.max_concurrent_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentPercentage", max_concurrent_percentage)?;
            }
            if let Some(ref region_concurrency_type) = self.region_concurrency_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionConcurrencyType", region_concurrency_type)?;
            }
            if let Some(ref region_order) = self.region_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionOrder", region_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OperationPreferences {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OperationPreferences, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OperationPreferences;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OperationPreferences")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_tolerance_count: Option<::Value<u32>> = None;
                    let mut failure_tolerance_percentage: Option<::Value<u32>> = None;
                    let mut max_concurrent_count: Option<::Value<u32>> = None;
                    let mut max_concurrent_percentage: Option<::Value<u32>> = None;
                    let mut region_concurrency_type: Option<::Value<String>> = None;
                    let mut region_order: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureToleranceCount" => {
                                failure_tolerance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureTolerancePercentage" => {
                                failure_tolerance_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConcurrentCount" => {
                                max_concurrent_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConcurrentPercentage" => {
                                max_concurrent_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionConcurrencyType" => {
                                region_concurrency_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionOrder" => {
                                region_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OperationPreferences {
                        failure_tolerance_count: failure_tolerance_count,
                        failure_tolerance_percentage: failure_tolerance_percentage,
                        max_concurrent_count: max_concurrent_count,
                        max_concurrent_percentage: max_concurrent_percentage,
                        region_concurrency_type: region_concurrency_type,
                        region_order: region_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFormation::StackSet.Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-parameter.html) property type.
    #[derive(Debug, Default)]
    pub struct Parameter {
        /// Property [`ParameterKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-parameter.html#cfn-cloudformation-stackset-parameter-parameterkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_key: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-parameter.html#cfn-cloudformation-stackset-parameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Parameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterKey", &self.parameter_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_key: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterKey" => {
                                parameter_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameter {
                        parameter_key: parameter_key.ok_or(::serde::de::Error::missing_field("ParameterKey"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFormation::StackSet.StackInstances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-stackinstances.html) property type.
    #[derive(Debug, Default)]
    pub struct StackInstances {
        /// Property [`DeploymentTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-stackinstances.html#cfn-cloudformation-stackset-stackinstances-deploymenttargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deployment_targets: ::Value<DeploymentTargets>,
        /// Property [`ParameterOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-stackinstances.html#cfn-cloudformation-stackset-stackinstances-parameteroverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_overrides: Option<::ValueList<Parameter>>,
        /// Property [`Regions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-stackinstances.html#cfn-cloudformation-stackset-stackinstances-regions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regions: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for StackInstances {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentTargets", &self.deployment_targets)?;
            if let Some(ref parameter_overrides) = self.parameter_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterOverrides", parameter_overrides)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regions", &self.regions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StackInstances {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StackInstances, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StackInstances;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StackInstances")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut deployment_targets: Option<::Value<DeploymentTargets>> = None;
                    let mut parameter_overrides: Option<::ValueList<Parameter>> = None;
                    let mut regions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeploymentTargets" => {
                                deployment_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterOverrides" => {
                                parameter_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regions" => {
                                regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StackInstances {
                        deployment_targets: deployment_targets.ok_or(::serde::de::Error::missing_field("DeploymentTargets"))?,
                        parameter_overrides: parameter_overrides,
                        regions: regions.ok_or(::serde::de::Error::missing_field("Regions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod type_activation {
    //! Property types for the `TypeActivation` resource.

    /// The [`AWS::CloudFormation::TypeActivation.LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-typeactivation-loggingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfig {
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-typeactivation-loggingconfig.html#cfn-cloudformation-typeactivation-loggingconfig-loggroupname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-typeactivation-loggingconfig.html#cfn-cloudformation-typeactivation-loggingconfig-logrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_role_arn) = self.log_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogRoleArn", log_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogRoleArn" => {
                                log_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfig {
                        log_group_name: log_group_name,
                        log_role_arn: log_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
