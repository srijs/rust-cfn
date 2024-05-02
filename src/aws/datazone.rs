//! Types for the `DataZone` service.

/// The [`AWS::DataZone::DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html) resource type.
#[derive(Debug, Default)]
pub struct DataSource {
    properties: DataSourceProperties
}

/// Properties for the `DataSource` resource.
#[derive(Debug, Default)]
pub struct DataSourceProperties {
    /// Property [`AssetFormsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-assetformsinput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_forms_input: Option<::ValueList<self::data_source::FormInput>>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<self::data_source::DataSourceConfigurationInput>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`EnableSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-enablesetting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_setting: Option<::Value<String>>,
    /// Property [`EnvironmentIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-environmentidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_identifier: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ProjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-projectidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_identifier: ::Value<String>,
    /// Property [`PublishOnImport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-publishonimport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publish_on_import: Option<::Value<bool>>,
    /// Property [`Recommendation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-recommendation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recommendation: Option<::Value<self::data_source::RecommendationConfiguration>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<self::data_source::ScheduleConfiguration>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html#cfn-datazone-datasource-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for DataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref asset_forms_input) = self.asset_forms_input {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetFormsInput", asset_forms_input)?;
        }
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        if let Some(ref enable_setting) = self.enable_setting {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSetting", enable_setting)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentIdentifier", &self.environment_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectIdentifier", &self.project_identifier)?;
        if let Some(ref publish_on_import) = self.publish_on_import {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishOnImport", publish_on_import)?;
        }
        if let Some(ref recommendation) = self.recommendation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recommendation", recommendation)?;
        }
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut asset_forms_input: Option<::ValueList<self::data_source::FormInput>> = None;
                let mut configuration: Option<::Value<self::data_source::DataSourceConfigurationInput>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut enable_setting: Option<::Value<String>> = None;
                let mut environment_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut project_identifier: Option<::Value<String>> = None;
                let mut publish_on_import: Option<::Value<bool>> = None;
                let mut recommendation: Option<::Value<self::data_source::RecommendationConfiguration>> = None;
                let mut schedule: Option<::Value<self::data_source::ScheduleConfiguration>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssetFormsInput" => {
                            asset_forms_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableSetting" => {
                            enable_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentIdentifier" => {
                            environment_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectIdentifier" => {
                            project_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublishOnImport" => {
                            publish_on_import = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Recommendation" => {
                            recommendation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataSourceProperties {
                    asset_forms_input: asset_forms_input,
                    configuration: configuration,
                    description: description,
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    enable_setting: enable_setting,
                    environment_identifier: environment_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentIdentifier"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    project_identifier: project_identifier.ok_or(::serde::de::Error::missing_field("ProjectIdentifier"))?,
                    publish_on_import: publish_on_import,
                    recommendation: recommendation,
                    schedule: schedule,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataSource {
    type Properties = DataSourceProperties;
    const TYPE: &'static str = "AWS::DataZone::DataSource";
    fn properties(&self) -> &DataSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataSource {}

impl From<DataSourceProperties> for DataSource {
    fn from(properties: DataSourceProperties) -> DataSource {
        DataSource { properties }
    }
}

/// The [`AWS::DataZone::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-domainexecutionrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_execution_role: ::Value<String>,
    /// Property [`KmsKeyIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-kmskeyidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_identifier: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SingleSignOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-singlesignon).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub single_sign_on: Option<::Value<self::domain::SingleSignOn>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html#cfn-datazone-domain-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainExecutionRole", &self.domain_execution_role)?;
        if let Some(ref kms_key_identifier) = self.kms_key_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyIdentifier", kms_key_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref single_sign_on) = self.single_sign_on {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSignOn", single_sign_on)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut domain_execution_role: Option<::Value<String>> = None;
                let mut kms_key_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut single_sign_on: Option<::Value<self::domain::SingleSignOn>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainExecutionRole" => {
                            domain_execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyIdentifier" => {
                            kms_key_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SingleSignOn" => {
                            single_sign_on = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    description: description,
                    domain_execution_role: domain_execution_role.ok_or(::serde::de::Error::missing_field("DomainExecutionRole"))?,
                    kms_key_identifier: kms_key_identifier,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    single_sign_on: single_sign_on,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::DataZone::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

/// The [`AWS::DataZone::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html) resource type.
#[derive(Debug, Default)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`EnvironmentProfileIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-environmentprofileidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_profile_identifier: ::Value<String>,
    /// Property [`GlossaryTerms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-glossaryterms).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub glossary_terms: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ProjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-projectidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_identifier: ::Value<String>,
    /// Property [`UserParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html#cfn-datazone-environment-userparameters).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_parameters: Option<::ValueList<self::environment::EnvironmentParameter>>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentProfileIdentifier", &self.environment_profile_identifier)?;
        if let Some(ref glossary_terms) = self.glossary_terms {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlossaryTerms", glossary_terms)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectIdentifier", &self.project_identifier)?;
        if let Some(ref user_parameters) = self.user_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserParameters", user_parameters)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut environment_profile_identifier: Option<::Value<String>> = None;
                let mut glossary_terms: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut project_identifier: Option<::Value<String>> = None;
                let mut user_parameters: Option<::ValueList<self::environment::EnvironmentParameter>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentProfileIdentifier" => {
                            environment_profile_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlossaryTerms" => {
                            glossary_terms = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectIdentifier" => {
                            project_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserParameters" => {
                            user_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    description: description,
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    environment_profile_identifier: environment_profile_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentProfileIdentifier"))?,
                    glossary_terms: glossary_terms,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    project_identifier: project_identifier.ok_or(::serde::de::Error::missing_field("ProjectIdentifier"))?,
                    user_parameters: user_parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::DataZone::Environment";
    fn properties(&self) -> &EnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Environment {}

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

/// The [`AWS::DataZone::EnvironmentBlueprintConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct EnvironmentBlueprintConfiguration {
    properties: EnvironmentBlueprintConfigurationProperties
}

