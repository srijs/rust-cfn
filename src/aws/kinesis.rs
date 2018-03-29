/// The [`AWS::Kinesis::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Serialize, Deserialize)]
pub struct StreamProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RetentionPeriodHours")]
    pub retention_period_hours: u32,
    #[serde(rename="ShardCount")]
    pub shard_count: u32,
    #[serde(rename="StreamEncryption")]
    pub stream_encryption: self::stream::StreamEncryption,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<StreamProperties> for Stream {
    fn from(properties: StreamProperties) -> Stream {
        Stream { properties }
    }
}

pub mod stream {
    #[derive(Serialize, Deserialize)]
    pub struct StreamEncryption {
        #[serde(rename="EncryptionType")]
        pub encryption_type: String,
        #[serde(rename="KeyId")]
        pub key_id: String,
    }

}

