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
    pub resource_lifecycle_config: (),
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
    pub source_bundle: (),
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
    pub option_settings: Vec<()>,
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
    #[serde(rename="SourceConfiguration")]
    pub source_configuration: (),
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
    pub option_settings: Vec<()>,
    #[serde(rename="PlatformArn")]
    pub platform_arn: String,
    #[serde(rename="SolutionStackName")]
    pub solution_stack_name: String,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="TemplateName")]
    pub template_name: String,
    #[serde(rename="Tier")]
    pub tier: (),
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

