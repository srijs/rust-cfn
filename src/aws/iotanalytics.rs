//! Types for the `IoTAnalytics` service.

/// The [`AWS::IoTAnalytics::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html#cfn-iotanalytics-channel-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`ChannelStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html#cfn-iotanalytics-channel-channelstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_storage: Option<::Value<self::channel::ChannelStorage>>,
    /// Property [`RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html#cfn-iotanalytics-channel-retentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_period: Option<::Value<self::channel::RetentionPeriod>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html#cfn-iotanalytics-channel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
        if let Some(ref channel_storage) = self.channel_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelStorage", channel_storage)?;
        }
        if let Some(ref retention_period) = self.retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriod", retention_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_name: Option<::Value<String>> = None;
                let mut channel_storage: Option<::Value<self::channel::ChannelStorage>> = None;
                let mut retention_period: Option<::Value<self::channel::RetentionPeriod>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelStorage" => {
                            channel_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionPeriod" => {
                            retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    channel_name: channel_name,
                    channel_storage: channel_storage,
                    retention_period: retention_period,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::IoTAnalytics::Channel";
    fn properties(&self) -> &ChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Channel {}

impl From<ChannelProperties> for Channel {
    fn from(properties: ChannelProperties) -> Channel {
        Channel { properties }
    }
}

/// The [`AWS::IoTAnalytics::Dataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html) resource type.
#[derive(Debug, Default)]
pub struct Dataset {
    properties: DatasetProperties
}

/// Properties for the `Dataset` resource.
#[derive(Debug, Default)]
pub struct DatasetProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: ::ValueList<self::dataset::Action>,
    /// Property [`ContentDeliveryRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-contentdeliveryrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_delivery_rules: Option<::ValueList<self::dataset::DatasetContentDeliveryRule>>,
    /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-datasetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_name: Option<::Value<String>>,
    /// Property [`LateDataRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-latedatarules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub late_data_rules: Option<::ValueList<self::dataset::LateDataRule>>,
    /// Property [`RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-retentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_period: Option<::Value<self::dataset::RetentionPeriod>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Triggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-triggers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub triggers: Option<::ValueList<self::dataset::Trigger>>,
    /// Property [`VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html#cfn-iotanalytics-dataset-versioningconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub versioning_configuration: Option<::Value<self::dataset::VersioningConfiguration>>,
}

