//! Types for the `ElasticBeanstalk` service.

/// The [`AWS::ElasticBeanstalk::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk.html) resource type.
#[derive(Debug)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug)]
pub struct ApplicationProperties {
    /// Property `ApplicationName`.
    pub application_name: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `ResourceLifecycleConfig`.
    pub resource_lifecycle_config: Option<::Value<self::application::ApplicationResourceLifecycleConfig>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLifecycleConfig", &self.resource_lifecycle_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name = None;
                let mut description = None;
                let mut resource_lifecycle_config = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResourceLifecycleConfig" => {
                            resource_lifecycle_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    application_name: application_name,
                    description: description,
                    resource_lifecycle_config: resource_lifecycle_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::ElasticBeanstalk::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::ApplicationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-version.html) resource type.
#[derive(Debug)]
pub struct ApplicationVersion {
    properties: ApplicationVersionProperties
}

/// Properties for the `ApplicationVersion` resource.
#[derive(Debug)]
pub struct ApplicationVersionProperties {
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `SourceBundle`.
    pub source_bundle: ::Value<self::application_version::SourceBundle>,
}

impl ::serde::Serialize for ApplicationVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceBundle", &self.source_bundle)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name = None;
                let mut description = None;
                let mut source_bundle = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceBundle" => {
                            source_bundle = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationVersionProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    description: description,
                    source_bundle: source_bundle.ok_or(::serde::de::Error::missing_field("SourceBundle"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ApplicationVersion {
    type Properties = ApplicationVersionProperties;
    const TYPE: &'static str = "AWS::ElasticBeanstalk::ApplicationVersion";
    fn properties(&self) -> &ApplicationVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationVersion {}

impl From<ApplicationVersionProperties> for ApplicationVersion {
    fn from(properties: ApplicationVersionProperties) -> ApplicationVersion {
        ApplicationVersion { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::ConfigurationTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-configurationtemplate.html) resource type.
#[derive(Debug)]
pub struct ConfigurationTemplate {
    properties: ConfigurationTemplateProperties
}

/// Properties for the `ConfigurationTemplate` resource.
#[derive(Debug)]
pub struct ConfigurationTemplateProperties {
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EnvironmentId`.
    pub environment_id: Option<::Value<String>>,
    /// Property `OptionSettings`.
    pub option_settings: Option<::ValueList<self::configuration_template::ConfigurationOptionSetting>>,
    /// Property `PlatformArn`.
    pub platform_arn: Option<::Value<String>>,
    /// Property `SolutionStackName`.
    pub solution_stack_name: Option<::Value<String>>,
    /// Property `SourceConfiguration`.
    pub source_configuration: Option<::Value<self::configuration_template::SourceConfiguration>>,
}

impl ::serde::Serialize for ConfigurationTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentId", &self.environment_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionSettings", &self.option_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformArn", &self.platform_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SolutionStackName", &self.solution_stack_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConfiguration", &self.source_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name = None;
                let mut description = None;
                let mut environment_id = None;
                let mut option_settings = None;
                let mut platform_arn = None;
                let mut solution_stack_name = None;
                let mut source_configuration = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnvironmentId" => {
                            environment_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OptionSettings" => {
                            option_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlatformArn" => {
                            platform_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SolutionStackName" => {
                            solution_stack_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceConfiguration" => {
                            source_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationTemplateProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    description: description,
                    environment_id: environment_id,
                    option_settings: option_settings,
                    platform_arn: platform_arn,
                    solution_stack_name: solution_stack_name,
                    source_configuration: source_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ConfigurationTemplate {
    type Properties = ConfigurationTemplateProperties;
    const TYPE: &'static str = "AWS::ElasticBeanstalk::ConfigurationTemplate";
    fn properties(&self) -> &ConfigurationTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationTemplate {}

impl From<ConfigurationTemplateProperties> for ConfigurationTemplate {
    fn from(properties: ConfigurationTemplateProperties) -> ConfigurationTemplate {
        ConfigurationTemplate { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html) resource type.
#[derive(Debug)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug)]
pub struct EnvironmentProperties {
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `CNAMEPrefix`.
    pub cname_prefix: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EnvironmentName`.
    pub environment_name: Option<::Value<String>>,
    /// Property `OptionSettings`.
    pub option_settings: Option<::ValueList<self::environment::OptionSetting>>,
    /// Property `PlatformArn`.
    pub platform_arn: Option<::Value<String>>,
    /// Property `SolutionStackName`.
    pub solution_stack_name: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TemplateName`.
    pub template_name: Option<::Value<String>>,
    /// Property `Tier`.
    pub tier: Option<::Value<self::environment::Tier>>,
    /// Property `VersionLabel`.
    pub version_label: Option<::Value<String>>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CNAMEPrefix", &self.cname_prefix)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentName", &self.environment_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionSettings", &self.option_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformArn", &self.platform_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SolutionStackName", &self.solution_stack_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", &self.tier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionLabel", &self.version_label)?;
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
                let mut application_name = None;
                let mut cname_prefix = None;
                let mut description = None;
                let mut environment_name = None;
                let mut option_settings = None;
                let mut platform_arn = None;
                let mut solution_stack_name = None;
                let mut tags = None;
                let mut template_name = None;
                let mut tier = None;
                let mut version_label = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CNAMEPrefix" => {
                            cname_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnvironmentName" => {
                            environment_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OptionSettings" => {
                            option_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlatformArn" => {
                            platform_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SolutionStackName" => {
                            solution_stack_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TemplateName" => {
                            template_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tier" => {
                            tier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VersionLabel" => {
                            version_label = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    cname_prefix: cname_prefix,
                    description: description,
                    environment_name: environment_name,
                    option_settings: option_settings,
                    platform_arn: platform_arn,
                    solution_stack_name: solution_stack_name,
                    tags: tags,
                    template_name: template_name,
                    tier: tier,
                    version_label: version_label,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::ElasticBeanstalk::Environment";
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

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::ElasticBeanstalk::Application.ApplicationResourceLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationresourcelifecycleconfig.html) property type.
    #[derive(Debug)]
    pub struct ApplicationResourceLifecycleConfig {
        /// Property `ServiceRole`.
        pub service_role: Option<::Value<String>>,
        /// Property `VersionLifecycleConfig`.
        pub version_lifecycle_config: Option<::Value<ApplicationVersionLifecycleConfig>>,
    }

    impl ::codec::SerializeValue for ApplicationResourceLifecycleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionLifecycleConfig", &self.version_lifecycle_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationResourceLifecycleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationResourceLifecycleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationResourceLifecycleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationResourceLifecycleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut service_role = None;
                    let mut version_lifecycle_config = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServiceRole" => {
                                service_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VersionLifecycleConfig" => {
                                version_lifecycle_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationResourceLifecycleConfig {
                        service_role: service_role,
                        version_lifecycle_config: version_lifecycle_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticBeanstalk::Application.ApplicationVersionLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationversionlifecycleconfig.html) property type.
    #[derive(Debug)]
    pub struct ApplicationVersionLifecycleConfig {
        /// Property `MaxAgeRule`.
        pub max_age_rule: Option<::Value<MaxAgeRule>>,
        /// Property `MaxCountRule`.
        pub max_count_rule: Option<::Value<MaxCountRule>>,
    }

    impl ::codec::SerializeValue for ApplicationVersionLifecycleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAgeRule", &self.max_age_rule)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCountRule", &self.max_count_rule)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationVersionLifecycleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationVersionLifecycleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationVersionLifecycleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationVersionLifecycleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_age_rule = None;
                    let mut max_count_rule = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxAgeRule" => {
                                max_age_rule = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaxCountRule" => {
                                max_count_rule = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationVersionLifecycleConfig {
                        max_age_rule: max_age_rule,
                        max_count_rule: max_count_rule,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxAgeRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxagerule.html) property type.
    #[derive(Debug)]
    pub struct MaxAgeRule {
        /// Property `DeleteSourceFromS3`.
        pub delete_source_from_s3: Option<::Value<bool>>,
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `MaxAgeInDays`.
        pub max_age_in_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for MaxAgeRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteSourceFromS3", &self.delete_source_from_s3)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAgeInDays", &self.max_age_in_days)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaxAgeRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaxAgeRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaxAgeRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaxAgeRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_source_from_s3 = None;
                    let mut enabled = None;
                    let mut max_age_in_days = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteSourceFromS3" => {
                                delete_source_from_s3 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaxAgeInDays" => {
                                max_age_in_days = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaxAgeRule {
                        delete_source_from_s3: delete_source_from_s3,
                        enabled: enabled,
                        max_age_in_days: max_age_in_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxCountRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxcountrule.html) property type.
    #[derive(Debug)]
    pub struct MaxCountRule {
        /// Property `DeleteSourceFromS3`.
        pub delete_source_from_s3: Option<::Value<bool>>,
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `MaxCount`.
        pub max_count: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for MaxCountRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteSourceFromS3", &self.delete_source_from_s3)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCount", &self.max_count)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaxCountRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaxCountRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaxCountRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaxCountRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_source_from_s3 = None;
                    let mut enabled = None;
                    let mut max_count = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteSourceFromS3" => {
                                delete_source_from_s3 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaxCount" => {
                                max_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaxCountRule {
                        delete_source_from_s3: delete_source_from_s3,
                        enabled: enabled,
                        max_count: max_count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod application_version {
    //! Property types for the `ApplicationVersion` resource.

    /// The [`AWS::ElasticBeanstalk::ApplicationVersion.SourceBundle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-sourcebundle.html) property type.
    #[derive(Debug)]
    pub struct SourceBundle {
        /// Property `S3Bucket`.
        pub s3_bucket: ::Value<String>,
        /// Property `S3Key`.
        pub s3_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceBundle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceBundle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceBundle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceBundle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceBundle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket = None;
                    let mut s3_key = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Key" => {
                                s3_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceBundle {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_template {
    //! Property types for the `ConfigurationTemplate` resource.

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.ConfigurationOptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-configurationoptionsetting.html) property type.
    #[derive(Debug)]
    pub struct ConfigurationOptionSetting {
        /// Property `Namespace`.
        pub namespace: ::Value<String>,
        /// Property `OptionName`.
        pub option_name: ::Value<String>,
        /// Property `ResourceName`.
        pub resource_name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigurationOptionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionName", &self.option_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceName", &self.resource_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationOptionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationOptionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationOptionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationOptionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace = None;
                    let mut option_name = None;
                    let mut resource_name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OptionName" => {
                                option_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourceName" => {
                                resource_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationOptionSetting {
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                        option_name: option_name.ok_or(::serde::de::Error::missing_field("OptionName"))?,
                        resource_name: resource_name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-sourceconfiguration.html) property type.
    #[derive(Debug)]
    pub struct SourceConfiguration {
        /// Property `ApplicationName`.
        pub application_name: ::Value<String>,
        /// Property `TemplateName`.
        pub template_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_name = None;
                    let mut template_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationName" => {
                                application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TemplateName" => {
                                template_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConfiguration {
                        application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                        template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::ElasticBeanstalk::Environment.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-option-settings.html) property type.
    #[derive(Debug)]
    pub struct OptionSetting {
        /// Property `Namespace`.
        pub namespace: ::Value<String>,
        /// Property `OptionName`.
        pub option_name: ::Value<String>,
        /// Property `ResourceName`.
        pub resource_name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OptionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionName", &self.option_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceName", &self.resource_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OptionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace = None;
                    let mut option_name = None;
                    let mut resource_name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OptionName" => {
                                option_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourceName" => {
                                resource_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionSetting {
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                        option_name: option_name.ok_or(::serde::de::Error::missing_field("OptionName"))?,
                        resource_name: resource_name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticBeanstalk::Environment.Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment-tier.html) property type.
    #[derive(Debug)]
    pub struct Tier {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Version`.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Tier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut type_ = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Tier {
                        name: name,
                        type_: type_,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
