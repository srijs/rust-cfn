//! Types for the `Route53` service.

/// The [`AWS::Route53::HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html) resource type.
#[derive(Debug)]
pub struct HealthCheck {
    properties: HealthCheckProperties
}

/// Properties for the `HealthCheck` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckProperties {
    /// Property `HealthCheckConfig`.
    #[serde(rename="HealthCheckConfig")]
    pub health_check_config: self::health_check::HealthCheckConfig,
    /// Property `HealthCheckTags`.
    #[serde(rename="HealthCheckTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_tags: Option<Vec<self::health_check::HealthCheckTag>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct HostedZoneProperties {
    /// Property `HostedZoneConfig`.
    #[serde(rename="HostedZoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_config: Option<self::hosted_zone::HostedZoneConfig>,
    /// Property `HostedZoneTags`.
    #[serde(rename="HostedZoneTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_tags: Option<Vec<self::hosted_zone::HostedZoneTag>>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `QueryLoggingConfig`.
    #[serde(rename="QueryLoggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_config: Option<self::hosted_zone::QueryLoggingConfig>,
    /// Property `VPCs`.
    #[serde(rename="VPCs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_cs: Option<Vec<self::hosted_zone::VPC>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordSetProperties {
    /// Property `AliasTarget`.
    #[serde(rename="AliasTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_target: Option<self::record_set::AliasTarget>,
    /// Property `Comment`.
    #[serde(rename="Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Property `Failover`.
    #[serde(rename="Failover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<String>,
    /// Property `GeoLocation`.
    #[serde(rename="GeoLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<self::record_set::GeoLocation>,
    /// Property `HealthCheckId`.
    #[serde(rename="HealthCheckId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<String>,
    /// Property `HostedZoneId`.
    #[serde(rename="HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// Property `HostedZoneName`.
    #[serde(rename="HostedZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_name: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Region`.
    #[serde(rename="Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Property `ResourceRecords`.
    #[serde(rename="ResourceRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_records: Option<Vec<String>>,
    /// Property `SetIdentifier`.
    #[serde(rename="SetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_identifier: Option<String>,
    /// Property `TTL`.
    #[serde(rename="TTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
    /// Property `Weight`.
    #[serde(rename="Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordSetGroupProperties {
    /// Property `Comment`.
    #[serde(rename="Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Property `HostedZoneId`.
    #[serde(rename="HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// Property `HostedZoneName`.
    #[serde(rename="HostedZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_name: Option<String>,
    /// Property `RecordSets`.
    #[serde(rename="RecordSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_sets: Option<Vec<self::record_set_group::RecordSet>>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AlarmIdentifier {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Region`.
        #[serde(rename="Region")]
        pub region: String,
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HealthCheckConfig {
        /// Property `AlarmIdentifier`.
        #[serde(rename="AlarmIdentifier")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alarm_identifier: Option<AlarmIdentifier>,
        /// Property `ChildHealthChecks`.
        #[serde(rename="ChildHealthChecks")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub child_health_checks: Option<Vec<String>>,
        /// Property `EnableSNI`.
        #[serde(rename="EnableSNI")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_sni: Option<bool>,
        /// Property `FailureThreshold`.
        #[serde(rename="FailureThreshold")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub failure_threshold: Option<u32>,
        /// Property `FullyQualifiedDomainName`.
        #[serde(rename="FullyQualifiedDomainName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fully_qualified_domain_name: Option<String>,
        /// Property `HealthThreshold`.
        #[serde(rename="HealthThreshold")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub health_threshold: Option<u32>,
        /// Property `IPAddress`.
        #[serde(rename="IPAddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        /// Property `InsufficientDataHealthStatus`.
        #[serde(rename="InsufficientDataHealthStatus")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub insufficient_data_health_status: Option<String>,
        /// Property `Inverted`.
        #[serde(rename="Inverted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inverted: Option<bool>,
        /// Property `MeasureLatency`.
        #[serde(rename="MeasureLatency")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub measure_latency: Option<bool>,
        /// Property `Port`.
        #[serde(rename="Port")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<u32>,
        /// Property `Regions`.
        #[serde(rename="Regions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub regions: Option<Vec<String>>,
        /// Property `RequestInterval`.
        #[serde(rename="RequestInterval")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub request_interval: Option<u32>,
        /// Property `ResourcePath`.
        #[serde(rename="ResourcePath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_path: Option<String>,
        /// Property `SearchString`.
        #[serde(rename="SearchString")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search_string: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HealthCheckTag {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}

pub mod hosted_zone {
    //! Property types for the `HostedZone` resource.

    /// The [`AWS::Route53::HostedZone.HostedZoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzoneconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostedZoneConfig {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
    }

    /// The [`AWS::Route53::HostedZone.HostedZoneTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetags.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostedZoneTag {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::Route53::HostedZone.QueryLoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-queryloggingconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueryLoggingConfig {
        /// Property `CloudWatchLogsLogGroupArn`.
        #[serde(rename="CloudWatchLogsLogGroupArn")]
        pub cloud_watch_logs_log_group_arn: String,
    }

    /// The [`AWS::Route53::HostedZone.VPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone-hostedzonevpcs.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VPC {
        /// Property `VPCId`.
        #[serde(rename="VPCId")]
        pub vpc_id: String,
        /// Property `VPCRegion`.
        #[serde(rename="VPCRegion")]
        pub vpc_region: String,
    }
}

