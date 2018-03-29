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
    #[serde(rename="ComputeEnvironmentName")]
    pub compute_environment_name: String,
    /// Property `ComputeResources`.
    #[serde(rename="ComputeResources")]
    pub compute_resources: self::compute_environment::ComputeResources,
    /// Property `ServiceRole`.
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    /// Property `State`.
    #[serde(rename="State")]
    pub state: String,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
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
    #[serde(rename="ContainerProperties")]
    pub container_properties: self::job_definition::ContainerProperties,
    /// Property `JobDefinitionName`.
    #[serde(rename="JobDefinitionName")]
    pub job_definition_name: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    pub parameters: ::json::Value,
    /// Property `RetryStrategy`.
    #[serde(rename="RetryStrategy")]
    pub retry_strategy: self::job_definition::RetryStrategy,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
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
    #[serde(rename="ComputeEnvironmentOrder")]
    pub compute_environment_order: Vec<self::job_queue::ComputeEnvironmentOrder>,
    /// Property `JobQueueName`.
    #[serde(rename="JobQueueName")]
    pub job_queue_name: String,
    /// Property `Priority`.
    #[serde(rename="Priority")]
    pub priority: u32,
    /// Property `State`.
    #[serde(rename="State")]
    pub state: String,
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
        #[serde(rename="BidPercentage")]
        pub bid_percentage: u32,
        /// Property `DesiredvCpus`.
        #[serde(rename="DesiredvCpus")]
        pub desiredv_cpus: u32,
        /// Property `Ec2KeyPair`.
        #[serde(rename="Ec2KeyPair")]
        pub ec2_key_pair: String,
        /// Property `ImageId`.
        #[serde(rename="ImageId")]
        pub image_id: String,
        /// Property `InstanceRole`.
        #[serde(rename="InstanceRole")]
        pub instance_role: String,
        /// Property `InstanceTypes`.
        #[serde(rename="InstanceTypes")]
        pub instance_types: Vec<String>,
        /// Property `MaxvCpus`.
        #[serde(rename="MaxvCpus")]
        pub maxv_cpus: u32,
        /// Property `MinvCpus`.
        #[serde(rename="MinvCpus")]
        pub minv_cpus: u32,
        /// Property `SecurityGroupIds`.
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        /// Property `SpotIamFleetRole`.
        #[serde(rename="SpotIamFleetRole")]
        pub spot_iam_fleet_role: String,
        /// Property `Subnets`.
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
        /// Property `Tags`.
        #[serde(rename="Tags")]
        pub tags: ::json::Value,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod job_definition {
    //! Property types for the `JobDefinition` resource.

    /// The [`AWS::Batch::JobDefinition.ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContainerProperties {
        /// Property `Command`.
        #[serde(rename="Command")]
        pub command: Vec<String>,
        /// Property `Environment`.
        #[serde(rename="Environment")]
        pub environment: Vec<Environment>,
        /// Property `Image`.
        #[serde(rename="Image")]
        pub image: String,
        /// Property `JobRoleArn`.
        #[serde(rename="JobRoleArn")]
        pub job_role_arn: String,
        /// Property `Memory`.
        #[serde(rename="Memory")]
        pub memory: u32,
        /// Property `MountPoints`.
        #[serde(rename="MountPoints")]
        pub mount_points: Vec<MountPoints>,
        /// Property `Privileged`.
        #[serde(rename="Privileged")]
        pub privileged: bool,
        /// Property `ReadonlyRootFilesystem`.
        #[serde(rename="ReadonlyRootFilesystem")]
        pub readonly_root_filesystem: bool,
        /// Property `Ulimits`.
        #[serde(rename="Ulimits")]
        pub ulimits: Vec<Ulimit>,
        /// Property `User`.
        #[serde(rename="User")]
        pub user: String,
        /// Property `Vcpus`.
        #[serde(rename="Vcpus")]
        pub vcpus: u32,
        /// Property `Volumes`.
        #[serde(rename="Volumes")]
        pub volumes: Vec<Volumes>,
    }

    /// The [`AWS::Batch::JobDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::Batch::JobDefinition.MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MountPoints {
        /// Property `ContainerPath`.
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        /// Property `ReadOnly`.
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        /// Property `SourceVolume`.
        #[serde(rename="SourceVolume")]
        pub source_volume: String,
    }

    /// The [`AWS::Batch::JobDefinition.RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RetryStrategy {
        /// Property `Attempts`.
        #[serde(rename="Attempts")]
        pub attempts: u32,
    }

    /// The [`AWS::Batch::JobDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        #[serde(rename="HardLimit")]
        pub hard_limit: u32,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `SoftLimit`.
        #[serde(rename="SoftLimit")]
        pub soft_limit: u32,
    }

    /// The [`AWS::Batch::JobDefinition.Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Volumes {
        /// Property `Host`.
        #[serde(rename="Host")]
        pub host: VolumesHost,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::Batch::JobDefinition.VolumesHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumesHost {
        /// Property `SourcePath`.
        #[serde(rename="SourcePath")]
        pub source_path: String,
    }
}

pub mod job_queue {
    //! Property types for the `JobQueue` resource.

    /// The [`AWS::Batch::JobQueue.ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComputeEnvironmentOrder {
        /// Property `ComputeEnvironment`.
        #[serde(rename="ComputeEnvironment")]
        pub compute_environment: String,
        /// Property `Order`.
        #[serde(rename="Order")]
        pub order: u32,
    }
}