/// Properties for the `EnvironmentBlueprintConfiguration` resource.
#[derive(Debug, Default)]
pub struct EnvironmentBlueprintConfigurationProperties {
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`EnabledRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-enabledregions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled_regions: ::ValueList<String>,
    /// Property [`EnvironmentBlueprintIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-environmentblueprintidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_blueprint_identifier: ::Value<String>,
    /// Property [`ManageAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-manageaccessrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manage_access_role_arn: Option<::Value<String>>,
    /// Property [`ProvisioningRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-provisioningrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_role_arn: Option<::Value<String>>,
    /// Property [`RegionalParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html#cfn-datazone-environmentblueprintconfiguration-regionalparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regional_parameters: Option<::ValueList<self::environment_blueprint_configuration::RegionalParameter>>,
}

impl ::serde::Serialize for EnvironmentBlueprintConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnabledRegions", &self.enabled_regions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentBlueprintIdentifier", &self.environment_blueprint_identifier)?;
        if let Some(ref manage_access_role_arn) = self.manage_access_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageAccessRoleArn", manage_access_role_arn)?;
        }
        if let Some(ref provisioning_role_arn) = self.provisioning_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningRoleArn", provisioning_role_arn)?;
        }
        if let Some(ref regional_parameters) = self.regional_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionalParameters", regional_parameters)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentBlueprintConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentBlueprintConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentBlueprintConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentBlueprintConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut enabled_regions: Option<::ValueList<String>> = None;
                let mut environment_blueprint_identifier: Option<::Value<String>> = None;
                let mut manage_access_role_arn: Option<::Value<String>> = None;
                let mut provisioning_role_arn: Option<::Value<String>> = None;
                let mut regional_parameters: Option<::ValueList<self::environment_blueprint_configuration::RegionalParameter>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnabledRegions" => {
                            enabled_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentBlueprintIdentifier" => {
                            environment_blueprint_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManageAccessRoleArn" => {
                            manage_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningRoleArn" => {
                            provisioning_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegionalParameters" => {
                            regional_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentBlueprintConfigurationProperties {
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    enabled_regions: enabled_regions.ok_or(::serde::de::Error::missing_field("EnabledRegions"))?,
                    environment_blueprint_identifier: environment_blueprint_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentBlueprintIdentifier"))?,
                    manage_access_role_arn: manage_access_role_arn,
                    provisioning_role_arn: provisioning_role_arn,
                    regional_parameters: regional_parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnvironmentBlueprintConfiguration {
    type Properties = EnvironmentBlueprintConfigurationProperties;
    const TYPE: &'static str = "AWS::DataZone::EnvironmentBlueprintConfiguration";
    fn properties(&self) -> &EnvironmentBlueprintConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentBlueprintConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentBlueprintConfiguration {}

impl From<EnvironmentBlueprintConfigurationProperties> for EnvironmentBlueprintConfiguration {
    fn from(properties: EnvironmentBlueprintConfigurationProperties) -> EnvironmentBlueprintConfiguration {
        EnvironmentBlueprintConfiguration { properties }
    }
}

/// The [`AWS::DataZone::EnvironmentProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html) resource type.
#[derive(Debug, Default)]
pub struct EnvironmentProfile {
    properties: EnvironmentProfileProperties
}

/// Properties for the `EnvironmentProfile` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProfileProperties {
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-awsaccountid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aws_account_id: ::Value<String>,
    /// Property [`AwsAccountRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-awsaccountregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aws_account_region: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`EnvironmentBlueprintIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-environmentblueprintidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_blueprint_identifier: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ProjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-projectidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_identifier: ::Value<String>,
    /// Property [`UserParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html#cfn-datazone-environmentprofile-userparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_parameters: Option<::ValueList<self::environment_profile::EnvironmentParameter>>,
}

impl ::serde::Serialize for EnvironmentProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountRegion", &self.aws_account_region)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentBlueprintIdentifier", &self.environment_blueprint_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectIdentifier", &self.project_identifier)?;
        if let Some(ref user_parameters) = self.user_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserParameters", user_parameters)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut aws_account_region: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut environment_blueprint_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut project_identifier: Option<::Value<String>> = None;
                let mut user_parameters: Option<::ValueList<self::environment_profile::EnvironmentParameter>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AwsAccountRegion" => {
                            aws_account_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentBlueprintIdentifier" => {
                            environment_blueprint_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectIdentifier" => {
                            project_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserParameters" => {
                            user_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProfileProperties {
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    aws_account_region: aws_account_region.ok_or(::serde::de::Error::missing_field("AwsAccountRegion"))?,
                    description: description,
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    environment_blueprint_identifier: environment_blueprint_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentBlueprintIdentifier"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    project_identifier: project_identifier.ok_or(::serde::de::Error::missing_field("ProjectIdentifier"))?,
                    user_parameters: user_parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnvironmentProfile {
    type Properties = EnvironmentProfileProperties;
    const TYPE: &'static str = "AWS::DataZone::EnvironmentProfile";
    fn properties(&self) -> &EnvironmentProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentProfile {}

impl From<EnvironmentProfileProperties> for EnvironmentProfile {
    fn from(properties: EnvironmentProfileProperties) -> EnvironmentProfile {
        EnvironmentProfile { properties }
    }
}

/// The [`AWS::DataZone::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html#cfn-datazone-project-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html#cfn-datazone-project-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`GlossaryTerms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html#cfn-datazone-project-glossaryterms).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub glossary_terms: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html#cfn-datazone-project-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        if let Some(ref glossary_terms) = self.glossary_terms {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlossaryTerms", glossary_terms)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut glossary_terms: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlossaryTerms" => {
                            glossary_terms = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    description: description,
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    glossary_terms: glossary_terms,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::DataZone::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

/// The [`AWS::DataZone::SubscriptionTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html) resource type.
#[derive(Debug, Default)]
pub struct SubscriptionTarget {
    properties: SubscriptionTargetProperties
}

/// Properties for the `SubscriptionTarget` resource.
#[derive(Debug, Default)]
pub struct SubscriptionTargetProperties {
    /// Property [`ApplicableAssetTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-applicableassettypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub applicable_asset_types: ::ValueList<String>,
    /// Property [`AuthorizedPrincipals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-authorizedprincipals).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorized_principals: ::ValueList<String>,
    /// Property [`DomainIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-domainidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_identifier: ::Value<String>,
    /// Property [`EnvironmentIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-environmentidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_identifier: ::Value<String>,
    /// Property [`ManageAccessRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-manageaccessrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manage_access_role: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Provider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-provider).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provider: Option<::Value<String>>,
    /// Property [`SubscriptionTargetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-subscriptiontargetconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscription_target_config: ::ValueList<self::subscription_target::SubscriptionTargetForm>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html#cfn-datazone-subscriptiontarget-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for SubscriptionTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicableAssetTypes", &self.applicable_asset_types)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizedPrincipals", &self.authorized_principals)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIdentifier", &self.domain_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentIdentifier", &self.environment_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageAccessRole", &self.manage_access_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref provider) = self.provider {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", provider)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionTargetConfig", &self.subscription_target_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubscriptionTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubscriptionTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut applicable_asset_types: Option<::ValueList<String>> = None;
                let mut authorized_principals: Option<::ValueList<String>> = None;
                let mut domain_identifier: Option<::Value<String>> = None;
                let mut environment_identifier: Option<::Value<String>> = None;
                let mut manage_access_role: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut provider: Option<::Value<String>> = None;
                let mut subscription_target_config: Option<::ValueList<self::subscription_target::SubscriptionTargetForm>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicableAssetTypes" => {
                            applicable_asset_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizedPrincipals" => {
                            authorized_principals = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIdentifier" => {
                            domain_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentIdentifier" => {
                            environment_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManageAccessRole" => {
                            manage_access_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Provider" => {
                            provider = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubscriptionTargetConfig" => {
                            subscription_target_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubscriptionTargetProperties {
                    applicable_asset_types: applicable_asset_types.ok_or(::serde::de::Error::missing_field("ApplicableAssetTypes"))?,
                    authorized_principals: authorized_principals.ok_or(::serde::de::Error::missing_field("AuthorizedPrincipals"))?,
                    domain_identifier: domain_identifier.ok_or(::serde::de::Error::missing_field("DomainIdentifier"))?,
                    environment_identifier: environment_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentIdentifier"))?,
                    manage_access_role: manage_access_role.ok_or(::serde::de::Error::missing_field("ManageAccessRole"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    provider: provider,
                    subscription_target_config: subscription_target_config.ok_or(::serde::de::Error::missing_field("SubscriptionTargetConfig"))?,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubscriptionTarget {
    type Properties = SubscriptionTargetProperties;
    const TYPE: &'static str = "AWS::DataZone::SubscriptionTarget";
    fn properties(&self) -> &SubscriptionTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubscriptionTarget {}

impl From<SubscriptionTargetProperties> for SubscriptionTarget {
    fn from(properties: SubscriptionTargetProperties) -> SubscriptionTarget {
        SubscriptionTarget { properties }
    }
}

pub mod data_source {
    //! Property types for the `DataSource` resource.

    /// The [`AWS::DataZone::DataSource.DataSourceConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-datasourceconfigurationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceConfigurationInput {
        /// Property [`GlueRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-datasourceconfigurationinput.html#cfn-datazone-datasource-datasourceconfigurationinput-gluerunconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_run_configuration: Option<::Value<GlueRunConfigurationInput>>,
        /// Property [`RedshiftRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-datasourceconfigurationinput.html#cfn-datazone-datasource-datasourceconfigurationinput-redshiftrunconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_run_configuration: Option<::Value<RedshiftRunConfigurationInput>>,
    }

    impl ::codec::SerializeValue for DataSourceConfigurationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref glue_run_configuration) = self.glue_run_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueRunConfiguration", glue_run_configuration)?;
            }
            if let Some(ref redshift_run_configuration) = self.redshift_run_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftRunConfiguration", redshift_run_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceConfigurationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceConfigurationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceConfigurationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceConfigurationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut glue_run_configuration: Option<::Value<GlueRunConfigurationInput>> = None;
                    let mut redshift_run_configuration: Option<::Value<RedshiftRunConfigurationInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GlueRunConfiguration" => {
                                glue_run_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftRunConfiguration" => {
                                redshift_run_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceConfigurationInput {
                        glue_run_configuration: glue_run_configuration,
                        redshift_run_configuration: redshift_run_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.FilterExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-filterexpression.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterExpression {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-filterexpression.html#cfn-datazone-datasource-filterexpression-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-filterexpression.html#cfn-datazone-datasource-filterexpression-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for FilterExpression {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterExpression {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterExpression, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterExpression;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterExpression")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterExpression {
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.FormInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html) property type.
    #[derive(Debug, Default)]
    pub struct FormInput {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html#cfn-datazone-datasource-forminput-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: Option<::Value<String>>,
        /// Property [`FormName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html#cfn-datazone-datasource-forminput-formname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub form_name: ::Value<String>,
        /// Property [`TypeIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html#cfn-datazone-datasource-forminput-typeidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_identifier: Option<::Value<String>>,
        /// Property [`TypeRevision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html#cfn-datazone-datasource-forminput-typerevision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_revision: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FormInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content) = self.content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", content)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormName", &self.form_name)?;
            if let Some(ref type_identifier) = self.type_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeIdentifier", type_identifier)?;
            }
            if let Some(ref type_revision) = self.type_revision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeRevision", type_revision)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;
                    let mut form_name: Option<::Value<String>> = None;
                    let mut type_identifier: Option<::Value<String>> = None;
                    let mut type_revision: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FormName" => {
                                form_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TypeIdentifier" => {
                                type_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TypeRevision" => {
                                type_revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormInput {
                        content: content,
                        form_name: form_name.ok_or(::serde::de::Error::missing_field("FormName"))?,
                        type_identifier: type_identifier,
                        type_revision: type_revision,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.GlueRunConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-gluerunconfigurationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueRunConfigurationInput {
        /// Property [`DataAccessRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-gluerunconfigurationinput.html#cfn-datazone-datasource-gluerunconfigurationinput-dataaccessrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_access_role: Option<::Value<String>>,
        /// Property [`RelationalFilterConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-gluerunconfigurationinput.html#cfn-datazone-datasource-gluerunconfigurationinput-relationalfilterconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relational_filter_configurations: ::ValueList<RelationalFilterConfiguration>,
    }

    impl ::codec::SerializeValue for GlueRunConfigurationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_access_role) = self.data_access_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataAccessRole", data_access_role)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalFilterConfigurations", &self.relational_filter_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueRunConfigurationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueRunConfigurationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueRunConfigurationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueRunConfigurationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_access_role: Option<::Value<String>> = None;
                    let mut relational_filter_configurations: Option<::ValueList<RelationalFilterConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataAccessRole" => {
                                data_access_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationalFilterConfigurations" => {
                                relational_filter_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueRunConfigurationInput {
                        data_access_role: data_access_role,
                        relational_filter_configurations: relational_filter_configurations.ok_or(::serde::de::Error::missing_field("RelationalFilterConfigurations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RecommendationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-recommendationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RecommendationConfiguration {
        /// Property [`EnableBusinessNameGeneration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-recommendationconfiguration.html#cfn-datazone-datasource-recommendationconfiguration-enablebusinessnamegeneration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_business_name_generation: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for RecommendationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_business_name_generation) = self.enable_business_name_generation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableBusinessNameGeneration", enable_business_name_generation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecommendationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecommendationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecommendationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecommendationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_business_name_generation: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableBusinessNameGeneration" => {
                                enable_business_name_generation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecommendationConfiguration {
                        enable_business_name_generation: enable_business_name_generation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RedshiftClusterStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftclusterstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftClusterStorage {
        /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftclusterstorage.html#cfn-datazone-datasource-redshiftclusterstorage-clustername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftClusterStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftClusterStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftClusterStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftClusterStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftClusterStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterName" => {
                                cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftClusterStorage {
                        cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RedshiftCredentialConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftcredentialconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftCredentialConfiguration {
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftcredentialconfiguration.html#cfn-datazone-datasource-redshiftcredentialconfiguration-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftCredentialConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", &self.secret_manager_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftCredentialConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftCredentialConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftCredentialConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftCredentialConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secret_manager_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftCredentialConfiguration {
                        secret_manager_arn: secret_manager_arn.ok_or(::serde::de::Error::missing_field("SecretManagerArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RedshiftRunConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftRunConfigurationInput {
        /// Property [`DataAccessRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html#cfn-datazone-datasource-redshiftrunconfigurationinput-dataaccessrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_access_role: Option<::Value<String>>,
        /// Property [`RedshiftCredentialConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html#cfn-datazone-datasource-redshiftrunconfigurationinput-redshiftcredentialconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_credential_configuration: ::Value<RedshiftCredentialConfiguration>,
        /// Property [`RedshiftStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html#cfn-datazone-datasource-redshiftrunconfigurationinput-redshiftstorage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_storage: ::Value<RedshiftStorage>,
        /// Property [`RelationalFilterConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html#cfn-datazone-datasource-redshiftrunconfigurationinput-relationalfilterconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relational_filter_configurations: ::ValueList<RelationalFilterConfiguration>,
    }

    impl ::codec::SerializeValue for RedshiftRunConfigurationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_access_role) = self.data_access_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataAccessRole", data_access_role)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftCredentialConfiguration", &self.redshift_credential_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftStorage", &self.redshift_storage)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalFilterConfigurations", &self.relational_filter_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftRunConfigurationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftRunConfigurationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftRunConfigurationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftRunConfigurationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_access_role: Option<::Value<String>> = None;
                    let mut redshift_credential_configuration: Option<::Value<RedshiftCredentialConfiguration>> = None;
                    let mut redshift_storage: Option<::Value<RedshiftStorage>> = None;
                    let mut relational_filter_configurations: Option<::ValueList<RelationalFilterConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataAccessRole" => {
                                data_access_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftCredentialConfiguration" => {
                                redshift_credential_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftStorage" => {
                                redshift_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationalFilterConfigurations" => {
                                relational_filter_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftRunConfigurationInput {
                        data_access_role: data_access_role,
                        redshift_credential_configuration: redshift_credential_configuration.ok_or(::serde::de::Error::missing_field("RedshiftCredentialConfiguration"))?,
                        redshift_storage: redshift_storage.ok_or(::serde::de::Error::missing_field("RedshiftStorage"))?,
                        relational_filter_configurations: relational_filter_configurations.ok_or(::serde::de::Error::missing_field("RelationalFilterConfigurations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RedshiftServerlessStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftserverlessstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftServerlessStorage {
        /// Property [`WorkgroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftserverlessstorage.html#cfn-datazone-datasource-redshiftserverlessstorage-workgroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftServerlessStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupName", &self.workgroup_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftServerlessStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftServerlessStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftServerlessStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftServerlessStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut workgroup_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkgroupName" => {
                                workgroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftServerlessStorage {
                        workgroup_name: workgroup_name.ok_or(::serde::de::Error::missing_field("WorkgroupName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RedshiftStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftStorage {
        /// Property [`RedshiftClusterSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftstorage.html#cfn-datazone-datasource-redshiftstorage-redshiftclustersource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_cluster_source: Option<::Value<RedshiftClusterStorage>>,
        /// Property [`RedshiftServerlessSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftstorage.html#cfn-datazone-datasource-redshiftstorage-redshiftserverlesssource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_serverless_source: Option<::Value<RedshiftServerlessStorage>>,
    }

    impl ::codec::SerializeValue for RedshiftStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref redshift_cluster_source) = self.redshift_cluster_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftClusterSource", redshift_cluster_source)?;
            }
            if let Some(ref redshift_serverless_source) = self.redshift_serverless_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftServerlessSource", redshift_serverless_source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut redshift_cluster_source: Option<::Value<RedshiftClusterStorage>> = None;
                    let mut redshift_serverless_source: Option<::Value<RedshiftServerlessStorage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RedshiftClusterSource" => {
                                redshift_cluster_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftServerlessSource" => {
                                redshift_serverless_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftStorage {
                        redshift_cluster_source: redshift_cluster_source,
                        redshift_serverless_source: redshift_serverless_source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.RelationalFilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-relationalfilterconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationalFilterConfiguration {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-relationalfilterconfiguration.html#cfn-datazone-datasource-relationalfilterconfiguration-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`FilterExpressions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-relationalfilterconfiguration.html#cfn-datazone-datasource-relationalfilterconfiguration-filterexpressions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_expressions: Option<::ValueList<FilterExpression>>,
        /// Property [`SchemaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-relationalfilterconfiguration.html#cfn-datazone-datasource-relationalfilterconfiguration-schemaname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RelationalFilterConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            if let Some(ref filter_expressions) = self.filter_expressions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterExpressions", filter_expressions)?;
            }
            if let Some(ref schema_name) = self.schema_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaName", schema_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationalFilterConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationalFilterConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationalFilterConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationalFilterConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut filter_expressions: Option<::ValueList<FilterExpression>> = None;
                    let mut schema_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterExpressions" => {
                                filter_expressions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaName" => {
                                schema_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationalFilterConfiguration {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        filter_expressions: filter_expressions,
                        schema_name: schema_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataZone::DataSource.ScheduleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-scheduleconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduleConfiguration {
        /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-scheduleconfiguration.html#cfn-datazone-datasource-scheduleconfiguration-schedule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule: Option<::Value<String>>,
        /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-scheduleconfiguration.html#cfn-datazone-datasource-scheduleconfiguration-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScheduleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schedule) = self.schedule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
            }
            if let Some(ref timezone) = self.timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule: Option<::Value<String>> = None;
                    let mut timezone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Schedule" => {
                                schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timezone" => {
                                timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduleConfiguration {
                        schedule: schedule,
                        timezone: timezone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::DataZone::Domain.SingleSignOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-domain-singlesignon.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleSignOn {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-domain-singlesignon.html#cfn-datazone-domain-singlesignon-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`UserAssignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-domain-singlesignon.html#cfn-datazone-domain-singlesignon-userassignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_assignment: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SingleSignOn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref user_assignment) = self.user_assignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAssignment", user_assignment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleSignOn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleSignOn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleSignOn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleSignOn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut user_assignment: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAssignment" => {
                                user_assignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SingleSignOn {
                        r#type: r#type,
                        user_assignment: user_assignment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::DataZone::Environment.EnvironmentParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environment-environmentparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environment-environmentparameter.html#cfn-datazone-environment-environmentparameter-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environment-environmentparameter.html#cfn-datazone-environment-environmentparameter-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EnvironmentParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentParameter {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod environment_blueprint_configuration {
    //! Property types for the `EnvironmentBlueprintConfiguration` resource.

    /// The [`AWS::DataZone::EnvironmentBlueprintConfiguration.RegionalParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-regionalparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct RegionalParameter {
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-regionalparameter.html#cfn-datazone-environmentblueprintconfiguration-regionalparameter-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueMap<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-regionalparameter.html#cfn-datazone-environmentblueprintconfiguration-regionalparameter-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RegionalParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegionalParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegionalParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegionalParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegionalParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameters: Option<::ValueMap<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegionalParameter {
                        parameters: parameters,
                        region: region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod environment_profile {
    //! Property types for the `EnvironmentProfile` resource.

    /// The [`AWS::DataZone::EnvironmentProfile.EnvironmentParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentprofile-environmentparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentprofile-environmentparameter.html#cfn-datazone-environmentprofile-environmentparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentprofile-environmentparameter.html#cfn-datazone-environmentprofile-environmentparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EnvironmentParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentParameter {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod subscription_target {
    //! Property types for the `SubscriptionTarget` resource.

    /// The [`AWS::DataZone::SubscriptionTarget.SubscriptionTargetForm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-subscriptiontarget-subscriptiontargetform.html) property type.
    #[derive(Debug, Default)]
    pub struct SubscriptionTargetForm {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-subscriptiontarget-subscriptiontargetform.html#cfn-datazone-subscriptiontarget-subscriptiontargetform-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: ::Value<String>,
        /// Property [`FormName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-subscriptiontarget-subscriptiontargetform.html#cfn-datazone-subscriptiontarget-subscriptiontargetform-formname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub form_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubscriptionTargetForm {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormName", &self.form_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubscriptionTargetForm {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionTargetForm, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubscriptionTargetForm;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubscriptionTargetForm")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;
                    let mut form_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FormName" => {
                                form_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubscriptionTargetForm {
                        content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                        form_name: form_name.ok_or(::serde::de::Error::missing_field("FormName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
