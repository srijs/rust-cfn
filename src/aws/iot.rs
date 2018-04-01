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
    #[serde(rename = "CertificateSigningRequest")]
    pub certificate_signing_request: ::Value<String>,
    /// Property `Status`.
    #[serde(rename = "Status")]
    pub status: ::Value<String>,
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
    #[serde(rename = "PolicyDocument")]
    pub policy_document: ::Value<::json::Value>,
    /// Property `PolicyName`.
    #[serde(rename = "PolicyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<::Value<String>>,
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
    #[serde(rename = "PolicyName")]
    pub policy_name: ::Value<String>,
    /// Property `Principal`.
    #[serde(rename = "Principal")]
    pub principal: ::Value<String>,
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
    #[serde(rename = "AttributePayload")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<::Value<self::thing::AttributePayload>>,
    /// Property `ThingName`.
    #[serde(rename = "ThingName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<::Value<String>>,
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
    #[serde(rename = "Principal")]
    pub principal: ::Value<String>,
    /// Property `ThingName`.
    #[serde(rename = "ThingName")]
    pub thing_name: ::Value<String>,
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
    #[serde(rename = "RuleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<::Value<String>>,
    /// Property `TopicRulePayload`.
    #[serde(rename = "TopicRulePayload")]
    pub topic_rule_payload: ::Value<self::topic_rule::TopicRulePayload>,
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
        #[serde(rename = "Attributes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attributes: Option<::std::collections::HashMap<String, ::Value<String>>>,
    }

    cfn_internal__inherit_codec_impls!(AttributePayload);
}

pub mod topic_rule {
    //! Property types for the `TopicRule` resource.

