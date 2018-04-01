//! Types for the `Batch` service.

/// The [`AWS::Batch::ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html) resource type.
#[derive(Debug)]
pub struct ComputeEnvironment {
    properties: ComputeEnvironmentProperties
}

/// Properties for the `ComputeEnvironment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeEnvironmentProperties {
    /// Property `ComputeEnvironmentName`.
    #[serde(rename = "ComputeEnvironmentName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<::Value<String>>,
    /// Property `ComputeResources`.
    #[serde(rename = "ComputeResources")]
    pub compute_resources: ::Value<self::compute_environment::ComputeResources>,
    /// Property `ServiceRole`.
    #[serde(rename = "ServiceRole")]
    pub service_role: ::Value<String>,
    /// Property `State`.
    #[serde(rename = "State")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<::Value<String>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
}

impl<'a> ::Resource<'a> for ComputeEnvironment {
    type Properties = ComputeEnvironmentProperties;
    const TYPE: &'static str = "AWS::Batch::ComputeEnvironment";
    fn properties(&self) -> &ComputeEnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComputeEnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ComputeEnvironment {}

impl From<ComputeEnvironmentProperties> for ComputeEnvironment {
    fn from(properties: ComputeEnvironmentProperties) -> ComputeEnvironment {
        ComputeEnvironment { properties }
    }
}

/// The [`AWS::Batch::JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html) resource type.
#[derive(Debug)]
pub struct JobDefinition {
    properties: JobDefinitionProperties
}

/// Properties for the `JobDefinition` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    /// Property `ContainerProperties`.
    #[serde(rename = "ContainerProperties")]
    pub container_properties: ::Value<self::job_definition::ContainerProperties>,
    /// Property `JobDefinitionName`.
    #[serde(rename = "JobDefinitionName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<::Value<String>>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::Value<::json::Value>>,
    /// Property `RetryStrategy`.
    #[serde(rename = "RetryStrategy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<::Value<self::job_definition::RetryStrategy>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
}

impl<'a> ::Resource<'a> for JobDefinition {
    type Properties = JobDefinitionProperties;
    const TYPE: &'static str = "AWS::Batch::JobDefinition";
    fn properties(&self) -> &JobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for JobDefinition {}

impl From<JobDefinitionProperties> for JobDefinition {
    fn from(properties: JobDefinitionProperties) -> JobDefinition {
        JobDefinition { properties }
    }
}

/// The [`AWS::Batch::JobQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html) resource type.
#[derive(Debug)]
pub struct JobQueue {
    properties: JobQueueProperties
}

/// Properties for the `JobQueue` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct JobQueueProperties {
    /// Property `ComputeEnvironmentOrder`.
    #[serde(rename = "ComputeEnvironmentOrder")]
    pub compute_environment_order: ::ValueList<self::job_queue::ComputeEnvironmentOrder>,
    /// Property `JobQueueName`.
    #[serde(rename = "JobQueueName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<::Value<String>>,
    /// Property `Priority`.
    #[serde(rename = "Priority")]
    pub priority: ::Value<u32>,
    /// Property `State`.
    #[serde(rename = "State")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for JobQueue {
    type Properties = JobQueueProperties;
    const TYPE: &'static str = "AWS::Batch::JobQueue";
    fn properties(&self) -> &JobQueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobQueueProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for JobQueue {}

impl From<JobQueueProperties> for JobQueue {
    fn from(properties: JobQueueProperties) -> JobQueue {
        JobQueue { properties }
    }
}

pub mod compute_environment {
    //! Property types for the `ComputeEnvironment` resource.

