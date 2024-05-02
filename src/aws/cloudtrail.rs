//! Types for the `CloudTrail` service.

/// The [`AWS::CloudTrail::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html#cfn-cloudtrail-channel-destinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destinations: Option<::ValueList<self::channel::Destination>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html#cfn-cloudtrail-channel-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html#cfn-cloudtrail-channel-source).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html#cfn-cloudtrail-channel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref destinations) = self.destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref source) = self.source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
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
                let mut destinations: Option<::ValueList<self::channel::Destination>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut source: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Destinations" => {
                            destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    destinations: destinations,
                    name: name,
                    source: source,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::CloudTrail::Channel";
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

/// The [`AWS::CloudTrail::EventDataStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html) resource type.
#[derive(Debug, Default)]
pub struct EventDataStore {
    properties: EventDataStoreProperties
}

/// Properties for the `EventDataStore` resource.
#[derive(Debug, Default)]
pub struct EventDataStoreProperties {
    /// Property [`AdvancedEventSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-advancedeventselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub advanced_event_selectors: Option<::ValueList<self::event_data_store::AdvancedEventSelector>>,
    /// Property [`BillingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-billingmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_mode: Option<::Value<String>>,
    /// Property [`FederationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-federationenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub federation_enabled: Option<::Value<bool>>,
    /// Property [`FederationRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-federationrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub federation_role_arn: Option<::Value<String>>,
    /// Property [`IngestionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-ingestionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ingestion_enabled: Option<::Value<bool>>,
    /// Property [`InsightSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-insightselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insight_selectors: Option<::ValueList<self::event_data_store::InsightSelector>>,
    /// Property [`InsightsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-insightsdestination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insights_destination: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`MultiRegionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-multiregionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub multi_region_enabled: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`OrganizationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-organizationenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organization_enabled: Option<::Value<bool>>,
    /// Property [`RetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-retentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_period: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TerminationProtectionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html#cfn-cloudtrail-eventdatastore-terminationprotectionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub termination_protection_enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for EventDataStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref advanced_event_selectors) = self.advanced_event_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedEventSelectors", advanced_event_selectors)?;
        }
        if let Some(ref billing_mode) = self.billing_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingMode", billing_mode)?;
        }
        if let Some(ref federation_enabled) = self.federation_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationEnabled", federation_enabled)?;
        }
        if let Some(ref federation_role_arn) = self.federation_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationRoleArn", federation_role_arn)?;
        }
        if let Some(ref ingestion_enabled) = self.ingestion_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestionEnabled", ingestion_enabled)?;
        }
        if let Some(ref insight_selectors) = self.insight_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightSelectors", insight_selectors)?;
        }
        if let Some(ref insights_destination) = self.insights_destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightsDestination", insights_destination)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref multi_region_enabled) = self.multi_region_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiRegionEnabled", multi_region_enabled)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref organization_enabled) = self.organization_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationEnabled", organization_enabled)?;
        }
        if let Some(ref retention_period) = self.retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriod", retention_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref termination_protection_enabled) = self.termination_protection_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationProtectionEnabled", termination_protection_enabled)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventDataStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventDataStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventDataStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventDataStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut advanced_event_selectors: Option<::ValueList<self::event_data_store::AdvancedEventSelector>> = None;
                let mut billing_mode: Option<::Value<String>> = None;
                let mut federation_enabled: Option<::Value<bool>> = None;
                let mut federation_role_arn: Option<::Value<String>> = None;
                let mut ingestion_enabled: Option<::Value<bool>> = None;
                let mut insight_selectors: Option<::ValueList<self::event_data_store::InsightSelector>> = None;
                let mut insights_destination: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut multi_region_enabled: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut organization_enabled: Option<::Value<bool>> = None;
                let mut retention_period: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut termination_protection_enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdvancedEventSelectors" => {
                            advanced_event_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingMode" => {
                            billing_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FederationEnabled" => {
                            federation_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FederationRoleArn" => {
                            federation_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IngestionEnabled" => {
                            ingestion_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsightSelectors" => {
                            insight_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsightsDestination" => {
                            insights_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiRegionEnabled" => {
                            multi_region_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationEnabled" => {
                            organization_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionPeriod" => {
                            retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TerminationProtectionEnabled" => {
                            termination_protection_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventDataStoreProperties {
                    advanced_event_selectors: advanced_event_selectors,
                    billing_mode: billing_mode,
                    federation_enabled: federation_enabled,
                    federation_role_arn: federation_role_arn,
                    ingestion_enabled: ingestion_enabled,
                    insight_selectors: insight_selectors,
                    insights_destination: insights_destination,
                    kms_key_id: kms_key_id,
                    multi_region_enabled: multi_region_enabled,
                    name: name,
                    organization_enabled: organization_enabled,
                    retention_period: retention_period,
                    tags: tags,
                    termination_protection_enabled: termination_protection_enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventDataStore {
    type Properties = EventDataStoreProperties;
    const TYPE: &'static str = "AWS::CloudTrail::EventDataStore";
    fn properties(&self) -> &EventDataStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventDataStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventDataStore {}

impl From<EventDataStoreProperties> for EventDataStore {
    fn from(properties: EventDataStoreProperties) -> EventDataStore {
        EventDataStore { properties }
    }
}

/// The [`AWS::CloudTrail::ResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-resourcepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ResourcePolicy {
    properties: ResourcePolicyProperties
}

/// Properties for the `ResourcePolicy` resource.
#[derive(Debug, Default)]
pub struct ResourcePolicyProperties {
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-resourcepolicy.html#cfn-cloudtrail-resourcepolicy-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`ResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-resourcepolicy.html#cfn-cloudtrail-resourcepolicy-resourcepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_policy: ::Value<::json::Value>,
}

impl ::serde::Serialize for ResourcePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePolicy", &self.resource_policy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourcePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourcePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_arn: Option<::Value<String>> = None;
                let mut resource_policy: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcePolicy" => {
                            resource_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourcePolicyProperties {
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    resource_policy: resource_policy.ok_or(::serde::de::Error::missing_field("ResourcePolicy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourcePolicy {
    type Properties = ResourcePolicyProperties;
    const TYPE: &'static str = "AWS::CloudTrail::ResourcePolicy";
    fn properties(&self) -> &ResourcePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourcePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourcePolicy {}

impl From<ResourcePolicyProperties> for ResourcePolicy {
    fn from(properties: ResourcePolicyProperties) -> ResourcePolicy {
        ResourcePolicy { properties }
    }
}

/// The [`AWS::CloudTrail::Trail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html) resource type.
#[derive(Debug, Default)]
pub struct Trail {
    properties: TrailProperties
}

/// Properties for the `Trail` resource.
#[derive(Debug, Default)]
pub struct TrailProperties {
    /// Property [`AdvancedEventSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-advancedeventselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub advanced_event_selectors: Option<::ValueList<self::trail::AdvancedEventSelector>>,
    /// Property [`CloudWatchLogsLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-cloudwatchlogsloggrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_logs_log_group_arn: Option<::Value<String>>,
    /// Property [`CloudWatchLogsRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-cloudwatchlogsrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_logs_role_arn: Option<::Value<String>>,
    /// Property [`EnableLogFileValidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-enablelogfilevalidation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_log_file_validation: Option<::Value<bool>>,
    /// Property [`EventSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-eventselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_selectors: Option<::ValueList<self::trail::EventSelector>>,
    /// Property [`IncludeGlobalServiceEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-includeglobalserviceevents).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub include_global_service_events: Option<::Value<bool>>,
    /// Property [`InsightSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-insightselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insight_selectors: Option<::ValueList<self::trail::InsightSelector>>,
    /// Property [`IsLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-islogging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_logging: ::Value<bool>,
    /// Property [`IsMultiRegionTrail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-ismultiregiontrail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_multi_region_trail: Option<::Value<bool>>,
    /// Property [`IsOrganizationTrail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-isorganizationtrail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_organization_trail: Option<::Value<bool>>,
    /// Property [`KMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-s3bucketname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_bucket_name: ::Value<String>,
    /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-s3keyprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_key_prefix: Option<::Value<String>>,
    /// Property [`SnsTopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-snstopicname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrailName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-trailname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub trail_name: Option<::Value<String>>,
}

impl ::serde::Serialize for TrailProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref advanced_event_selectors) = self.advanced_event_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedEventSelectors", advanced_event_selectors)?;
        }
        if let Some(ref cloud_watch_logs_log_group_arn) = self.cloud_watch_logs_log_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", cloud_watch_logs_log_group_arn)?;
        }
        if let Some(ref cloud_watch_logs_role_arn) = self.cloud_watch_logs_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsRoleArn", cloud_watch_logs_role_arn)?;
        }
        if let Some(ref enable_log_file_validation) = self.enable_log_file_validation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableLogFileValidation", enable_log_file_validation)?;
        }
        if let Some(ref event_selectors) = self.event_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSelectors", event_selectors)?;
        }
        if let Some(ref include_global_service_events) = self.include_global_service_events {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeGlobalServiceEvents", include_global_service_events)?;
        }
        if let Some(ref insight_selectors) = self.insight_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightSelectors", insight_selectors)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLogging", &self.is_logging)?;
        if let Some(ref is_multi_region_trail) = self.is_multi_region_trail {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsMultiRegionTrail", is_multi_region_trail)?;
        }
        if let Some(ref is_organization_trail) = self.is_organization_trail {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsOrganizationTrail", is_organization_trail)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
        if let Some(ref s3_key_prefix) = self.s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
        }
        if let Some(ref sns_topic_name) = self.sns_topic_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicName", sns_topic_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref trail_name) = self.trail_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrailName", trail_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrailProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrailProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrailProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrailProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut advanced_event_selectors: Option<::ValueList<self::trail::AdvancedEventSelector>> = None;
                let mut cloud_watch_logs_log_group_arn: Option<::Value<String>> = None;
                let mut cloud_watch_logs_role_arn: Option<::Value<String>> = None;
                let mut enable_log_file_validation: Option<::Value<bool>> = None;
                let mut event_selectors: Option<::ValueList<self::trail::EventSelector>> = None;
                let mut include_global_service_events: Option<::Value<bool>> = None;
                let mut insight_selectors: Option<::ValueList<self::trail::InsightSelector>> = None;
                let mut is_logging: Option<::Value<bool>> = None;
                let mut is_multi_region_trail: Option<::Value<bool>> = None;
                let mut is_organization_trail: Option<::Value<bool>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut s3_bucket_name: Option<::Value<String>> = None;
                let mut s3_key_prefix: Option<::Value<String>> = None;
                let mut sns_topic_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut trail_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdvancedEventSelectors" => {
                            advanced_event_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchLogsLogGroupArn" => {
                            cloud_watch_logs_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchLogsRoleArn" => {
                            cloud_watch_logs_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableLogFileValidation" => {
                            enable_log_file_validation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSelectors" => {
                            event_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncludeGlobalServiceEvents" => {
                            include_global_service_events = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsightSelectors" => {
                            insight_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsLogging" => {
                            is_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsMultiRegionTrail" => {
                            is_multi_region_trail = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsOrganizationTrail" => {
                            is_organization_trail = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3BucketName" => {
                            s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3KeyPrefix" => {
                            s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicName" => {
                            sns_topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrailName" => {
                            trail_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrailProperties {
                    advanced_event_selectors: advanced_event_selectors,
                    cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn,
                    cloud_watch_logs_role_arn: cloud_watch_logs_role_arn,
                    enable_log_file_validation: enable_log_file_validation,
                    event_selectors: event_selectors,
                    include_global_service_events: include_global_service_events,
                    insight_selectors: insight_selectors,
                    is_logging: is_logging.ok_or(::serde::de::Error::missing_field("IsLogging"))?,
                    is_multi_region_trail: is_multi_region_trail,
                    is_organization_trail: is_organization_trail,
                    kms_key_id: kms_key_id,
                    s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                    s3_key_prefix: s3_key_prefix,
                    sns_topic_name: sns_topic_name,
                    tags: tags,
                    trail_name: trail_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Trail {
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

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::CloudTrail::Channel.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-channel-destination.html) property type.
    #[derive(Debug, Default)]
    pub struct Destination {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-channel-destination.html#cfn-cloudtrail-channel-destination-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-channel-destination.html#cfn-cloudtrail-channel-destination-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut location: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Destination {
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_data_store {
    //! Property types for the `EventDataStore` resource.

    /// The [`AWS::CloudTrail::EventDataStore.AdvancedEventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedeventselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedEventSelector {
        /// Property [`FieldSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedeventselector.html#cfn-cloudtrail-eventdatastore-advancedeventselector-fieldselectors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_selectors: ::ValueList<AdvancedFieldSelector>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedeventselector.html#cfn-cloudtrail-eventdatastore-advancedeventselector-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AdvancedEventSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldSelectors", &self.field_selectors)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedEventSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedEventSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedEventSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedEventSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_selectors: Option<::ValueList<AdvancedFieldSelector>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldSelectors" => {
                                field_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedEventSelector {
                        field_selectors: field_selectors.ok_or(::serde::de::Error::missing_field("FieldSelectors"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::EventDataStore.AdvancedFieldSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedFieldSelector {
        /// Property [`EndsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-endswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ends_with: Option<::ValueList<String>>,
        /// Property [`Equals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-equals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub equals: Option<::ValueList<String>>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: ::Value<String>,
        /// Property [`NotEndsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-notendswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_ends_with: Option<::ValueList<String>>,
        /// Property [`NotEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-notequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_equals: Option<::ValueList<String>>,
        /// Property [`NotStartsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-notstartswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_starts_with: Option<::ValueList<String>>,
        /// Property [`StartsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html#cfn-cloudtrail-eventdatastore-advancedfieldselector-startswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub starts_with: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AdvancedFieldSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ends_with) = self.ends_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndsWith", ends_with)?;
            }
            if let Some(ref equals) = self.equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Equals", equals)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", &self.field)?;
            if let Some(ref not_ends_with) = self.not_ends_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotEndsWith", not_ends_with)?;
            }
            if let Some(ref not_equals) = self.not_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotEquals", not_equals)?;
            }
            if let Some(ref not_starts_with) = self.not_starts_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotStartsWith", not_starts_with)?;
            }
            if let Some(ref starts_with) = self.starts_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartsWith", starts_with)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedFieldSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedFieldSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedFieldSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedFieldSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ends_with: Option<::ValueList<String>> = None;
                    let mut equals: Option<::ValueList<String>> = None;
                    let mut field: Option<::Value<String>> = None;
                    let mut not_ends_with: Option<::ValueList<String>> = None;
                    let mut not_equals: Option<::ValueList<String>> = None;
                    let mut not_starts_with: Option<::ValueList<String>> = None;
                    let mut starts_with: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndsWith" => {
                                ends_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Equals" => {
                                equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotEndsWith" => {
                                not_ends_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotEquals" => {
                                not_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotStartsWith" => {
                                not_starts_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartsWith" => {
                                starts_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedFieldSelector {
                        ends_with: ends_with,
                        equals: equals,
                        field: field.ok_or(::serde::de::Error::missing_field("Field"))?,
                        not_ends_with: not_ends_with,
                        not_equals: not_equals,
                        not_starts_with: not_starts_with,
                        starts_with: starts_with,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::EventDataStore.InsightSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-insightselector.html) property type.
    #[derive(Debug, Default)]
    pub struct InsightSelector {
        /// Property [`InsightType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-insightselector.html#cfn-cloudtrail-eventdatastore-insightselector-insighttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insight_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InsightSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref insight_type) = self.insight_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightType", insight_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InsightSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InsightSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InsightSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InsightSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut insight_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InsightType" => {
                                insight_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InsightSelector {
                        insight_type: insight_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod trail {
    //! Property types for the `Trail` resource.

    /// The [`AWS::CloudTrail::Trail.AdvancedEventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedeventselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedEventSelector {
        /// Property [`FieldSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedeventselector.html#cfn-cloudtrail-trail-advancedeventselector-fieldselectors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_selectors: ::ValueList<AdvancedFieldSelector>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedeventselector.html#cfn-cloudtrail-trail-advancedeventselector-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AdvancedEventSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldSelectors", &self.field_selectors)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedEventSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedEventSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedEventSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedEventSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_selectors: Option<::ValueList<AdvancedFieldSelector>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldSelectors" => {
                                field_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedEventSelector {
                        field_selectors: field_selectors.ok_or(::serde::de::Error::missing_field("FieldSelectors"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.AdvancedFieldSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedFieldSelector {
        /// Property [`EndsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-endswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ends_with: Option<::ValueList<String>>,
        /// Property [`Equals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-equals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub equals: Option<::ValueList<String>>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: ::Value<String>,
        /// Property [`NotEndsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-notendswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_ends_with: Option<::ValueList<String>>,
        /// Property [`NotEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-notequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_equals: Option<::ValueList<String>>,
        /// Property [`NotStartsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-notstartswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_starts_with: Option<::ValueList<String>>,
        /// Property [`StartsWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html#cfn-cloudtrail-trail-advancedfieldselector-startswith).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub starts_with: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AdvancedFieldSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ends_with) = self.ends_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndsWith", ends_with)?;
            }
            if let Some(ref equals) = self.equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Equals", equals)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", &self.field)?;
            if let Some(ref not_ends_with) = self.not_ends_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotEndsWith", not_ends_with)?;
            }
            if let Some(ref not_equals) = self.not_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotEquals", not_equals)?;
            }
            if let Some(ref not_starts_with) = self.not_starts_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotStartsWith", not_starts_with)?;
            }
            if let Some(ref starts_with) = self.starts_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartsWith", starts_with)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedFieldSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedFieldSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedFieldSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedFieldSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ends_with: Option<::ValueList<String>> = None;
                    let mut equals: Option<::ValueList<String>> = None;
                    let mut field: Option<::Value<String>> = None;
                    let mut not_ends_with: Option<::ValueList<String>> = None;
                    let mut not_equals: Option<::ValueList<String>> = None;
                    let mut not_starts_with: Option<::ValueList<String>> = None;
                    let mut starts_with: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndsWith" => {
                                ends_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Equals" => {
                                equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotEndsWith" => {
                                not_ends_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotEquals" => {
                                not_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotStartsWith" => {
                                not_starts_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartsWith" => {
                                starts_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedFieldSelector {
                        ends_with: ends_with,
                        equals: equals,
                        field: field.ok_or(::serde::de::Error::missing_field("Field"))?,
                        not_ends_with: not_ends_with,
                        not_equals: not_equals,
                        not_starts_with: not_starts_with,
                        starts_with: starts_with,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.DataResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataResource {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html#cfn-cloudtrail-trail-dataresource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html#cfn-cloudtrail-trail-dataresource-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DataResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataResource {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.EventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html) property type.
    #[derive(Debug, Default)]
    pub struct EventSelector {
        /// Property [`DataResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-dataresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_resources: Option<::ValueList<DataResource>>,
        /// Property [`ExcludeManagementEventSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-excludemanagementeventsources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_management_event_sources: Option<::ValueList<String>>,
        /// Property [`IncludeManagementEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-includemanagementevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_management_events: Option<::Value<bool>>,
        /// Property [`ReadWriteType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-readwritetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_write_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_resources) = self.data_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataResources", data_resources)?;
            }
            if let Some(ref exclude_management_event_sources) = self.exclude_management_event_sources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeManagementEventSources", exclude_management_event_sources)?;
            }
            if let Some(ref include_management_events) = self.include_management_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeManagementEvents", include_management_events)?;
            }
            if let Some(ref read_write_type) = self.read_write_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadWriteType", read_write_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_resources: Option<::ValueList<DataResource>> = None;
                    let mut exclude_management_event_sources: Option<::ValueList<String>> = None;
                    let mut include_management_events: Option<::Value<bool>> = None;
                    let mut read_write_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataResources" => {
                                data_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeManagementEventSources" => {
                                exclude_management_event_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeManagementEvents" => {
                                include_management_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadWriteType" => {
                                read_write_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventSelector {
                        data_resources: data_resources,
                        exclude_management_event_sources: exclude_management_event_sources,
                        include_management_events: include_management_events,
                        read_write_type: read_write_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.InsightSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-insightselector.html) property type.
    #[derive(Debug, Default)]
    pub struct InsightSelector {
        /// Property [`InsightType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-insightselector.html#cfn-cloudtrail-trail-insightselector-insighttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insight_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InsightSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref insight_type) = self.insight_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightType", insight_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InsightSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InsightSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InsightSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InsightSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut insight_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InsightType" => {
                                insight_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InsightSelector {
                        insight_type: insight_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