    /// The [`AWS::IoT::TopicRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `CloudwatchAlarm`.
        #[serde(rename = "CloudwatchAlarm")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloudwatch_alarm: Option<::Value<CloudwatchAlarmAction>>,
        /// Property `CloudwatchMetric`.
        #[serde(rename = "CloudwatchMetric")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloudwatch_metric: Option<::Value<CloudwatchMetricAction>>,
        /// Property `DynamoDB`.
        #[serde(rename = "DynamoDB")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dynamo_db: Option<::Value<DynamoDBAction>>,
        /// Property `DynamoDBv2`.
        #[serde(rename = "DynamoDBv2")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dynamo_d_bv2: Option<::Value<DynamoDBv2Action>>,
        /// Property `Elasticsearch`.
        #[serde(rename = "Elasticsearch")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub elasticsearch: Option<::Value<ElasticsearchAction>>,
        /// Property `Firehose`.
        #[serde(rename = "Firehose")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub firehose: Option<::Value<FirehoseAction>>,
        /// Property `Kinesis`.
        #[serde(rename = "Kinesis")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis: Option<::Value<KinesisAction>>,
        /// Property `Lambda`.
        #[serde(rename = "Lambda")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda: Option<::Value<LambdaAction>>,
        /// Property `Republish`.
        #[serde(rename = "Republish")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub republish: Option<::Value<RepublishAction>>,
        /// Property `S3`.
        #[serde(rename = "S3")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3: Option<::Value<S3Action>>,
        /// Property `Sns`.
        #[serde(rename = "Sns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sns: Option<::Value<SnsAction>>,
        /// Property `Sqs`.
        #[serde(rename = "Sqs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sqs: Option<::Value<SqsAction>>,
    }

    cfn_internal__inherit_codec_impls!(Action);

    /// The [`AWS::IoT::TopicRule.CloudwatchAlarmAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudwatchAlarmAction {
        /// Property `AlarmName`.
        #[serde(rename = "AlarmName")]
        pub alarm_name: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `StateReason`.
        #[serde(rename = "StateReason")]
        pub state_reason: ::Value<String>,
        /// Property `StateValue`.
        #[serde(rename = "StateValue")]
        pub state_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CloudwatchAlarmAction);

    /// The [`AWS::IoT::TopicRule.CloudwatchMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudwatchMetricAction {
        /// Property `MetricName`.
        #[serde(rename = "MetricName")]
        pub metric_name: ::Value<String>,
        /// Property `MetricNamespace`.
        #[serde(rename = "MetricNamespace")]
        pub metric_namespace: ::Value<String>,
        /// Property `MetricTimestamp`.
        #[serde(rename = "MetricTimestamp")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metric_timestamp: Option<::Value<String>>,
        /// Property `MetricUnit`.
        #[serde(rename = "MetricUnit")]
        pub metric_unit: ::Value<String>,
        /// Property `MetricValue`.
        #[serde(rename = "MetricValue")]
        pub metric_value: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CloudwatchMetricAction);

    /// The [`AWS::IoT::TopicRule.DynamoDBAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DynamoDBAction {
        /// Property `HashKeyField`.
        #[serde(rename = "HashKeyField")]
        pub hash_key_field: ::Value<String>,
        /// Property `HashKeyType`.
        #[serde(rename = "HashKeyType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hash_key_type: Option<::Value<String>>,
        /// Property `HashKeyValue`.
        #[serde(rename = "HashKeyValue")]
        pub hash_key_value: ::Value<String>,
        /// Property `PayloadField`.
        #[serde(rename = "PayloadField")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload_field: Option<::Value<String>>,
        /// Property `RangeKeyField`.
        #[serde(rename = "RangeKeyField")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub range_key_field: Option<::Value<String>>,
        /// Property `RangeKeyType`.
        #[serde(rename = "RangeKeyType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub range_key_type: Option<::Value<String>>,
        /// Property `RangeKeyValue`.
        #[serde(rename = "RangeKeyValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub range_key_value: Option<::Value<String>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `TableName`.
        #[serde(rename = "TableName")]
        pub table_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(DynamoDBAction);

    /// The [`AWS::IoT::TopicRule.DynamoDBv2Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DynamoDBv2Action {
        /// Property `PutItem`.
        #[serde(rename = "PutItem")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub put_item: Option<::Value<PutItemInput>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DynamoDBv2Action);

    /// The [`AWS::IoT::TopicRule.ElasticsearchAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchAction {
        /// Property `Endpoint`.
        #[serde(rename = "Endpoint")]
        pub endpoint: ::Value<String>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Index`.
        #[serde(rename = "Index")]
        pub index: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ElasticsearchAction);

    /// The [`AWS::IoT::TopicRule.FirehoseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FirehoseAction {
        /// Property `DeliveryStreamName`.
        #[serde(rename = "DeliveryStreamName")]
        pub delivery_stream_name: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `Separator`.
        #[serde(rename = "Separator")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub separator: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(FirehoseAction);

    /// The [`AWS::IoT::TopicRule.KinesisAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisAction {
        /// Property `PartitionKey`.
        #[serde(rename = "PartitionKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub partition_key: Option<::Value<String>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `StreamName`.
        #[serde(rename = "StreamName")]
        pub stream_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisAction);

    /// The [`AWS::IoT::TopicRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaAction {
        /// Property `FunctionArn`.
        #[serde(rename = "FunctionArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub function_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LambdaAction);

    /// The [`AWS::IoT::TopicRule.PutItemInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PutItemInput {
        /// Property `TableName`.
        #[serde(rename = "TableName")]
        pub table_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PutItemInput);

    /// The [`AWS::IoT::TopicRule.RepublishAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RepublishAction {
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `Topic`.
        #[serde(rename = "Topic")]
        pub topic: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RepublishAction);

    /// The [`AWS::IoT::TopicRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Action {
        /// Property `BucketName`.
        #[serde(rename = "BucketName")]
        pub bucket_name: ::Value<String>,
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(S3Action);

    /// The [`AWS::IoT::TopicRule.SnsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SnsAction {
        /// Property `MessageFormat`.
        #[serde(rename = "MessageFormat")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message_format: Option<::Value<String>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `TargetArn`.
        #[serde(rename = "TargetArn")]
        pub target_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SnsAction);

    /// The [`AWS::IoT::TopicRule.SqsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SqsAction {
        /// Property `QueueUrl`.
        #[serde(rename = "QueueUrl")]
        pub queue_url: ::Value<String>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        pub role_arn: ::Value<String>,
        /// Property `UseBase64`.
        #[serde(rename = "UseBase64")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub use_base64: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(SqsAction);

    /// The [`AWS::IoT::TopicRule.TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TopicRulePayload {
        /// Property `Actions`.
        #[serde(rename = "Actions")]
        pub actions: ::ValueList<Action>,
        /// Property `AwsIotSqlVersion`.
        #[serde(rename = "AwsIotSqlVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aws_iot_sql_version: Option<::Value<String>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `RuleDisabled`.
        #[serde(rename = "RuleDisabled")]
        pub rule_disabled: ::Value<bool>,
        /// Property `Sql`.
        #[serde(rename = "Sql")]
        pub sql: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(TopicRulePayload);
}
