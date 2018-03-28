/// The [`AWS::Kinesis::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Serialize, Deserialize)]
pub struct StreamProperties {
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RetentionPeriodHours")]
    pub retention_period_hours: (),
    #[serde(rename="ShardCount")]
    pub shard_count: (),
    #[serde(rename="StreamEncryption")]
    pub stream_encryption: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

