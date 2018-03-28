/// The [`AWS::Batch::ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ComputeEnvironment {
    properties: ComputeEnvironmentProperties
}

/// Properties for the `ComputeEnvironment` resource.
#[derive(Serialize, Deserialize)]
pub struct ComputeEnvironmentProperties {
    #[serde(rename="ComputeEnvironmentName")]
    pub compute_environment_name: String,
    #[serde(rename="ComputeResources")]
    pub compute_resources: (),
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    #[serde(rename="State")]
    pub state: String,
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

impl From<ComputeEnvironmentProperties> for ComputeEnvironment {
    fn from(properties: ComputeEnvironmentProperties) -> ComputeEnvironment {
        ComputeEnvironment { properties }
    }
}

/// The [`AWS::Batch::JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html) resource.
#[derive(Serialize, Deserialize)]
pub struct JobDefinition {
    properties: JobDefinitionProperties
}

/// Properties for the `JobDefinition` resource.
#[derive(Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    #[serde(rename="ContainerProperties")]
    pub container_properties: (),
    #[serde(rename="JobDefinitionName")]
    pub job_definition_name: String,
    #[serde(rename="Parameters")]
    pub parameters: String,
    #[serde(rename="RetryStrategy")]
    pub retry_strategy: (),
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

impl From<JobDefinitionProperties> for JobDefinition {
    fn from(properties: JobDefinitionProperties) -> JobDefinition {
        JobDefinition { properties }
    }
}

/// The [`AWS::Batch::JobQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html) resource.
#[derive(Serialize, Deserialize)]
pub struct JobQueue {
    properties: JobQueueProperties
}

/// Properties for the `JobQueue` resource.
#[derive(Serialize, Deserialize)]
pub struct JobQueueProperties {
    #[serde(rename="ComputeEnvironmentOrder")]
    pub compute_environment_order: Vec<()>,
    #[serde(rename="JobQueueName")]
    pub job_queue_name: String,
    #[serde(rename="Priority")]
    pub priority: u32,
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

impl From<JobQueueProperties> for JobQueue {
    fn from(properties: JobQueueProperties) -> JobQueue {
        JobQueue { properties }
    }
}

