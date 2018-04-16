//! Types for the `S3` service.

/// The [`AWS::S3::Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html) resource type.
#[derive(Debug)]
pub struct Bucket {
    properties: BucketProperties
}

/// Properties for the `Bucket` resource.
#[derive(Debug)]
pub struct BucketProperties {
    /// Property [`AccelerateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-accelerateconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accelerate_configuration: Option<::Value<self::bucket::AccelerateConfiguration>>,
    /// Property [`AccessControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-accesscontrol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_control: Option<::Value<String>>,
    /// Property [`AnalyticsConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-analyticsconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub analytics_configurations: Option<::ValueList<self::bucket::AnalyticsConfiguration>>,
    /// Property [`BucketEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-bucketencryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bucket_encryption: Option<::Value<self::bucket::BucketEncryption>>,
    /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_name: Option<::Value<String>>,
    /// Property [`CorsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-crossoriginconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cors_configuration: Option<::Value<self::bucket::CorsConfiguration>>,
    /// Property [`InventoryConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-inventoryconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub inventory_configurations: Option<::ValueList<self::bucket::InventoryConfiguration>>,
    /// Property [`LifecycleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-lifecycleconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_configuration: Option<::Value<self::bucket::LifecycleConfiguration>>,
    /// Property [`LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-loggingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_configuration: Option<::Value<self::bucket::LoggingConfiguration>>,
    /// Property [`MetricsConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-metricsconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metrics_configurations: Option<::ValueList<self::bucket::MetricsConfiguration>>,
    /// Property [`NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-notification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_configuration: Option<::Value<self::bucket::NotificationConfiguration>>,
    /// Property [`ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-replicationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_configuration: Option<::Value<self::bucket::ReplicationConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-versioning).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub versioning_configuration: Option<::Value<self::bucket::VersioningConfiguration>>,
    /// Property [`WebsiteConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html#cfn-s3-bucket-websiteconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub website_configuration: Option<::Value<self::bucket::WebsiteConfiguration>>,
}

