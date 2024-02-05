//! Types for the `Rekognition` service.

/// The [`AWS::Rekognition::Collection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html) resource type.
#[derive(Debug, Default)]
pub struct Collection {
    properties: CollectionProperties
}

/// Properties for the `Collection` resource.
#[derive(Debug, Default)]
pub struct CollectionProperties {
    /// Property [`CollectionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html#cfn-rekognition-collection-collectionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub collection_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html#cfn-rekognition-collection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CollectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionId", &self.collection_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CollectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CollectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CollectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CollectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collection_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CollectionId" => {
                            collection_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CollectionProperties {
                    collection_id: collection_id.ok_or(::serde::de::Error::missing_field("CollectionId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Collection {
    type Properties = CollectionProperties;
    const TYPE: &'static str = "AWS::Rekognition::Collection";
    fn properties(&self) -> &CollectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CollectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Collection {}

impl From<CollectionProperties> for Collection {
    fn from(properties: CollectionProperties) -> Collection {
        Collection { properties }
    }
}

/// The [`AWS::Rekognition::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-project.html#cfn-rekognition-project-projectname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_name: ::Value<String>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", &self.project_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut project_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    project_name: project_name.ok_or(::serde::de::Error::missing_field("ProjectName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::Rekognition::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

/// The [`AWS::Rekognition::StreamProcessor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html) resource type.
#[derive(Debug, Default)]
pub struct StreamProcessor {
    properties: StreamProcessorProperties
}

/// Properties for the `StreamProcessor` resource.
#[derive(Debug, Default)]
pub struct StreamProcessorProperties {
    /// Property [`BoundingBoxRegionsOfInterest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-boundingboxregionsofinterest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bounding_box_regions_of_interest: Option<::ValueList<self::stream_processor::BoundingBox>>,
    /// Property [`ConnectedHomeSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-connectedhomesettings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connected_home_settings: Option<::Value<self::stream_processor::ConnectedHomeSettings>>,
    /// Property [`DataSharingPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-datasharingpreference).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_sharing_preference: Option<::Value<self::stream_processor::DataSharingPreference>>,
    /// Property [`FaceSearchSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-facesearchsettings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub face_search_settings: Option<::Value<self::stream_processor::FaceSearchSettings>>,
    /// Property [`KinesisDataStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-kinesisdatastream).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kinesis_data_stream: Option<::Value<self::stream_processor::KinesisDataStream>>,
    /// Property [`KinesisVideoStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-kinesisvideostream).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kinesis_video_stream: ::Value<self::stream_processor::KinesisVideoStream>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NotificationChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-notificationchannel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub notification_channel: Option<::Value<self::stream_processor::NotificationChannel>>,
    /// Property [`PolygonRegionsOfInterest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-polygonregionsofinterest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub polygon_regions_of_interest: Option<::Value<::json::Value>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-s3destination).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_destination: Option<::Value<self::stream_processor::S3Destination>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html#cfn-rekognition-streamprocessor-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StreamProcessorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref bounding_box_regions_of_interest) = self.bounding_box_regions_of_interest {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BoundingBoxRegionsOfInterest", bounding_box_regions_of_interest)?;
        }
        if let Some(ref connected_home_settings) = self.connected_home_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectedHomeSettings", connected_home_settings)?;
        }
        if let Some(ref data_sharing_preference) = self.data_sharing_preference {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSharingPreference", data_sharing_preference)?;
        }
        if let Some(ref face_search_settings) = self.face_search_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FaceSearchSettings", face_search_settings)?;
        }
        if let Some(ref kinesis_data_stream) = self.kinesis_data_stream {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisDataStream", kinesis_data_stream)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisVideoStream", &self.kinesis_video_stream)?;
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref notification_channel) = self.notification_channel {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationChannel", notification_channel)?;
        }
        if let Some(ref polygon_regions_of_interest) = self.polygon_regions_of_interest {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolygonRegionsOfInterest", polygon_regions_of_interest)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref s3_destination) = self.s3_destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Destination", s3_destination)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamProcessorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamProcessorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamProcessorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamProcessorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bounding_box_regions_of_interest: Option<::ValueList<self::stream_processor::BoundingBox>> = None;
                let mut connected_home_settings: Option<::Value<self::stream_processor::ConnectedHomeSettings>> = None;
                let mut data_sharing_preference: Option<::Value<self::stream_processor::DataSharingPreference>> = None;
                let mut face_search_settings: Option<::Value<self::stream_processor::FaceSearchSettings>> = None;
                let mut kinesis_data_stream: Option<::Value<self::stream_processor::KinesisDataStream>> = None;
                let mut kinesis_video_stream: Option<::Value<self::stream_processor::KinesisVideoStream>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut notification_channel: Option<::Value<self::stream_processor::NotificationChannel>> = None;
                let mut polygon_regions_of_interest: Option<::Value<::json::Value>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut s3_destination: Option<::Value<self::stream_processor::S3Destination>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BoundingBoxRegionsOfInterest" => {
                            bounding_box_regions_of_interest = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectedHomeSettings" => {
                            connected_home_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSharingPreference" => {
                            data_sharing_preference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FaceSearchSettings" => {
                            face_search_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisDataStream" => {
                            kinesis_data_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisVideoStream" => {
                            kinesis_video_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationChannel" => {
                            notification_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolygonRegionsOfInterest" => {
                            polygon_regions_of_interest = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Destination" => {
                            s3_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamProcessorProperties {
                    bounding_box_regions_of_interest: bounding_box_regions_of_interest,
                    connected_home_settings: connected_home_settings,
                    data_sharing_preference: data_sharing_preference,
                    face_search_settings: face_search_settings,
                    kinesis_data_stream: kinesis_data_stream,
                    kinesis_video_stream: kinesis_video_stream.ok_or(::serde::de::Error::missing_field("KinesisVideoStream"))?,
                    kms_key_id: kms_key_id,
                    name: name,
                    notification_channel: notification_channel,
                    polygon_regions_of_interest: polygon_regions_of_interest,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    s3_destination: s3_destination,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StreamProcessor {
    type Properties = StreamProcessorProperties;
    const TYPE: &'static str = "AWS::Rekognition::StreamProcessor";
    fn properties(&self) -> &StreamProcessorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamProcessorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StreamProcessor {}

impl From<StreamProcessorProperties> for StreamProcessor {
    fn from(properties: StreamProcessorProperties) -> StreamProcessor {
        StreamProcessor { properties }
    }
}

pub mod stream_processor {
    //! Property types for the `StreamProcessor` resource.

    /// The [`AWS::Rekognition::StreamProcessor.BoundingBox`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html) property type.
    #[derive(Debug, Default)]
    pub struct BoundingBox {
        /// Property [`Height`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html#cfn-rekognition-streamprocessor-boundingbox-height).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub height: ::Value<f64>,
        /// Property [`Left`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html#cfn-rekognition-streamprocessor-boundingbox-left).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub left: ::Value<f64>,
        /// Property [`Top`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html#cfn-rekognition-streamprocessor-boundingbox-top).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub top: ::Value<f64>,
        /// Property [`Width`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html#cfn-rekognition-streamprocessor-boundingbox-width).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub width: ::Value<f64>,
    }

    impl ::codec::SerializeValue for BoundingBox {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Height", &self.height)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Left", &self.left)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Top", &self.top)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Width", &self.width)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BoundingBox {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BoundingBox, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BoundingBox;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BoundingBox")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut height: Option<::Value<f64>> = None;
                    let mut left: Option<::Value<f64>> = None;
                    let mut top: Option<::Value<f64>> = None;
                    let mut width: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Height" => {
                                height = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Left" => {
                                left = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Top" => {
                                top = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Width" => {
                                width = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BoundingBox {
                        height: height.ok_or(::serde::de::Error::missing_field("Height"))?,
                        left: left.ok_or(::serde::de::Error::missing_field("Left"))?,
                        top: top.ok_or(::serde::de::Error::missing_field("Top"))?,
                        width: width.ok_or(::serde::de::Error::missing_field("Width"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.ConnectedHomeSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-connectedhomesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectedHomeSettings {
        /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-connectedhomesettings.html#cfn-rekognition-streamprocessor-connectedhomesettings-labels).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub labels: ::ValueList<String>,
        /// Property [`MinConfidence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-connectedhomesettings.html#cfn-rekognition-streamprocessor-connectedhomesettings-minconfidence).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub min_confidence: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ConnectedHomeSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", &self.labels)?;
            if let Some(ref min_confidence) = self.min_confidence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinConfidence", min_confidence)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectedHomeSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectedHomeSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectedHomeSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectedHomeSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut labels: Option<::ValueList<String>> = None;
                    let mut min_confidence: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Labels" => {
                                labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinConfidence" => {
                                min_confidence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectedHomeSettings {
                        labels: labels.ok_or(::serde::de::Error::missing_field("Labels"))?,
                        min_confidence: min_confidence,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.DataSharingPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-datasharingpreference.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSharingPreference {
        /// Property [`OptIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-datasharingpreference.html#cfn-rekognition-streamprocessor-datasharingpreference-optin).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub opt_in: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DataSharingPreference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptIn", &self.opt_in)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSharingPreference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSharingPreference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSharingPreference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSharingPreference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut opt_in: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OptIn" => {
                                opt_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSharingPreference {
                        opt_in: opt_in.ok_or(::serde::de::Error::missing_field("OptIn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.FaceSearchSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-facesearchsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FaceSearchSettings {
        /// Property [`CollectionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-facesearchsettings.html#cfn-rekognition-streamprocessor-facesearchsettings-collectionid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub collection_id: ::Value<String>,
        /// Property [`FaceMatchThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-facesearchsettings.html#cfn-rekognition-streamprocessor-facesearchsettings-facematchthreshold).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub face_match_threshold: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for FaceSearchSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionId", &self.collection_id)?;
            if let Some(ref face_match_threshold) = self.face_match_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FaceMatchThreshold", face_match_threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FaceSearchSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FaceSearchSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FaceSearchSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FaceSearchSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut collection_id: Option<::Value<String>> = None;
                    let mut face_match_threshold: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CollectionId" => {
                                collection_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FaceMatchThreshold" => {
                                face_match_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FaceSearchSettings {
                        collection_id: collection_id.ok_or(::serde::de::Error::missing_field("CollectionId"))?,
                        face_match_threshold: face_match_threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.KinesisDataStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisdatastream.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisDataStream {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisdatastream.html#cfn-rekognition-streamprocessor-kinesisdatastream-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisDataStream {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisDataStream {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisDataStream, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisDataStream;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisDataStream")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisDataStream {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.KinesisVideoStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisvideostream.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisVideoStream {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisvideostream.html#cfn-rekognition-streamprocessor-kinesisvideostream-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisVideoStream {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisVideoStream {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisVideoStream, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisVideoStream;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisVideoStream")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisVideoStream {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.NotificationChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-notificationchannel.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationChannel {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-notificationchannel.html#cfn-rekognition-streamprocessor-notificationchannel-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for NotificationChannel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationChannel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationChannel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationChannel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationChannel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationChannel {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Rekognition::StreamProcessor.S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-s3destination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Destination {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-s3destination.html#cfn-rekognition-streamprocessor-s3destination-bucketname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`ObjectKeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-s3destination.html#cfn-rekognition-streamprocessor-s3destination-objectkeyprefix).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref object_key_prefix) = self.object_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectKeyPrefix", object_key_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Destination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Destination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Destination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Destination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut object_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectKeyPrefix" => {
                                object_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Destination {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        object_key_prefix: object_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
