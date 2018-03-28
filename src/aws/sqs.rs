/// The [`AWS::SQS::Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Queue {
    properties: QueueProperties
}

/// Properties for the `Queue` resource.
#[derive(Serialize, Deserialize)]
pub struct QueueProperties {
    #[serde(rename="ContentBasedDeduplication")]
    pub content_based_deduplication: (),
    #[serde(rename="DelaySeconds")]
    pub delay_seconds: (),
    #[serde(rename="FifoQueue")]
    pub fifo_queue: (),
    #[serde(rename="KmsDataKeyReusePeriodSeconds")]
    pub kms_data_key_reuse_period_seconds: (),
    #[serde(rename="KmsMasterKeyId")]
    pub kms_master_key_id: (),
    #[serde(rename="MaximumMessageSize")]
    pub maximum_message_size: (),
    #[serde(rename="MessageRetentionPeriod")]
    pub message_retention_period: (),
    #[serde(rename="QueueName")]
    pub queue_name: (),
    #[serde(rename="ReceiveMessageWaitTimeSeconds")]
    pub receive_message_wait_time_seconds: (),
    #[serde(rename="RedrivePolicy")]
    pub redrive_policy: (),
    #[serde(rename="VisibilityTimeout")]
    pub visibility_timeout: (),
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

impl From<QueueProperties> for Queue {
    fn from(properties: QueueProperties) -> Queue {
        Queue { properties }
    }
}

/// The [`AWS::SQS::QueuePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html) resource.
#[derive(Serialize, Deserialize)]
pub struct QueuePolicy {
    properties: QueuePolicyProperties
}

/// Properties for the `QueuePolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct QueuePolicyProperties {
    #[serde(rename="PolicyDocument")]
    pub policy_document: (),
    #[serde(rename="Queues")]
    pub queues: (),
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

impl From<QueuePolicyProperties> for QueuePolicy {
    fn from(properties: QueuePolicyProperties) -> QueuePolicy {
        QueuePolicy { properties }
    }
}