pub mod record_set {
    //! Property types for the `RecordSet` resource.

    /// The [`AWS::Route53::RecordSet.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AliasTarget {
        /// Property `DNSName`.
        #[serde(rename="DNSName")]
        pub dns_name: String,
        /// Property `EvaluateTargetHealth`.
        #[serde(rename="EvaluateTargetHealth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub evaluate_target_health: Option<bool>,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    /// The [`AWS::Route53::RecordSet.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        #[serde(rename="ContinentCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continent_code: Option<String>,
        /// Property `CountryCode`.
        #[serde(rename="CountryCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country_code: Option<String>,
        /// Property `SubdivisionCode`.
        #[serde(rename="SubdivisionCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subdivision_code: Option<String>,
    }
}

pub mod record_set_group {
    //! Property types for the `RecordSetGroup` resource.

    /// The [`AWS::Route53::RecordSetGroup.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AliasTarget {
        /// Property `DNSName`.
        #[serde(rename="DNSName")]
        pub dns_name: String,
        /// Property `EvaluateTargetHealth`.
        #[serde(rename="EvaluateTargetHealth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub evaluate_target_health: Option<bool>,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    /// The [`AWS::Route53::RecordSetGroup.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        #[serde(rename="ContinentCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continent_code: Option<String>,
        /// Property `CountryCode`.
        #[serde(rename="CountryCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country_code: Option<String>,
        /// Property `SubdivisionCode`.
        #[serde(rename="SubdivisionCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subdivision_code: Option<String>,
    }

    /// The [`AWS::Route53::RecordSetGroup.RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordSet {
        /// Property `AliasTarget`.
        #[serde(rename="AliasTarget")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alias_target: Option<AliasTarget>,
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        /// Property `Failover`.
        #[serde(rename="Failover")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub failover: Option<String>,
        /// Property `GeoLocation`.
        #[serde(rename="GeoLocation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geo_location: Option<GeoLocation>,
        /// Property `HealthCheckId`.
        #[serde(rename="HealthCheckId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub health_check_id: Option<String>,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hosted_zone_id: Option<String>,
        /// Property `HostedZoneName`.
        #[serde(rename="HostedZoneName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hosted_zone_name: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Region`.
        #[serde(rename="Region")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        /// Property `ResourceRecords`.
        #[serde(rename="ResourceRecords")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_records: Option<Vec<String>>,
        /// Property `SetIdentifier`.
        #[serde(rename="SetIdentifier")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub set_identifier: Option<String>,
        /// Property `TTL`.
        #[serde(rename="TTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ttl: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Weight`.
        #[serde(rename="Weight")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub weight: Option<u32>,
    }
}
