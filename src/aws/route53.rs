/// The [`AWS::Route53::HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html) resource.
#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    properties: HealthCheckProperties
}

/// Properties for the `HealthCheck` resource.
#[derive(Serialize, Deserialize)]
pub struct HealthCheckProperties {
    #[serde(rename="HealthCheckConfig")]
    pub health_check_config: (),
    #[serde(rename="HealthCheckTags")]
    pub health_check_tags: (),
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
    pub hosted_zone_config: (),
    #[serde(rename="HostedZoneTags")]
    pub hosted_zone_tags: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="QueryLoggingConfig")]
    pub query_logging_config: (),
    #[serde(rename="VPCs")]
    pub vp_cs: (),
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
    pub alias_target: (),
    #[serde(rename="Comment")]
    pub comment: (),
    #[serde(rename="Failover")]
    pub failover: (),
    #[serde(rename="GeoLocation")]
    pub geo_location: (),
    #[serde(rename="HealthCheckId")]
    pub health_check_id: (),
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: (),
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Region")]
    pub region: (),
    #[serde(rename="ResourceRecords")]
    pub resource_records: (),
    #[serde(rename="SetIdentifier")]
    pub set_identifier: (),
    #[serde(rename="TTL")]
    pub ttl: (),
    #[serde(rename="Type")]
    pub type_: (),
    #[serde(rename="Weight")]
    pub weight: (),
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
    pub comment: (),
    #[serde(rename="HostedZoneId")]
    pub hosted_zone_id: (),
    #[serde(rename="HostedZoneName")]
    pub hosted_zone_name: (),
    #[serde(rename="RecordSets")]
    pub record_sets: (),
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

