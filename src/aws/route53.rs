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
    pub health_check_tags: Vec<self::health_check::HealthCheckTag>,
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
    pub hosted_zone_config: self::hosted_zone::HostedZoneConfig,
    /// Property `HostedZoneTags`.
    #[serde(rename="HostedZoneTags")]
    pub hosted_zone_tags: Vec<self::hosted_zone::HostedZoneTag>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `QueryLoggingConfig`.
    #[serde(rename="QueryLoggingConfig")]
    pub query_logging_config: self::hosted_zone::QueryLoggingConfig,
    /// Property `VPCs`.
    #[serde(rename="VPCs")]
    pub vp_cs: Vec<self::hosted_zone::VPC>,
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
    pub alias_target: self::record_set::AliasTarget,
    /// Property `Comment`.
    #[serde(rename="Comment")]
    pub comment: String,
    /// Property `Failover`.
    #[serde(rename="Failover")]
    pub failover: String,
    /// Property `GeoLocation`.
    #[serde(rename="GeoLocation")]
    pub geo_location: self::record_set::GeoLocation,
    /// Property `HealthCheckId`.
    #[serde(rename="HealthCheckId")]
    pub health_check_id: String,
    /// Property `HostedZoneId`.
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: String,
    /// Property `HostedZoneName`.
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Region`.
    #[serde(rename="Region")]
    pub region: String,
    /// Property `ResourceRecords`.
    #[serde(rename="ResourceRecords")]
    pub resource_records: Vec<String>,
    /// Property `SetIdentifier`.
    #[serde(rename="SetIdentifier")]
    pub set_identifier: String,
    /// Property `TTL`.
    #[serde(rename="TTL")]
    pub ttl: String,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
    /// Property `Weight`.
    #[serde(rename="Weight")]
    pub weight: u32,
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
    pub comment: String,
    /// Property `HostedZoneId`.
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: String,
    /// Property `HostedZoneName`.
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: String,
    /// Property `RecordSets`.
    #[serde(rename="RecordSets")]
    pub record_sets: Vec<self::record_set_group::RecordSet>,
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
        pub alarm_identifier: AlarmIdentifier,
        /// Property `ChildHealthChecks`.
        #[serde(rename="ChildHealthChecks")]
        pub child_health_checks: Vec<String>,
        /// Property `EnableSNI`.
        #[serde(rename="EnableSNI")]
        pub enable_sni: bool,
        /// Property `FailureThreshold`.
        #[serde(rename="FailureThreshold")]
        pub failure_threshold: u32,
        /// Property `FullyQualifiedDomainName`.
        #[serde(rename="FullyQualifiedDomainName")]
        pub fully_qualified_domain_name: String,
        /// Property `HealthThreshold`.
        #[serde(rename="HealthThreshold")]
        pub health_threshold: u32,
        /// Property `IPAddress`.
        #[serde(rename="IPAddress")]
        pub ip_address: String,
        /// Property `InsufficientDataHealthStatus`.
        #[serde(rename="InsufficientDataHealthStatus")]
        pub insufficient_data_health_status: String,
        /// Property `Inverted`.
        #[serde(rename="Inverted")]
        pub inverted: bool,
        /// Property `MeasureLatency`.
        #[serde(rename="MeasureLatency")]
        pub measure_latency: bool,
        /// Property `Port`.
        #[serde(rename="Port")]
        pub port: u32,
        /// Property `Regions`.
        #[serde(rename="Regions")]
        pub regions: Vec<String>,
        /// Property `RequestInterval`.
        #[serde(rename="RequestInterval")]
        pub request_interval: u32,
        /// Property `ResourcePath`.
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        /// Property `SearchString`.
        #[serde(rename="SearchString")]
        pub search_string: String,
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
        pub comment: String,
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
        pub evaluate_target_health: bool,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    /// The [`AWS::Route53::RecordSet.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        #[serde(rename="ContinentCode")]
        pub continent_code: String,
        /// Property `CountryCode`.
        #[serde(rename="CountryCode")]
        pub country_code: String,
        /// Property `SubdivisionCode`.
        #[serde(rename="SubdivisionCode")]
        pub subdivision_code: String,
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
        pub evaluate_target_health: bool,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    /// The [`AWS::Route53::RecordSetGroup.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoLocation {
        /// Property `ContinentCode`.
        #[serde(rename="ContinentCode")]
        pub continent_code: String,
        /// Property `CountryCode`.
        #[serde(rename="CountryCode")]
        pub country_code: String,
        /// Property `SubdivisionCode`.
        #[serde(rename="SubdivisionCode")]
        pub subdivision_code: String,
    }

    /// The [`AWS::Route53::RecordSetGroup.RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordSet {
        /// Property `AliasTarget`.
        #[serde(rename="AliasTarget")]
        pub alias_target: AliasTarget,
        /// Property `Comment`.
        #[serde(rename="Comment")]
        pub comment: String,
        /// Property `Failover`.
        #[serde(rename="Failover")]
        pub failover: String,
        /// Property `GeoLocation`.
        #[serde(rename="GeoLocation")]
        pub geo_location: GeoLocation,
        /// Property `HealthCheckId`.
        #[serde(rename="HealthCheckId")]
        pub health_check_id: String,
        /// Property `HostedZoneId`.
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
        /// Property `HostedZoneName`.
        #[serde(rename="HostedZoneName")]
        pub hosted_zone_name: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Region`.
        #[serde(rename="Region")]
        pub region: String,
        /// Property `ResourceRecords`.
        #[serde(rename="ResourceRecords")]
        pub resource_records: Vec<String>,
        /// Property `SetIdentifier`.
        #[serde(rename="SetIdentifier")]
        pub set_identifier: String,
        /// Property `TTL`.
        #[serde(rename="TTL")]
        pub ttl: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Weight`.
        #[serde(rename="Weight")]
        pub weight: u32,
    }
}
