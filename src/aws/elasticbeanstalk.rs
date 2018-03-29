/// The [`AWS::ElasticBeanstalk::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Description")]
    pub description: String,
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

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::ApplicationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-version.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationVersion {
    properties: ApplicationVersionProperties
}

/// Properties for the `ApplicationVersion` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationVersionProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Description")]
    pub description: String,
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

impl From<ApplicationVersionProperties> for ApplicationVersion {
    fn from(properties: ApplicationVersionProperties) -> ApplicationVersion {
        ApplicationVersion { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::ConfigurationTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-configurationtemplate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationTemplate {
    properties: ConfigurationTemplateProperties
}

/// Properties for the `ConfigurationTemplate` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationTemplateProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EnvironmentId")]
    pub environment_id: String,
    #[serde(rename="OptionSettings")]
    pub option_settings: Vec<self::configuration_template::ConfigurationOptionSetting>,
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
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

impl From<ConfigurationTemplateProperties> for ConfigurationTemplate {
    fn from(properties: ConfigurationTemplateProperties) -> ConfigurationTemplate {
        ConfigurationTemplate { properties }
    }
}

/// The [`AWS::ElasticBeanstalk::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Serialize, Deserialize)]
pub struct EnvironmentProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="CNAMEPrefix")]
    pub cname_prefix: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EnvironmentName")]
    pub environment_name: String,
    #[serde(rename="OptionSettings")]
    pub option_settings: Vec<self::environment::OptionSetting>,
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="TemplateName")]
    pub template_name: String,
    #[serde(rename="Tier")]
    pub tier: self::environment::Tier,
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

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

pub mod application {
    #[derive(Serialize, Deserialize)]
    pub struct ApplicationResourceLifecycleConfig {
        #[serde(rename="ServiceRole")]
        pub service_role: String,
        #[serde(rename="VersionLifecycleConfig")]
        pub version_lifecycle_config: ApplicationVersionLifecycleConfig,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ApplicationVersionLifecycleConfig {
        #[serde(rename="MaxAgeRule")]
        pub max_age_rule: MaxAgeRule,
        #[serde(rename="MaxCountRule")]
        pub max_count_rule: MaxCountRule,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaxAgeRule {
        #[serde(rename="DeleteSourceFromS3")]
        pub delete_source_from_s3: bool,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="MaxAgeInDays")]
        pub max_age_in_days: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaxCountRule {
        #[serde(rename="DeleteSourceFromS3")]
        pub delete_source_from_s3: bool,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="MaxCount")]
        pub max_count: u32,
    }

}

pub mod application_version {
    #[derive(Serialize, Deserialize)]
    pub struct SourceBundle {
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        #[serde(rename="S3Key")]
        pub s3_key: String,
    }

}

pub mod configuration_template {
    #[derive(Serialize, Deserialize)]
    pub struct ConfigurationOptionSetting {
        #[serde(rename="Namespace")]
        pub namespace: String,
        #[serde(rename="OptionName")]
        pub option_name: String,
        #[serde(rename="ResourceName")]
        pub resource_name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SourceConfiguration {
        #[serde(rename="ApplicationName")]
        pub application_name: String,
        #[serde(rename="TemplateName")]
        pub template_name: String,
    }

}

pub mod environment {
    #[derive(Serialize, Deserialize)]
    pub struct OptionSetting {
        #[serde(rename="Namespace")]
        pub namespace: String,
        #[serde(rename="OptionName")]
        pub option_name: String,
        #[serde(rename="ResourceName")]
        pub resource_name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Tier {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Version")]
        pub version: String,
    }

}

