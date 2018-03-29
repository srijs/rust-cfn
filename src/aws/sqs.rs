//! Types for the `SQS` service.

/// The [`AWS::SQS::Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html) resource type.
#[derive(Debug)]
pub struct Queue {
    properties: QueueProperties
}

/// Properties for the `Queue` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueueProperties {
    /// Property `ContentBasedDeduplication`.
    #[serde(rename="ContentBasedDeduplication")]
    pub content_based_deduplication: bool,
    /// Property `DelaySeconds`.
    #[serde(rename="DelaySeconds")]
    pub delay_seconds: u32,
    /// Property `FifoQueue`.
    #[serde(rename="FifoQueue")]
    pub fifo_queue: bool,
    /// Property `KmsDataKeyReusePeriodSeconds`.
    #[serde(rename="KmsDataKeyReusePeriodSeconds")]
    pub kms_data_key_reuse_period_seconds: u32,
    /// Property `KmsMasterKeyId`.
    #[serde(rename="KmsMasterKeyId")]
    pub kms_master_key_id: String,
    /// Property `MaximumMessageSize`.
    #[serde(rename="MaximumMessageSize")]
    pub maximum_message_size: u32,
    /// Property `MessageRetentionPeriod`.
    #[serde(rename="MessageRetentionPeriod")]
    pub message_retention_period: u32,
    /// Property `QueueName`.
    #[serde(rename="QueueName")]
    pub queue_name: String,
    /// Property `ReceiveMessageWaitTimeSeconds`.
    #[serde(rename="ReceiveMessageWaitTimeSeconds")]
    pub receive_message_wait_time_seconds: u32,
    /// Property `RedrivePolicy`.
    #[serde(rename="RedrivePolicy")]
    pub redrive_policy: ::json::Value,
    /// Property `VisibilityTimeout`.
    #[serde(rename="VisibilityTimeout")]
    pub visibility_timeout: u32,
}

impl<'a> ::Resource<'a> for Queue {
    type Properties = QueueProperties;
    const TYPE: &'static str = "AWS::SQS::Queue";
    fn properties(&self) -> &QueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueueProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Queue {}

impl From<QueueProperties> for Queue {
    fn from(properties: QueueProperties) -> Queue {
        Queue { properties }
    }
}

/// The [`AWS::SQS::QueuePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html) resource type.
#[derive(Debug)]
pub struct QueuePolicy {
    properties: QueuePolicyProperties
}

/// Properties for the `QueuePolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueuePolicyProperties {
    /// Property `PolicyDocument`.
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    /// Property `Queues`.
    #[serde(rename="Queues")]
    pub queues: Vec<String>,
}

impl<'a> ::Resource<'a> for QueuePolicy {
    type Properties = QueuePolicyProperties;
    const TYPE: &'static str = "AWS::SQS::QueuePolicy";
    fn properties(&self) -> &QueuePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueuePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for QueuePolicy {}

impl From<QueuePolicyProperties> for QueuePolicy {
    fn from(properties: QueuePolicyProperties) -> QueuePolicy {
        QueuePolicy { properties }
    }
}
