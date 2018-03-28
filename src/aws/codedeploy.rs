/// The [`AWS::CodeDeploy::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-application.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="ComputePlatform")]
    pub compute_platform: String,
}

impl<'a> ::Resource<'a> for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::Application";
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

/// The [`AWS::CodeDeploy::DeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentconfig.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentConfig {
    properties: DeploymentConfigProperties
}

/// Properties for the `DeploymentConfig` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentConfigProperties {
    #[serde(rename="DeploymentConfigName")]
    pub deployment_config_name: String,
    #[serde(rename="MinimumHealthyHosts")]
    pub minimum_healthy_hosts: (),
}

impl<'a> ::Resource<'a> for DeploymentConfig {
    type Properties = DeploymentConfigProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::DeploymentConfig";
    fn properties(&self) -> &DeploymentConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentConfigProperties {
        &mut self.properties
    }
}

impl From<DeploymentConfigProperties> for DeploymentConfig {
    fn from(properties: DeploymentConfigProperties) -> DeploymentConfig {
        DeploymentConfig { properties }
    }
}

/// The [`AWS::CodeDeploy::DeploymentGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentGroup {
    properties: DeploymentGroupProperties
}

/// Properties for the `DeploymentGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentGroupProperties {
    #[serde(rename="AlarmConfiguration")]
    pub alarm_configuration: (),
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="AutoRollbackConfiguration")]
    pub auto_rollback_configuration: (),
    #[serde(rename="AutoScalingGroups")]
    pub auto_scaling_groups: Vec<String>,
    #[serde(rename="Deployment")]
    pub deployment: (),
    #[serde(rename="DeploymentConfigName")]
    pub deployment_config_name: String,
    #[serde(rename="DeploymentGroupName")]
    pub deployment_group_name: String,
    #[serde(rename="DeploymentStyle")]
    pub deployment_style: (),
    #[serde(rename="Ec2TagFilters")]
    pub ec2_tag_filters: Vec<()>,
    #[serde(rename="LoadBalancerInfo")]
    pub load_balancer_info: (),
    #[serde(rename="OnPremisesInstanceTagFilters")]
    pub on_premises_instance_tag_filters: Vec<()>,
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[serde(rename="TriggerConfigurations")]
    pub trigger_configurations: Vec<()>,
}

impl<'a> ::Resource<'a> for DeploymentGroup {
    type Properties = DeploymentGroupProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::DeploymentGroup";
    fn properties(&self) -> &DeploymentGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentGroupProperties {
        &mut self.properties
    }
}

impl From<DeploymentGroupProperties> for DeploymentGroup {
    fn from(properties: DeploymentGroupProperties) -> DeploymentGroup {
        DeploymentGroup { properties }
    }
}