    /// The [`AWS::Batch::ComputeEnvironment.ComputeResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComputeResources {
        /// Property `BidPercentage`.
        #[serde(rename = "BidPercentage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_percentage: Option<::Value<u32>>,
        /// Property `DesiredvCpus`.
        #[serde(rename = "DesiredvCpus")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub desiredv_cpus: Option<::Value<u32>>,
        /// Property `Ec2KeyPair`.
        #[serde(rename = "Ec2KeyPair")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_key_pair: Option<::Value<String>>,
        /// Property `ImageId`.
        #[serde(rename = "ImageId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image_id: Option<::Value<String>>,
        /// Property `InstanceRole`.
        #[serde(rename = "InstanceRole")]
        pub instance_role: ::Value<String>,
        /// Property `InstanceTypes`.
        #[serde(rename = "InstanceTypes")]
        pub instance_types: ::ValueList<String>,
        /// Property `MaxvCpus`.
        #[serde(rename = "MaxvCpus")]
        pub maxv_cpus: ::Value<u32>,
        /// Property `MinvCpus`.
        #[serde(rename = "MinvCpus")]
        pub minv_cpus: ::Value<u32>,
        /// Property `SecurityGroupIds`.
        #[serde(rename = "SecurityGroupIds")]
        pub security_group_ids: ::ValueList<String>,
        /// Property `SpotIamFleetRole`.
        #[serde(rename = "SpotIamFleetRole")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spot_iam_fleet_role: Option<::Value<String>>,
        /// Property `Subnets`.
        #[serde(rename = "Subnets")]
        pub subnets: ::ValueList<String>,
        /// Property `Tags`.
        #[serde(rename = "Tags")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<::Value<::json::Value>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ComputeResources);
}

pub mod job_definition {
    //! Property types for the `JobDefinition` resource.

    /// The [`AWS::Batch::JobDefinition.ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContainerProperties {
        /// Property `Command`.
        #[serde(rename = "Command")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command: Option<::ValueList<String>>,
        /// Property `Environment`.
        #[serde(rename = "Environment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub environment: Option<::ValueList<Environment>>,
        /// Property `Image`.
        #[serde(rename = "Image")]
        pub image: ::Value<String>,
        /// Property `JobRoleArn`.
        #[serde(rename = "JobRoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub job_role_arn: Option<::Value<String>>,
        /// Property `Memory`.
        #[serde(rename = "Memory")]
        pub memory: ::Value<u32>,
        /// Property `MountPoints`.
        #[serde(rename = "MountPoints")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_points: Option<::ValueList<MountPoints>>,
        /// Property `Privileged`.
        #[serde(rename = "Privileged")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privileged: Option<::Value<bool>>,
        /// Property `ReadonlyRootFilesystem`.
        #[serde(rename = "ReadonlyRootFilesystem")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub readonly_root_filesystem: Option<::Value<bool>>,
        /// Property `Ulimits`.
        #[serde(rename = "Ulimits")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ulimits: Option<::ValueList<Ulimit>>,
        /// Property `User`.
        #[serde(rename = "User")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<::Value<String>>,
        /// Property `Vcpus`.
        #[serde(rename = "Vcpus")]
        pub vcpus: ::Value<u32>,
        /// Property `Volumes`.
        #[serde(rename = "Volumes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volumes: Option<::ValueList<Volumes>>,
    }

    cfn_internal__inherit_codec_impls!(ContainerProperties);

    /// The [`AWS::Batch::JobDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Environment);

    /// The [`AWS::Batch::JobDefinition.MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MountPoints {
        /// Property `ContainerPath`.
        #[serde(rename = "ContainerPath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container_path: Option<::Value<String>>,
        /// Property `ReadOnly`.
        #[serde(rename = "ReadOnly")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceVolume`.
        #[serde(rename = "SourceVolume")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_volume: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(MountPoints);

    /// The [`AWS::Batch::JobDefinition.RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RetryStrategy {
        /// Property `Attempts`.
        #[serde(rename = "Attempts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attempts: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(RetryStrategy);

    /// The [`AWS::Batch::JobDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        #[serde(rename = "HardLimit")]
        pub hard_limit: ::Value<u32>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `SoftLimit`.
        #[serde(rename = "SoftLimit")]
        pub soft_limit: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(Ulimit);

    /// The [`AWS::Batch::JobDefinition.Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Volumes {
        /// Property `Host`.
        #[serde(rename = "Host")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host: Option<::Value<VolumesHost>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Volumes);

    /// The [`AWS::Batch::JobDefinition.VolumesHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumesHost {
        /// Property `SourcePath`.
        #[serde(rename = "SourcePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_path: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(VolumesHost);
}

pub mod job_queue {
    //! Property types for the `JobQueue` resource.

    /// The [`AWS::Batch::JobQueue.ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComputeEnvironmentOrder {
        /// Property `ComputeEnvironment`.
        #[serde(rename = "ComputeEnvironment")]
        pub compute_environment: ::Value<String>,
        /// Property `Order`.
        #[serde(rename = "Order")]
        pub order: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ComputeEnvironmentOrder);
}
