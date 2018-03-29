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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_based_deduplication: Option<bool>,
    /// Property `DelaySeconds`.
    #[serde(rename="DelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<u32>,
    /// Property `FifoQueue`.
    #[serde(rename="FifoQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fifo_queue: Option<bool>,
    /// Property `KmsDataKeyReusePeriodSeconds`.
    #[serde(rename="KmsDataKeyReusePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<u32>,
    /// Property `KmsMasterKeyId`.
    #[serde(rename="KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// Property `MaximumMessageSize`.
    #[serde(rename="MaximumMessageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_message_size: Option<u32>,
    /// Property `MessageRetentionPeriod`.
    #[serde(rename="MessageRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_period: Option<u32>,
    /// Property `QueueName`.
    #[serde(rename="QueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    /// Property `ReceiveMessageWaitTimeSeconds`.
    #[serde(rename="ReceiveMessageWaitTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_message_wait_time_seconds: Option<u32>,
    /// Property `RedrivePolicy`.
    #[serde(rename="RedrivePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_policy: Option<::json::Value>,
    /// Property `VisibilityTimeout`.
    #[serde(rename="VisibilityTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout: Option<u32>,
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
