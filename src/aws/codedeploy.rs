//! Types for the `CodeDeploy` service.

/// The [`AWS::CodeDeploy::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-application.html) resource type.
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
    /// Property `ComputePlatform`.
    #[serde(rename="ComputePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
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
#[derive(Debug)]
pub struct DeploymentConfig {
    properties: DeploymentConfigProperties
}

/// Properties for the `DeploymentConfig` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentConfigProperties {
    /// Property `DeploymentConfigName`.
    #[serde(rename="DeploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// Property `MinimumHealthyHosts`.
    #[serde(rename="MinimumHealthyHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<self::deployment_config::MinimumHealthyHosts>,
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
#[derive(Debug)]
pub struct DeploymentGroup {
    properties: DeploymentGroupProperties
}

/// Properties for the `DeploymentGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentGroupProperties {
    /// Property `AlarmConfiguration`.
    #[serde(rename="AlarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<self::deployment_group::AlarmConfiguration>,
    /// Property `ApplicationName`.
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    /// Property `AutoRollbackConfiguration`.
    #[serde(rename="AutoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<self::deployment_group::AutoRollbackConfiguration>,
    /// Property `AutoScalingGroups`.
    #[serde(rename="AutoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// Property `Deployment`.
    #[serde(rename="Deployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<self::deployment_group::Deployment>,
    /// Property `DeploymentConfigName`.
    #[serde(rename="DeploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// Property `DeploymentGroupName`.
    #[serde(rename="DeploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// Property `DeploymentStyle`.
    #[serde(rename="DeploymentStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<self::deployment_group::DeploymentStyle>,
    /// Property `Ec2TagFilters`.
    #[serde(rename="Ec2TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_filters: Option<Vec<self::deployment_group::EC2TagFilter>>,
    /// Property `LoadBalancerInfo`.
    #[serde(rename="LoadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<self::deployment_group::LoadBalancerInfo>,
    /// Property `OnPremisesInstanceTagFilters`.
    #[serde(rename="OnPremisesInstanceTagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<self::deployment_group::TagFilter>>,
    /// Property `ServiceRoleArn`.
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    /// Property `TriggerConfigurations`.
    #[serde(rename="TriggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<self::deployment_group::TriggerConfig>>,
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
    //! Property types for the `DeploymentConfig` resource.

    /// The [`AWS::CodeDeploy::DeploymentConfig.MinimumHealthyHosts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-minimumhealthyhosts.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MinimumHealthyHosts {
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: u32,
    }
}

pub mod deployment_group {
    //! Property types for the `DeploymentGroup` resource.

    /// The [`AWS::CodeDeploy::DeploymentGroup.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarm.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Alarm {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AlarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarmconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AlarmConfiguration {
        /// Property `Alarms`.
        #[serde(rename="Alarms")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alarms: Option<Vec<Alarm>>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        /// Property `IgnorePollAlarmFailure`.
        #[serde(rename="IgnorePollAlarmFailure")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ignore_poll_alarm_failure: Option<bool>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AutoRollbackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-autorollbackconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoRollbackConfiguration {
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        /// Property `Events`.
        #[serde(rename="Events")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<String>>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Deployment {
        /// Property `Description`.
        #[serde(rename="Description")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// Property `IgnoreApplicationStopFailures`.
        #[serde(rename="IgnoreApplicationStopFailures")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ignore_application_stop_failures: Option<bool>,
        /// Property `Revision`.
        #[serde(rename="Revision")]
        pub revision: RevisionLocation,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.DeploymentStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentstyle.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeploymentStyle {
        /// Property `DeploymentOption`.
        #[serde(rename="DeploymentOption")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deployment_option: Option<String>,
        /// Property `DeploymentType`.
        #[serde(rename="DeploymentType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deployment_type: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.EC2TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagfilters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EC2TagFilter {
        /// Property `Key`.
        #[serde(rename="Key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.ELBInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-elbinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ELBInfo {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.GitHubLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-githublocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GitHubLocation {
        /// Property `CommitId`.
        #[serde(rename="CommitId")]
        pub commit_id: String,
        /// Property `Repository`.
        #[serde(rename="Repository")]
        pub repository: String,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.LoadBalancerInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-loadbalancerinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancerInfo {
        /// Property `ElbInfoList`.
        #[serde(rename="ElbInfoList")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub elb_info_list: Option<Vec<ELBInfo>>,
        /// Property `TargetGroupInfoList`.
        #[serde(rename="TargetGroupInfoList")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target_group_info_list: Option<Vec<TargetGroupInfo>>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.RevisionLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RevisionLocation {
        /// Property `GitHubLocation`.
        #[serde(rename="GitHubLocation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub git_hub_location: Option<GitHubLocation>,
        /// Property `RevisionType`.
        #[serde(rename="RevisionType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub revision_type: Option<String>,
        /// Property `S3Location`.
        #[serde(rename="S3Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_location: Option<S3Location>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-s3location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Location {
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `BundleType`.
        #[serde(rename="BundleType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bundle_type: Option<String>,
        /// Property `ETag`.
        #[serde(rename="ETag")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub e_tag: Option<String>,
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Version`.
        #[serde(rename="Version")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisesinstancetagfilters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagFilter {
        /// Property `Key`.
        #[serde(rename="Key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TargetGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgroupinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetGroupInfo {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-triggerconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TriggerConfig {
        /// Property `TriggerEvents`.
        #[serde(rename="TriggerEvents")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_events: Option<Vec<String>>,
        /// Property `TriggerName`.
        #[serde(rename="TriggerName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_name: Option<String>,
        /// Property `TriggerTargetArn`.
        #[serde(rename="TriggerTargetArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_target_arn: Option<String>,
    }
}
