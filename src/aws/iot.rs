//! Types for the `IoT` service.

/// The [`AWS::IoT::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html) resource type.
#[derive(Debug)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateProperties {
    /// Property `CertificateSigningRequest`.
    #[serde(rename="CertificateSigningRequest")]
    pub certificate_signing_request: String,
    /// Property `Status`.
    #[serde(rename="Status")]
    pub status: String,
}

impl<'a> ::Resource<'a> for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::IoT::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::IoT::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html) resource type.
#[derive(Debug)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyProperties {
    /// Property `PolicyDocument`.
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    /// Property `PolicyName`.
    #[serde(rename="PolicyName")]
    pub policy_name: String,
}

impl<'a> ::Resource<'a> for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::IoT::Policy";
    fn properties(&self) -> &PolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Policy {}

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IoT::PolicyPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html) resource type.
#[derive(Debug)]
pub struct PolicyPrincipalAttachment {
    properties: PolicyPrincipalAttachmentProperties
}

/// Properties for the `PolicyPrincipalAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyPrincipalAttachmentProperties {
    /// Property `PolicyName`.
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    /// Property `Principal`.
    #[serde(rename="Principal")]
    pub principal: String,
}

impl<'a> ::Resource<'a> for PolicyPrincipalAttachment {
    type Properties = PolicyPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::PolicyPrincipalAttachment";
    fn properties(&self) -> &PolicyPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PolicyPrincipalAttachment {}

impl From<PolicyPrincipalAttachmentProperties> for PolicyPrincipalAttachment {
    fn from(properties: PolicyPrincipalAttachmentProperties) -> PolicyPrincipalAttachment {
        PolicyPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::Thing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html) resource type.
#[derive(Debug)]
pub struct Thing {
    properties: ThingProperties
}

/// Properties for the `Thing` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThingProperties {
    /// Property `AttributePayload`.
    #[serde(rename="AttributePayload")]
    pub attribute_payload: self::thing::AttributePayload,
    /// Property `ThingName`.
    #[serde(rename="ThingName")]
    pub thing_name: String,
}

impl<'a> ::Resource<'a> for Thing {
    type Properties = ThingProperties;
    const TYPE: &'static str = "AWS::IoT::Thing";
    fn properties(&self) -> &ThingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Thing {}

impl From<ThingProperties> for Thing {
    fn from(properties: ThingProperties) -> Thing {
        Thing { properties }
    }
}

/// The [`AWS::IoT::ThingPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html) resource type.
#[derive(Debug)]
pub struct ThingPrincipalAttachment {
    properties: ThingPrincipalAttachmentProperties
}

/// Properties for the `ThingPrincipalAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThingPrincipalAttachmentProperties {
    /// Property `Principal`.
    #[serde(rename="Principal")]
    pub principal: String,
    /// Property `ThingName`.
    #[serde(rename="ThingName")]
    pub thing_name: String,
}

impl<'a> ::Resource<'a> for ThingPrincipalAttachment {
    type Properties = ThingPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::ThingPrincipalAttachment";
    fn properties(&self) -> &ThingPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThingPrincipalAttachment {}

impl From<ThingPrincipalAttachmentProperties> for ThingPrincipalAttachment {
    fn from(properties: ThingPrincipalAttachmentProperties) -> ThingPrincipalAttachment {
        ThingPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::TopicRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html) resource type.
#[derive(Debug)]
pub struct TopicRule {
    properties: TopicRuleProperties
}

/// Properties for the `TopicRule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicRuleProperties {
    /// Property `RuleName`.
    #[serde(rename="RuleName")]
    pub rule_name: String,
    /// Property `TopicRulePayload`.
    #[serde(rename="TopicRulePayload")]
    pub topic_rule_payload: self::topic_rule::TopicRulePayload,
}

impl<'a> ::Resource<'a> for TopicRule {
    type Properties = TopicRuleProperties;
    const TYPE: &'static str = "AWS::IoT::TopicRule";
    fn properties(&self) -> &TopicRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicRule {}

impl From<TopicRuleProperties> for TopicRule {
    fn from(properties: TopicRuleProperties) -> TopicRule {
        TopicRule { properties }
    }
}

pub mod thing {
    //! Property types for the `Thing` resource.

    /// The [`AWS::IoT::Thing.AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributePayload {
        /// Property `Attributes`.
        #[serde(rename="Attributes")]
        pub attributes: ::std::collections::HashMap<String, String>,
    }
}

pub mod topic_rule {
    //! Property types for the `TopicRule` resource.

