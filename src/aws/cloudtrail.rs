//! Types for the `CloudTrail` service.

/// The [`AWS::CloudTrail::Trail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html) resource type.
#[derive(Debug)]
pub struct Trail {
    properties: TrailProperties
}

/// Properties for the `Trail` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrailProperties {
    /// Property `CloudWatchLogsLogGroupArn`.
    #[serde(rename="CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: String,
    /// Property `CloudWatchLogsRoleArn`.
    #[serde(rename="CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: String,
    /// Property `EnableLogFileValidation`.
    #[serde(rename="EnableLogFileValidation")]
    pub enable_log_file_validation: bool,
    /// Property `EventSelectors`.
    #[serde(rename="EventSelectors")]
    pub event_selectors: Vec<self::trail::EventSelector>,
    /// Property `IncludeGlobalServiceEvents`.
    #[serde(rename="IncludeGlobalServiceEvents")]
    pub include_global_service_events: bool,
    /// Property `IsLogging`.
    #[serde(rename="IsLogging")]
    pub is_logging: bool,
    /// Property `IsMultiRegionTrail`.
    #[serde(rename="IsMultiRegionTrail")]
    pub is_multi_region_trail: bool,
    /// Property `KMSKeyId`.
    #[serde(rename="KMSKeyId")]
    pub kms_key_id: String,
    /// Property `S3BucketName`.
    #[serde(rename="S3BucketName")]
    pub s3_bucket_name: String,
    /// Property `S3KeyPrefix`.
    #[serde(rename="S3KeyPrefix")]
    pub s3_key_prefix: String,
    /// Property `SnsTopicName`.
    #[serde(rename="SnsTopicName")]
    pub sns_topic_name: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `TrailName`.
    #[serde(rename="TrailName")]
    pub trail_name: String,
}

impl<'a> ::Resource<'a> for Trail {
    type Properties = TrailProperties;
    const TYPE: &'static str = "AWS::CloudTrail::Trail";
    fn properties(&self) -> &TrailProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrailProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Trail {}

impl From<TrailProperties> for Trail {
    fn from(properties: TrailProperties) -> Trail {
        Trail { properties }
    }
}

pub mod trail {
    //! Property types for the `Trail` resource.

    /// The [`AWS::CloudTrail::Trail.DataResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataResource {
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::CloudTrail::Trail.EventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EventSelector {
        /// Property `DataResources`.
        #[serde(rename="DataResources")]
        pub data_resources: Vec<DataResource>,
        /// Property `IncludeManagementEvents`.
        #[serde(rename="IncludeManagementEvents")]
        pub include_management_events: bool,
        /// Property `ReadWriteType`.
        #[serde(rename="ReadWriteType")]
        pub read_write_type: String,
    }
}
