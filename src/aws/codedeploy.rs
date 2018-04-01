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
    #[serde(rename = "ApplicationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<::Value<String>>,
    /// Property `ComputePlatform`.
    #[serde(rename = "ComputePlatform")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<::Value<String>>,
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
    #[serde(rename = "DeploymentConfigName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<::Value<String>>,
    /// Property `MinimumHealthyHosts`.
    #[serde(rename = "MinimumHealthyHosts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<::Value<self::deployment_config::MinimumHealthyHosts>>,
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
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<::Value<self::deployment_group::AlarmConfiguration>>,
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `AutoRollbackConfiguration`.
    #[serde(rename = "AutoRollbackConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<::Value<self::deployment_group::AutoRollbackConfiguration>>,
    /// Property `AutoScalingGroups`.
    #[serde(rename = "AutoScalingGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<::ValueList<String>>,
    /// Property `Deployment`.
    #[serde(rename = "Deployment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<::Value<self::deployment_group::Deployment>>,
    /// Property `DeploymentConfigName`.
    #[serde(rename = "DeploymentConfigName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<::Value<String>>,
    /// Property `DeploymentGroupName`.
    #[serde(rename = "DeploymentGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<::Value<String>>,
    /// Property `DeploymentStyle`.
    #[serde(rename = "DeploymentStyle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<::Value<self::deployment_group::DeploymentStyle>>,
    /// Property `Ec2TagFilters`.
    #[serde(rename = "Ec2TagFilters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_tag_filters: Option<::ValueList<self::deployment_group::EC2TagFilter>>,
    /// Property `LoadBalancerInfo`.
    #[serde(rename = "LoadBalancerInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<::Value<self::deployment_group::LoadBalancerInfo>>,
    /// Property `OnPremisesInstanceTagFilters`.
    #[serde(rename = "OnPremisesInstanceTagFilters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<::ValueList<self::deployment_group::TagFilter>>,
    /// Property `ServiceRoleArn`.
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: ::Value<String>,
    /// Property `TriggerConfigurations`.
    #[serde(rename = "TriggerConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<::ValueList<self::deployment_group::TriggerConfig>>,
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
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(MinimumHealthyHosts);
}

pub mod deployment_group {
    //! Property types for the `DeploymentGroup` resource.

    /// The [`AWS::CodeDeploy::DeploymentGroup.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarm.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Alarm {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Alarm);

    /// The [`AWS::CodeDeploy::DeploymentGroup.AlarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarmconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AlarmConfiguration {
        /// Property `Alarms`.
        #[serde(rename = "Alarms")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alarms: Option<::ValueList<Alarm>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
        /// Property `IgnorePollAlarmFailure`.
        #[serde(rename = "IgnorePollAlarmFailure")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ignore_poll_alarm_failure: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(AlarmConfiguration);

    /// The [`AWS::CodeDeploy::DeploymentGroup.AutoRollbackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-autorollbackconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoRollbackConfiguration {
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
        /// Property `Events`.
        #[serde(rename = "Events")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub events: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(AutoRollbackConfiguration);

    /// The [`AWS::CodeDeploy::DeploymentGroup.Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Deployment {
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `IgnoreApplicationStopFailures`.
        #[serde(rename = "IgnoreApplicationStopFailures")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ignore_application_stop_failures: Option<::Value<bool>>,
        /// Property `Revision`.
        #[serde(rename = "Revision")]
        pub revision: ::Value<RevisionLocation>,
    }

    cfn_internal__inherit_codec_impls!(Deployment);

    /// The [`AWS::CodeDeploy::DeploymentGroup.DeploymentStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentstyle.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeploymentStyle {
        /// Property `DeploymentOption`.
        #[serde(rename = "DeploymentOption")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deployment_option: Option<::Value<String>>,
        /// Property `DeploymentType`.
        #[serde(rename = "DeploymentType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deployment_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DeploymentStyle);

    /// The [`AWS::CodeDeploy::DeploymentGroup.EC2TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagfilters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EC2TagFilter {
        /// Property `Key`.
        #[serde(rename = "Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(EC2TagFilter);

    /// The [`AWS::CodeDeploy::DeploymentGroup.ELBInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-elbinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ELBInfo {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ELBInfo);

    /// The [`AWS::CodeDeploy::DeploymentGroup.GitHubLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-githublocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GitHubLocation {
        /// Property `CommitId`.
        #[serde(rename = "CommitId")]
        pub commit_id: ::Value<String>,
        /// Property `Repository`.
        #[serde(rename = "Repository")]
        pub repository: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(GitHubLocation);

    /// The [`AWS::CodeDeploy::DeploymentGroup.LoadBalancerInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-loadbalancerinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancerInfo {
        /// Property `ElbInfoList`.
        #[serde(rename = "ElbInfoList")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub elb_info_list: Option<::ValueList<ELBInfo>>,
        /// Property `TargetGroupInfoList`.
        #[serde(rename = "TargetGroupInfoList")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_group_info_list: Option<::ValueList<TargetGroupInfo>>,
    }

    cfn_internal__inherit_codec_impls!(LoadBalancerInfo);

    /// The [`AWS::CodeDeploy::DeploymentGroup.RevisionLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RevisionLocation {
        /// Property `GitHubLocation`.
        #[serde(rename = "GitHubLocation")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub git_hub_location: Option<::Value<GitHubLocation>>,
        /// Property `RevisionType`.
        #[serde(rename = "RevisionType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub revision_type: Option<::Value<String>>,
        /// Property `S3Location`.
        #[serde(rename = "S3Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_location: Option<::Value<S3Location>>,
    }

    cfn_internal__inherit_codec_impls!(RevisionLocation);

    /// The [`AWS::CodeDeploy::DeploymentGroup.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-s3location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Location {
        /// Property `Bucket`.
        #[serde(rename = "Bucket")]
        pub bucket: ::Value<String>,
        /// Property `BundleType`.
        #[serde(rename = "BundleType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bundle_type: Option<::Value<String>>,
        /// Property `ETag`.
        #[serde(rename = "ETag")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub e_tag: Option<::Value<String>>,
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(S3Location);

    /// The [`AWS::CodeDeploy::DeploymentGroup.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisesinstancetagfilters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagFilter {
        /// Property `Key`.
        #[serde(rename = "Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(TagFilter);

    /// The [`AWS::CodeDeploy::DeploymentGroup.TargetGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgroupinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetGroupInfo {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(TargetGroupInfo);

    /// The [`AWS::CodeDeploy::DeploymentGroup.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-triggerconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TriggerConfig {
        /// Property `TriggerEvents`.
        #[serde(rename = "TriggerEvents")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trigger_events: Option<::ValueList<String>>,
        /// Property `TriggerName`.
        #[serde(rename = "TriggerName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trigger_name: Option<::Value<String>>,
        /// Property `TriggerTargetArn`.
        #[serde(rename = "TriggerTargetArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trigger_target_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(TriggerConfig);
}