    /// The [`AWS::IoT::TopicRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `CloudwatchAlarm`.
        #[serde(rename="CloudwatchAlarm")]
        pub cloudwatch_alarm: CloudwatchAlarmAction,
        /// Property `CloudwatchMetric`.
        #[serde(rename="CloudwatchMetric")]
        pub cloudwatch_metric: CloudwatchMetricAction,
        /// Property `DynamoDB`.
        #[serde(rename="DynamoDB")]
        pub dynamo_db: DynamoDBAction,
        /// Property `DynamoDBv2`.
        #[serde(rename="DynamoDBv2")]
        pub dynamo_d_bv2: DynamoDBv2Action,
        /// Property `Elasticsearch`.
        #[serde(rename="Elasticsearch")]
        pub elasticsearch: ElasticsearchAction,
        /// Property `Firehose`.
        #[serde(rename="Firehose")]
        pub firehose: FirehoseAction,
        /// Property `Kinesis`.
        #[serde(rename="Kinesis")]
        pub kinesis: KinesisAction,
        /// Property `Lambda`.
        #[serde(rename="Lambda")]
        pub lambda: LambdaAction,
        /// Property `Republish`.
        #[serde(rename="Republish")]
        pub republish: RepublishAction,
        /// Property `S3`.
        #[serde(rename="S3")]
        pub s3: S3Action,
        /// Property `Sns`.
        #[serde(rename="Sns")]
        pub sns: SnsAction,
        /// Property `Sqs`.
        #[serde(rename="Sqs")]
        pub sqs: SqsAction,
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchAlarmAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudwatchAlarmAction {
        /// Property `AlarmName`.
        #[serde(rename="AlarmName")]
        pub alarm_name: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `StateReason`.
        #[serde(rename="StateReason")]
        pub state_reason: String,
        /// Property `StateValue`.
        #[serde(rename="StateValue")]
        pub state_value: String,
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudwatchMetricAction {
        /// Property `MetricName`.
        #[serde(rename="MetricName")]
        pub metric_name: String,
        /// Property `MetricNamespace`.
        #[serde(rename="MetricNamespace")]
        pub metric_namespace: String,
        /// Property `MetricTimestamp`.
        #[serde(rename="MetricTimestamp")]
        pub metric_timestamp: String,
        /// Property `MetricUnit`.
        #[serde(rename="MetricUnit")]
        pub metric_unit: String,
        /// Property `MetricValue`.
        #[serde(rename="MetricValue")]
        pub metric_value: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DynamoDBAction {
        /// Property `HashKeyField`.
        #[serde(rename="HashKeyField")]
        pub hash_key_field: String,
        /// Property `HashKeyType`.
        #[serde(rename="HashKeyType")]
        pub hash_key_type: String,
        /// Property `HashKeyValue`.
        #[serde(rename="HashKeyValue")]
        pub hash_key_value: String,
        /// Property `PayloadField`.
        #[serde(rename="PayloadField")]
        pub payload_field: String,
        /// Property `RangeKeyField`.
        #[serde(rename="RangeKeyField")]
        pub range_key_field: String,
        /// Property `RangeKeyType`.
        #[serde(rename="RangeKeyType")]
        pub range_key_type: String,
        /// Property `RangeKeyValue`.
        #[serde(rename="RangeKeyValue")]
        pub range_key_value: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `TableName`.
        #[serde(rename="TableName")]
        pub table_name: String,
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBv2Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DynamoDBv2Action {
        /// Property `PutItem`.
        #[serde(rename="PutItem")]
        pub put_item: PutItemInput,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    /// The [`AWS::IoT::TopicRule.ElasticsearchAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchAction {
        /// Property `Endpoint`.
        #[serde(rename="Endpoint")]
        pub endpoint: String,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Index`.
        #[serde(rename="Index")]
        pub index: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::IoT::TopicRule.FirehoseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FirehoseAction {
        /// Property `DeliveryStreamName`.
        #[serde(rename="DeliveryStreamName")]
        pub delivery_stream_name: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `Separator`.
        #[serde(rename="Separator")]
        pub separator: String,
    }

    /// The [`AWS::IoT::TopicRule.KinesisAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisAction {
        /// Property `PartitionKey`.
        #[serde(rename="PartitionKey")]
        pub partition_key: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `StreamName`.
        #[serde(rename="StreamName")]
        pub stream_name: String,
    }

    /// The [`AWS::IoT::TopicRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaAction {
        /// Property `FunctionArn`.
        #[serde(rename="FunctionArn")]
        pub function_arn: String,
    }

    /// The [`AWS::IoT::TopicRule.PutItemInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PutItemInput {
        /// Property `TableName`.
        #[serde(rename="TableName")]
        pub table_name: String,
    }

    /// The [`AWS::IoT::TopicRule.RepublishAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RepublishAction {
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `Topic`.
        #[serde(rename="Topic")]
        pub topic: String,
    }

    /// The [`AWS::IoT::TopicRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Action {
        /// Property `BucketName`.
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    /// The [`AWS::IoT::TopicRule.SnsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SnsAction {
        /// Property `MessageFormat`.
        #[serde(rename="MessageFormat")]
        pub message_format: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `TargetArn`.
        #[serde(rename="TargetArn")]
        pub target_arn: String,
    }

    /// The [`AWS::IoT::TopicRule.SqsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SqsAction {
        /// Property `QueueUrl`.
        #[serde(rename="QueueUrl")]
        pub queue_url: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `UseBase64`.
        #[serde(rename="UseBase64")]
        pub use_base64: bool,
    }

    /// The [`AWS::IoT::TopicRule.TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TopicRulePayload {
        /// Property `Actions`.
        #[serde(rename="Actions")]
        pub actions: Vec<Action>,
        /// Property `AwsIotSqlVersion`.
        #[serde(rename="AwsIotSqlVersion")]
        pub aws_iot_sql_version: String,
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `RuleDisabled`.
        #[serde(rename="RuleDisabled")]
        pub rule_disabled: bool,
        /// Property `Sql`.
        #[serde(rename="Sql")]
        pub sql: String,
    }
}
