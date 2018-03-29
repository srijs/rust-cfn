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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// Property `CloudWatchLogsRoleArn`.
    #[serde(rename="CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// Property `EnableLogFileValidation`.
    #[serde(rename="EnableLogFileValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,
    /// Property `EventSelectors`.
    #[serde(rename="EventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<self::trail::EventSelector>>,
    /// Property `IncludeGlobalServiceEvents`.
    #[serde(rename="IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// Property `IsLogging`.
    #[serde(rename="IsLogging")]
    pub is_logging: bool,
    /// Property `IsMultiRegionTrail`.
    #[serde(rename="IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// Property `KMSKeyId`.
    #[serde(rename="KMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Property `S3BucketName`.
    #[serde(rename="S3BucketName")]
    pub s3_bucket_name: String,
    /// Property `S3KeyPrefix`.
    #[serde(rename="S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// Property `SnsTopicName`.
    #[serde(rename="SnsTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `TrailName`.
    #[serde(rename="TrailName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Vec<String>>,
    }

    /// The [`AWS::CloudTrail::Trail.EventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EventSelector {
        /// Property `DataResources`.
        #[serde(rename="DataResources")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data_resources: Option<Vec<DataResource>>,
        /// Property `IncludeManagementEvents`.
        #[serde(rename="IncludeManagementEvents")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include_management_events: Option<bool>,
        /// Property `ReadWriteType`.
        #[serde(rename="ReadWriteType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub read_write_type: Option<String>,
    }
}