impl ::serde::Serialize for DatasetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        if let Some(ref content_delivery_rules) = self.content_delivery_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentDeliveryRules", content_delivery_rules)?;
        }
        if let Some(ref dataset_name) = self.dataset_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", dataset_name)?;
        }
        if let Some(ref late_data_rules) = self.late_data_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LateDataRules", late_data_rules)?;
        }
        if let Some(ref retention_period) = self.retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriod", retention_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref triggers) = self.triggers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Triggers", triggers)?;
        }
        if let Some(ref versioning_configuration) = self.versioning_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersioningConfiguration", versioning_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatasetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatasetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatasetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<self::dataset::Action>> = None;
                let mut content_delivery_rules: Option<::ValueList<self::dataset::DatasetContentDeliveryRule>> = None;
                let mut dataset_name: Option<::Value<String>> = None;
                let mut late_data_rules: Option<::ValueList<self::dataset::LateDataRule>> = None;
                let mut retention_period: Option<::Value<self::dataset::RetentionPeriod>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut triggers: Option<::ValueList<self::dataset::Trigger>> = None;
                let mut versioning_configuration: Option<::Value<self::dataset::VersioningConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContentDeliveryRules" => {
                            content_delivery_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetName" => {
                            dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LateDataRules" => {
                            late_data_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionPeriod" => {
                            retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Triggers" => {
                            triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersioningConfiguration" => {
                            versioning_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    content_delivery_rules: content_delivery_rules,
                    dataset_name: dataset_name,
                    late_data_rules: late_data_rules,
                    retention_period: retention_period,
                    tags: tags,
                    triggers: triggers,
                    versioning_configuration: versioning_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dataset {
    type Properties = DatasetProperties;
    const TYPE: &'static str = "AWS::IoTAnalytics::Dataset";
    fn properties(&self) -> &DatasetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatasetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dataset {}

impl From<DatasetProperties> for Dataset {
    fn from(properties: DatasetProperties) -> Dataset {
        Dataset { properties }
    }
}

/// The [`AWS::IoTAnalytics::Datastore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html) resource type.
#[derive(Debug, Default)]
pub struct Datastore {
    properties: DatastoreProperties
}

/// Properties for the `Datastore` resource.
#[derive(Debug, Default)]
pub struct DatastoreProperties {
    /// Property [`DatastoreName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-datastorename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub datastore_name: Option<::Value<String>>,
    /// Property [`DatastorePartitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-datastorepartitions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub datastore_partitions: Option<::Value<self::datastore::DatastorePartitions>>,
    /// Property [`DatastoreStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-datastorestorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub datastore_storage: Option<::Value<self::datastore::DatastoreStorage>>,
    /// Property [`FileFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-fileformatconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_format_configuration: Option<::Value<self::datastore::FileFormatConfiguration>>,
    /// Property [`RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-retentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_period: Option<::Value<self::datastore::RetentionPeriod>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html#cfn-iotanalytics-datastore-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DatastoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref datastore_name) = self.datastore_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreName", datastore_name)?;
        }
        if let Some(ref datastore_partitions) = self.datastore_partitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastorePartitions", datastore_partitions)?;
        }
        if let Some(ref datastore_storage) = self.datastore_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreStorage", datastore_storage)?;
        }
        if let Some(ref file_format_configuration) = self.file_format_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileFormatConfiguration", file_format_configuration)?;
        }
        if let Some(ref retention_period) = self.retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriod", retention_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatastoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatastoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatastoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatastoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut datastore_name: Option<::Value<String>> = None;
                let mut datastore_partitions: Option<::Value<self::datastore::DatastorePartitions>> = None;
                let mut datastore_storage: Option<::Value<self::datastore::DatastoreStorage>> = None;
                let mut file_format_configuration: Option<::Value<self::datastore::FileFormatConfiguration>> = None;
                let mut retention_period: Option<::Value<self::datastore::RetentionPeriod>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatastoreName" => {
                            datastore_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatastorePartitions" => {
                            datastore_partitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatastoreStorage" => {
                            datastore_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileFormatConfiguration" => {
                            file_format_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionPeriod" => {
                            retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatastoreProperties {
                    datastore_name: datastore_name,
                    datastore_partitions: datastore_partitions,
                    datastore_storage: datastore_storage,
                    file_format_configuration: file_format_configuration,
                    retention_period: retention_period,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Datastore {
    type Properties = DatastoreProperties;
    const TYPE: &'static str = "AWS::IoTAnalytics::Datastore";
    fn properties(&self) -> &DatastoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatastoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Datastore {}

impl From<DatastoreProperties> for Datastore {
    fn from(properties: DatastoreProperties) -> Datastore {
        Datastore { properties }
    }
}

/// The [`AWS::IoTAnalytics::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-pipeline.html) resource type.
#[derive(Debug, Default)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug, Default)]
pub struct PipelineProperties {
    /// Property [`PipelineActivities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-pipeline.html#cfn-iotanalytics-pipeline-pipelineactivities).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_activities: ::ValueList<self::pipeline::Activity>,
    /// Property [`PipelineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-pipeline.html#cfn-iotanalytics-pipeline-pipelinename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pipeline_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-pipeline.html#cfn-iotanalytics-pipeline-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineActivities", &self.pipeline_activities)?;
        if let Some(ref pipeline_name) = self.pipeline_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineName", pipeline_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut pipeline_activities: Option<::ValueList<self::pipeline::Activity>> = None;
                let mut pipeline_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PipelineActivities" => {
                            pipeline_activities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineName" => {
                            pipeline_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PipelineProperties {
                    pipeline_activities: pipeline_activities.ok_or(::serde::de::Error::missing_field("PipelineActivities"))?,
                    pipeline_name: pipeline_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::IoTAnalytics::Pipeline";
    fn properties(&self) -> &PipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::IoTAnalytics::Channel.ChannelStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-channelstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct ChannelStorage {
        /// Property [`CustomerManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-channelstorage.html#cfn-iotanalytics-channel-channelstorage-customermanageds3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customer_managed_s3: Option<::Value<CustomerManagedS3>>,
        /// Property [`ServiceManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-channelstorage.html#cfn-iotanalytics-channel-channelstorage-servicemanageds3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_managed_s3: Option<::Value<ServiceManagedS3>>,
    }

    impl ::codec::SerializeValue for ChannelStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customer_managed_s3) = self.customer_managed_s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedS3", customer_managed_s3)?;
            }
            if let Some(ref service_managed_s3) = self.service_managed_s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceManagedS3", service_managed_s3)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChannelStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChannelStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChannelStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customer_managed_s3: Option<::Value<CustomerManagedS3>> = None;
                    let mut service_managed_s3: Option<::Value<ServiceManagedS3>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomerManagedS3" => {
                                customer_managed_s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceManagedS3" => {
                                service_managed_s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChannelStorage {
                        customer_managed_s3: customer_managed_s3,
                        service_managed_s3: service_managed_s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Channel.CustomerManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-customermanageds3.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomerManagedS3 {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-customermanageds3.html#cfn-iotanalytics-channel-customermanageds3-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-customermanageds3.html#cfn-iotanalytics-channel-customermanageds3-keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_prefix: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-customermanageds3.html#cfn-iotanalytics-channel-customermanageds3-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomerManagedS3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key_prefix) = self.key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPrefix", key_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomerManagedS3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomerManagedS3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomerManagedS3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomerManagedS3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key_prefix: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyPrefix" => {
                                key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomerManagedS3 {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key_prefix: key_prefix,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Channel.RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-retentionperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct RetentionPeriod {
        /// Property [`NumberOfDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-retentionperiod.html#cfn-iotanalytics-channel-retentionperiod-numberofdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_days: Option<::Value<u32>>,
        /// Property [`Unlimited`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-retentionperiod.html#cfn-iotanalytics-channel-retentionperiod-unlimited).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unlimited: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for RetentionPeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref number_of_days) = self.number_of_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDays", number_of_days)?;
            }
            if let Some(ref unlimited) = self.unlimited {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unlimited", unlimited)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetentionPeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetentionPeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetentionPeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetentionPeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut number_of_days: Option<::Value<u32>> = None;
                    let mut unlimited: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumberOfDays" => {
                                number_of_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unlimited" => {
                                unlimited = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetentionPeriod {
                        number_of_days: number_of_days,
                        unlimited: unlimited,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Channel.ServiceManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-servicemanageds3.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceManagedS3 {
    }

    impl ::codec::SerializeValue for ServiceManagedS3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceManagedS3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceManagedS3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceManagedS3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceManagedS3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ServiceManagedS3 {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod dataset {
    //! Property types for the `Dataset` resource.

    /// The [`AWS::IoTAnalytics::Dataset.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`ActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-action.html#cfn-iotanalytics-dataset-action-actionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_name: ::Value<String>,
        /// Property [`ContainerAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-action.html#cfn-iotanalytics-dataset-action-containeraction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_action: Option<::Value<ContainerAction>>,
        /// Property [`QueryAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-action.html#cfn-iotanalytics-dataset-action-queryaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_action: Option<::Value<QueryAction>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionName", &self.action_name)?;
            if let Some(ref container_action) = self.container_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerAction", container_action)?;
            }
            if let Some(ref query_action) = self.query_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryAction", query_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_name: Option<::Value<String>> = None;
                    let mut container_action: Option<::Value<ContainerAction>> = None;
                    let mut query_action: Option<::Value<QueryAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionName" => {
                                action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerAction" => {
                                container_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryAction" => {
                                query_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        action_name: action_name.ok_or(::serde::de::Error::missing_field("ActionName"))?,
                        container_action: container_action,
                        query_action: query_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.ContainerAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerAction {
        /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html#cfn-iotanalytics-dataset-containeraction-executionrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role_arn: ::Value<String>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html#cfn-iotanalytics-dataset-containeraction-image).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image: ::Value<String>,
        /// Property [`ResourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html#cfn-iotanalytics-dataset-containeraction-resourceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_configuration: ::Value<ResourceConfiguration>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html#cfn-iotanalytics-dataset-containeraction-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: Option<::ValueList<Variable>>,
    }

    impl ::codec::SerializeValue for ContainerAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceConfiguration", &self.resource_configuration)?;
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role_arn: Option<::Value<String>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut resource_configuration: Option<::Value<ResourceConfiguration>> = None;
                    let mut variables: Option<::ValueList<Variable>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRoleArn" => {
                                execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceConfiguration" => {
                                resource_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerAction {
                        execution_role_arn: execution_role_arn.ok_or(::serde::de::Error::missing_field("ExecutionRoleArn"))?,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        resource_configuration: resource_configuration.ok_or(::serde::de::Error::missing_field("ResourceConfiguration"))?,
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.DatasetContentDeliveryRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryrule.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetContentDeliveryRule {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryrule.html#cfn-iotanalytics-dataset-datasetcontentdeliveryrule-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<DatasetContentDeliveryRuleDestination>,
        /// Property [`EntryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryrule.html#cfn-iotanalytics-dataset-datasetcontentdeliveryrule-entryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatasetContentDeliveryRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            if let Some(ref entry_name) = self.entry_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryName", entry_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatasetContentDeliveryRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetContentDeliveryRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatasetContentDeliveryRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatasetContentDeliveryRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<DatasetContentDeliveryRuleDestination>> = None;
                    let mut entry_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryName" => {
                                entry_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatasetContentDeliveryRule {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        entry_name: entry_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.DatasetContentDeliveryRuleDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryruledestination.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetContentDeliveryRuleDestination {
        /// Property [`IotEventsDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryruledestination.html#cfn-iotanalytics-dataset-datasetcontentdeliveryruledestination-ioteventsdestinationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_events_destination_configuration: Option<::Value<IotEventsDestinationConfiguration>>,
        /// Property [`S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryruledestination.html#cfn-iotanalytics-dataset-datasetcontentdeliveryruledestination-s3destinationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_destination_configuration: Option<::Value<S3DestinationConfiguration>>,
    }

    impl ::codec::SerializeValue for DatasetContentDeliveryRuleDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iot_events_destination_configuration) = self.iot_events_destination_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotEventsDestinationConfiguration", iot_events_destination_configuration)?;
            }
            if let Some(ref s3_destination_configuration) = self.s3_destination_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DestinationConfiguration", s3_destination_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatasetContentDeliveryRuleDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetContentDeliveryRuleDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatasetContentDeliveryRuleDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatasetContentDeliveryRuleDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iot_events_destination_configuration: Option<::Value<IotEventsDestinationConfiguration>> = None;
                    let mut s3_destination_configuration: Option<::Value<S3DestinationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IotEventsDestinationConfiguration" => {
                                iot_events_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DestinationConfiguration" => {
                                s3_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatasetContentDeliveryRuleDestination {
                        iot_events_destination_configuration: iot_events_destination_configuration,
                        s3_destination_configuration: s3_destination_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.DatasetContentVersionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable-datasetcontentversionvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetContentVersionValue {
        /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable-datasetcontentversionvalue.html#cfn-iotanalytics-dataset-variable-datasetcontentversionvalue-datasetname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatasetContentVersionValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dataset_name) = self.dataset_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", dataset_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatasetContentVersionValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetContentVersionValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatasetContentVersionValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatasetContentVersionValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dataset_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatasetName" => {
                                dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatasetContentVersionValue {
                        dataset_name: dataset_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.DeltaTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatime.html) property type.
    #[derive(Debug, Default)]
    pub struct DeltaTime {
        /// Property [`OffsetSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatime.html#cfn-iotanalytics-dataset-deltatime-offsetseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset_seconds: ::Value<u32>,
        /// Property [`TimeExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatime.html#cfn-iotanalytics-dataset-deltatime-timeexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for DeltaTime {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OffsetSeconds", &self.offset_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeExpression", &self.time_expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeltaTime {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeltaTime, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeltaTime;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeltaTime")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut offset_seconds: Option<::Value<u32>> = None;
                    let mut time_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OffsetSeconds" => {
                                offset_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeExpression" => {
                                time_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeltaTime {
                        offset_seconds: offset_seconds.ok_or(::serde::de::Error::missing_field("OffsetSeconds"))?,
                        time_expression: time_expression.ok_or(::serde::de::Error::missing_field("TimeExpression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.DeltaTimeSessionWindowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatimesessionwindowconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DeltaTimeSessionWindowConfiguration {
        /// Property [`TimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatimesessionwindowconfiguration.html#cfn-iotanalytics-dataset-deltatimesessionwindowconfiguration-timeoutinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for DeltaTimeSessionWindowConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMinutes", &self.timeout_in_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeltaTimeSessionWindowConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeltaTimeSessionWindowConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeltaTimeSessionWindowConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeltaTimeSessionWindowConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut timeout_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TimeoutInMinutes" => {
                                timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeltaTimeSessionWindowConfiguration {
                        timeout_in_minutes: timeout_in_minutes.ok_or(::serde::de::Error::missing_field("TimeoutInMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`DeltaTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-filter.html#cfn-iotanalytics-dataset-filter-deltatime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_time: Option<::Value<DeltaTime>>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delta_time) = self.delta_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaTime", delta_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delta_time: Option<::Value<DeltaTime>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeltaTime" => {
                                delta_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        delta_time: delta_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.GlueConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-glueconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueConfiguration {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-glueconfiguration.html#cfn-iotanalytics-dataset-glueconfiguration-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-glueconfiguration.html#cfn-iotanalytics-dataset-glueconfiguration-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GlueConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueConfiguration {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.IotEventsDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-ioteventsdestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IotEventsDestinationConfiguration {
        /// Property [`InputName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-ioteventsdestinationconfiguration.html#cfn-iotanalytics-dataset-ioteventsdestinationconfiguration-inputname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-ioteventsdestinationconfiguration.html#cfn-iotanalytics-dataset-ioteventsdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for IotEventsDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputName", &self.input_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IotEventsDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotEventsDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotEventsDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotEventsDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputName" => {
                                input_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotEventsDestinationConfiguration {
                        input_name: input_name.ok_or(::serde::de::Error::missing_field("InputName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.LateDataRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedatarule.html) property type.
    #[derive(Debug, Default)]
    pub struct LateDataRule {
        /// Property [`RuleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedatarule.html#cfn-iotanalytics-dataset-latedatarule-ruleconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_configuration: ::Value<LateDataRuleConfiguration>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedatarule.html#cfn-iotanalytics-dataset-latedatarule-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LateDataRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleConfiguration", &self.rule_configuration)?;
            if let Some(ref rule_name) = self.rule_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LateDataRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LateDataRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LateDataRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LateDataRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule_configuration: Option<::Value<LateDataRuleConfiguration>> = None;
                    let mut rule_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuleConfiguration" => {
                                rule_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LateDataRule {
                        rule_configuration: rule_configuration.ok_or(::serde::de::Error::missing_field("RuleConfiguration"))?,
                        rule_name: rule_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.LateDataRuleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedataruleconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LateDataRuleConfiguration {
        /// Property [`DeltaTimeSessionWindowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedataruleconfiguration.html#cfn-iotanalytics-dataset-latedataruleconfiguration-deltatimesessionwindowconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_time_session_window_configuration: Option<::Value<DeltaTimeSessionWindowConfiguration>>,
    }

    impl ::codec::SerializeValue for LateDataRuleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delta_time_session_window_configuration) = self.delta_time_session_window_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaTimeSessionWindowConfiguration", delta_time_session_window_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LateDataRuleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LateDataRuleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LateDataRuleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LateDataRuleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delta_time_session_window_configuration: Option<::Value<DeltaTimeSessionWindowConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeltaTimeSessionWindowConfiguration" => {
                                delta_time_session_window_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LateDataRuleConfiguration {
                        delta_time_session_window_configuration: delta_time_session_window_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.OutputFileUriValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable-outputfileurivalue.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputFileUriValue {
        /// Property [`FileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable-outputfileurivalue.html#cfn-iotanalytics-dataset-variable-outputfileurivalue-filename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutputFileUriValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file_name) = self.file_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileName", file_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputFileUriValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputFileUriValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputFileUriValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputFileUriValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FileName" => {
                                file_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputFileUriValue {
                        file_name: file_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.QueryAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-queryaction.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryAction {
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-queryaction.html#cfn-iotanalytics-dataset-queryaction-filters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filters: Option<::ValueList<Filter>>,
        /// Property [`SqlQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-queryaction.html#cfn-iotanalytics-dataset-queryaction-sqlquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_query: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueryAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filters) = self.filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", filters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlQuery", &self.sql_query)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filters: Option<::ValueList<Filter>> = None;
                    let mut sql_query: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filters" => {
                                filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlQuery" => {
                                sql_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryAction {
                        filters: filters,
                        sql_query: sql_query.ok_or(::serde::de::Error::missing_field("SqlQuery"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.ResourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-resourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceConfiguration {
        /// Property [`ComputeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-resourceconfiguration.html#cfn-iotanalytics-dataset-resourceconfiguration-computetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_type: ::Value<String>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-resourceconfiguration.html#cfn-iotanalytics-dataset-resourceconfiguration-volumesizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ResourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeType", &self.compute_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_type: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeType" => {
                                compute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceConfiguration {
                        compute_type: compute_type.ok_or(::serde::de::Error::missing_field("ComputeType"))?,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-retentionperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct RetentionPeriod {
        /// Property [`NumberOfDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-retentionperiod.html#cfn-iotanalytics-dataset-retentionperiod-numberofdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_days: ::Value<u32>,
        /// Property [`Unlimited`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-retentionperiod.html#cfn-iotanalytics-dataset-retentionperiod-unlimited).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unlimited: ::Value<bool>,
    }

    impl ::codec::SerializeValue for RetentionPeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDays", &self.number_of_days)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unlimited", &self.unlimited)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetentionPeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetentionPeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetentionPeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetentionPeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut number_of_days: Option<::Value<u32>> = None;
                    let mut unlimited: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumberOfDays" => {
                                number_of_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unlimited" => {
                                unlimited = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetentionPeriod {
                        number_of_days: number_of_days.ok_or(::serde::de::Error::missing_field("NumberOfDays"))?,
                        unlimited: unlimited.ok_or(::serde::de::Error::missing_field("Unlimited"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3DestinationConfiguration {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html#cfn-iotanalytics-dataset-s3destinationconfiguration-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`GlueConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html#cfn-iotanalytics-dataset-s3destinationconfiguration-glueconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_configuration: Option<::Value<GlueConfiguration>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html#cfn-iotanalytics-dataset-s3destinationconfiguration-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html#cfn-iotanalytics-dataset-s3destinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3DestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref glue_configuration) = self.glue_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueConfiguration", glue_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3DestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3DestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3DestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3DestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut glue_configuration: Option<::Value<GlueConfiguration>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GlueConfiguration" => {
                                glue_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3DestinationConfiguration {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        glue_configuration: glue_configuration,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger-schedule.html#cfn-iotanalytics-dataset-trigger-schedule-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger.html) property type.
    #[derive(Debug, Default)]
    pub struct Trigger {
        /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger.html#cfn-iotanalytics-dataset-trigger-schedule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule: Option<::Value<Schedule>>,
        /// Property [`TriggeringDataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger.html#cfn-iotanalytics-dataset-trigger-triggeringdataset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub triggering_dataset: Option<::Value<TriggeringDataset>>,
    }

    impl ::codec::SerializeValue for Trigger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schedule) = self.schedule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
            }
            if let Some(ref triggering_dataset) = self.triggering_dataset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggeringDataset", triggering_dataset)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Trigger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Trigger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Trigger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Trigger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule: Option<::Value<Schedule>> = None;
                    let mut triggering_dataset: Option<::Value<TriggeringDataset>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Schedule" => {
                                schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggeringDataset" => {
                                triggering_dataset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Trigger {
                        schedule: schedule,
                        triggering_dataset: triggering_dataset,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.TriggeringDataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-triggeringdataset.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggeringDataset {
        /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-triggeringdataset.html#cfn-iotanalytics-dataset-triggeringdataset-datasetname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for TriggeringDataset {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", &self.dataset_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggeringDataset {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggeringDataset, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggeringDataset;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggeringDataset")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dataset_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatasetName" => {
                                dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggeringDataset {
                        dataset_name: dataset_name.ok_or(::serde::de::Error::missing_field("DatasetName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.Variable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html) property type.
    #[derive(Debug, Default)]
    pub struct Variable {
        /// Property [`DatasetContentVersionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html#cfn-iotanalytics-dataset-variable-datasetcontentversionvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_content_version_value: Option<::Value<DatasetContentVersionValue>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html#cfn-iotanalytics-dataset-variable-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<f64>>,
        /// Property [`OutputFileUriValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html#cfn-iotanalytics-dataset-variable-outputfileurivalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_file_uri_value: Option<::Value<OutputFileUriValue>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html#cfn-iotanalytics-dataset-variable-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
        /// Property [`VariableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html#cfn-iotanalytics-dataset-variable-variablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Variable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dataset_content_version_value) = self.dataset_content_version_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetContentVersionValue", dataset_content_version_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref output_file_uri_value) = self.output_file_uri_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFileUriValue", output_file_uri_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableName", &self.variable_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Variable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Variable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Variable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Variable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dataset_content_version_value: Option<::Value<DatasetContentVersionValue>> = None;
                    let mut double_value: Option<::Value<f64>> = None;
                    let mut output_file_uri_value: Option<::Value<OutputFileUriValue>> = None;
                    let mut string_value: Option<::Value<String>> = None;
                    let mut variable_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatasetContentVersionValue" => {
                                dataset_content_version_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputFileUriValue" => {
                                output_file_uri_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariableName" => {
                                variable_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Variable {
                        dataset_content_version_value: dataset_content_version_value,
                        double_value: double_value,
                        output_file_uri_value: output_file_uri_value,
                        string_value: string_value,
                        variable_name: variable_name.ok_or(::serde::de::Error::missing_field("VariableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Dataset.VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-versioningconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VersioningConfiguration {
        /// Property [`MaxVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-versioningconfiguration.html#cfn-iotanalytics-dataset-versioningconfiguration-maxversions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_versions: Option<::Value<u32>>,
        /// Property [`Unlimited`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-versioningconfiguration.html#cfn-iotanalytics-dataset-versioningconfiguration-unlimited).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unlimited: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for VersioningConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_versions) = self.max_versions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxVersions", max_versions)?;
            }
            if let Some(ref unlimited) = self.unlimited {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unlimited", unlimited)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VersioningConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VersioningConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VersioningConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VersioningConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_versions: Option<::Value<u32>> = None;
                    let mut unlimited: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxVersions" => {
                                max_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unlimited" => {
                                unlimited = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VersioningConfiguration {
                        max_versions: max_versions,
                        unlimited: unlimited,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod datastore {
    //! Property types for the `Datastore` resource.

    /// The [`AWS::IoTAnalytics::Datastore.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-column.html) property type.
    #[derive(Debug, Default)]
    pub struct Column {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-column.html#cfn-iotanalytics-datastore-column-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-column.html#cfn-iotanalytics-datastore-column-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Column {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Column, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Column;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Column")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Column {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.CustomerManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomerManagedS3 {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3.html#cfn-iotanalytics-datastore-customermanageds3-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3.html#cfn-iotanalytics-datastore-customermanageds3-keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_prefix: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3.html#cfn-iotanalytics-datastore-customermanageds3-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomerManagedS3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key_prefix) = self.key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPrefix", key_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomerManagedS3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomerManagedS3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomerManagedS3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomerManagedS3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key_prefix: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyPrefix" => {
                                key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomerManagedS3 {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key_prefix: key_prefix,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.DatastorePartition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartition.html) property type.
    #[derive(Debug, Default)]
    pub struct DatastorePartition {
        /// Property [`Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartition.html#cfn-iotanalytics-datastore-datastorepartition-partition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition: Option<::Value<Partition>>,
        /// Property [`TimestampPartition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartition.html#cfn-iotanalytics-datastore-datastorepartition-timestamppartition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_partition: Option<::Value<TimestampPartition>>,
    }

    impl ::codec::SerializeValue for DatastorePartition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref partition) = self.partition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Partition", partition)?;
            }
            if let Some(ref timestamp_partition) = self.timestamp_partition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampPartition", timestamp_partition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatastorePartition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatastorePartition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatastorePartition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatastorePartition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition: Option<::Value<Partition>> = None;
                    let mut timestamp_partition: Option<::Value<TimestampPartition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Partition" => {
                                partition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampPartition" => {
                                timestamp_partition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatastorePartition {
                        partition: partition,
                        timestamp_partition: timestamp_partition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.DatastorePartitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartitions.html) property type.
    #[derive(Debug, Default)]
    pub struct DatastorePartitions {
        /// Property [`Partitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartitions.html#cfn-iotanalytics-datastore-datastorepartitions-partitions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partitions: Option<::ValueList<DatastorePartition>>,
    }

    impl ::codec::SerializeValue for DatastorePartitions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref partitions) = self.partitions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Partitions", partitions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatastorePartitions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatastorePartitions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatastorePartitions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatastorePartitions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partitions: Option<::ValueList<DatastorePartition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Partitions" => {
                                partitions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatastorePartitions {
                        partitions: partitions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.DatastoreStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorestorage.html) property type.
    #[derive(Debug, Default)]
    pub struct DatastoreStorage {
        /// Property [`CustomerManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorestorage.html#cfn-iotanalytics-datastore-datastorestorage-customermanageds3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customer_managed_s3: Option<::Value<CustomerManagedS3>>,
        /// Property [`ServiceManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorestorage.html#cfn-iotanalytics-datastore-datastorestorage-servicemanageds3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_managed_s3: Option<::Value<ServiceManagedS3>>,
    }

    impl ::codec::SerializeValue for DatastoreStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customer_managed_s3) = self.customer_managed_s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedS3", customer_managed_s3)?;
            }
            if let Some(ref service_managed_s3) = self.service_managed_s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceManagedS3", service_managed_s3)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatastoreStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatastoreStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatastoreStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatastoreStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customer_managed_s3: Option<::Value<CustomerManagedS3>> = None;
                    let mut service_managed_s3: Option<::Value<ServiceManagedS3>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomerManagedS3" => {
                                customer_managed_s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceManagedS3" => {
                                service_managed_s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatastoreStorage {
                        customer_managed_s3: customer_managed_s3,
                        service_managed_s3: service_managed_s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.FileFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-fileformatconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FileFormatConfiguration {
        /// Property [`JsonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-fileformatconfiguration.html#cfn-iotanalytics-datastore-fileformatconfiguration-jsonconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_configuration: Option<::Value<JsonConfiguration>>,
        /// Property [`ParquetConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-fileformatconfiguration.html#cfn-iotanalytics-datastore-fileformatconfiguration-parquetconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parquet_configuration: Option<::Value<ParquetConfiguration>>,
    }

    impl ::codec::SerializeValue for FileFormatConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref json_configuration) = self.json_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonConfiguration", json_configuration)?;
            }
            if let Some(ref parquet_configuration) = self.parquet_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParquetConfiguration", parquet_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileFormatConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileFormatConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileFormatConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileFormatConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut json_configuration: Option<::Value<JsonConfiguration>> = None;
                    let mut parquet_configuration: Option<::Value<ParquetConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JsonConfiguration" => {
                                json_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParquetConfiguration" => {
                                parquet_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileFormatConfiguration {
                        json_configuration: json_configuration,
                        parquet_configuration: parquet_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.JsonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-jsonconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonConfiguration {
    }

    impl ::codec::SerializeValue for JsonConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(JsonConfiguration {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.ParquetConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-parquetconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ParquetConfiguration {
        /// Property [`SchemaDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-parquetconfiguration.html#cfn-iotanalytics-datastore-parquetconfiguration-schemadefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_definition: Option<::Value<SchemaDefinition>>,
    }

    impl ::codec::SerializeValue for ParquetConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schema_definition) = self.schema_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaDefinition", schema_definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParquetConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParquetConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParquetConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParquetConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schema_definition: Option<::Value<SchemaDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SchemaDefinition" => {
                                schema_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParquetConfiguration {
                        schema_definition: schema_definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-partition.html) property type.
    #[derive(Debug, Default)]
    pub struct Partition {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-partition.html#cfn-iotanalytics-datastore-partition-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Partition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Partition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Partition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Partition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Partition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Partition {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-retentionperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct RetentionPeriod {
        /// Property [`NumberOfDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-retentionperiod.html#cfn-iotanalytics-datastore-retentionperiod-numberofdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_days: Option<::Value<u32>>,
        /// Property [`Unlimited`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-retentionperiod.html#cfn-iotanalytics-datastore-retentionperiod-unlimited).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unlimited: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for RetentionPeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref number_of_days) = self.number_of_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDays", number_of_days)?;
            }
            if let Some(ref unlimited) = self.unlimited {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unlimited", unlimited)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetentionPeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetentionPeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetentionPeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetentionPeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut number_of_days: Option<::Value<u32>> = None;
                    let mut unlimited: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumberOfDays" => {
                                number_of_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unlimited" => {
                                unlimited = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetentionPeriod {
                        number_of_days: number_of_days,
                        unlimited: unlimited,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.SchemaDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-schemadefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaDefinition {
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-schemadefinition.html#cfn-iotanalytics-datastore-schemadefinition-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: Option<::ValueList<Column>>,
    }

    impl ::codec::SerializeValue for SchemaDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref columns) = self.columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", columns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut columns: Option<::ValueList<Column>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaDefinition {
                        columns: columns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.ServiceManagedS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-servicemanageds3.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceManagedS3 {
    }

    impl ::codec::SerializeValue for ServiceManagedS3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceManagedS3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceManagedS3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceManagedS3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceManagedS3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ServiceManagedS3 {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Datastore.TimestampPartition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-timestamppartition.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestampPartition {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-timestamppartition.html#cfn-iotanalytics-datastore-timestamppartition-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`TimestampFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-timestamppartition.html#cfn-iotanalytics-datastore-timestamppartition-timestampformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TimestampPartition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            if let Some(ref timestamp_format) = self.timestamp_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampFormat", timestamp_format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestampPartition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestampPartition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestampPartition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestampPartition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut timestamp_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampFormat" => {
                                timestamp_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestampPartition {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        timestamp_format: timestamp_format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod pipeline {
    //! Property types for the `Pipeline` resource.

    /// The [`AWS::IoTAnalytics::Pipeline.Activity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html) property type.
    #[derive(Debug, Default)]
    pub struct Activity {
        /// Property [`AddAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-addattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_attributes: Option<::Value<AddAttributes>>,
        /// Property [`Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-channel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel: Option<::Value<Channel>>,
        /// Property [`Datastore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-datastore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datastore: Option<::Value<Datastore>>,
        /// Property [`DeviceRegistryEnrich`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-deviceregistryenrich).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_registry_enrich: Option<::Value<DeviceRegistryEnrich>>,
        /// Property [`DeviceShadowEnrich`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-deviceshadowenrich).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_shadow_enrich: Option<::Value<DeviceShadowEnrich>>,
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<Filter>>,
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<::Value<Lambda>>,
        /// Property [`Math`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-math).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub math: Option<::Value<Math>>,
        /// Property [`RemoveAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-removeattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_attributes: Option<::Value<RemoveAttributes>>,
        /// Property [`SelectAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html#cfn-iotanalytics-pipeline-activity-selectattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub select_attributes: Option<::Value<SelectAttributes>>,
    }

    impl ::codec::SerializeValue for Activity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_attributes) = self.add_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddAttributes", add_attributes)?;
            }
            if let Some(ref channel) = self.channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Channel", channel)?;
            }
            if let Some(ref datastore) = self.datastore {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datastore", datastore)?;
            }
            if let Some(ref device_registry_enrich) = self.device_registry_enrich {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceRegistryEnrich", device_registry_enrich)?;
            }
            if let Some(ref device_shadow_enrich) = self.device_shadow_enrich {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceShadowEnrich", device_shadow_enrich)?;
            }
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            if let Some(ref math) = self.math {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Math", math)?;
            }
            if let Some(ref remove_attributes) = self.remove_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveAttributes", remove_attributes)?;
            }
            if let Some(ref select_attributes) = self.select_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectAttributes", select_attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Activity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Activity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Activity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Activity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_attributes: Option<::Value<AddAttributes>> = None;
                    let mut channel: Option<::Value<Channel>> = None;
                    let mut datastore: Option<::Value<Datastore>> = None;
                    let mut device_registry_enrich: Option<::Value<DeviceRegistryEnrich>> = None;
                    let mut device_shadow_enrich: Option<::Value<DeviceShadowEnrich>> = None;
                    let mut filter: Option<::Value<Filter>> = None;
                    let mut lambda: Option<::Value<Lambda>> = None;
                    let mut math: Option<::Value<Math>> = None;
                    let mut remove_attributes: Option<::Value<RemoveAttributes>> = None;
                    let mut select_attributes: Option<::Value<SelectAttributes>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddAttributes" => {
                                add_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Channel" => {
                                channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datastore" => {
                                datastore = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceRegistryEnrich" => {
                                device_registry_enrich = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceShadowEnrich" => {
                                device_shadow_enrich = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Math" => {
                                math = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveAttributes" => {
                                remove_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectAttributes" => {
                                select_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Activity {
                        add_attributes: add_attributes,
                        channel: channel,
                        datastore: datastore,
                        device_registry_enrich: device_registry_enrich,
                        device_shadow_enrich: device_shadow_enrich,
                        filter: filter,
                        lambda: lambda,
                        math: math,
                        remove_attributes: remove_attributes,
                        select_attributes: select_attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.AddAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-addattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct AddAttributes {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-addattributes.html#cfn-iotanalytics-pipeline-addattributes-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::Value<::json::Value>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-addattributes.html#cfn-iotanalytics-pipeline-addattributes-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-addattributes.html#cfn-iotanalytics-pipeline-addattributes-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AddAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::Value<::json::Value>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AddAttributes {
                        attributes: attributes,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-channel.html) property type.
    #[derive(Debug, Default)]
    pub struct Channel {
        /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-channel.html#cfn-iotanalytics-pipeline-channel-channelname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-channel.html#cfn-iotanalytics-pipeline-channel-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-channel.html#cfn-iotanalytics-pipeline-channel-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Channel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel_name) = self.channel_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Channel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Channel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Channel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Channel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChannelName" => {
                                channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Channel {
                        channel_name: channel_name,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.Datastore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-datastore.html) property type.
    #[derive(Debug, Default)]
    pub struct Datastore {
        /// Property [`DatastoreName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-datastore.html#cfn-iotanalytics-pipeline-datastore-datastorename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datastore_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-datastore.html#cfn-iotanalytics-pipeline-datastore-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Datastore {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref datastore_name) = self.datastore_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreName", datastore_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Datastore {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Datastore, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Datastore;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Datastore")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut datastore_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatastoreName" => {
                                datastore_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Datastore {
                        datastore_name: datastore_name,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.DeviceRegistryEnrich`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceRegistryEnrich {
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html#cfn-iotanalytics-pipeline-deviceregistryenrich-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html#cfn-iotanalytics-pipeline-deviceregistryenrich-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html#cfn-iotanalytics-pipeline-deviceregistryenrich-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html#cfn-iotanalytics-pipeline-deviceregistryenrich-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html#cfn-iotanalytics-pipeline-deviceregistryenrich-thingname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thing_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeviceRegistryEnrich {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute) = self.attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", attribute)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref thing_name) = self.thing_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", thing_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceRegistryEnrich {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceRegistryEnrich, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceRegistryEnrich;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceRegistryEnrich")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut thing_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingName" => {
                                thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceRegistryEnrich {
                        attribute: attribute,
                        name: name,
                        next: next,
                        role_arn: role_arn,
                        thing_name: thing_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.DeviceShadowEnrich`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceShadowEnrich {
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html#cfn-iotanalytics-pipeline-deviceshadowenrich-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html#cfn-iotanalytics-pipeline-deviceshadowenrich-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html#cfn-iotanalytics-pipeline-deviceshadowenrich-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html#cfn-iotanalytics-pipeline-deviceshadowenrich-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html#cfn-iotanalytics-pipeline-deviceshadowenrich-thingname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thing_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeviceShadowEnrich {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute) = self.attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", attribute)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref thing_name) = self.thing_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", thing_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceShadowEnrich {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceShadowEnrich, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceShadowEnrich;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceShadowEnrich")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut thing_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingName" => {
                                thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceShadowEnrich {
                        attribute: attribute,
                        name: name,
                        next: next,
                        role_arn: role_arn,
                        thing_name: thing_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-filter.html#cfn-iotanalytics-pipeline-filter-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-filter.html#cfn-iotanalytics-pipeline-filter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-filter.html#cfn-iotanalytics-pipeline-filter-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filter: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        filter: filter,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html) property type.
    #[derive(Debug, Default)]
    pub struct Lambda {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html#cfn-iotanalytics-pipeline-lambda-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`LambdaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html#cfn-iotanalytics-pipeline-lambda-lambdaname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html#cfn-iotanalytics-pipeline-lambda-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html#cfn-iotanalytics-pipeline-lambda-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Lambda {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref lambda_name) = self.lambda_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaName", lambda_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Lambda {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Lambda, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Lambda;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Lambda")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut lambda_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaName" => {
                                lambda_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Lambda {
                        batch_size: batch_size,
                        lambda_name: lambda_name,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.Math`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html) property type.
    #[derive(Debug, Default)]
    pub struct Math {
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html#cfn-iotanalytics-pipeline-math-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: Option<::Value<String>>,
        /// Property [`Math`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html#cfn-iotanalytics-pipeline-math-math).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub math: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html#cfn-iotanalytics-pipeline-math-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html#cfn-iotanalytics-pipeline-math-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Math {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute) = self.attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", attribute)?;
            }
            if let Some(ref math) = self.math {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Math", math)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Math {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Math, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Math;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Math")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute: Option<::Value<String>> = None;
                    let mut math: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Math" => {
                                math = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Math {
                        attribute: attribute,
                        math: math,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.RemoveAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-removeattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct RemoveAttributes {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-removeattributes.html#cfn-iotanalytics-pipeline-removeattributes-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-removeattributes.html#cfn-iotanalytics-pipeline-removeattributes-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-removeattributes.html#cfn-iotanalytics-pipeline-removeattributes-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RemoveAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RemoveAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RemoveAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RemoveAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RemoveAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RemoveAttributes {
                        attributes: attributes,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTAnalytics::Pipeline.SelectAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-selectattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct SelectAttributes {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-selectattributes.html#cfn-iotanalytics-pipeline-selectattributes-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-selectattributes.html#cfn-iotanalytics-pipeline-selectattributes-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Next`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-selectattributes.html#cfn-iotanalytics-pipeline-selectattributes-next).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SelectAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref next) = self.next {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Next", next)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelectAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelectAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelectAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelectAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Next" => {
                                next = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelectAttributes {
                        attributes: attributes,
                        name: name,
                        next: next,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
