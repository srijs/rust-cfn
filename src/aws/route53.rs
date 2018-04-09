//! Types for the `Route53` service.

/// The [`AWS::Route53::HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html) resource type.
#[derive(Debug)]
pub struct HealthCheck {
    properties: HealthCheckProperties
}

/// Properties for the `HealthCheck` resource.
#[derive(Debug)]
pub struct HealthCheckProperties {
    /// Property `HealthCheckConfig`.
    pub health_check_config: ::Value<self::health_check::HealthCheckConfig>,
    /// Property `HealthCheckTags`.
    pub health_check_tags: Option<::ValueList<self::health_check::HealthCheckTag>>,
}

impl ::serde::Serialize for HealthCheckProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckConfig", &self.health_check_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckTags", &self.health_check_tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HealthCheckProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HealthCheckProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HealthCheckProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut health_check_config = None;
                let mut health_check_tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HealthCheckConfig" => {
                            health_check_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheckTags" => {
                            health_check_tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(HealthCheckProperties {
                    health_check_config: health_check_config.ok_or(::serde::de::Error::missing_field("HealthCheckConfig"))?,
                    health_check_tags: health_check_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for HealthCheck {
    type Properties = HealthCheckProperties;
    const TYPE: &'static str = "AWS::Route53::HealthCheck";
    fn properties(&self) -> &HealthCheckProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HealthCheckProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HealthCheck {}

impl From<HealthCheckProperties> for HealthCheck {
    fn from(properties: HealthCheckProperties) -> HealthCheck {
        HealthCheck { properties }
    }
}

/// The [`AWS::Route53::HostedZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html) resource type.
#[derive(Debug)]
pub struct HostedZone {
    properties: HostedZoneProperties
}

/// Properties for the `HostedZone` resource.
#[derive(Debug)]
pub struct HostedZoneProperties {
    /// Property `HostedZoneConfig`.
    pub hosted_zone_config: Option<::Value<self::hosted_zone::HostedZoneConfig>>,
    /// Property `HostedZoneTags`.
    pub hosted_zone_tags: Option<::ValueList<self::hosted_zone::HostedZoneTag>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `QueryLoggingConfig`.
    pub query_logging_config: Option<::Value<self::hosted_zone::QueryLoggingConfig>>,
    /// Property `VPCs`.
    pub vp_cs: Option<::ValueList<self::hosted_zone::VPC>>,
}

impl ::serde::Serialize for HostedZoneProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneConfig", &self.hosted_zone_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneTags", &self.hosted_zone_tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryLoggingConfig", &self.query_logging_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCs", &self.vp_cs)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HostedZoneProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HostedZoneProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HostedZoneProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hosted_zone_config = None;
                let mut hosted_zone_tags = None;
                let mut name = None;
                let mut query_logging_config = None;
                let mut vp_cs = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HostedZoneConfig" => {
                            hosted_zone_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostedZoneTags" => {
                            hosted_zone_tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "QueryLoggingConfig" => {
                            query_logging_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VPCs" => {
                            vp_cs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(HostedZoneProperties {
                    hosted_zone_config: hosted_zone_config,
                    hosted_zone_tags: hosted_zone_tags,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    query_logging_config: query_logging_config,
                    vp_cs: vp_cs,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for HostedZone {
    type Properties = HostedZoneProperties;
    const TYPE: &'static str = "AWS::Route53::HostedZone";
    fn properties(&self) -> &HostedZoneProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HostedZoneProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HostedZone {}

impl From<HostedZoneProperties> for HostedZone {
    fn from(properties: HostedZoneProperties) -> HostedZone {
        HostedZone { properties }
    }
}

/// The [`AWS::Route53::RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) resource type.
#[derive(Debug)]
pub struct RecordSet {
    properties: RecordSetProperties
}

/// Properties for the `RecordSet` resource.
#[derive(Debug)]
pub struct RecordSetProperties {
    /// Property `AliasTarget`.
    pub alias_target: Option<::Value<self::record_set::AliasTarget>>,
    /// Property `Comment`.
    pub comment: Option<::Value<String>>,
    /// Property `Failover`.
    pub failover: Option<::Value<String>>,
    /// Property `GeoLocation`.
    pub geo_location: Option<::Value<self::record_set::GeoLocation>>,
    /// Property `HealthCheckId`.
    pub health_check_id: Option<::Value<String>>,
    /// Property `HostedZoneId`.
    pub hosted_zone_id: Option<::Value<String>>,
    /// Property `HostedZoneName`.
    pub hosted_zone_name: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Region`.
    pub region: Option<::Value<String>>,
    /// Property `ResourceRecords`.
    pub resource_records: Option<::ValueList<String>>,
    /// Property `SetIdentifier`.
    pub set_identifier: Option<::Value<String>>,
    /// Property `TTL`.
    pub ttl: Option<::Value<String>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
    /// Property `Weight`.
    pub weight: Option<::Value<u32>>,
}

impl ::serde::Serialize for RecordSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasTarget", &self.alias_target)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Failover", &self.failover)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoLocation", &self.geo_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckId", &self.health_check_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", &self.hosted_zone_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRecords", &self.resource_records)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetIdentifier", &self.set_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TTL", &self.ttl)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", &self.weight)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecordSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecordSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecordSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alias_target = None;
                let mut comment = None;
                let mut failover = None;
                let mut geo_location = None;
                let mut health_check_id = None;
                let mut hosted_zone_id = None;
                let mut hosted_zone_name = None;
                let mut name = None;
                let mut region = None;
                let mut resource_records = None;
                let mut set_identifier = None;
                let mut ttl = None;
                let mut type_ = None;
                let mut weight = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AliasTarget" => {
                            alias_target = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Comment" => {
                            comment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Failover" => {
                            failover = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GeoLocation" => {
                            geo_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheckId" => {
                            health_check_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostedZoneId" => {
                            hosted_zone_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostedZoneName" => {
                            hosted_zone_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Region" => {
                            region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResourceRecords" => {
                            resource_records = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SetIdentifier" => {
                            set_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TTL" => {
                            ttl = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Weight" => {
                            weight = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RecordSetProperties {
                    alias_target: alias_target,
                    comment: comment,
                    failover: failover,
                    geo_location: geo_location,
                    health_check_id: health_check_id,
                    hosted_zone_id: hosted_zone_id,
                    hosted_zone_name: hosted_zone_name,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    region: region,
                    resource_records: resource_records,
                    set_identifier: set_identifier,
                    ttl: ttl,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    weight: weight,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for RecordSet {
    type Properties = RecordSetProperties;
    const TYPE: &'static str = "AWS::Route53::RecordSet";
    fn properties(&self) -> &RecordSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecordSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RecordSet {}

impl From<RecordSetProperties> for RecordSet {
    fn from(properties: RecordSetProperties) -> RecordSet {
        RecordSet { properties }
    }
}

/// The [`AWS::Route53::RecordSetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html) resource type.
#[derive(Debug)]
pub struct RecordSetGroup {
    properties: RecordSetGroupProperties
}

/// Properties for the `RecordSetGroup` resource.
#[derive(Debug)]
pub struct RecordSetGroupProperties {
    /// Property `Comment`.
    pub comment: Option<::Value<String>>,
    /// Property `HostedZoneId`.
    pub hosted_zone_id: Option<::Value<String>>,
    /// Property `HostedZoneName`.
    pub hosted_zone_name: Option<::Value<String>>,
    /// Property `RecordSets`.
    pub record_sets: Option<::ValueList<self::record_set_group::RecordSet>>,
}

impl ::serde::Serialize for RecordSetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", &self.hosted_zone_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSets", &self.record_sets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecordSetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecordSetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecordSetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut comment = None;
                let mut hosted_zone_id = None;
                let mut hosted_zone_name = None;
                let mut record_sets = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Comment" => {
                            comment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostedZoneId" => {
                            hosted_zone_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostedZoneName" => {
                            hosted_zone_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RecordSets" => {
                            record_sets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RecordSetGroupProperties {
                    comment: comment,
                    hosted_zone_id: hosted_zone_id,
                    hosted_zone_name: hosted_zone_name,
                    record_sets: record_sets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for RecordSetGroup {
    type Properties = RecordSetGroupProperties;
    const TYPE: &'static str = "AWS::Route53::RecordSetGroup";
    fn properties(&self) -> &RecordSetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecordSetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RecordSetGroup {}

impl From<RecordSetGroupProperties> for RecordSetGroup {
    fn from(properties: RecordSetGroupProperties) -> RecordSetGroup {
        RecordSetGroup { properties }
    }
}

pub mod health_check {
    //! Property types for the `HealthCheck` resource.

    /// The [`AWS::Route53::HealthCheck.AlarmIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-alarmidentifier.html) property type.
    #[derive(Debug)]
    pub struct AlarmIdentifier {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Region`.
        pub region: ::Value<String>,
    }

    impl ::codec::SerializeValue for AlarmIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlarmIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlarmIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlarmIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut region = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Region" => {
                                region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AlarmIdentifier {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html) property type.
    #[derive(Debug)]
    pub struct HealthCheckConfig {
        /// Property `AlarmIdentifier`.
        pub alarm_identifier: Option<::Value<AlarmIdentifier>>,
        /// Property `ChildHealthChecks`.
        pub child_health_checks: Option<::ValueList<String>>,
        /// Property `EnableSNI`.
        pub enable_sni: Option<::Value<bool>>,
        /// Property `FailureThreshold`.
        pub failure_threshold: Option<::Value<u32>>,
        /// Property `FullyQualifiedDomainName`.
        pub fully_qualified_domain_name: Option<::Value<String>>,
        /// Property `HealthThreshold`.
        pub health_threshold: Option<::Value<u32>>,
        /// Property `IPAddress`.
        pub ip_address: Option<::Value<String>>,
        /// Property `InsufficientDataHealthStatus`.
        pub insufficient_data_health_status: Option<::Value<String>>,
        /// Property `Inverted`.
        pub inverted: Option<::Value<bool>>,
        /// Property `MeasureLatency`.
        pub measure_latency: Option<::Value<bool>>,
        /// Property `Port`.
        pub port: Option<::Value<u32>>,
        /// Property `Regions`.
        pub regions: Option<::ValueList<String>>,
        /// Property `RequestInterval`.
        pub request_interval: Option<::Value<u32>>,
        /// Property `ResourcePath`.
        pub resource_path: Option<::Value<String>>,
        /// Property `SearchString`.
        pub search_string: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheckConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmIdentifier", &self.alarm_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildHealthChecks", &self.child_health_checks)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSNI", &self.enable_sni)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureThreshold", &self.failure_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedDomainName", &self.fully_qualified_domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthThreshold", &self.health_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPAddress", &self.ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataHealthStatus", &self.insufficient_data_health_status)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inverted", &self.inverted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeasureLatency", &self.measure_latency)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regions", &self.regions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestInterval", &self.request_interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", &self.resource_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchString", &self.search_string)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_identifier = None;
                    let mut child_health_checks = None;
                    let mut enable_sni = None;
                    let mut failure_threshold = None;
                    let mut fully_qualified_domain_name = None;
                    let mut health_threshold = None;
                    let mut ip_address = None;
                    let mut insufficient_data_health_status = None;
                    let mut inverted = None;
                    let mut measure_latency = None;
                    let mut port = None;
                    let mut regions = None;
                    let mut request_interval = None;
                    let mut resource_path = None;
                    let mut search_string = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmIdentifier" => {
                                alarm_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ChildHealthChecks" => {
                                child_health_checks = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EnableSNI" => {
                                enable_sni = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FailureThreshold" => {
                                failure_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FullyQualifiedDomainName" => {
                                fully_qualified_domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HealthThreshold" => {
                                health_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IPAddress" => {
                                ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InsufficientDataHealthStatus" => {
                                insufficient_data_health_status = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Inverted" => {
                                inverted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MeasureLatency" => {
                                measure_latency = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Port" => {
                                port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Regions" => {
                                regions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequestInterval" => {
                                request_interval = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourcePath" => {
                                resource_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SearchString" => {
                                search_string = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckConfig {
                        alarm_identifier: alarm_identifier,
                        child_health_checks: child_health_checks,
                        enable_sni: enable_sni,
                        failure_threshold: failure_threshold,
                        fully_qualified_domain_name: fully_qualified_domain_name,
                        health_threshold: health_threshold,
                        ip_address: ip_address,
                        insufficient_data_health_status: insufficient_data_health_status,
                        inverted: inverted,
                        measure_latency: measure_latency,
                        port: port,
                        regions: regions,
                        request_interval: request_interval,
                        resource_path: resource_path,
                        search_string: search_string,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html) property type.
    #[derive(Debug)]
    pub struct HealthCheckTag {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheckTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod hosted_zone {
    //! Property types for the `HostedZone` resource.

    /// The [`AWS::Route53::HostedZone.HostedZoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzoneconfig.html) property type.
    #[derive(Debug)]
    pub struct HostedZoneConfig {
        /// Property `Comment`.
        pub comment: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostedZoneConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostedZoneConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostedZoneConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostedZoneConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HostedZoneConfig {
                        comment: comment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.HostedZoneTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetags.html) property type.
    #[derive(Debug)]
    pub struct HostedZoneTag {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HostedZoneTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostedZoneTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostedZoneTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostedZoneTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HostedZoneTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.QueryLoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-queryloggingconfig.html) property type.
    #[derive(Debug)]
    pub struct QueryLoggingConfig {
        /// Property `CloudWatchLogsLogGroupArn`.
        pub cloud_watch_logs_log_group_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueryLoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", &self.cloud_watch_logs_log_group_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryLoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryLoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryLoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryLoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_log_group_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsLogGroupArn" => {
                                cloud_watch_logs_log_group_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryLoggingConfig {
                        cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn.ok_or(::serde::de::Error::missing_field("CloudWatchLogsLogGroupArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.VPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone-hostedzonevpcs.html) property type.
    #[derive(Debug)]
    pub struct VPC {
        /// Property `VPCId`.
        pub vpc_id: ::Value<String>,
        /// Property `VPCRegion`.
        pub vpc_region: ::Value<String>,
    }

    impl ::codec::SerializeValue for VPC {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCId", &self.vpc_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCRegion", &self.vpc_region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VPC {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VPC, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VPC;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VPC")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vpc_id = None;
                    let mut vpc_region = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VPCId" => {
                                vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VPCRegion" => {
                                vpc_region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VPC {
                        vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VPCId"))?,
                        vpc_region: vpc_region.ok_or(::serde::de::Error::missing_field("VPCRegion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod record_set {
    //! Property types for the `RecordSet` resource.

    /// The [`AWS::Route53::RecordSet.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug)]
    pub struct AliasTarget {
        /// Property `DNSName`.
        pub dns_name: ::Value<String>,
        /// Property `EvaluateTargetHealth`.
        pub evaluate_target_health: Option<::Value<bool>>,
        /// Property `HostedZoneId`.
        pub hosted_zone_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AliasTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateTargetHealth", &self.evaluate_target_health)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AliasTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name = None;
                    let mut evaluate_target_health = None;
                    let mut hosted_zone_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EvaluateTargetHealth" => {
                                evaluate_target_health = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasTarget {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        evaluate_target_health: evaluate_target_health,
                        hosted_zone_id: hosted_zone_id.ok_or(::serde::de::Error::missing_field("HostedZoneId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSet.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        pub continent_code: Option<::Value<String>>,
        /// Property `CountryCode`.
        pub country_code: Option<::Value<String>>,
        /// Property `SubdivisionCode`.
        pub subdivision_code: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeoLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinentCode", &self.continent_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", &self.country_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubdivisionCode", &self.subdivision_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut continent_code = None;
                    let mut country_code = None;
                    let mut subdivision_code = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContinentCode" => {
                                continent_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CountryCode" => {
                                country_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubdivisionCode" => {
                                subdivision_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoLocation {
                        continent_code: continent_code,
                        country_code: country_code,
                        subdivision_code: subdivision_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod record_set_group {
    //! Property types for the `RecordSetGroup` resource.

    /// The [`AWS::Route53::RecordSetGroup.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug)]
    pub struct AliasTarget {
        /// Property `DNSName`.
        pub dns_name: ::Value<String>,
        /// Property `EvaluateTargetHealth`.
        pub evaluate_target_health: Option<::Value<bool>>,
        /// Property `HostedZoneId`.
        pub hosted_zone_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AliasTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateTargetHealth", &self.evaluate_target_health)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AliasTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name = None;
                    let mut evaluate_target_health = None;
                    let mut hosted_zone_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EvaluateTargetHealth" => {
                                evaluate_target_health = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasTarget {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        evaluate_target_health: evaluate_target_health,
                        hosted_zone_id: hosted_zone_id.ok_or(::serde::de::Error::missing_field("HostedZoneId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSetGroup.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        pub continent_code: Option<::Value<String>>,
        /// Property `CountryCode`.
        pub country_code: Option<::Value<String>>,
        /// Property `SubdivisionCode`.
        pub subdivision_code: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeoLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinentCode", &self.continent_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", &self.country_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubdivisionCode", &self.subdivision_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut continent_code = None;
                    let mut country_code = None;
                    let mut subdivision_code = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContinentCode" => {
                                continent_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CountryCode" => {
                                country_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubdivisionCode" => {
                                subdivision_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoLocation {
                        continent_code: continent_code,
                        country_code: country_code,
                        subdivision_code: subdivision_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSetGroup.RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) property type.
    #[derive(Debug)]
    pub struct RecordSet {
        /// Property `AliasTarget`.
        pub alias_target: Option<::Value<AliasTarget>>,
        /// Property `Comment`.
        pub comment: Option<::Value<String>>,
        /// Property `Failover`.
        pub failover: Option<::Value<String>>,
        /// Property `GeoLocation`.
        pub geo_location: Option<::Value<GeoLocation>>,
        /// Property `HealthCheckId`.
        pub health_check_id: Option<::Value<String>>,
        /// Property `HostedZoneId`.
        pub hosted_zone_id: Option<::Value<String>>,
        /// Property `HostedZoneName`.
        pub hosted_zone_name: Option<::Value<String>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Region`.
        pub region: Option<::Value<String>>,
        /// Property `ResourceRecords`.
        pub resource_records: Option<::ValueList<String>>,
        /// Property `SetIdentifier`.
        pub set_identifier: Option<::Value<String>>,
        /// Property `TTL`.
        pub ttl: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
        /// Property `Weight`.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RecordSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasTarget", &self.alias_target)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Failover", &self.failover)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoLocation", &self.geo_location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckId", &self.health_check_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", &self.hosted_zone_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRecords", &self.resource_records)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetIdentifier", &self.set_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TTL", &self.ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", &self.weight)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alias_target = None;
                    let mut comment = None;
                    let mut failover = None;
                    let mut geo_location = None;
                    let mut health_check_id = None;
                    let mut hosted_zone_id = None;
                    let mut hosted_zone_name = None;
                    let mut name = None;
                    let mut region = None;
                    let mut resource_records = None;
                    let mut set_identifier = None;
                    let mut ttl = None;
                    let mut type_ = None;
                    let mut weight = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AliasTarget" => {
                                alias_target = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Comment" => {
                                comment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Failover" => {
                                failover = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "GeoLocation" => {
                                geo_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HealthCheckId" => {
                                health_check_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostedZoneName" => {
                                hosted_zone_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Region" => {
                                region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourceRecords" => {
                                resource_records = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SetIdentifier" => {
                                set_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TTL" => {
                                ttl = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Weight" => {
                                weight = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordSet {
                        alias_target: alias_target,
                        comment: comment,
                        failover: failover,
                        geo_location: geo_location,
                        health_check_id: health_check_id,
                        hosted_zone_id: hosted_zone_id,
                        hosted_zone_name: hosted_zone_name,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        region: region,
                        resource_records: resource_records,
                        set_identifier: set_identifier,
                        ttl: ttl,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
