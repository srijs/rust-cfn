/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryStreamProperties {
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: (),
    #[serde(rename="DeliveryStreamType")]
    pub delivery_stream_type: (),
    #[serde(rename="ElasticsearchDestinationConfiguration")]
    pub elasticsearch_destination_configuration: (),
    #[serde(rename="ExtendedS3DestinationConfiguration")]
    pub extended_s3_destination_configuration: (),
    #[serde(rename="KinesisStreamSourceConfiguration")]
    pub kinesis_stream_source_configuration: (),
    #[serde(rename="RedshiftDestinationConfiguration")]
    pub redshift_destination_configuration: (),
    #[serde(rename="S3DestinationConfiguration")]
    pub s3_destination_configuration: (),
}

impl<'a> ::Resource<'a> for DeliveryStream {
    type Properties = DeliveryStreamProperties;
    const TYPE: &'static str = "AWS::KinesisFirehose::DeliveryStream";
    fn properties(&self) -> &DeliveryStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeliveryStreamProperties {
        &mut self.properties
    }
}

impl From<DeliveryStreamProperties> for DeliveryStream {
    fn from(properties: DeliveryStreamProperties) -> DeliveryStream {
        DeliveryStream { properties }
    }
}

