/// The [`AWS::CodeDeploy::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-application.html) resource type.
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

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::CodeDeploy::DeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentconfig.html) resource type.
pub struct DeploymentConfig {
    properties: DeploymentConfigProperties
}

/// Properties for the `DeploymentConfig` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentConfigProperties {
    #[serde(rename="DeploymentConfigName")]
    pub deployment_config_name: String,
    #[serde(rename="MinimumHealthyHosts")]
    pub minimum_healthy_hosts: self::deployment_config::MinimumHealthyHosts,
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

impl ::private::Sealed for DeploymentConfig {}

impl From<DeploymentConfigProperties> for DeploymentConfig {
    fn from(properties: DeploymentConfigProperties) -> DeploymentConfig {
        DeploymentConfig { properties }
    }
}

/// The [`AWS::CodeDeploy::DeploymentGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentgroup.html) resource type.
pub struct DeploymentGroup {
    properties: DeploymentGroupProperties
}

/// Properties for the `DeploymentGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentGroupProperties {
    #[serde(rename="AlarmConfiguration")]
    pub alarm_configuration: self::deployment_group::AlarmConfiguration,
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="AutoRollbackConfiguration")]
    pub auto_rollback_configuration: self::deployment_group::AutoRollbackConfiguration,
    #[serde(rename="AutoScalingGroups")]
    pub auto_scaling_groups: Vec<String>,
    #[serde(rename="Deployment")]
    pub deployment: self::deployment_group::Deployment,
    #[serde(rename="DeploymentConfigName")]
    pub deployment_config_name: String,
    #[serde(rename="DeploymentGroupName")]
    pub deployment_group_name: String,
    #[serde(rename="DeploymentStyle")]
    pub deployment_style: self::deployment_group::DeploymentStyle,
    #[serde(rename="Ec2TagFilters")]
    pub ec2_tag_filters: Vec<self::deployment_group::EC2TagFilter>,
    #[serde(rename="LoadBalancerInfo")]
    pub load_balancer_info: self::deployment_group::LoadBalancerInfo,
    #[serde(rename="OnPremisesInstanceTagFilters")]
    pub on_premises_instance_tag_filters: Vec<self::deployment_group::TagFilter>,
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[serde(rename="TriggerConfigurations")]
    pub trigger_configurations: Vec<self::deployment_group::TriggerConfig>,
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

impl ::private::Sealed for DeploymentGroup {}

impl From<DeploymentGroupProperties> for DeploymentGroup {
    fn from(properties: DeploymentGroupProperties) -> DeploymentGroup {
        DeploymentGroup { properties }
    }
}

pub mod deployment_config {
    /// The [`AWS::CodeDeploy::DeploymentConfig.MinimumHealthyHosts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-minimumhealthyhosts.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MinimumHealthyHosts {
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Value")]
        pub value: u32,
    }

}

pub mod deployment_group {
    /// The [`AWS::CodeDeploy::DeploymentGroup.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarm.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Alarm {
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AlarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarmconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AlarmConfiguration {
        #[serde(rename="Alarms")]
        pub alarms: Vec<Alarm>,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="IgnorePollAlarmFailure")]
        pub ignore_poll_alarm_failure: bool,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AutoRollbackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-autorollbackconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AutoRollbackConfiguration {
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Events")]
        pub events: Vec<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Deployment {
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="IgnoreApplicationStopFailures")]
        pub ignore_application_stop_failures: bool,
        #[serde(rename="Revision")]
        pub revision: RevisionLocation,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.DeploymentStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentstyle.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DeploymentStyle {
        #[serde(rename="DeploymentOption")]
        pub deployment_option: String,
        #[serde(rename="DeploymentType")]
        pub deployment_type: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.EC2TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagfilters.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct EC2TagFilter {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.ELBInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-elbinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ELBInfo {
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.GitHubLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-githublocation.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct GitHubLocation {
        #[serde(rename="CommitId")]
        pub commit_id: String,
        #[serde(rename="Repository")]
        pub repository: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.LoadBalancerInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-loadbalancerinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LoadBalancerInfo {
        #[serde(rename="ElbInfoList")]
        pub elb_info_list: Vec<ELBInfo>,
        #[serde(rename="TargetGroupInfoList")]
        pub target_group_info_list: Vec<TargetGroupInfo>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.RevisionLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct RevisionLocation {
        #[serde(rename="GitHubLocation")]
        pub git_hub_location: GitHubLocation,
        #[serde(rename="RevisionType")]
        pub revision_type: String,
        #[serde(rename="S3Location")]
        pub s3_location: S3Location,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-s3location.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3Location {
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="BundleType")]
        pub bundle_type: String,
        #[serde(rename="ETag")]
        pub e_tag: String,
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Version")]
        pub version: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisesinstancetagfilters.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TagFilter {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TargetGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgroupinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TargetGroupInfo {
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-triggerconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TriggerConfig {
        #[serde(rename="TriggerEvents")]
        pub trigger_events: Vec<String>,
        #[serde(rename="TriggerName")]
        pub trigger_name: String,
        #[serde(rename="TriggerTargetArn")]
        pub trigger_target_arn: String,
    }

}

