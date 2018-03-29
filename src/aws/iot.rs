/// The [`AWS::IoT::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html) resource.
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(rename="CertificateSigningRequest")]
    pub certificate_signing_request: String,
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

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::IoT::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html) resource.
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Serialize, Deserialize)]
pub struct PolicyProperties {
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
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

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IoT::PolicyPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html) resource.
pub struct PolicyPrincipalAttachment {
    properties: PolicyPrincipalAttachmentProperties
}

/// Properties for the `PolicyPrincipalAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct PolicyPrincipalAttachmentProperties {
    #[serde(rename="PolicyName")]
    pub policy_name: String,
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

impl From<PolicyPrincipalAttachmentProperties> for PolicyPrincipalAttachment {
    fn from(properties: PolicyPrincipalAttachmentProperties) -> PolicyPrincipalAttachment {
        PolicyPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::Thing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html) resource.
pub struct Thing {
    properties: ThingProperties
}

/// Properties for the `Thing` resource.
#[derive(Serialize, Deserialize)]
pub struct ThingProperties {
    #[serde(rename="AttributePayload")]
    pub attribute_payload: self::thing::AttributePayload,
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

impl From<ThingProperties> for Thing {
    fn from(properties: ThingProperties) -> Thing {
        Thing { properties }
    }
}

/// The [`AWS::IoT::ThingPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html) resource.
pub struct ThingPrincipalAttachment {
    properties: ThingPrincipalAttachmentProperties
}

/// Properties for the `ThingPrincipalAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct ThingPrincipalAttachmentProperties {
    #[serde(rename="Principal")]
    pub principal: String,
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

impl From<ThingPrincipalAttachmentProperties> for ThingPrincipalAttachment {
    fn from(properties: ThingPrincipalAttachmentProperties) -> ThingPrincipalAttachment {
        ThingPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::TopicRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html) resource.
pub struct TopicRule {
    properties: TopicRuleProperties
}

/// Properties for the `TopicRule` resource.
#[derive(Serialize, Deserialize)]
pub struct TopicRuleProperties {
    #[serde(rename="RuleName")]
    pub rule_name: String,
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

impl From<TopicRuleProperties> for TopicRule {
    fn from(properties: TopicRuleProperties) -> TopicRule {
        TopicRule { properties }
    }
}

pub mod thing {
    #[derive(Serialize, Deserialize)]
    pub struct AttributePayload {
        #[serde(rename="Attributes")]
        pub attributes: ::std::collections::HashMap<String, String>,
    }

}

pub mod topic_rule {
    #[derive(Serialize, Deserialize)]
    pub struct Action {
        #[serde(rename="CloudwatchAlarm")]
        pub cloudwatch_alarm: CloudwatchAlarmAction,
        #[serde(rename="CloudwatchMetric")]
        pub cloudwatch_metric: CloudwatchMetricAction,
        #[serde(rename="DynamoDB")]
        pub dynamo_db: DynamoDBAction,
        #[serde(rename="DynamoDBv2")]
        pub dynamo_d_bv2: DynamoDBv2Action,
        #[serde(rename="Elasticsearch")]
        pub elasticsearch: ElasticsearchAction,
        #[serde(rename="Firehose")]
        pub firehose: FirehoseAction,
        #[serde(rename="Kinesis")]
        pub kinesis: KinesisAction,
        #[serde(rename="Lambda")]
        pub lambda: LambdaAction,
        #[serde(rename="Republish")]
        pub republish: RepublishAction,
        #[serde(rename="S3")]
        pub s3: S3Action,
        #[serde(rename="Sns")]
        pub sns: SnsAction,
        #[serde(rename="Sqs")]
        pub sqs: SqsAction,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CloudwatchAlarmAction {
        #[serde(rename="AlarmName")]
        pub alarm_name: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="StateReason")]
        pub state_reason: String,
        #[serde(rename="StateValue")]
        pub state_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CloudwatchMetricAction {
        #[serde(rename="MetricName")]
        pub metric_name: String,
        #[serde(rename="MetricNamespace")]
        pub metric_namespace: String,
        #[serde(rename="MetricTimestamp")]
        pub metric_timestamp: String,
        #[serde(rename="MetricUnit")]
        pub metric_unit: String,
        #[serde(rename="MetricValue")]
        pub metric_value: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DynamoDBAction {
        #[serde(rename="HashKeyField")]
        pub hash_key_field: String,
        #[serde(rename="HashKeyType")]
        pub hash_key_type: String,
        #[serde(rename="HashKeyValue")]
        pub hash_key_value: String,
        #[serde(rename="PayloadField")]
        pub payload_field: String,
        #[serde(rename="RangeKeyField")]
        pub range_key_field: String,
        #[serde(rename="RangeKeyType")]
        pub range_key_type: String,
        #[serde(rename="RangeKeyValue")]
        pub range_key_value: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="TableName")]
        pub table_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DynamoDBv2Action {
        #[serde(rename="PutItem")]
        pub put_item: PutItemInput,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticsearchAction {
        #[serde(rename="Endpoint")]
        pub endpoint: String,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Index")]
        pub index: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct FirehoseAction {
        #[serde(rename="DeliveryStreamName")]
        pub delivery_stream_name: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="Separator")]
        pub separator: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisAction {
        #[serde(rename="PartitionKey")]
        pub partition_key: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="StreamName")]
        pub stream_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LambdaAction {
        #[serde(rename="FunctionArn")]
        pub function_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PutItemInput {
        #[serde(rename="TableName")]
        pub table_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RepublishAction {
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="Topic")]
        pub topic: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct S3Action {
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SnsAction {
        #[serde(rename="MessageFormat")]
        pub message_format: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="TargetArn")]
        pub target_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SqsAction {
        #[serde(rename="QueueUrl")]
        pub queue_url: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="UseBase64")]
        pub use_base64: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TopicRulePayload {
        #[serde(rename="Actions")]
        pub actions: Vec<Action>,
        #[serde(rename="AwsIotSqlVersion")]
        pub aws_iot_sql_version: String,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="RuleDisabled")]
        pub rule_disabled: bool,
        #[serde(rename="Sql")]
        pub sql: String,
    }

}

