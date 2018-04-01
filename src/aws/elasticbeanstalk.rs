//! Types for the `ElasticBeanstalk` service.

/// The [`AWS::ElasticBeanstalk::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk.html) resource type.
#[derive(Debug)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `ResourceLifecycleConfig`.
    #[serde(rename = "ResourceLifecycleConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_lifecycle_config: Option<::Value<self::application::ApplicationResourceLifecycleConfig>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationVersionProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `SourceBundle`.
    #[serde(rename = "SourceBundle")]
    pub source_bundle: ::Value<self::application_version::SourceBundle>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationTemplateProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `EnvironmentId`.
    #[serde(rename = "EnvironmentId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<::Value<String>>,
    /// Property `OptionSettings`.
    #[serde(rename = "OptionSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<::ValueList<self::configuration_template::ConfigurationOptionSetting>>,
    /// Property `PlatformArn`.
    #[serde(rename = "PlatformArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<::Value<String>>,
    /// Property `SolutionStackName`.
    #[serde(rename = "SolutionStackName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<::Value<String>>,
    /// Property `SourceConfiguration`.
    #[serde(rename = "SourceConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<::Value<self::configuration_template::SourceConfiguration>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `CNAMEPrefix`.
    #[serde(rename = "CNAMEPrefix")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname_prefix: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `EnvironmentName`.
    #[serde(rename = "EnvironmentName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<::Value<String>>,
    /// Property `OptionSettings`.
    #[serde(rename = "OptionSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<::ValueList<self::environment::OptionSetting>>,
    /// Property `PlatformArn`.
    #[serde(rename = "PlatformArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<::Value<String>>,
    /// Property `SolutionStackName`.
    #[serde(rename = "SolutionStackName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TemplateName`.
    #[serde(rename = "TemplateName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_name: Option<::Value<String>>,
    /// Property `Tier`.
    #[serde(rename = "Tier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<::Value<self::environment::Tier>>,
    /// Property `VersionLabel`.
    #[serde(rename = "VersionLabel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_label: Option<::Value<String>>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApplicationResourceLifecycleConfig {
        /// Property `ServiceRole`.
        #[serde(rename = "ServiceRole")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service_role: Option<::Value<String>>,
        /// Property `VersionLifecycleConfig`.
        #[serde(rename = "VersionLifecycleConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version_lifecycle_config: Option<::Value<ApplicationVersionLifecycleConfig>>,
    }

    cfn_internal__inherit_codec_impls!(ApplicationResourceLifecycleConfig);

    /// The [`AWS::ElasticBeanstalk::Application.ApplicationVersionLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationversionlifecycleconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApplicationVersionLifecycleConfig {
        /// Property `MaxAgeRule`.
        #[serde(rename = "MaxAgeRule")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_age_rule: Option<::Value<MaxAgeRule>>,
        /// Property `MaxCountRule`.
        #[serde(rename = "MaxCountRule")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_count_rule: Option<::Value<MaxCountRule>>,
    }

    cfn_internal__inherit_codec_impls!(ApplicationVersionLifecycleConfig);

    /// The [`AWS::ElasticBeanstalk::Application.MaxAgeRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxagerule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxAgeRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename = "DeleteSourceFromS3")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_source_from_s3: Option<::Value<bool>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
        /// Property `MaxAgeInDays`.
        #[serde(rename = "MaxAgeInDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_age_in_days: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(MaxAgeRule);

    /// The [`AWS::ElasticBeanstalk::Application.MaxCountRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxcountrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxCountRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename = "DeleteSourceFromS3")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_source_from_s3: Option<::Value<bool>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
        /// Property `MaxCount`.
        #[serde(rename = "MaxCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_count: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(MaxCountRule);
}

pub mod application_version {
    //! Property types for the `ApplicationVersion` resource.

    /// The [`AWS::ElasticBeanstalk::ApplicationVersion.SourceBundle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-sourcebundle.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceBundle {
        /// Property `S3Bucket`.
        #[serde(rename = "S3Bucket")]
        pub s3_bucket: ::Value<String>,
        /// Property `S3Key`.
        #[serde(rename = "S3Key")]
        pub s3_key: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SourceBundle);
}

pub mod configuration_template {
    //! Property types for the `ConfigurationTemplate` resource.

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.ConfigurationOptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-configurationoptionsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigurationOptionSetting {
        /// Property `Namespace`.
        #[serde(rename = "Namespace")]
        pub namespace: ::Value<String>,
        /// Property `OptionName`.
        #[serde(rename = "OptionName")]
        pub option_name: ::Value<String>,
        /// Property `ResourceName`.
        #[serde(rename = "ResourceName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ConfigurationOptionSetting);

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-sourceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceConfiguration {
        /// Property `ApplicationName`.
        #[serde(rename = "ApplicationName")]
        pub application_name: ::Value<String>,
        /// Property `TemplateName`.
        #[serde(rename = "TemplateName")]
        pub template_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SourceConfiguration);
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::ElasticBeanstalk::Environment.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-option-settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionSetting {
        /// Property `Namespace`.
        #[serde(rename = "Namespace")]
        pub namespace: ::Value<String>,
        /// Property `OptionName`.
        #[serde(rename = "OptionName")]
        pub option_name: ::Value<String>,
        /// Property `ResourceName`.
        #[serde(rename = "ResourceName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(OptionSetting);

    /// The [`AWS::ElasticBeanstalk::Environment.Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment-tier.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tier {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Tier);
}
