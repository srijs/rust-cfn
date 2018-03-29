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
    #[serde(rename="ApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `ResourceLifecycleConfig`.
    #[serde(rename="ResourceLifecycleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_lifecycle_config: Option<self::application::ApplicationResourceLifecycleConfig>,
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
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `SourceBundle`.
    #[serde(rename="SourceBundle")]
    pub source_bundle: self::application_version::SourceBundle,
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
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `EnvironmentId`.
    #[serde(rename="EnvironmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// Property `OptionSettings`.
    #[serde(rename="OptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<Vec<self::configuration_template::ConfigurationOptionSetting>>,
    /// Property `PlatformArn`.
    #[serde(rename="PlatformArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    /// Property `SolutionStackName`.
    #[serde(rename="SolutionStackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    /// Property `SourceConfiguration`.
    #[serde(rename="SourceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<self::configuration_template::SourceConfiguration>,
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
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    /// Property `CNAMEPrefix`.
    #[serde(rename="CNAMEPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_prefix: Option<String>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `EnvironmentName`.
    #[serde(rename="EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    /// Property `OptionSettings`.
    #[serde(rename="OptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<Vec<self::environment::OptionSetting>>,
    /// Property `PlatformArn`.
    #[serde(rename="PlatformArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    /// Property `SolutionStackName`.
    #[serde(rename="SolutionStackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `TemplateName`.
    #[serde(rename="TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// Property `Tier`.
    #[serde(rename="Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<self::environment::Tier>,
    /// Property `VersionLabel`.
    #[serde(rename="VersionLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
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
        #[serde(rename="ServiceRole")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_role: Option<String>,
        /// Property `VersionLifecycleConfig`.
        #[serde(rename="VersionLifecycleConfig")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version_lifecycle_config: Option<ApplicationVersionLifecycleConfig>,
    }

    /// The [`AWS::ElasticBeanstalk::Application.ApplicationVersionLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationversionlifecycleconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApplicationVersionLifecycleConfig {
        /// Property `MaxAgeRule`.
        #[serde(rename="MaxAgeRule")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_age_rule: Option<MaxAgeRule>,
        /// Property `MaxCountRule`.
        #[serde(rename="MaxCountRule")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_count_rule: Option<MaxCountRule>,
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxAgeRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxagerule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxAgeRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename="DeleteSourceFromS3")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete_source_from_s3: Option<bool>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        /// Property `MaxAgeInDays`.
        #[serde(rename="MaxAgeInDays")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_age_in_days: Option<u32>,
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxCountRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxcountrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxCountRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename="DeleteSourceFromS3")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete_source_from_s3: Option<bool>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        /// Property `MaxCount`.
        #[serde(rename="MaxCount")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_count: Option<u32>,
    }
}

pub mod application_version {
    //! Property types for the `ApplicationVersion` resource.

    /// The [`AWS::ElasticBeanstalk::ApplicationVersion.SourceBundle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-sourcebundle.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceBundle {
        /// Property `S3Bucket`.
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        /// Property `S3Key`.
        #[serde(rename="S3Key")]
        pub s3_key: String,
    }
}

pub mod configuration_template {
    //! Property types for the `ConfigurationTemplate` resource.

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.ConfigurationOptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-configurationoptionsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigurationOptionSetting {
        /// Property `Namespace`.
        #[serde(rename="Namespace")]
        pub namespace: String,
        /// Property `OptionName`.
        #[serde(rename="OptionName")]
        pub option_name: String,
        /// Property `ResourceName`.
        #[serde(rename="ResourceName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_name: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    /// The [`AWS::ElasticBeanstalk::ConfigurationTemplate.SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-sourceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceConfiguration {
        /// Property `ApplicationName`.
        #[serde(rename="ApplicationName")]
        pub application_name: String,
        /// Property `TemplateName`.
        #[serde(rename="TemplateName")]
        pub template_name: String,
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::ElasticBeanstalk::Environment.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-option-settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionSetting {
        /// Property `Namespace`.
        #[serde(rename="Namespace")]
        pub namespace: String,
        /// Property `OptionName`.
        #[serde(rename="OptionName")]
        pub option_name: String,
        /// Property `ResourceName`.
        #[serde(rename="ResourceName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_name: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    /// The [`AWS::ElasticBeanstalk::Environment.Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment-tier.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tier {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Version`.
        #[serde(rename="Version")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
}
