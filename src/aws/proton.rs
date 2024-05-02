//! Types for the `Proton` service.

/// The [`AWS::Proton::EnvironmentAccountConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html) resource type.
#[derive(Debug, Default)]
pub struct EnvironmentAccountConnection {
    properties: EnvironmentAccountConnectionProperties
}

/// Properties for the `EnvironmentAccountConnection` resource.
#[derive(Debug, Default)]
pub struct EnvironmentAccountConnectionProperties {
    /// Property [`CodebuildRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-codebuildrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub codebuild_role_arn: Option<::Value<String>>,
    /// Property [`ComponentRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-componentrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub component_role_arn: Option<::Value<String>>,
    /// Property [`EnvironmentAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-environmentaccountid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_account_id: Option<::Value<String>>,
    /// Property [`EnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-environmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_name: Option<::Value<String>>,
    /// Property [`ManagementAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-managementaccountid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub management_account_id: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmentaccountconnection.html#cfn-proton-environmentaccountconnection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EnvironmentAccountConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref codebuild_role_arn) = self.codebuild_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodebuildRoleArn", codebuild_role_arn)?;
        }
        if let Some(ref component_role_arn) = self.component_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentRoleArn", component_role_arn)?;
        }
        if let Some(ref environment_account_id) = self.environment_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentAccountId", environment_account_id)?;
        }
        if let Some(ref environment_name) = self.environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentName", environment_name)?;
        }
        if let Some(ref management_account_id) = self.management_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagementAccountId", management_account_id)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentAccountConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentAccountConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentAccountConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentAccountConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut codebuild_role_arn: Option<::Value<String>> = None;
                let mut component_role_arn: Option<::Value<String>> = None;
                let mut environment_account_id: Option<::Value<String>> = None;
                let mut environment_name: Option<::Value<String>> = None;
                let mut management_account_id: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CodebuildRoleArn" => {
                            codebuild_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComponentRoleArn" => {
                            component_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentAccountId" => {
                            environment_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentName" => {
                            environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagementAccountId" => {
                            management_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(EnvironmentAccountConnectionProperties {
                    codebuild_role_arn: codebuild_role_arn,
                    component_role_arn: component_role_arn,
                    environment_account_id: environment_account_id,
                    environment_name: environment_name,
                    management_account_id: management_account_id,
                    role_arn: role_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnvironmentAccountConnection {
    type Properties = EnvironmentAccountConnectionProperties;
    const TYPE: &'static str = "AWS::Proton::EnvironmentAccountConnection";
    fn properties(&self) -> &EnvironmentAccountConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentAccountConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentAccountConnection {}

impl From<EnvironmentAccountConnectionProperties> for EnvironmentAccountConnection {
    fn from(properties: EnvironmentAccountConnectionProperties) -> EnvironmentAccountConnection {
        EnvironmentAccountConnection { properties }
    }
}

/// The [`AWS::Proton::EnvironmentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html) resource type.
#[derive(Debug, Default)]
pub struct EnvironmentTemplate {
    properties: EnvironmentTemplateProperties
}

/// Properties for the `EnvironmentTemplate` resource.
#[derive(Debug, Default)]
pub struct EnvironmentTemplateProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-encryptionkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_key: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Provisioning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-provisioning).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provisioning: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-environmenttemplate.html#cfn-proton-environmenttemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EnvironmentTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref provisioning) = self.provisioning {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provisioning", provisioning)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut provisioning: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKey" => {
                            encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Provisioning" => {
                            provisioning = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentTemplateProperties {
                    description: description,
                    display_name: display_name,
                    encryption_key: encryption_key,
                    name: name,
                    provisioning: provisioning,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnvironmentTemplate {
    type Properties = EnvironmentTemplateProperties;
    const TYPE: &'static str = "AWS::Proton::EnvironmentTemplate";
    fn properties(&self) -> &EnvironmentTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentTemplate {}

impl From<EnvironmentTemplateProperties> for EnvironmentTemplate {
    fn from(properties: EnvironmentTemplateProperties) -> EnvironmentTemplate {
        EnvironmentTemplate { properties }
    }
}

/// The [`AWS::Proton::ServiceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html) resource type.
#[derive(Debug, Default)]
pub struct ServiceTemplate {
    properties: ServiceTemplateProperties
}

/// Properties for the `ServiceTemplate` resource.
#[derive(Debug, Default)]
pub struct ServiceTemplateProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-encryptionkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_key: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`PipelineProvisioning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-pipelineprovisioning).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pipeline_provisioning: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-proton-servicetemplate.html#cfn-proton-servicetemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ServiceTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref pipeline_provisioning) = self.pipeline_provisioning {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineProvisioning", pipeline_provisioning)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut pipeline_provisioning: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKey" => {
                            encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineProvisioning" => {
                            pipeline_provisioning = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceTemplateProperties {
                    description: description,
                    display_name: display_name,
                    encryption_key: encryption_key,
                    name: name,
                    pipeline_provisioning: pipeline_provisioning,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServiceTemplate {
    type Properties = ServiceTemplateProperties;
    const TYPE: &'static str = "AWS::Proton::ServiceTemplate";
    fn properties(&self) -> &ServiceTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServiceTemplate {}

impl From<ServiceTemplateProperties> for ServiceTemplate {
    fn from(properties: ServiceTemplateProperties) -> ServiceTemplate {
        ServiceTemplate { properties }
    }
}