impl ::serde::Serialize for BucketProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accelerate_configuration) = self.accelerate_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccelerateConfiguration", accelerate_configuration)?;
        }
        if let Some(ref access_control) = self.access_control {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControl", access_control)?;
        }
        if let Some(ref analytics_configurations) = self.analytics_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalyticsConfigurations", analytics_configurations)?;
        }
        if let Some(ref bucket_encryption) = self.bucket_encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketEncryption", bucket_encryption)?;
        }
        if let Some(ref bucket_name) = self.bucket_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
        }
        if let Some(ref cors_configuration) = self.cors_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorsConfiguration", cors_configuration)?;
        }
        if let Some(ref inventory_configurations) = self.inventory_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InventoryConfigurations", inventory_configurations)?;
        }
        if let Some(ref lifecycle_configuration) = self.lifecycle_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleConfiguration", lifecycle_configuration)?;
        }
        if let Some(ref logging_configuration) = self.logging_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfiguration", logging_configuration)?;
        }
        if let Some(ref metrics_configurations) = self.metrics_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsConfigurations", metrics_configurations)?;
        }
        if let Some(ref notification_configuration) = self.notification_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfiguration", notification_configuration)?;
        }
        if let Some(ref replication_configuration) = self.replication_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationConfiguration", replication_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref versioning_configuration) = self.versioning_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersioningConfiguration", versioning_configuration)?;
        }
        if let Some(ref website_configuration) = self.website_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebsiteConfiguration", website_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BucketProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BucketProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BucketProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BucketProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accelerate_configuration: Option<::Value<self::bucket::AccelerateConfiguration>> = None;
                let mut access_control: Option<::Value<String>> = None;
                let mut analytics_configurations: Option<::ValueList<self::bucket::AnalyticsConfiguration>> = None;
                let mut bucket_encryption: Option<::Value<self::bucket::BucketEncryption>> = None;
                let mut bucket_name: Option<::Value<String>> = None;
                let mut cors_configuration: Option<::Value<self::bucket::CorsConfiguration>> = None;
                let mut inventory_configurations: Option<::ValueList<self::bucket::InventoryConfiguration>> = None;
                let mut lifecycle_configuration: Option<::Value<self::bucket::LifecycleConfiguration>> = None;
                let mut logging_configuration: Option<::Value<self::bucket::LoggingConfiguration>> = None;
                let mut metrics_configurations: Option<::ValueList<self::bucket::MetricsConfiguration>> = None;
                let mut notification_configuration: Option<::Value<self::bucket::NotificationConfiguration>> = None;
                let mut replication_configuration: Option<::Value<self::bucket::ReplicationConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut versioning_configuration: Option<::Value<self::bucket::VersioningConfiguration>> = None;
                let mut website_configuration: Option<::Value<self::bucket::WebsiteConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccelerateConfiguration" => {
                            accelerate_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccessControl" => {
                            access_control = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnalyticsConfigurations" => {
                            analytics_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketEncryption" => {
                            bucket_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketName" => {
                            bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CorsConfiguration" => {
                            cors_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InventoryConfigurations" => {
                            inventory_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleConfiguration" => {
                            lifecycle_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfiguration" => {
                            logging_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricsConfigurations" => {
                            metrics_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationConfiguration" => {
                            notification_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationConfiguration" => {
                            replication_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersioningConfiguration" => {
                            versioning_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WebsiteConfiguration" => {
                            website_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BucketProperties {
                    accelerate_configuration: accelerate_configuration,
                    access_control: access_control,
                    analytics_configurations: analytics_configurations,
                    bucket_encryption: bucket_encryption,
                    bucket_name: bucket_name,
                    cors_configuration: cors_configuration,
                    inventory_configurations: inventory_configurations,
                    lifecycle_configuration: lifecycle_configuration,
                    logging_configuration: logging_configuration,
                    metrics_configurations: metrics_configurations,
                    notification_configuration: notification_configuration,
                    replication_configuration: replication_configuration,
                    tags: tags,
                    versioning_configuration: versioning_configuration,
                    website_configuration: website_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Bucket {
    type Properties = BucketProperties;
    const TYPE: &'static str = "AWS::S3::Bucket";
    fn properties(&self) -> &BucketProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BucketProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Bucket {}

impl From<BucketProperties> for Bucket {
    fn from(properties: BucketProperties) -> Bucket {
        Bucket { properties }
    }
}

/// The [`AWS::S3::BucketPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html) resource type.
#[derive(Debug)]
pub struct BucketPolicy {
    properties: BucketPolicyProperties
}

/// Properties for the `BucketPolicy` resource.
#[derive(Debug)]
pub struct BucketPolicyProperties {
    /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html#aws-properties-s3-policy-bucket).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket: ::Value<String>,
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html#aws-properties-s3-policy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
}

impl ::serde::Serialize for BucketPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BucketPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BucketPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BucketPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BucketPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bucket: Option<::Value<String>> = None;
                let mut policy_document: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Bucket" => {
                            bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BucketPolicyProperties {
                    bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BucketPolicy {
    type Properties = BucketPolicyProperties;
    const TYPE: &'static str = "AWS::S3::BucketPolicy";
    fn properties(&self) -> &BucketPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BucketPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BucketPolicy {}

impl From<BucketPolicyProperties> for BucketPolicy {
    fn from(properties: BucketPolicyProperties) -> BucketPolicy {
        BucketPolicy { properties }
    }
}

pub mod bucket {
    //! Property types for the `Bucket` resource.

    /// The [`AWS::S3::Bucket.AbortIncompleteMultipartUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-abortincompletemultipartupload.html) property type.
    #[derive(Debug)]
    pub struct AbortIncompleteMultipartUpload {
        /// Property [`DaysAfterInitiation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-abortincompletemultipartupload.html#cfn-s3-bucket-abortincompletemultipartupload-daysafterinitiation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub days_after_initiation: ::Value<u32>,
    }

    impl ::codec::SerializeValue for AbortIncompleteMultipartUpload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DaysAfterInitiation", &self.days_after_initiation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AbortIncompleteMultipartUpload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AbortIncompleteMultipartUpload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AbortIncompleteMultipartUpload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AbortIncompleteMultipartUpload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut days_after_initiation: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DaysAfterInitiation" => {
                                days_after_initiation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AbortIncompleteMultipartUpload {
                        days_after_initiation: days_after_initiation.ok_or(::serde::de::Error::missing_field("DaysAfterInitiation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.AccelerateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accelerateconfiguration.html) property type.
    #[derive(Debug)]
    pub struct AccelerateConfiguration {
        /// Property [`AccelerationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accelerateconfiguration.html#cfn-s3-bucket-accelerateconfiguration-accelerationstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acceleration_status: ::Value<String>,
    }

    impl ::codec::SerializeValue for AccelerateConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccelerationStatus", &self.acceleration_status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccelerateConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccelerateConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccelerateConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccelerateConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acceleration_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccelerationStatus" => {
                                acceleration_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccelerateConfiguration {
                        acceleration_status: acceleration_status.ok_or(::serde::de::Error::missing_field("AccelerationStatus"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.AccessControlTranslation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accesscontroltranslation.html) property type.
    #[derive(Debug)]
    pub struct AccessControlTranslation {
        /// Property [`Owner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accesscontroltranslation.html#cfn-s3-bucket-accesscontroltranslation-owner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub owner: ::Value<String>,
    }

    impl ::codec::SerializeValue for AccessControlTranslation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", &self.owner)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlTranslation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlTranslation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlTranslation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlTranslation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut owner: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Owner" => {
                                owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlTranslation {
                        owner: owner.ok_or(::serde::de::Error::missing_field("Owner"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.AnalyticsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html) property type.
    #[derive(Debug)]
    pub struct AnalyticsConfiguration {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html#cfn-s3-bucket-analyticsconfiguration-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html#cfn-s3-bucket-analyticsconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`StorageClassAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html#cfn-s3-bucket-analyticsconfiguration-storageclassanalysis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_class_analysis: ::Value<StorageClassAnalysis>,
        /// Property [`TagFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html#cfn-s3-bucket-analyticsconfiguration-tagfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_filters: Option<::ValueList<TagFilter>>,
    }

    impl ::codec::SerializeValue for AnalyticsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageClassAnalysis", &self.storage_class_analysis)?;
            if let Some(ref tag_filters) = self.tag_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagFilters", tag_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalyticsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalyticsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalyticsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalyticsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut storage_class_analysis: Option<::Value<StorageClassAnalysis>> = None;
                    let mut tag_filters: Option<::ValueList<TagFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageClassAnalysis" => {
                                storage_class_analysis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagFilters" => {
                                tag_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalyticsConfiguration {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        prefix: prefix,
                        storage_class_analysis: storage_class_analysis.ok_or(::serde::de::Error::missing_field("StorageClassAnalysis"))?,
                        tag_filters: tag_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.BucketEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-bucketencryption.html) property type.
    #[derive(Debug)]
    pub struct BucketEncryption {
        /// Property [`ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-bucketencryption.html#cfn-s3-bucket-bucketencryption-serversideencryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_side_encryption_configuration: ::ValueList<ServerSideEncryptionRule>,
    }

    impl ::codec::SerializeValue for BucketEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionConfiguration", &self.server_side_encryption_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BucketEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BucketEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BucketEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BucketEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_side_encryption_configuration: Option<::ValueList<ServerSideEncryptionRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerSideEncryptionConfiguration" => {
                                server_side_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BucketEncryption {
                        server_side_encryption_configuration: server_side_encryption_configuration.ok_or(::serde::de::Error::missing_field("ServerSideEncryptionConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.CorsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors.html) property type.
    #[derive(Debug)]
    pub struct CorsConfiguration {
        /// Property [`CorsRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors.html#cfn-s3-bucket-cors-corsrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cors_rules: ::ValueList<CorsRule>,
    }

    impl ::codec::SerializeValue for CorsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorsRules", &self.cors_rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CorsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CorsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CorsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CorsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cors_rules: Option<::ValueList<CorsRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CorsRules" => {
                                cors_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CorsConfiguration {
                        cors_rules: cors_rules.ok_or(::serde::de::Error::missing_field("CorsRules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.CorsRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html) property type.
    #[derive(Debug)]
    pub struct CorsRule {
        /// Property [`AllowedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-allowedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_headers: Option<::ValueList<String>>,
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-allowedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_methods: ::ValueList<String>,
        /// Property [`AllowedOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-allowedorigins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_origins: ::ValueList<String>,
        /// Property [`ExposedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-exposedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exposed_headers: Option<::ValueList<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`MaxAge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html#cfn-s3-bucket-cors-corsrule-maxage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_age: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CorsRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_headers) = self.allowed_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedHeaders", allowed_headers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", &self.allowed_methods)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOrigins", &self.allowed_origins)?;
            if let Some(ref exposed_headers) = self.exposed_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExposedHeaders", exposed_headers)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref max_age) = self.max_age {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAge", max_age)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CorsRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CorsRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CorsRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CorsRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_headers: Option<::ValueList<String>> = None;
                    let mut allowed_methods: Option<::ValueList<String>> = None;
                    let mut allowed_origins: Option<::ValueList<String>> = None;
                    let mut exposed_headers: Option<::ValueList<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut max_age: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedHeaders" => {
                                allowed_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedOrigins" => {
                                allowed_origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExposedHeaders" => {
                                exposed_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxAge" => {
                                max_age = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CorsRule {
                        allowed_headers: allowed_headers,
                        allowed_methods: allowed_methods.ok_or(::serde::de::Error::missing_field("AllowedMethods"))?,
                        allowed_origins: allowed_origins.ok_or(::serde::de::Error::missing_field("AllowedOrigins"))?,
                        exposed_headers: exposed_headers,
                        id: id,
                        max_age: max_age,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.DataExport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html) property type.
    #[derive(Debug)]
    pub struct DataExport {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html#cfn-s3-bucket-dataexport-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<Destination>,
        /// Property [`OutputSchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html#cfn-s3-bucket-dataexport-outputschemaversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_schema_version: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataExport {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSchemaVersion", &self.output_schema_version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataExport {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataExport, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataExport;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataExport")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<Destination>> = None;
                    let mut output_schema_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputSchemaVersion" => {
                                output_schema_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataExport {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        output_schema_version: output_schema_version.ok_or(::serde::de::Error::missing_field("OutputSchemaVersion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html) property type.
    #[derive(Debug)]
    pub struct Destination {
        /// Property [`BucketAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html#cfn-s3-bucket-destination-bucketaccountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_account_id: Option<::Value<String>>,
        /// Property [`BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html#cfn-s3-bucket-destination-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html#cfn-s3-bucket-destination-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html#cfn-s3-bucket-destination-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_account_id) = self.bucket_account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketAccountId", bucket_account_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketArn", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Destination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Destination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Destination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Destination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_account_id: Option<::Value<String>> = None;
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketAccountId" => {
                                bucket_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketArn" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Destination {
                        bucket_account_id: bucket_account_id,
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketArn"))?,
                        format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-encryptionconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EncryptionConfiguration {
        /// Property [`ReplicaKmsKeyID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-encryptionconfiguration.html#cfn-s3-bucket-encryptionconfiguration-replicakmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replica_kms_key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaKmsKeyID", &self.replica_kms_key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut replica_kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReplicaKmsKeyID" => {
                                replica_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        replica_kms_key_id: replica_kms_key_id.ok_or(::serde::de::Error::missing_field("ReplicaKmsKeyID"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.FilterRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key-rules.html) property type.
    #[derive(Debug)]
    pub struct FilterRule {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key-rules.html#cfn-s3-bucket-notificationconfiguraiton-config-filter-s3key-rules-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key-rules.html#cfn-s3-bucket-notificationconfiguraiton-config-filter-s3key-rules-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for FilterRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterRule {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.InventoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html) property type.
    #[derive(Debug)]
    pub struct InventoryConfiguration {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<Destination>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`IncludedObjectVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-includedobjectversions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_object_versions: ::Value<String>,
        /// Property [`OptionalFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-optionalfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub optional_fields: Option<::ValueList<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`ScheduleFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html#cfn-s3-bucket-inventoryconfiguration-schedulefrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_frequency: ::Value<String>,
    }

    impl ::codec::SerializeValue for InventoryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedObjectVersions", &self.included_object_versions)?;
            if let Some(ref optional_fields) = self.optional_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionalFields", optional_fields)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleFrequency", &self.schedule_frequency)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InventoryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InventoryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InventoryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InventoryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<Destination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut included_object_versions: Option<::Value<String>> = None;
                    let mut optional_fields: Option<::ValueList<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut schedule_frequency: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedObjectVersions" => {
                                included_object_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionalFields" => {
                                optional_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleFrequency" => {
                                schedule_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InventoryConfiguration {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        included_object_versions: included_object_versions.ok_or(::serde::de::Error::missing_field("IncludedObjectVersions"))?,
                        optional_fields: optional_fields,
                        prefix: prefix,
                        schedule_frequency: schedule_frequency.ok_or(::serde::de::Error::missing_field("ScheduleFrequency"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.LambdaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html) property type.
    #[derive(Debug)]
    pub struct LambdaConfiguration {
        /// Property [`Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html#cfn-s3-bucket-notificationconfig-lambdaconfig-event).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event: ::Value<String>,
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html#cfn-s3-bucket-notificationconfig-lambdaconfig-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property [`Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html#cfn-s3-bucket-notificationconfig-lambdaconfig-function).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Event", &self.event)?;
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Function", &self.function)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event: Option<::Value<String>> = None;
                    let mut filter: Option<::Value<NotificationFilter>> = None;
                    let mut function: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Event" => {
                                event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Function" => {
                                function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConfiguration {
                        event: event.ok_or(::serde::de::Error::missing_field("Event"))?,
                        filter: filter,
                        function: function.ok_or(::serde::de::Error::missing_field("Function"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.LifecycleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig.html) property type.
    #[derive(Debug)]
    pub struct LifecycleConfiguration {
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig.html#cfn-s3-bucket-lifecycleconfig-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<Rule>,
    }

    impl ::codec::SerializeValue for LifecycleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecycleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecycleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecycleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rules: Option<::ValueList<Rule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleConfiguration {
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfig.html) property type.
    #[derive(Debug)]
    pub struct LoggingConfiguration {
        /// Property [`DestinationBucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfig.html#cfn-s3-bucket-loggingconfig-destinationbucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_bucket_name: Option<::Value<String>>,
        /// Property [`LogFilePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfig.html#cfn-s3-bucket-loggingconfig-logfileprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_file_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_bucket_name) = self.destination_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationBucketName", destination_bucket_name)?;
            }
            if let Some(ref log_file_prefix) = self.log_file_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogFilePrefix", log_file_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_bucket_name: Option<::Value<String>> = None;
                    let mut log_file_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationBucketName" => {
                                destination_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogFilePrefix" => {
                                log_file_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfiguration {
                        destination_bucket_name: destination_bucket_name,
                        log_file_prefix: log_file_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.MetricsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html) property type.
    #[derive(Debug)]
    pub struct MetricsConfiguration {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html#cfn-s3-bucket-metricsconfiguration-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html#cfn-s3-bucket-metricsconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`TagFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html#cfn-s3-bucket-metricsconfiguration-tagfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_filters: Option<::ValueList<TagFilter>>,
    }

    impl ::codec::SerializeValue for MetricsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref tag_filters) = self.tag_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagFilters", tag_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut tag_filters: Option<::ValueList<TagFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagFilters" => {
                                tag_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricsConfiguration {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        prefix: prefix,
                        tag_filters: tag_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.NoncurrentVersionTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition.html) property type.
    #[derive(Debug)]
    pub struct NoncurrentVersionTransition {
        /// Property [`StorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition.html#cfn-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition-storageclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_class: ::Value<String>,
        /// Property [`TransitionInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition.html#cfn-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition-transitionindays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_in_days: ::Value<u32>,
    }

    impl ::codec::SerializeValue for NoncurrentVersionTransition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageClass", &self.storage_class)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionInDays", &self.transition_in_days)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NoncurrentVersionTransition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NoncurrentVersionTransition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NoncurrentVersionTransition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NoncurrentVersionTransition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut storage_class: Option<::Value<String>> = None;
                    let mut transition_in_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StorageClass" => {
                                storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitionInDays" => {
                                transition_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NoncurrentVersionTransition {
                        storage_class: storage_class.ok_or(::serde::de::Error::missing_field("StorageClass"))?,
                        transition_in_days: transition_in_days.ok_or(::serde::de::Error::missing_field("TransitionInDays"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html) property type.
    #[derive(Debug)]
    pub struct NotificationConfiguration {
        /// Property [`LambdaConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html#cfn-s3-bucket-notificationconfig-lambdaconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_configurations: Option<::ValueList<LambdaConfiguration>>,
        /// Property [`QueueConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html#cfn-s3-bucket-notificationconfig-queueconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_configurations: Option<::ValueList<QueueConfiguration>>,
        /// Property [`TopicConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html#cfn-s3-bucket-notificationconfig-topicconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_configurations: Option<::ValueList<TopicConfiguration>>,
    }

    impl ::codec::SerializeValue for NotificationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_configurations) = self.lambda_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConfigurations", lambda_configurations)?;
            }
            if let Some(ref queue_configurations) = self.queue_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueConfigurations", queue_configurations)?;
            }
            if let Some(ref topic_configurations) = self.topic_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicConfigurations", topic_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_configurations: Option<::ValueList<LambdaConfiguration>> = None;
                    let mut queue_configurations: Option<::ValueList<QueueConfiguration>> = None;
                    let mut topic_configurations: Option<::ValueList<TopicConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaConfigurations" => {
                                lambda_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueConfigurations" => {
                                queue_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicConfigurations" => {
                                topic_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfiguration {
                        lambda_configurations: lambda_configurations,
                        queue_configurations: queue_configurations,
                        topic_configurations: topic_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.NotificationFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter.html) property type.
    #[derive(Debug)]
    pub struct NotificationFilter {
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter.html#cfn-s3-bucket-notificationconfiguraiton-config-filter-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: ::Value<S3KeyFilter>,
    }

    impl ::codec::SerializeValue for NotificationFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_key: Option<::Value<S3KeyFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationFilter {
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.QueueConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html) property type.
    #[derive(Debug)]
    pub struct QueueConfiguration {
        /// Property [`Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html#cfn-s3-bucket-notificationconfig-queueconfig-event).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event: ::Value<String>,
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html#cfn-s3-bucket-notificationconfig-queueconfig-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property [`Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html#cfn-s3-bucket-notificationconfig-queueconfig-queue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueueConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Event", &self.event)?;
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queue", &self.queue)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueueConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueueConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueueConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event: Option<::Value<String>> = None;
                    let mut filter: Option<::Value<NotificationFilter>> = None;
                    let mut queue: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Event" => {
                                event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Queue" => {
                                queue = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueueConfiguration {
                        event: event.ok_or(::serde::de::Error::missing_field("Event"))?,
                        filter: filter,
                        queue: queue.ok_or(::serde::de::Error::missing_field("Queue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.RedirectAllRequestsTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-redirectallrequeststo.html) property type.
    #[derive(Debug)]
    pub struct RedirectAllRequestsTo {
        /// Property [`HostName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-redirectallrequeststo.html#cfn-s3-websiteconfiguration-redirectallrequeststo-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_name: ::Value<String>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-redirectallrequeststo.html#cfn-s3-websiteconfiguration-redirectallrequeststo-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RedirectAllRequestsTo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostName", &self.host_name)?;
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedirectAllRequestsTo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedirectAllRequestsTo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedirectAllRequestsTo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedirectAllRequestsTo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host_name: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HostName" => {
                                host_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedirectAllRequestsTo {
                        host_name: host_name.ok_or(::serde::de::Error::missing_field("HostName"))?,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.RedirectRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html) property type.
    #[derive(Debug)]
    pub struct RedirectRule {
        /// Property [`HostName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html#cfn-s3-websiteconfiguration-redirectrule-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_name: Option<::Value<String>>,
        /// Property [`HttpRedirectCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html#cfn-s3-websiteconfiguration-redirectrule-httpredirectcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_redirect_code: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html#cfn-s3-websiteconfiguration-redirectrule-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`ReplaceKeyPrefixWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html#cfn-s3-websiteconfiguration-redirectrule-replacekeyprefixwith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_key_prefix_with: Option<::Value<String>>,
        /// Property [`ReplaceKeyWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html#cfn-s3-websiteconfiguration-redirectrule-replacekeywith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_key_with: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RedirectRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref host_name) = self.host_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostName", host_name)?;
            }
            if let Some(ref http_redirect_code) = self.http_redirect_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRedirectCode", http_redirect_code)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref replace_key_prefix_with) = self.replace_key_prefix_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceKeyPrefixWith", replace_key_prefix_with)?;
            }
            if let Some(ref replace_key_with) = self.replace_key_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceKeyWith", replace_key_with)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedirectRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedirectRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedirectRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedirectRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host_name: Option<::Value<String>> = None;
                    let mut http_redirect_code: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut replace_key_prefix_with: Option<::Value<String>> = None;
                    let mut replace_key_with: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HostName" => {
                                host_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpRedirectCode" => {
                                http_redirect_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplaceKeyPrefixWith" => {
                                replace_key_prefix_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplaceKeyWith" => {
                                replace_key_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedirectRule {
                        host_name: host_name,
                        http_redirect_code: http_redirect_code,
                        protocol: protocol,
                        replace_key_prefix_with: replace_key_prefix_with,
                        replace_key_with: replace_key_with,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ReplicationConfiguration {
        /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html#cfn-s3-bucket-replicationconfiguration-role).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role: ::Value<String>,
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html#cfn-s3-bucket-replicationconfiguration-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<ReplicationRule>,
    }

    impl ::codec::SerializeValue for ReplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role: Option<::Value<String>> = None;
                    let mut rules: Option<::ValueList<ReplicationRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Role" => {
                                role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationConfiguration {
                        role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.ReplicationDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html) property type.
    #[derive(Debug)]
    pub struct ReplicationDestination {
        /// Property [`AccessControlTranslation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html#cfn-s3-bucket-replicationdestination-accesscontroltranslation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_translation: Option<::Value<AccessControlTranslation>>,
        /// Property [`Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html#cfn-s3-bucket-replicationdestination-account).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account: Option<::Value<String>>,
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html#cfn-s3-bucket-replicationconfiguration-rules-destination-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html#cfn-s3-bucket-replicationdestination-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`StorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html#cfn-s3-bucket-replicationconfiguration-rules-destination-storageclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_class: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReplicationDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_control_translation) = self.access_control_translation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlTranslation", access_control_translation)?;
            }
            if let Some(ref account) = self.account {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Account", account)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            if let Some(ref storage_class) = self.storage_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageClass", storage_class)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_control_translation: Option<::Value<AccessControlTranslation>> = None;
                    let mut account: Option<::Value<String>> = None;
                    let mut bucket: Option<::Value<String>> = None;
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut storage_class: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessControlTranslation" => {
                                access_control_translation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Account" => {
                                account = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageClass" => {
                                storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationDestination {
                        access_control_translation: access_control_translation,
                        account: account,
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        encryption_configuration: encryption_configuration,
                        storage_class: storage_class,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.ReplicationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html) property type.
    #[derive(Debug)]
    pub struct ReplicationRule {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html#cfn-s3-bucket-replicationconfiguration-rules-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<ReplicationDestination>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html#cfn-s3-bucket-replicationconfiguration-rules-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html#cfn-s3-bucket-replicationconfiguration-rules-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: ::Value<String>,
        /// Property [`SourceSelectionCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html#cfn-s3-bucket-replicationrule-sourceselectioncriteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_selection_criteria: Option<::Value<SourceSelectionCriteria>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html#cfn-s3-bucket-replicationconfiguration-rules-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReplicationRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            if let Some(ref source_selection_criteria) = self.source_selection_criteria {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSelectionCriteria", source_selection_criteria)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<ReplicationDestination>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut source_selection_criteria: Option<::Value<SourceSelectionCriteria>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceSelectionCriteria" => {
                                source_selection_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationRule {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        id: id,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                        source_selection_criteria: source_selection_criteria,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.RoutingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html) property type.
    #[derive(Debug)]
    pub struct RoutingRule {
        /// Property [`RedirectRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html#cfn-s3-websiteconfiguration-routingrules-redirectrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redirect_rule: ::Value<RedirectRule>,
        /// Property [`RoutingRuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html#cfn-s3-websiteconfiguration-routingrules-routingrulecondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub routing_rule_condition: Option<::Value<RoutingRuleCondition>>,
    }

    impl ::codec::SerializeValue for RoutingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectRule", &self.redirect_rule)?;
            if let Some(ref routing_rule_condition) = self.routing_rule_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingRuleCondition", routing_rule_condition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut redirect_rule: Option<::Value<RedirectRule>> = None;
                    let mut routing_rule_condition: Option<::Value<RoutingRuleCondition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RedirectRule" => {
                                redirect_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoutingRuleCondition" => {
                                routing_rule_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingRule {
                        redirect_rule: redirect_rule.ok_or(::serde::de::Error::missing_field("RedirectRule"))?,
                        routing_rule_condition: routing_rule_condition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.RoutingRuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-routingrulecondition.html) property type.
    #[derive(Debug)]
    pub struct RoutingRuleCondition {
        /// Property [`HttpErrorCodeReturnedEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-routingrulecondition.html#cfn-s3-websiteconfiguration-routingrules-routingrulecondition-httperrorcodereturnedequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_error_code_returned_equals: Option<::Value<String>>,
        /// Property [`KeyPrefixEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-routingrulecondition.html#cfn-s3-websiteconfiguration-routingrules-routingrulecondition-keyprefixequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_prefix_equals: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RoutingRuleCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_error_code_returned_equals) = self.http_error_code_returned_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpErrorCodeReturnedEquals", http_error_code_returned_equals)?;
            }
            if let Some(ref key_prefix_equals) = self.key_prefix_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPrefixEquals", key_prefix_equals)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingRuleCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingRuleCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingRuleCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingRuleCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_error_code_returned_equals: Option<::Value<String>> = None;
                    let mut key_prefix_equals: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpErrorCodeReturnedEquals" => {
                                http_error_code_returned_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyPrefixEquals" => {
                                key_prefix_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingRuleCondition {
                        http_error_code_returned_equals: http_error_code_returned_equals,
                        key_prefix_equals: key_prefix_equals,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html) property type.
    #[derive(Debug)]
    pub struct Rule {
        /// Property [`AbortIncompleteMultipartUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-rule-abortincompletemultipartupload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub abort_incomplete_multipart_upload: Option<::Value<AbortIncompleteMultipartUpload>>,
        /// Property [`ExpirationDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-expirationdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expiration_date: Option<::Value<String>>,
        /// Property [`ExpirationInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-expirationindays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expiration_in_days: Option<::Value<u32>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`NoncurrentVersionExpirationInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-noncurrentversionexpirationindays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub noncurrent_version_expiration_in_days: Option<::Value<u32>>,
        /// Property [`NoncurrentVersionTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub noncurrent_version_transition: Option<::Value<NoncurrentVersionTransition>>,
        /// Property [`NoncurrentVersionTransitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-noncurrentversiontransitions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub noncurrent_version_transitions: Option<::ValueList<NoncurrentVersionTransition>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
        /// Property [`TagFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-rule-tagfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_filters: Option<::ValueList<TagFilter>>,
        /// Property [`Transition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-transition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition: Option<::Value<Transition>>,
        /// Property [`Transitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html#cfn-s3-bucket-lifecycleconfig-rule-transitions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transitions: Option<::ValueList<Transition>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref abort_incomplete_multipart_upload) = self.abort_incomplete_multipart_upload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AbortIncompleteMultipartUpload", abort_incomplete_multipart_upload)?;
            }
            if let Some(ref expiration_date) = self.expiration_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpirationDate", expiration_date)?;
            }
            if let Some(ref expiration_in_days) = self.expiration_in_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpirationInDays", expiration_in_days)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref noncurrent_version_expiration_in_days) = self.noncurrent_version_expiration_in_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoncurrentVersionExpirationInDays", noncurrent_version_expiration_in_days)?;
            }
            if let Some(ref noncurrent_version_transition) = self.noncurrent_version_transition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoncurrentVersionTransition", noncurrent_version_transition)?;
            }
            if let Some(ref noncurrent_version_transitions) = self.noncurrent_version_transitions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoncurrentVersionTransitions", noncurrent_version_transitions)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            if let Some(ref tag_filters) = self.tag_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagFilters", tag_filters)?;
            }
            if let Some(ref transition) = self.transition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Transition", transition)?;
            }
            if let Some(ref transitions) = self.transitions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Transitions", transitions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut abort_incomplete_multipart_upload: Option<::Value<AbortIncompleteMultipartUpload>> = None;
                    let mut expiration_date: Option<::Value<String>> = None;
                    let mut expiration_in_days: Option<::Value<u32>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut noncurrent_version_expiration_in_days: Option<::Value<u32>> = None;
                    let mut noncurrent_version_transition: Option<::Value<NoncurrentVersionTransition>> = None;
                    let mut noncurrent_version_transitions: Option<::ValueList<NoncurrentVersionTransition>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut tag_filters: Option<::ValueList<TagFilter>> = None;
                    let mut transition: Option<::Value<Transition>> = None;
                    let mut transitions: Option<::ValueList<Transition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AbortIncompleteMultipartUpload" => {
                                abort_incomplete_multipart_upload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpirationDate" => {
                                expiration_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpirationInDays" => {
                                expiration_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoncurrentVersionExpirationInDays" => {
                                noncurrent_version_expiration_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoncurrentVersionTransition" => {
                                noncurrent_version_transition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoncurrentVersionTransitions" => {
                                noncurrent_version_transitions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagFilters" => {
                                tag_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Transition" => {
                                transition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Transitions" => {
                                transitions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        abort_incomplete_multipart_upload: abort_incomplete_multipart_upload,
                        expiration_date: expiration_date,
                        expiration_in_days: expiration_in_days,
                        id: id,
                        noncurrent_version_expiration_in_days: noncurrent_version_expiration_in_days,
                        noncurrent_version_transition: noncurrent_version_transition,
                        noncurrent_version_transitions: noncurrent_version_transitions,
                        prefix: prefix,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                        tag_filters: tag_filters,
                        transition: transition,
                        transitions: transitions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.S3KeyFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key.html) property type.
    #[derive(Debug)]
    pub struct S3KeyFilter {
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key.html#cfn-s3-bucket-notificationconfiguraiton-config-filter-s3key-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<FilterRule>,
    }

    impl ::codec::SerializeValue for S3KeyFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3KeyFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3KeyFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3KeyFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3KeyFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rules: Option<::ValueList<FilterRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3KeyFilter {
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.ServerSideEncryptionByDefault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html) property type.
    #[derive(Debug)]
    pub struct ServerSideEncryptionByDefault {
        /// Property [`KMSMasterKeyID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html#cfn-s3-bucket-serversideencryptionbydefault-kmsmasterkeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_master_key_id: Option<::Value<String>>,
        /// Property [`SSEAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html#cfn-s3-bucket-serversideencryptionbydefault-ssealgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_algorithm: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServerSideEncryptionByDefault {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_master_key_id) = self.kms_master_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSMasterKeyID", kms_master_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEAlgorithm", &self.sse_algorithm)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerSideEncryptionByDefault {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerSideEncryptionByDefault, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerSideEncryptionByDefault;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerSideEncryptionByDefault")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_master_key_id: Option<::Value<String>> = None;
                    let mut sse_algorithm: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSMasterKeyID" => {
                                kms_master_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSEAlgorithm" => {
                                sse_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerSideEncryptionByDefault {
                        kms_master_key_id: kms_master_key_id,
                        sse_algorithm: sse_algorithm.ok_or(::serde::de::Error::missing_field("SSEAlgorithm"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.ServerSideEncryptionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionrule.html) property type.
    #[derive(Debug)]
    pub struct ServerSideEncryptionRule {
        /// Property [`ServerSideEncryptionByDefault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionrule.html#cfn-s3-bucket-serversideencryptionrule-serversideencryptionbydefault).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_side_encryption_by_default: Option<::Value<ServerSideEncryptionByDefault>>,
    }

    impl ::codec::SerializeValue for ServerSideEncryptionRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref server_side_encryption_by_default) = self.server_side_encryption_by_default {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionByDefault", server_side_encryption_by_default)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerSideEncryptionRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerSideEncryptionRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerSideEncryptionRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerSideEncryptionRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_side_encryption_by_default: Option<::Value<ServerSideEncryptionByDefault>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerSideEncryptionByDefault" => {
                                server_side_encryption_by_default = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerSideEncryptionRule {
                        server_side_encryption_by_default: server_side_encryption_by_default,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.SourceSelectionCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-sourceselectioncriteria.html) property type.
    #[derive(Debug)]
    pub struct SourceSelectionCriteria {
        /// Property [`SseKmsEncryptedObjects`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-sourceselectioncriteria.html#cfn-s3-bucket-sourceselectioncriteria-ssekmsencryptedobjects).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_kms_encrypted_objects: ::Value<SseKmsEncryptedObjects>,
    }

    impl ::codec::SerializeValue for SourceSelectionCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseKmsEncryptedObjects", &self.sse_kms_encrypted_objects)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceSelectionCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceSelectionCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceSelectionCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceSelectionCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sse_kms_encrypted_objects: Option<::Value<SseKmsEncryptedObjects>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SseKmsEncryptedObjects" => {
                                sse_kms_encrypted_objects = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceSelectionCriteria {
                        sse_kms_encrypted_objects: sse_kms_encrypted_objects.ok_or(::serde::de::Error::missing_field("SseKmsEncryptedObjects"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.SseKmsEncryptedObjects`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ssekmsencryptedobjects.html) property type.
    #[derive(Debug)]
    pub struct SseKmsEncryptedObjects {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ssekmsencryptedobjects.html#cfn-s3-bucket-ssekmsencryptedobjects-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for SseKmsEncryptedObjects {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseKmsEncryptedObjects {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseKmsEncryptedObjects, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseKmsEncryptedObjects;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseKmsEncryptedObjects")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseKmsEncryptedObjects {
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.StorageClassAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-storageclassanalysis.html) property type.
    #[derive(Debug)]
    pub struct StorageClassAnalysis {
        /// Property [`DataExport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-storageclassanalysis.html#cfn-s3-bucket-storageclassanalysis-dataexport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_export: Option<::Value<DataExport>>,
    }

    impl ::codec::SerializeValue for StorageClassAnalysis {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_export) = self.data_export {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataExport", data_export)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StorageClassAnalysis {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageClassAnalysis, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StorageClassAnalysis;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StorageClassAnalysis")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_export: Option<::Value<DataExport>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataExport" => {
                                data_export = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StorageClassAnalysis {
                        data_export: data_export,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html) property type.
    #[derive(Debug)]
    pub struct TagFilter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html#cfn-s3-bucket-tagfilter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html#cfn-s3-bucket-tagfilter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TagFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagFilter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.TopicConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html) property type.
    #[derive(Debug)]
    pub struct TopicConfiguration {
        /// Property [`Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html#cfn-s3-bucket-notificationconfig-topicconfig-event).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event: ::Value<String>,
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html#cfn-s3-bucket-notificationconfig-topicconfig-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html#cfn-s3-bucket-notificationconfig-topicconfig-topic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic: ::Value<String>,
    }

    impl ::codec::SerializeValue for TopicConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Event", &self.event)?;
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", &self.topic)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TopicConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TopicConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TopicConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event: Option<::Value<String>> = None;
                    let mut filter: Option<::Value<NotificationFilter>> = None;
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Event" => {
                                event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TopicConfiguration {
                        event: event.ok_or(::serde::de::Error::missing_field("Event"))?,
                        filter: filter,
                        topic: topic.ok_or(::serde::de::Error::missing_field("Topic"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.Transition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html) property type.
    #[derive(Debug)]
    pub struct Transition {
        /// Property [`StorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html#cfn-s3-bucket-lifecycleconfig-rule-transition-storageclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_class: ::Value<String>,
        /// Property [`TransitionDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html#cfn-s3-bucket-lifecycleconfig-rule-transition-transitiondate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_date: Option<::Value<String>>,
        /// Property [`TransitionInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html#cfn-s3-bucket-lifecycleconfig-rule-transition-transitionindays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_in_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Transition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageClass", &self.storage_class)?;
            if let Some(ref transition_date) = self.transition_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionDate", transition_date)?;
            }
            if let Some(ref transition_in_days) = self.transition_in_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionInDays", transition_in_days)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Transition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Transition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Transition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Transition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut storage_class: Option<::Value<String>> = None;
                    let mut transition_date: Option<::Value<String>> = None;
                    let mut transition_in_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StorageClass" => {
                                storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitionDate" => {
                                transition_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitionInDays" => {
                                transition_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Transition {
                        storage_class: storage_class.ok_or(::serde::de::Error::missing_field("StorageClass"))?,
                        transition_date: transition_date,
                        transition_in_days: transition_in_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-versioningconfig.html) property type.
    #[derive(Debug)]
    pub struct VersioningConfiguration {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-versioningconfig.html#cfn-s3-bucket-versioningconfig-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for VersioningConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
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
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VersioningConfiguration {
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3::Bucket.WebsiteConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html) property type.
    #[derive(Debug)]
    pub struct WebsiteConfiguration {
        /// Property [`ErrorDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html#cfn-s3-websiteconfiguration-errordocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_document: Option<::Value<String>>,
        /// Property [`IndexDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html#cfn-s3-websiteconfiguration-indexdocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_document: Option<::Value<String>>,
        /// Property [`RedirectAllRequestsTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html#cfn-s3-websiteconfiguration-redirectallrequeststo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redirect_all_requests_to: Option<::Value<RedirectAllRequestsTo>>,
        /// Property [`RoutingRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html#cfn-s3-websiteconfiguration-routingrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub routing_rules: Option<::ValueList<RoutingRule>>,
    }

    impl ::codec::SerializeValue for WebsiteConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_document) = self.error_document {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorDocument", error_document)?;
            }
            if let Some(ref index_document) = self.index_document {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexDocument", index_document)?;
            }
            if let Some(ref redirect_all_requests_to) = self.redirect_all_requests_to {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectAllRequestsTo", redirect_all_requests_to)?;
            }
            if let Some(ref routing_rules) = self.routing_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingRules", routing_rules)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebsiteConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebsiteConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebsiteConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebsiteConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_document: Option<::Value<String>> = None;
                    let mut index_document: Option<::Value<String>> = None;
                    let mut redirect_all_requests_to: Option<::Value<RedirectAllRequestsTo>> = None;
                    let mut routing_rules: Option<::ValueList<RoutingRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorDocument" => {
                                error_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexDocument" => {
                                index_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedirectAllRequestsTo" => {
                                redirect_all_requests_to = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoutingRules" => {
                                routing_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebsiteConfiguration {
                        error_document: error_document,
                        index_document: index_document,
                        redirect_all_requests_to: redirect_all_requests_to,
                        routing_rules: routing_rules,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
