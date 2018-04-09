//! Types for the `SQS` service.

/// The [`AWS::SQS::Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html) resource type.
#[derive(Debug)]
pub struct Queue {
    properties: QueueProperties
}

/// Properties for the `Queue` resource.
#[derive(Debug)]
pub struct QueueProperties {
    /// Property `ContentBasedDeduplication`.
    pub content_based_deduplication: Option<::Value<bool>>,
    /// Property `DelaySeconds`.
    pub delay_seconds: Option<::Value<u32>>,
    /// Property `FifoQueue`.
    pub fifo_queue: Option<::Value<bool>>,
    /// Property `KmsDataKeyReusePeriodSeconds`.
    pub kms_data_key_reuse_period_seconds: Option<::Value<u32>>,
    /// Property `KmsMasterKeyId`.
    pub kms_master_key_id: Option<::Value<String>>,
    /// Property `MaximumMessageSize`.
    pub maximum_message_size: Option<::Value<u32>>,
    /// Property `MessageRetentionPeriod`.
    pub message_retention_period: Option<::Value<u32>>,
    /// Property `QueueName`.
    pub queue_name: Option<::Value<String>>,
    /// Property `ReceiveMessageWaitTimeSeconds`.
    pub receive_message_wait_time_seconds: Option<::Value<u32>>,
    /// Property `RedrivePolicy`.
    pub redrive_policy: Option<::Value<::json::Value>>,
    /// Property `VisibilityTimeout`.
    pub visibility_timeout: Option<::Value<u32>>,
}

impl ::serde::Serialize for QueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentBasedDeduplication", &self.content_based_deduplication)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DelaySeconds", &self.delay_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FifoQueue", &self.fifo_queue)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsDataKeyReusePeriodSeconds", &self.kms_data_key_reuse_period_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsMasterKeyId", &self.kms_master_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumMessageSize", &self.maximum_message_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageRetentionPeriod", &self.message_retention_period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueName", &self.queue_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReceiveMessageWaitTimeSeconds", &self.receive_message_wait_time_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedrivePolicy", &self.redrive_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityTimeout", &self.visibility_timeout)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content_based_deduplication = None;
                let mut delay_seconds = None;
                let mut fifo_queue = None;
                let mut kms_data_key_reuse_period_seconds = None;
                let mut kms_master_key_id = None;
                let mut maximum_message_size = None;
                let mut message_retention_period = None;
                let mut queue_name = None;
                let mut receive_message_wait_time_seconds = None;
                let mut redrive_policy = None;
                let mut visibility_timeout = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContentBasedDeduplication" => {
                            content_based_deduplication = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DelaySeconds" => {
                            delay_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FifoQueue" => {
                            fifo_queue = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsDataKeyReusePeriodSeconds" => {
                            kms_data_key_reuse_period_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsMasterKeyId" => {
                            kms_master_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaximumMessageSize" => {
                            maximum_message_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MessageRetentionPeriod" => {
                            message_retention_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "QueueName" => {
                            queue_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReceiveMessageWaitTimeSeconds" => {
                            receive_message_wait_time_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RedrivePolicy" => {
                            redrive_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VisibilityTimeout" => {
                            visibility_timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(QueueProperties {
                    content_based_deduplication: content_based_deduplication,
                    delay_seconds: delay_seconds,
                    fifo_queue: fifo_queue,
                    kms_data_key_reuse_period_seconds: kms_data_key_reuse_period_seconds,
                    kms_master_key_id: kms_master_key_id,
                    maximum_message_size: maximum_message_size,
                    message_retention_period: message_retention_period,
                    queue_name: queue_name,
                    receive_message_wait_time_seconds: receive_message_wait_time_seconds,
                    redrive_policy: redrive_policy,
                    visibility_timeout: visibility_timeout,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct QueuePolicyProperties {
    /// Property `PolicyDocument`.
    pub policy_document: ::Value<::json::Value>,
    /// Property `Queues`.
    pub queues: ::ValueList<String>,
}

impl ::serde::Serialize for QueuePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queues", &self.queues)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueuePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueuePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueuePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueuePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_document = None;
                let mut queues = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Queues" => {
                            queues = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(QueuePolicyProperties {
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    queues: queues.ok_or(::serde::de::Error::missing_field("Queues"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
