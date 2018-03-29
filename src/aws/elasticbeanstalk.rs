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
    pub application_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `ResourceLifecycleConfig`.
    #[serde(rename="ResourceLifecycleConfig")]
    pub resource_lifecycle_config: self::application::ApplicationResourceLifecycleConfig,
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
    pub description: String,
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
    pub description: String,
    /// Property `EnvironmentId`.
    #[serde(rename="EnvironmentId")]
    pub environment_id: String,
    /// Property `OptionSettings`.
    #[serde(rename="OptionSettings")]
    pub option_settings: Vec<self::configuration_template::ConfigurationOptionSetting>,
    /// Property `PlatformArn`.
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    /// Property `SolutionStackName`.
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
    /// Property `SourceConfiguration`.
    #[serde(rename="SourceConfiguration")]
    pub source_configuration: self::configuration_template::SourceConfiguration,
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
    pub cname_prefix: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `EnvironmentName`.
    #[serde(rename="EnvironmentName")]
    pub environment_name: String,
    /// Property `OptionSettings`.
    #[serde(rename="OptionSettings")]
    pub option_settings: Vec<self::environment::OptionSetting>,
    /// Property `PlatformArn`.
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    /// Property `SolutionStackName`.
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `TemplateName`.
    #[serde(rename="TemplateName")]
    pub template_name: String,
    /// Property `Tier`.
    #[serde(rename="Tier")]
    pub tier: self::environment::Tier,
    /// Property `VersionLabel`.
    #[serde(rename="VersionLabel")]
    pub version_label: String,
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
        pub service_role: String,
        /// Property `VersionLifecycleConfig`.
        #[serde(rename="VersionLifecycleConfig")]
        pub version_lifecycle_config: ApplicationVersionLifecycleConfig,
    }

    /// The [`AWS::ElasticBeanstalk::Application.ApplicationVersionLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationversionlifecycleconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApplicationVersionLifecycleConfig {
        /// Property `MaxAgeRule`.
        #[serde(rename="MaxAgeRule")]
        pub max_age_rule: MaxAgeRule,
        /// Property `MaxCountRule`.
        #[serde(rename="MaxCountRule")]
        pub max_count_rule: MaxCountRule,
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxAgeRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxagerule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxAgeRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename="DeleteSourceFromS3")]
        pub delete_source_from_s3: bool,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `MaxAgeInDays`.
        #[serde(rename="MaxAgeInDays")]
        pub max_age_in_days: u32,
    }

    /// The [`AWS::ElasticBeanstalk::Application.MaxCountRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxcountrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaxCountRule {
        /// Property `DeleteSourceFromS3`.
        #[serde(rename="DeleteSourceFromS3")]
        pub delete_source_from_s3: bool,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `MaxCount`.
        #[serde(rename="MaxCount")]
        pub max_count: u32,
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
        pub resource_name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
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
        pub resource_name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ElasticBeanstalk::Environment.Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment-tier.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tier {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Version`.
        #[serde(rename="Version")]
        pub version: String,
    }
}
