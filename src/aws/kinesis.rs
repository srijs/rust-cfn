//! Types for the `Kinesis` service.

/// The [`AWS::Kinesis::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html) resource type.
#[derive(Debug)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `RetentionPeriodHours`.
    #[serde(rename="RetentionPeriodHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_hours: Option<u32>,
    /// Property `ShardCount`.
    #[serde(rename="ShardCount")]
    pub shard_count: u32,
    /// Property `StreamEncryption`.
    #[serde(rename="StreamEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_encryption: Option<self::stream::StreamEncryption>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
}

impl<'a> ::Resource<'a> for Stream {
    type Properties = StreamProperties;
    const TYPE: &'static str = "AWS::Kinesis::Stream";
    fn properties(&self) -> &StreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stream {}

impl From<StreamProperties> for Stream {
    fn from(properties: StreamProperties) -> Stream {
        Stream { properties }
    }
}

pub mod stream {
    //! Property types for the `Stream` resource.

    /// The [`AWS::Kinesis::Stream.StreamEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streamencryption.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamEncryption {
        /// Property `EncryptionType`.
        #[serde(rename="EncryptionType")]
        pub encryption_type: String,
        /// Property `KeyId`.
        #[serde(rename="KeyId")]
        pub key_id: String,
    }
}
