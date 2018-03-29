/// The [`AWS::Route53::HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html) resource.
#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    properties: HealthCheckProperties
}

/// Properties for the `HealthCheck` resource.
#[derive(Serialize, Deserialize)]
pub struct HealthCheckProperties {
    #[serde(rename="HealthCheckConfig")]
    pub health_check_config: self::health_check::HealthCheckConfig,
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

impl From<HealthCheckProperties> for HealthCheck {
    fn from(properties: HealthCheckProperties) -> HealthCheck {
        HealthCheck { properties }
    }
}

/// The [`AWS::Route53::HostedZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html) resource.
#[derive(Serialize, Deserialize)]
pub struct HostedZone {
    properties: HostedZoneProperties
}

/// Properties for the `HostedZone` resource.
#[derive(Serialize, Deserialize)]
pub struct HostedZoneProperties {
    #[serde(rename="HostedZoneConfig")]
    pub hosted_zone_config: self::hosted_zone::HostedZoneConfig,
    #[serde(rename="HostedZoneTags")]
    pub hosted_zone_tags: Vec<self::hosted_zone::HostedZoneTag>,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="QueryLoggingConfig")]
    pub query_logging_config: self::hosted_zone::QueryLoggingConfig,
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

impl From<HostedZoneProperties> for HostedZone {
    fn from(properties: HostedZoneProperties) -> HostedZone {
        HostedZone { properties }
    }
}

/// The [`AWS::Route53::RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct RecordSet {
    properties: RecordSetProperties
}

/// Properties for the `RecordSet` resource.
#[derive(Serialize, Deserialize)]
pub struct RecordSetProperties {
    #[serde(rename="AliasTarget")]
    pub alias_target: self::record_set::AliasTarget,
    #[serde(rename="Comment")]
    pub comment: String,
    #[serde(rename="Failover")]
    pub failover: String,
    #[serde(rename="GeoLocation")]
    pub geo_location: self::record_set::GeoLocation,
    #[serde(rename="HealthCheckId")]
    pub health_check_id: String,
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: String,
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Region")]
    pub region: String,
    #[serde(rename="ResourceRecords")]
    pub resource_records: Vec<String>,
    #[serde(rename="SetIdentifier")]
    pub set_identifier: String,
    #[serde(rename="TTL")]
    pub ttl: String,
    #[serde(rename="Type")]
    pub type_: String,
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

impl From<RecordSetProperties> for RecordSet {
    fn from(properties: RecordSetProperties) -> RecordSet {
        RecordSet { properties }
    }
}

/// The [`AWS::Route53::RecordSetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct RecordSetGroup {
    properties: RecordSetGroupProperties
}

/// Properties for the `RecordSetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct RecordSetGroupProperties {
    #[serde(rename="Comment")]
    pub comment: String,
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: String,
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: String,
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

impl From<RecordSetGroupProperties> for RecordSetGroup {
    fn from(properties: RecordSetGroupProperties) -> RecordSetGroup {
        RecordSetGroup { properties }
    }
}

pub mod health_check {
    #[derive(Serialize, Deserialize)]
    pub struct AlarmIdentifier {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Region")]
        pub region: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct HealthCheckConfig {
        #[serde(rename="AlarmIdentifier")]
        pub alarm_identifier: AlarmIdentifier,
        #[serde(rename="ChildHealthChecks")]
        pub child_health_checks: Vec<String>,
        #[serde(rename="EnableSNI")]
        pub enable_sni: bool,
        #[serde(rename="FailureThreshold")]
        pub failure_threshold: u32,
        #[serde(rename="FullyQualifiedDomainName")]
        pub fully_qualified_domain_name: String,
        #[serde(rename="HealthThreshold")]
        pub health_threshold: u32,
        #[serde(rename="IPAddress")]
        pub ip_address: String,
        #[serde(rename="InsufficientDataHealthStatus")]
        pub insufficient_data_health_status: String,
        #[serde(rename="Inverted")]
        pub inverted: bool,
        #[serde(rename="MeasureLatency")]
        pub measure_latency: bool,
        #[serde(rename="Port")]
        pub port: u32,
        #[serde(rename="Regions")]
        pub regions: Vec<String>,
        #[serde(rename="RequestInterval")]
        pub request_interval: u32,
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        #[serde(rename="SearchString")]
        pub search_string: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct HealthCheckTag {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

pub mod hosted_zone {
    #[derive(Serialize, Deserialize)]
    pub struct HostedZoneConfig {
        #[serde(rename="Comment")]
        pub comment: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct HostedZoneTag {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct QueryLoggingConfig {
        #[serde(rename="CloudWatchLogsLogGroupArn")]
        pub cloud_watch_logs_log_group_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VPC {
        #[serde(rename="VPCId")]
        pub vpc_id: String,
        #[serde(rename="VPCRegion")]
        pub vpc_region: String,
    }

}

pub mod record_set {
    #[derive(Serialize, Deserialize)]
    pub struct AliasTarget {
        #[serde(rename="DNSName")]
        pub dns_name: String,
        #[serde(rename="EvaluateTargetHealth")]
        pub evaluate_target_health: bool,
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GeoLocation {
        #[serde(rename="ContinentCode")]
        pub continent_code: String,
        #[serde(rename="CountryCode")]
        pub country_code: String,
        #[serde(rename="SubdivisionCode")]
        pub subdivision_code: String,
    }

}

pub mod record_set_group {
    #[derive(Serialize, Deserialize)]
    pub struct AliasTarget {
        #[serde(rename="DNSName")]
        pub dns_name: String,
        #[serde(rename="EvaluateTargetHealth")]
        pub evaluate_target_health: bool,
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GeoLocation {
        #[serde(rename="ContinentCode")]
        pub continent_code: String,
        #[serde(rename="CountryCode")]
        pub country_code: String,
        #[serde(rename="SubdivisionCode")]
        pub subdivision_code: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RecordSet {
        #[serde(rename="AliasTarget")]
        pub alias_target: AliasTarget,
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="Failover")]
        pub failover: String,
        #[serde(rename="GeoLocation")]
        pub geo_location: GeoLocation,
        #[serde(rename="HealthCheckId")]
        pub health_check_id: String,
        #[serde(rename="HostedZoneId")]
        pub hosted_zone_id: String,
        #[serde(rename="HostedZoneName")]
        pub hosted_zone_name: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Region")]
        pub region: String,
        #[serde(rename="ResourceRecords")]
        pub resource_records: Vec<String>,
        #[serde(rename="SetIdentifier")]
        pub set_identifier: String,
        #[serde(rename="TTL")]
        pub ttl: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Weight")]
        pub weight: u32,
    }

}

