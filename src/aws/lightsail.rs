//! Types for the `Lightsail` service.

/// The [`AWS::Lightsail::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html) resource type.
#[derive(Debug, Default)]
pub struct Alarm {
    properties: AlarmProperties
}

/// Properties for the `Alarm` resource.
#[derive(Debug, Default)]
pub struct AlarmProperties {
    /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-alarmname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alarm_name: ::Value<String>,
    /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-comparisonoperator).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub comparison_operator: ::Value<String>,
    /// Property [`ContactProtocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-contactprotocols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_protocols: Option<::ValueList<String>>,
    /// Property [`DatapointsToAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-datapointstoalarm).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub datapoints_to_alarm: Option<::Value<u32>>,
    /// Property [`EvaluationPeriods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-evaluationperiods).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluation_periods: ::Value<u32>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: ::Value<String>,
    /// Property [`MonitoredResourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-monitoredresourcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitored_resource_name: ::Value<String>,
    /// Property [`NotificationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-notificationenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_enabled: Option<::Value<bool>>,
    /// Property [`NotificationTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-notificationtriggers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_triggers: Option<::ValueList<String>>,
    /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-threshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold: ::Value<f64>,
    /// Property [`TreatMissingData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html#cfn-lightsail-alarm-treatmissingdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treat_missing_data: Option<::Value<String>>,
}

impl ::serde::Serialize for AlarmProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
        if let Some(ref contact_protocols) = self.contact_protocols {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactProtocols", contact_protocols)?;
        }
        if let Some(ref datapoints_to_alarm) = self.datapoints_to_alarm {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatapointsToAlarm", datapoints_to_alarm)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoredResourceName", &self.monitored_resource_name)?;
        if let Some(ref notification_enabled) = self.notification_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationEnabled", notification_enabled)?;
        }
        if let Some(ref notification_triggers) = self.notification_triggers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTriggers", notification_triggers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
        if let Some(ref treat_missing_data) = self.treat_missing_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatMissingData", treat_missing_data)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AlarmProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AlarmProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AlarmProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alarm_name: Option<::Value<String>> = None;
                let mut comparison_operator: Option<::Value<String>> = None;
                let mut contact_protocols: Option<::ValueList<String>> = None;
                let mut datapoints_to_alarm: Option<::Value<u32>> = None;
                let mut evaluation_periods: Option<::Value<u32>> = None;
                let mut metric_name: Option<::Value<String>> = None;
                let mut monitored_resource_name: Option<::Value<String>> = None;
                let mut notification_enabled: Option<::Value<bool>> = None;
                let mut notification_triggers: Option<::ValueList<String>> = None;
                let mut threshold: Option<::Value<f64>> = None;
                let mut treat_missing_data: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AlarmName" => {
                            alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComparisonOperator" => {
                            comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContactProtocols" => {
                            contact_protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatapointsToAlarm" => {
                            datapoints_to_alarm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluationPeriods" => {
                            evaluation_periods = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoredResourceName" => {
                            monitored_resource_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationEnabled" => {
                            notification_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTriggers" => {
                            notification_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Threshold" => {
                            threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TreatMissingData" => {
                            treat_missing_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AlarmProperties {
                    alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                    comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                    contact_protocols: contact_protocols,
                    datapoints_to_alarm: datapoints_to_alarm,
                    evaluation_periods: evaluation_periods.ok_or(::serde::de::Error::missing_field("EvaluationPeriods"))?,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    monitored_resource_name: monitored_resource_name.ok_or(::serde::de::Error::missing_field("MonitoredResourceName"))?,
                    notification_enabled: notification_enabled,
                    notification_triggers: notification_triggers,
                    threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                    treat_missing_data: treat_missing_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alarm {
    type Properties = AlarmProperties;
    const TYPE: &'static str = "AWS::Lightsail::Alarm";
    fn properties(&self) -> &AlarmProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AlarmProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alarm {}

impl From<AlarmProperties> for Alarm {
    fn from(properties: AlarmProperties) -> Alarm {
        Alarm { properties }
    }
}

/// The [`AWS::Lightsail::Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html) resource type.
#[derive(Debug, Default)]
pub struct Bucket {
    properties: BucketProperties
}

/// Properties for the `Bucket` resource.
#[derive(Debug, Default)]
pub struct BucketProperties {
    /// Property [`AccessRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-accessrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_rules: Option<::Value<self::bucket::AccessRules>>,
    /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-bucketname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_name: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: ::Value<String>,
    /// Property [`ObjectVersioning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-objectversioning).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub object_versioning: Option<::Value<bool>>,
    /// Property [`ReadOnlyAccessAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-readonlyaccessaccounts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub read_only_access_accounts: Option<::ValueList<String>>,
    /// Property [`ResourcesReceivingAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-resourcesreceivingaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources_receiving_access: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html#cfn-lightsail-bucket-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BucketProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_rules) = self.access_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessRules", access_rules)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", &self.bundle_id)?;
        if let Some(ref object_versioning) = self.object_versioning {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersioning", object_versioning)?;
        }
        if let Some(ref read_only_access_accounts) = self.read_only_access_accounts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnlyAccessAccounts", read_only_access_accounts)?;
        }
        if let Some(ref resources_receiving_access) = self.resources_receiving_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcesReceivingAccess", resources_receiving_access)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut access_rules: Option<::Value<self::bucket::AccessRules>> = None;
                let mut bucket_name: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut object_versioning: Option<::Value<bool>> = None;
                let mut read_only_access_accounts: Option<::ValueList<String>> = None;
                let mut resources_receiving_access: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessRules" => {
                            access_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketName" => {
                            bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectVersioning" => {
                            object_versioning = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReadOnlyAccessAccounts" => {
                            read_only_access_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcesReceivingAccess" => {
                            resources_receiving_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BucketProperties {
                    access_rules: access_rules,
                    bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                    bundle_id: bundle_id.ok_or(::serde::de::Error::missing_field("BundleId"))?,
                    object_versioning: object_versioning,
                    read_only_access_accounts: read_only_access_accounts,
                    resources_receiving_access: resources_receiving_access,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Bucket {
    type Properties = BucketProperties;
    const TYPE: &'static str = "AWS::Lightsail::Bucket";
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

/// The [`AWS::Lightsail::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`CertificateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html#cfn-lightsail-certificate-certificatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_name: ::Value<String>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html#cfn-lightsail-certificate-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html#cfn-lightsail-certificate-subjectalternativenames).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subject_alternative_names: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html#cfn-lightsail-certificate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateName", &self.certificate_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref subject_alternative_names) = self.subject_alternative_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_name: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut subject_alternative_names: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateName" => {
                            certificate_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubjectAlternativeNames" => {
                            subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    certificate_name: certificate_name.ok_or(::serde::de::Error::missing_field("CertificateName"))?,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    subject_alternative_names: subject_alternative_names,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::Lightsail::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::Lightsail::Container`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html) resource type.
#[derive(Debug, Default)]
pub struct Container {
    properties: ContainerProperties
}

/// Properties for the `Container` resource.
#[derive(Debug, Default)]
pub struct ContainerProperties {
    /// Property [`ContainerServiceDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-containerservicedeployment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub container_service_deployment: Option<::Value<self::container::ContainerServiceDeployment>>,
    /// Property [`IsDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-isdisabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_disabled: Option<::Value<bool>>,
    /// Property [`Power`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-power).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub power: ::Value<String>,
    /// Property [`PrivateRegistryAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-privateregistryaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub private_registry_access: Option<::Value<self::container::PrivateRegistryAccess>>,
    /// Property [`PublicDomainNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-publicdomainnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_domain_names: Option<::ValueList<self::container::PublicDomainName>>,
    /// Property [`Scale`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-scale).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scale: ::Value<u32>,
    /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-servicename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html#cfn-lightsail-container-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ContainerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref container_service_deployment) = self.container_service_deployment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerServiceDeployment", container_service_deployment)?;
        }
        if let Some(ref is_disabled) = self.is_disabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsDisabled", is_disabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Power", &self.power)?;
        if let Some(ref private_registry_access) = self.private_registry_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateRegistryAccess", private_registry_access)?;
        }
        if let Some(ref public_domain_names) = self.public_domain_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicDomainNames", public_domain_names)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scale", &self.scale)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", &self.service_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContainerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContainerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_service_deployment: Option<::Value<self::container::ContainerServiceDeployment>> = None;
                let mut is_disabled: Option<::Value<bool>> = None;
                let mut power: Option<::Value<String>> = None;
                let mut private_registry_access: Option<::Value<self::container::PrivateRegistryAccess>> = None;
                let mut public_domain_names: Option<::ValueList<self::container::PublicDomainName>> = None;
                let mut scale: Option<::Value<u32>> = None;
                let mut service_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerServiceDeployment" => {
                            container_service_deployment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsDisabled" => {
                            is_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Power" => {
                            power = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateRegistryAccess" => {
                            private_registry_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicDomainNames" => {
                            public_domain_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scale" => {
                            scale = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceName" => {
                            service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContainerProperties {
                    container_service_deployment: container_service_deployment,
                    is_disabled: is_disabled,
                    power: power.ok_or(::serde::de::Error::missing_field("Power"))?,
                    private_registry_access: private_registry_access,
                    public_domain_names: public_domain_names,
                    scale: scale.ok_or(::serde::de::Error::missing_field("Scale"))?,
                    service_name: service_name.ok_or(::serde::de::Error::missing_field("ServiceName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Container {
    type Properties = ContainerProperties;
    const TYPE: &'static str = "AWS::Lightsail::Container";
    fn properties(&self) -> &ContainerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContainerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Container {}

impl From<ContainerProperties> for Container {
    fn from(properties: ContainerProperties) -> Container {
        Container { properties }
    }
}

/// The [`AWS::Lightsail::Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html) resource type.
#[derive(Debug, Default)]
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Debug, Default)]
pub struct DatabaseProperties {
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`BackupRetention`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-backupretention).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_retention: Option<::Value<bool>>,
    /// Property [`CaCertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-cacertificateidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ca_certificate_identifier: Option<::Value<String>>,
    /// Property [`MasterDatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-masterdatabasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_database_name: ::Value<String>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<::Value<String>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: ::Value<String>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`RelationalDatabaseBlueprintId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-relationaldatabaseblueprintid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub relational_database_blueprint_id: ::Value<String>,
    /// Property [`RelationalDatabaseBundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-relationaldatabasebundleid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub relational_database_bundle_id: ::Value<String>,
    /// Property [`RelationalDatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-relationaldatabasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub relational_database_name: ::Value<String>,
    /// Property [`RelationalDatabaseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-relationaldatabaseparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub relational_database_parameters: Option<::ValueList<self::database::RelationalDatabaseParameter>>,
    /// Property [`RotateMasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-rotatemasteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rotate_master_user_password: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html#cfn-lightsail-database-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DatabaseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref backup_retention) = self.backup_retention {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetention", backup_retention)?;
        }
        if let Some(ref ca_certificate_identifier) = self.ca_certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaCertificateIdentifier", ca_certificate_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterDatabaseName", &self.master_database_name)?;
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseBlueprintId", &self.relational_database_blueprint_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseBundleId", &self.relational_database_bundle_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseName", &self.relational_database_name)?;
        if let Some(ref relational_database_parameters) = self.relational_database_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseParameters", relational_database_parameters)?;
        }
        if let Some(ref rotate_master_user_password) = self.rotate_master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotateMasterUserPassword", rotate_master_user_password)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatabaseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatabaseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatabaseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zone: Option<::Value<String>> = None;
                let mut backup_retention: Option<::Value<bool>> = None;
                let mut ca_certificate_identifier: Option<::Value<String>> = None;
                let mut master_database_name: Option<::Value<String>> = None;
                let mut master_user_password: Option<::Value<String>> = None;
                let mut master_username: Option<::Value<String>> = None;
                let mut preferred_backup_window: Option<::Value<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut relational_database_blueprint_id: Option<::Value<String>> = None;
                let mut relational_database_bundle_id: Option<::Value<String>> = None;
                let mut relational_database_name: Option<::Value<String>> = None;
                let mut relational_database_parameters: Option<::ValueList<self::database::RelationalDatabaseParameter>> = None;
                let mut rotate_master_user_password: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetention" => {
                            backup_retention = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CaCertificateIdentifier" => {
                            ca_certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterDatabaseName" => {
                            master_database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelationalDatabaseBlueprintId" => {
                            relational_database_blueprint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelationalDatabaseBundleId" => {
                            relational_database_bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelationalDatabaseName" => {
                            relational_database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelationalDatabaseParameters" => {
                            relational_database_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RotateMasterUserPassword" => {
                            rotate_master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatabaseProperties {
                    availability_zone: availability_zone,
                    backup_retention: backup_retention,
                    ca_certificate_identifier: ca_certificate_identifier,
                    master_database_name: master_database_name.ok_or(::serde::de::Error::missing_field("MasterDatabaseName"))?,
                    master_user_password: master_user_password,
                    master_username: master_username.ok_or(::serde::de::Error::missing_field("MasterUsername"))?,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    relational_database_blueprint_id: relational_database_blueprint_id.ok_or(::serde::de::Error::missing_field("RelationalDatabaseBlueprintId"))?,
                    relational_database_bundle_id: relational_database_bundle_id.ok_or(::serde::de::Error::missing_field("RelationalDatabaseBundleId"))?,
                    relational_database_name: relational_database_name.ok_or(::serde::de::Error::missing_field("RelationalDatabaseName"))?,
                    relational_database_parameters: relational_database_parameters,
                    rotate_master_user_password: rotate_master_user_password,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Database {
    type Properties = DatabaseProperties;
    const TYPE: &'static str = "AWS::Lightsail::Database";
    fn properties(&self) -> &DatabaseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatabaseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Database {}

impl From<DatabaseProperties> for Database {
    fn from(properties: DatabaseProperties) -> Database {
        Database { properties }
    }
}

/// The [`AWS::Lightsail::Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html) resource type.
#[derive(Debug, Default)]
pub struct Disk {
    properties: DiskProperties
}

/// Properties for the `Disk` resource.
#[derive(Debug, Default)]
pub struct DiskProperties {
    /// Property [`AddOns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-addons).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub add_ons: Option<::ValueList<self::disk::AddOn>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`DiskName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-diskname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub disk_name: ::Value<String>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: Option<::Value<self::disk::Location>>,
    /// Property [`SizeInGb`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-sizeingb).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub size_in_gb: ::Value<u32>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html#cfn-lightsail-disk-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DiskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref add_ons) = self.add_ons {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddOns", add_ons)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiskName", &self.disk_name)?;
        if let Some(ref location) = self.location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGb", &self.size_in_gb)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DiskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DiskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DiskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DiskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut add_ons: Option<::ValueList<self::disk::AddOn>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut disk_name: Option<::Value<String>> = None;
                let mut location: Option<::Value<self::disk::Location>> = None;
                let mut size_in_gb: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AddOns" => {
                            add_ons = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DiskName" => {
                            disk_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SizeInGb" => {
                            size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DiskProperties {
                    add_ons: add_ons,
                    availability_zone: availability_zone,
                    disk_name: disk_name.ok_or(::serde::de::Error::missing_field("DiskName"))?,
                    location: location,
                    size_in_gb: size_in_gb.ok_or(::serde::de::Error::missing_field("SizeInGb"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Disk {
    type Properties = DiskProperties;
    const TYPE: &'static str = "AWS::Lightsail::Disk";
    fn properties(&self) -> &DiskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DiskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Disk {}

impl From<DiskProperties> for Disk {
    fn from(properties: DiskProperties) -> Disk {
        Disk { properties }
    }
}

/// The [`AWS::Lightsail::Distribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html) resource type.
#[derive(Debug, Default)]
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Debug, Default)]
pub struct DistributionProperties {
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: ::Value<String>,
    /// Property [`CacheBehaviorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-cachebehaviorsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_behavior_settings: Option<::Value<self::distribution::CacheSettings>>,
    /// Property [`CacheBehaviors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-cachebehaviors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_behaviors: Option<::ValueList<self::distribution::CacheBehaviorPerPath>>,
    /// Property [`CertificateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-certificatename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_name: Option<::Value<String>>,
    /// Property [`DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-defaultcachebehavior).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_cache_behavior: ::Value<self::distribution::CacheBehavior>,
    /// Property [`DistributionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-distributionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub distribution_name: ::Value<String>,
    /// Property [`IpAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-ipaddresstype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ip_address_type: Option<::Value<String>>,
    /// Property [`IsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-isenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_enabled: Option<::Value<bool>>,
    /// Property [`Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-origin).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub origin: ::Value<self::distribution::InputOrigin>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html#cfn-lightsail-distribution-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DistributionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", &self.bundle_id)?;
        if let Some(ref cache_behavior_settings) = self.cache_behavior_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheBehaviorSettings", cache_behavior_settings)?;
        }
        if let Some(ref cache_behaviors) = self.cache_behaviors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheBehaviors", cache_behaviors)?;
        }
        if let Some(ref certificate_name) = self.certificate_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateName", certificate_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCacheBehavior", &self.default_cache_behavior)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributionName", &self.distribution_name)?;
        if let Some(ref ip_address_type) = self.ip_address_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddressType", ip_address_type)?;
        }
        if let Some(ref is_enabled) = self.is_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsEnabled", is_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Origin", &self.origin)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DistributionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DistributionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DistributionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bundle_id: Option<::Value<String>> = None;
                let mut cache_behavior_settings: Option<::Value<self::distribution::CacheSettings>> = None;
                let mut cache_behaviors: Option<::ValueList<self::distribution::CacheBehaviorPerPath>> = None;
                let mut certificate_name: Option<::Value<String>> = None;
                let mut default_cache_behavior: Option<::Value<self::distribution::CacheBehavior>> = None;
                let mut distribution_name: Option<::Value<String>> = None;
                let mut ip_address_type: Option<::Value<String>> = None;
                let mut is_enabled: Option<::Value<bool>> = None;
                let mut origin: Option<::Value<self::distribution::InputOrigin>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheBehaviorSettings" => {
                            cache_behavior_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheBehaviors" => {
                            cache_behaviors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateName" => {
                            certificate_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultCacheBehavior" => {
                            default_cache_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DistributionName" => {
                            distribution_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddressType" => {
                            ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsEnabled" => {
                            is_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Origin" => {
                            origin = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DistributionProperties {
                    bundle_id: bundle_id.ok_or(::serde::de::Error::missing_field("BundleId"))?,
                    cache_behavior_settings: cache_behavior_settings,
                    cache_behaviors: cache_behaviors,
                    certificate_name: certificate_name,
                    default_cache_behavior: default_cache_behavior.ok_or(::serde::de::Error::missing_field("DefaultCacheBehavior"))?,
                    distribution_name: distribution_name.ok_or(::serde::de::Error::missing_field("DistributionName"))?,
                    ip_address_type: ip_address_type,
                    is_enabled: is_enabled,
                    origin: origin.ok_or(::serde::de::Error::missing_field("Origin"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Distribution {
    type Properties = DistributionProperties;
    const TYPE: &'static str = "AWS::Lightsail::Distribution";
    fn properties(&self) -> &DistributionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DistributionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Distribution {}

impl From<DistributionProperties> for Distribution {
    fn from(properties: DistributionProperties) -> Distribution {
        Distribution { properties }
    }
}

/// The [`AWS::Lightsail::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html) resource type.
#[derive(Debug, Default)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Default)]
pub struct InstanceProperties {
    /// Property [`AddOns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-addons).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub add_ons: Option<::ValueList<self::instance::AddOn>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`BlueprintId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-blueprintid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub blueprint_id: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-bundleid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bundle_id: ::Value<String>,
    /// Property [`Hardware`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-hardware).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hardware: Option<::Value<self::instance::Hardware>>,
    /// Property [`InstanceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-instancename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_name: ::Value<String>,
    /// Property [`KeyPairName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-keypairname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_pair_name: Option<::Value<String>>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: Option<::Value<self::instance::Location>>,
    /// Property [`Networking`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-networking).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub networking: Option<::Value<self::instance::Networking>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<self::instance::State>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html#cfn-lightsail-instance-userdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_data: Option<::Value<String>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref add_ons) = self.add_ons {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddOns", add_ons)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlueprintId", &self.blueprint_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", &self.bundle_id)?;
        if let Some(ref hardware) = self.hardware {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hardware", hardware)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceName", &self.instance_name)?;
        if let Some(ref key_pair_name) = self.key_pair_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPairName", key_pair_name)?;
        }
        if let Some(ref location) = self.location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
        }
        if let Some(ref networking) = self.networking {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Networking", networking)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_data) = self.user_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserData", user_data)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut add_ons: Option<::ValueList<self::instance::AddOn>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut blueprint_id: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut hardware: Option<::Value<self::instance::Hardware>> = None;
                let mut instance_name: Option<::Value<String>> = None;
                let mut key_pair_name: Option<::Value<String>> = None;
                let mut location: Option<::Value<self::instance::Location>> = None;
                let mut networking: Option<::Value<self::instance::Networking>> = None;
                let mut state: Option<::Value<self::instance::State>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_data: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AddOns" => {
                            add_ons = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BlueprintId" => {
                            blueprint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Hardware" => {
                            hardware = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceName" => {
                            instance_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyPairName" => {
                            key_pair_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Networking" => {
                            networking = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserData" => {
                            user_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    add_ons: add_ons,
                    availability_zone: availability_zone,
                    blueprint_id: blueprint_id.ok_or(::serde::de::Error::missing_field("BlueprintId"))?,
                    bundle_id: bundle_id.ok_or(::serde::de::Error::missing_field("BundleId"))?,
                    hardware: hardware,
                    instance_name: instance_name.ok_or(::serde::de::Error::missing_field("InstanceName"))?,
                    key_pair_name: key_pair_name,
                    location: location,
                    networking: networking,
                    state: state,
                    tags: tags,
                    user_data: user_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::Lightsail::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::Lightsail::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html) resource type.
#[derive(Debug, Default)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug, Default)]
pub struct LoadBalancerProperties {
    /// Property [`AttachedInstances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-attachedinstances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attached_instances: Option<::ValueList<String>>,
    /// Property [`HealthCheckPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-healthcheckpath).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_path: Option<::Value<String>>,
    /// Property [`InstancePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-instanceport).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_port: ::Value<u32>,
    /// Property [`IpAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-ipaddresstype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ip_address_type: Option<::Value<String>>,
    /// Property [`LoadBalancerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-loadbalancername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub load_balancer_name: ::Value<String>,
    /// Property [`SessionStickinessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-sessionstickinessenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_stickiness_enabled: Option<::Value<bool>>,
    /// Property [`SessionStickinessLBCookieDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-sessionstickinesslbcookiedurationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_stickiness_lb_cookie_duration_seconds: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TlsPolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html#cfn-lightsail-loadbalancer-tlspolicyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tls_policy_name: Option<::Value<String>>,
}

impl ::serde::Serialize for LoadBalancerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attached_instances) = self.attached_instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachedInstances", attached_instances)?;
        }
        if let Some(ref health_check_path) = self.health_check_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckPath", health_check_path)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancePort", &self.instance_port)?;
        if let Some(ref ip_address_type) = self.ip_address_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddressType", ip_address_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", &self.load_balancer_name)?;
        if let Some(ref session_stickiness_enabled) = self.session_stickiness_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionStickinessEnabled", session_stickiness_enabled)?;
        }
        if let Some(ref session_stickiness_lb_cookie_duration_seconds) = self.session_stickiness_lb_cookie_duration_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionStickinessLBCookieDurationSeconds", session_stickiness_lb_cookie_duration_seconds)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tls_policy_name) = self.tls_policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsPolicyName", tls_policy_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoadBalancerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attached_instances: Option<::ValueList<String>> = None;
                let mut health_check_path: Option<::Value<String>> = None;
                let mut instance_port: Option<::Value<u32>> = None;
                let mut ip_address_type: Option<::Value<String>> = None;
                let mut load_balancer_name: Option<::Value<String>> = None;
                let mut session_stickiness_enabled: Option<::Value<bool>> = None;
                let mut session_stickiness_lb_cookie_duration_seconds: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tls_policy_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttachedInstances" => {
                            attached_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckPath" => {
                            health_check_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstancePort" => {
                            instance_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddressType" => {
                            ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerName" => {
                            load_balancer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SessionStickinessEnabled" => {
                            session_stickiness_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SessionStickinessLBCookieDurationSeconds" => {
                            session_stickiness_lb_cookie_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TlsPolicyName" => {
                            tls_policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoadBalancerProperties {
                    attached_instances: attached_instances,
                    health_check_path: health_check_path,
                    instance_port: instance_port.ok_or(::serde::de::Error::missing_field("InstancePort"))?,
                    ip_address_type: ip_address_type,
                    load_balancer_name: load_balancer_name.ok_or(::serde::de::Error::missing_field("LoadBalancerName"))?,
                    session_stickiness_enabled: session_stickiness_enabled,
                    session_stickiness_lb_cookie_duration_seconds: session_stickiness_lb_cookie_duration_seconds,
                    tags: tags,
                    tls_policy_name: tls_policy_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoadBalancer {
    type Properties = LoadBalancerProperties;
    const TYPE: &'static str = "AWS::Lightsail::LoadBalancer";
    fn properties(&self) -> &LoadBalancerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoadBalancerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoadBalancer {}

impl From<LoadBalancerProperties> for LoadBalancer {
    fn from(properties: LoadBalancerProperties) -> LoadBalancer {
        LoadBalancer { properties }
    }
}

/// The [`AWS::Lightsail::LoadBalancerTlsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html) resource type.
#[derive(Debug, Default)]
pub struct LoadBalancerTlsCertificate {
    properties: LoadBalancerTlsCertificateProperties
}

/// Properties for the `LoadBalancerTlsCertificate` resource.
#[derive(Debug, Default)]
pub struct LoadBalancerTlsCertificateProperties {
    /// Property [`CertificateAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-certificatealternativenames).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_alternative_names: Option<::ValueList<String>>,
    /// Property [`CertificateDomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-certificatedomainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_domain_name: ::Value<String>,
    /// Property [`CertificateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-certificatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_name: ::Value<String>,
    /// Property [`HttpsRedirectionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-httpsredirectionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub https_redirection_enabled: Option<::Value<bool>>,
    /// Property [`IsAttached`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-isattached).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_attached: Option<::Value<bool>>,
    /// Property [`LoadBalancerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html#cfn-lightsail-loadbalancertlscertificate-loadbalancername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub load_balancer_name: ::Value<String>,
}

impl ::serde::Serialize for LoadBalancerTlsCertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate_alternative_names) = self.certificate_alternative_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAlternativeNames", certificate_alternative_names)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateDomainName", &self.certificate_domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateName", &self.certificate_name)?;
        if let Some(ref https_redirection_enabled) = self.https_redirection_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpsRedirectionEnabled", https_redirection_enabled)?;
        }
        if let Some(ref is_attached) = self.is_attached {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsAttached", is_attached)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", &self.load_balancer_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerTlsCertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerTlsCertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerTlsCertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoadBalancerTlsCertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_alternative_names: Option<::ValueList<String>> = None;
                let mut certificate_domain_name: Option<::Value<String>> = None;
                let mut certificate_name: Option<::Value<String>> = None;
                let mut https_redirection_enabled: Option<::Value<bool>> = None;
                let mut is_attached: Option<::Value<bool>> = None;
                let mut load_balancer_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateAlternativeNames" => {
                            certificate_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateDomainName" => {
                            certificate_domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateName" => {
                            certificate_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HttpsRedirectionEnabled" => {
                            https_redirection_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsAttached" => {
                            is_attached = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerName" => {
                            load_balancer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoadBalancerTlsCertificateProperties {
                    certificate_alternative_names: certificate_alternative_names,
                    certificate_domain_name: certificate_domain_name.ok_or(::serde::de::Error::missing_field("CertificateDomainName"))?,
                    certificate_name: certificate_name.ok_or(::serde::de::Error::missing_field("CertificateName"))?,
                    https_redirection_enabled: https_redirection_enabled,
                    is_attached: is_attached,
                    load_balancer_name: load_balancer_name.ok_or(::serde::de::Error::missing_field("LoadBalancerName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoadBalancerTlsCertificate {
    type Properties = LoadBalancerTlsCertificateProperties;
    const TYPE: &'static str = "AWS::Lightsail::LoadBalancerTlsCertificate";
    fn properties(&self) -> &LoadBalancerTlsCertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoadBalancerTlsCertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoadBalancerTlsCertificate {}

impl From<LoadBalancerTlsCertificateProperties> for LoadBalancerTlsCertificate {
    fn from(properties: LoadBalancerTlsCertificateProperties) -> LoadBalancerTlsCertificate {
        LoadBalancerTlsCertificate { properties }
    }
}

/// The [`AWS::Lightsail::StaticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-staticip.html) resource type.
#[derive(Debug, Default)]
pub struct StaticIp {
    properties: StaticIpProperties
}

/// Properties for the `StaticIp` resource.
#[derive(Debug, Default)]
pub struct StaticIpProperties {
    /// Property [`AttachedTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-staticip.html#cfn-lightsail-staticip-attachedto).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attached_to: Option<::Value<String>>,
    /// Property [`StaticIpName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-staticip.html#cfn-lightsail-staticip-staticipname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub static_ip_name: ::Value<String>,
}

impl ::serde::Serialize for StaticIpProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attached_to) = self.attached_to {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachedTo", attached_to)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StaticIpName", &self.static_ip_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StaticIpProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StaticIpProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StaticIpProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StaticIpProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attached_to: Option<::Value<String>> = None;
                let mut static_ip_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttachedTo" => {
                            attached_to = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StaticIpName" => {
                            static_ip_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StaticIpProperties {
                    attached_to: attached_to,
                    static_ip_name: static_ip_name.ok_or(::serde::de::Error::missing_field("StaticIpName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StaticIp {
    type Properties = StaticIpProperties;
    const TYPE: &'static str = "AWS::Lightsail::StaticIp";
    fn properties(&self) -> &StaticIpProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StaticIpProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StaticIp {}

impl From<StaticIpProperties> for StaticIp {
    fn from(properties: StaticIpProperties) -> StaticIp {
        StaticIp { properties }
    }
}

pub mod bucket {
    //! Property types for the `Bucket` resource.

    /// The [`AWS::Lightsail::Bucket.AccessRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-bucket-accessrules.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessRules {
        /// Property [`AllowPublicOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-bucket-accessrules.html#cfn-lightsail-bucket-accessrules-allowpublicoverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_public_overrides: Option<::Value<bool>>,
        /// Property [`GetObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-bucket-accessrules.html#cfn-lightsail-bucket-accessrules-getobject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub get_object: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessRules {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_public_overrides) = self.allow_public_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowPublicOverrides", allow_public_overrides)?;
            }
            if let Some(ref get_object) = self.get_object {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GetObject", get_object)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessRules {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessRules, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessRules;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessRules")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_public_overrides: Option<::Value<bool>> = None;
                    let mut get_object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowPublicOverrides" => {
                                allow_public_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GetObject" => {
                                get_object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessRules {
                        allow_public_overrides: allow_public_overrides,
                        get_object: get_object,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod container {
    //! Property types for the `Container` resource.

    /// The [`AWS::Lightsail::Container.Container`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html) property type.
    #[derive(Debug, Default)]
    pub struct Container {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html#cfn-lightsail-container-container-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html#cfn-lightsail-container-container-containername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html#cfn-lightsail-container-container-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<::ValueList<EnvironmentVariable>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html#cfn-lightsail-container-container-image).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image: Option<::Value<String>>,
        /// Property [`Ports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html#cfn-lightsail-container-container-ports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ports: Option<::ValueList<PortInfo>>,
    }

    impl ::codec::SerializeValue for Container {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref image) = self.image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", image)?;
            }
            if let Some(ref ports) = self.ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ports", ports)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Container {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Container, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Container;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Container")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<::ValueList<String>> = None;
                    let mut container_name: Option<::Value<String>> = None;
                    let mut environment: Option<::ValueList<EnvironmentVariable>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut ports: Option<::ValueList<PortInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ports" => {
                                ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Container {
                        command: command,
                        container_name: container_name,
                        environment: environment,
                        image: image,
                        ports: ports,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.ContainerServiceDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-containerservicedeployment.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerServiceDeployment {
        /// Property [`Containers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-containerservicedeployment.html#cfn-lightsail-container-containerservicedeployment-containers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub containers: Option<::ValueList<Container>>,
        /// Property [`PublicEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-containerservicedeployment.html#cfn-lightsail-container-containerservicedeployment-publicendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub public_endpoint: Option<::Value<PublicEndpoint>>,
    }

    impl ::codec::SerializeValue for ContainerServiceDeployment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref containers) = self.containers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Containers", containers)?;
            }
            if let Some(ref public_endpoint) = self.public_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicEndpoint", public_endpoint)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerServiceDeployment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerServiceDeployment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerServiceDeployment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerServiceDeployment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut containers: Option<::ValueList<Container>> = None;
                    let mut public_endpoint: Option<::Value<PublicEndpoint>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Containers" => {
                                containers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PublicEndpoint" => {
                                public_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerServiceDeployment {
                        containers: containers,
                        public_endpoint: public_endpoint,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.EcrImagePullerRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-ecrimagepullerrole.html) property type.
    #[derive(Debug, Default)]
    pub struct EcrImagePullerRole {
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-ecrimagepullerrole.html#cfn-lightsail-container-ecrimagepullerrole-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: Option<::Value<bool>>,
        /// Property [`PrincipalArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-ecrimagepullerrole.html#cfn-lightsail-container-ecrimagepullerrole-principalarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EcrImagePullerRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref is_active) = self.is_active {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", is_active)?;
            }
            if let Some(ref principal_arn) = self.principal_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalArn", principal_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcrImagePullerRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcrImagePullerRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcrImagePullerRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcrImagePullerRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut principal_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrincipalArn" => {
                                principal_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcrImagePullerRole {
                        is_active: is_active,
                        principal_arn: principal_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-environmentvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentVariable {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-environmentvariable.html#cfn-lightsail-container-environmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
        /// Property [`Variable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-environmentvariable.html#cfn-lightsail-container-environmentvariable-variable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            if let Some(ref variable) = self.variable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variable", variable)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;
                    let mut variable: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variable" => {
                                variable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentVariable {
                        value: value,
                        variable: variable,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheckConfig {
        /// Property [`HealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-healthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub healthy_threshold: Option<::Value<u32>>,
        /// Property [`IntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-intervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_seconds: Option<::Value<u32>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`SuccessCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-successcodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_codes: Option<::Value<String>>,
        /// Property [`TimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-timeoutseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_seconds: Option<::Value<u32>>,
        /// Property [`UnhealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html#cfn-lightsail-container-healthcheckconfig-unhealthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unhealthy_threshold: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HealthCheckConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref healthy_threshold) = self.healthy_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", healthy_threshold)?;
            }
            if let Some(ref interval_seconds) = self.interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalSeconds", interval_seconds)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref success_codes) = self.success_codes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessCodes", success_codes)?;
            }
            if let Some(ref timeout_seconds) = self.timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutSeconds", timeout_seconds)?;
            }
            if let Some(ref unhealthy_threshold) = self.unhealthy_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", unhealthy_threshold)?;
            }
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
                    let mut healthy_threshold: Option<::Value<u32>> = None;
                    let mut interval_seconds: Option<::Value<u32>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut success_codes: Option<::Value<String>> = None;
                    let mut timeout_seconds: Option<::Value<u32>> = None;
                    let mut unhealthy_threshold: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalSeconds" => {
                                interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessCodes" => {
                                success_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutSeconds" => {
                                timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckConfig {
                        healthy_threshold: healthy_threshold,
                        interval_seconds: interval_seconds,
                        path: path,
                        success_codes: success_codes,
                        timeout_seconds: timeout_seconds,
                        unhealthy_threshold: unhealthy_threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.PortInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-portinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct PortInfo {
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-portinfo.html#cfn-lightsail-container-portinfo-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-portinfo.html#cfn-lightsail-container-portinfo-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PortInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortInfo {
                        port: port,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.PrivateRegistryAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-privateregistryaccess.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateRegistryAccess {
        /// Property [`EcrImagePullerRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-privateregistryaccess.html#cfn-lightsail-container-privateregistryaccess-ecrimagepullerrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_puller_role: Option<::Value<EcrImagePullerRole>>,
    }

    impl ::codec::SerializeValue for PrivateRegistryAccess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ecr_image_puller_role) = self.ecr_image_puller_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImagePullerRole", ecr_image_puller_role)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateRegistryAccess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateRegistryAccess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateRegistryAccess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateRegistryAccess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ecr_image_puller_role: Option<::Value<EcrImagePullerRole>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EcrImagePullerRole" => {
                                ecr_image_puller_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateRegistryAccess {
                        ecr_image_puller_role: ecr_image_puller_role,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.PublicDomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicdomainname.html) property type.
    #[derive(Debug, Default)]
    pub struct PublicDomainName {
        /// Property [`CertificateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicdomainname.html#cfn-lightsail-container-publicdomainname-certificatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_name: Option<::Value<String>>,
        /// Property [`DomainNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicdomainname.html#cfn-lightsail-container-publicdomainname-domainnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PublicDomainName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_name) = self.certificate_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateName", certificate_name)?;
            }
            if let Some(ref domain_names) = self.domain_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainNames", domain_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublicDomainName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicDomainName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublicDomainName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublicDomainName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_name: Option<::Value<String>> = None;
                    let mut domain_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateName" => {
                                certificate_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainNames" => {
                                domain_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublicDomainName {
                        certificate_name: certificate_name,
                        domain_names: domain_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Container.PublicEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct PublicEndpoint {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicendpoint.html#cfn-lightsail-container-publicendpoint-containername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicendpoint.html#cfn-lightsail-container-publicendpoint-containerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicendpoint.html#cfn-lightsail-container-publicendpoint-healthcheckconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_check_config: Option<::Value<HealthCheckConfig>>,
    }

    impl ::codec::SerializeValue for PublicEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref health_check_config) = self.health_check_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckConfig", health_check_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublicEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublicEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublicEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_name: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut health_check_config: Option<::Value<HealthCheckConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthCheckConfig" => {
                                health_check_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublicEndpoint {
                        container_name: container_name,
                        container_port: container_port,
                        health_check_config: health_check_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Lightsail::Database.RelationalDatabaseParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationalDatabaseParameter {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::Value<String>>,
        /// Property [`ApplyMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-applymethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_method: Option<::Value<String>>,
        /// Property [`ApplyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-applytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_type: Option<::Value<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`IsModifiable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-ismodifiable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_modifiable: Option<::Value<bool>>,
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: Option<::Value<String>>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html#cfn-lightsail-database-relationaldatabaseparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RelationalDatabaseParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            if let Some(ref apply_method) = self.apply_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyMethod", apply_method)?;
            }
            if let Some(ref apply_type) = self.apply_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyType", apply_type)?;
            }
            if let Some(ref data_type) = self.data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref is_modifiable) = self.is_modifiable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsModifiable", is_modifiable)?;
            }
            if let Some(ref parameter_name) = self.parameter_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", parameter_name)?;
            }
            if let Some(ref parameter_value) = self.parameter_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", parameter_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationalDatabaseParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationalDatabaseParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationalDatabaseParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationalDatabaseParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::Value<String>> = None;
                    let mut apply_method: Option<::Value<String>> = None;
                    let mut apply_type: Option<::Value<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut is_modifiable: Option<::Value<bool>> = None;
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplyMethod" => {
                                apply_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplyType" => {
                                apply_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsModifiable" => {
                                is_modifiable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationalDatabaseParameter {
                        allowed_values: allowed_values,
                        apply_method: apply_method,
                        apply_type: apply_type,
                        data_type: data_type,
                        description: description,
                        is_modifiable: is_modifiable,
                        parameter_name: parameter_name,
                        parameter_value: parameter_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod disk {
    //! Property types for the `Disk` resource.

    /// The [`AWS::Lightsail::Disk.AddOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-addon.html) property type.
    #[derive(Debug, Default)]
    pub struct AddOn {
        /// Property [`AddOnType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-addon.html#cfn-lightsail-disk-addon-addontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_on_type: ::Value<String>,
        /// Property [`AutoSnapshotAddOnRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-addon.html#cfn-lightsail-disk-addon-autosnapshotaddonrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_snapshot_add_on_request: Option<::Value<AutoSnapshotAddOn>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-addon.html#cfn-lightsail-disk-addon-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AddOn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddOnType", &self.add_on_type)?;
            if let Some(ref auto_snapshot_add_on_request) = self.auto_snapshot_add_on_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoSnapshotAddOnRequest", auto_snapshot_add_on_request)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddOn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddOn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddOn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddOn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_on_type: Option<::Value<String>> = None;
                    let mut auto_snapshot_add_on_request: Option<::Value<AutoSnapshotAddOn>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddOnType" => {
                                add_on_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoSnapshotAddOnRequest" => {
                                auto_snapshot_add_on_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AddOn {
                        add_on_type: add_on_type.ok_or(::serde::de::Error::missing_field("AddOnType"))?,
                        auto_snapshot_add_on_request: auto_snapshot_add_on_request,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Disk.AutoSnapshotAddOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-autosnapshotaddon.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoSnapshotAddOn {
        /// Property [`SnapshotTimeOfDay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-autosnapshotaddon.html#cfn-lightsail-disk-autosnapshotaddon-snapshottimeofday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_time_of_day: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AutoSnapshotAddOn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref snapshot_time_of_day) = self.snapshot_time_of_day {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotTimeOfDay", snapshot_time_of_day)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoSnapshotAddOn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoSnapshotAddOn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoSnapshotAddOn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoSnapshotAddOn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut snapshot_time_of_day: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SnapshotTimeOfDay" => {
                                snapshot_time_of_day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoSnapshotAddOn {
                        snapshot_time_of_day: snapshot_time_of_day,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Disk.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-location.html) property type.
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-location.html#cfn-lightsail-disk-location-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`RegionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-location.html#cfn-lightsail-disk-location-regionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref region_name) = self.region_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionName", region_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut region_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionName" => {
                                region_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        availability_zone: availability_zone,
                        region_name: region_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod distribution {
    //! Property types for the `Distribution` resource.

    /// The [`AWS::Lightsail::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehavior.html) property type.
    #[derive(Debug, Default)]
    pub struct CacheBehavior {
        /// Property [`Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehavior.html#cfn-lightsail-distribution-cachebehavior-behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref behavior) = self.behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Behavior", behavior)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CacheBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut behavior: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Behavior" => {
                                behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheBehavior {
                        behavior: behavior,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.CacheBehaviorPerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehaviorperpath.html) property type.
    #[derive(Debug, Default)]
    pub struct CacheBehaviorPerPath {
        /// Property [`Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehaviorperpath.html#cfn-lightsail-distribution-cachebehaviorperpath-behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehaviorperpath.html#cfn-lightsail-distribution-cachebehaviorperpath-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CacheBehaviorPerPath {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref behavior) = self.behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Behavior", behavior)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CacheBehaviorPerPath {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheBehaviorPerPath, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheBehaviorPerPath;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheBehaviorPerPath")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut behavior: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Behavior" => {
                                behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheBehaviorPerPath {
                        behavior: behavior,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.CacheSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct CacheSettings {
        /// Property [`AllowedHTTPMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-allowedhttpmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_http_methods: Option<::Value<String>>,
        /// Property [`CachedHTTPMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-cachedhttpmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cached_http_methods: Option<::Value<String>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: Option<::Value<u32>>,
        /// Property [`ForwardedCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-forwardedcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_cookies: Option<::Value<CookieObject>>,
        /// Property [`ForwardedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-forwardedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_headers: Option<::Value<HeaderObject>>,
        /// Property [`ForwardedQueryStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-forwardedquerystrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_query_strings: Option<::Value<QueryStringObject>>,
        /// Property [`MaximumTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-maximumttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_ttl: Option<::Value<u32>>,
        /// Property [`MinimumTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html#cfn-lightsail-distribution-cachesettings-minimumttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_ttl: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CacheSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_http_methods) = self.allowed_http_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedHTTPMethods", allowed_http_methods)?;
            }
            if let Some(ref cached_http_methods) = self.cached_http_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachedHTTPMethods", cached_http_methods)?;
            }
            if let Some(ref default_ttl) = self.default_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", default_ttl)?;
            }
            if let Some(ref forwarded_cookies) = self.forwarded_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedCookies", forwarded_cookies)?;
            }
            if let Some(ref forwarded_headers) = self.forwarded_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedHeaders", forwarded_headers)?;
            }
            if let Some(ref forwarded_query_strings) = self.forwarded_query_strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedQueryStrings", forwarded_query_strings)?;
            }
            if let Some(ref maximum_ttl) = self.maximum_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumTTL", maximum_ttl)?;
            }
            if let Some(ref minimum_ttl) = self.minimum_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumTTL", minimum_ttl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CacheSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_http_methods: Option<::Value<String>> = None;
                    let mut cached_http_methods: Option<::Value<String>> = None;
                    let mut default_ttl: Option<::Value<u32>> = None;
                    let mut forwarded_cookies: Option<::Value<CookieObject>> = None;
                    let mut forwarded_headers: Option<::Value<HeaderObject>> = None;
                    let mut forwarded_query_strings: Option<::Value<QueryStringObject>> = None;
                    let mut maximum_ttl: Option<::Value<u32>> = None;
                    let mut minimum_ttl: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedHTTPMethods" => {
                                allowed_http_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachedHTTPMethods" => {
                                cached_http_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedCookies" => {
                                forwarded_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedHeaders" => {
                                forwarded_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedQueryStrings" => {
                                forwarded_query_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumTTL" => {
                                maximum_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumTTL" => {
                                minimum_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheSettings {
                        allowed_http_methods: allowed_http_methods,
                        cached_http_methods: cached_http_methods,
                        default_ttl: default_ttl,
                        forwarded_cookies: forwarded_cookies,
                        forwarded_headers: forwarded_headers,
                        forwarded_query_strings: forwarded_query_strings,
                        maximum_ttl: maximum_ttl,
                        minimum_ttl: minimum_ttl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.CookieObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cookieobject.html) property type.
    #[derive(Debug, Default)]
    pub struct CookieObject {
        /// Property [`CookiesAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cookieobject.html#cfn-lightsail-distribution-cookieobject-cookiesallowlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies_allow_list: Option<::ValueList<String>>,
        /// Property [`Option`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cookieobject.html#cfn-lightsail-distribution-cookieobject-option).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CookieObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookies_allow_list) = self.cookies_allow_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookiesAllowList", cookies_allow_list)?;
            }
            if let Some(ref option) = self.option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Option", option)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookieObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookieObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookieObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookieObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookies_allow_list: Option<::ValueList<String>> = None;
                    let mut option: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookiesAllowList" => {
                                cookies_allow_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Option" => {
                                option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookieObject {
                        cookies_allow_list: cookies_allow_list,
                        option: option,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.HeaderObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-headerobject.html) property type.
    #[derive(Debug, Default)]
    pub struct HeaderObject {
        /// Property [`HeadersAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-headerobject.html#cfn-lightsail-distribution-headerobject-headersallowlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers_allow_list: Option<::ValueList<String>>,
        /// Property [`Option`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-headerobject.html#cfn-lightsail-distribution-headerobject-option).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HeaderObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref headers_allow_list) = self.headers_allow_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeadersAllowList", headers_allow_list)?;
            }
            if let Some(ref option) = self.option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Option", option)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeaderObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeaderObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeaderObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeaderObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut headers_allow_list: Option<::ValueList<String>> = None;
                    let mut option: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeadersAllowList" => {
                                headers_allow_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Option" => {
                                option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeaderObject {
                        headers_allow_list: headers_allow_list,
                        option: option,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.InputOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-inputorigin.html) property type.
    #[derive(Debug, Default)]
    pub struct InputOrigin {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-inputorigin.html#cfn-lightsail-distribution-inputorigin-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-inputorigin.html#cfn-lightsail-distribution-inputorigin-protocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol_policy: Option<::Value<String>>,
        /// Property [`RegionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-inputorigin.html#cfn-lightsail-distribution-inputorigin-regionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputOrigin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref protocol_policy) = self.protocol_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolPolicy", protocol_policy)?;
            }
            if let Some(ref region_name) = self.region_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionName", region_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputOrigin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputOrigin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputOrigin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputOrigin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut protocol_policy: Option<::Value<String>> = None;
                    let mut region_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProtocolPolicy" => {
                                protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionName" => {
                                region_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputOrigin {
                        name: name,
                        protocol_policy: protocol_policy,
                        region_name: region_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Distribution.QueryStringObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-querystringobject.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringObject {
        /// Property [`Option`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-querystringobject.html#cfn-lightsail-distribution-querystringobject-option).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option: Option<::Value<bool>>,
        /// Property [`QueryStringsAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-querystringobject.html#cfn-lightsail-distribution-querystringobject-querystringsallowlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings_allow_list: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for QueryStringObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref option) = self.option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Option", option)?;
            }
            if let Some(ref query_strings_allow_list) = self.query_strings_allow_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringsAllowList", query_strings_allow_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryStringObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut option: Option<::Value<bool>> = None;
                    let mut query_strings_allow_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Option" => {
                                option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringsAllowList" => {
                                query_strings_allow_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringObject {
                        option: option,
                        query_strings_allow_list: query_strings_allow_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::Lightsail::Instance.AddOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-addon.html) property type.
    #[derive(Debug, Default)]
    pub struct AddOn {
        /// Property [`AddOnType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-addon.html#cfn-lightsail-instance-addon-addontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_on_type: ::Value<String>,
        /// Property [`AutoSnapshotAddOnRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-addon.html#cfn-lightsail-instance-addon-autosnapshotaddonrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_snapshot_add_on_request: Option<::Value<AutoSnapshotAddOn>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-addon.html#cfn-lightsail-instance-addon-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AddOn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddOnType", &self.add_on_type)?;
            if let Some(ref auto_snapshot_add_on_request) = self.auto_snapshot_add_on_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoSnapshotAddOnRequest", auto_snapshot_add_on_request)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddOn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddOn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddOn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddOn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_on_type: Option<::Value<String>> = None;
                    let mut auto_snapshot_add_on_request: Option<::Value<AutoSnapshotAddOn>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddOnType" => {
                                add_on_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoSnapshotAddOnRequest" => {
                                auto_snapshot_add_on_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AddOn {
                        add_on_type: add_on_type.ok_or(::serde::de::Error::missing_field("AddOnType"))?,
                        auto_snapshot_add_on_request: auto_snapshot_add_on_request,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.AutoSnapshotAddOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-autosnapshotaddon.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoSnapshotAddOn {
        /// Property [`SnapshotTimeOfDay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-autosnapshotaddon.html#cfn-lightsail-instance-autosnapshotaddon-snapshottimeofday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_time_of_day: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AutoSnapshotAddOn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref snapshot_time_of_day) = self.snapshot_time_of_day {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotTimeOfDay", snapshot_time_of_day)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoSnapshotAddOn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoSnapshotAddOn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoSnapshotAddOn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoSnapshotAddOn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut snapshot_time_of_day: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SnapshotTimeOfDay" => {
                                snapshot_time_of_day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoSnapshotAddOn {
                        snapshot_time_of_day: snapshot_time_of_day,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html) property type.
    #[derive(Debug, Default)]
    pub struct Disk {
        /// Property [`AttachedTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-attachedto).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attached_to: Option<::Value<String>>,
        /// Property [`AttachmentState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-attachmentstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_state: Option<::Value<String>>,
        /// Property [`DiskName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-diskname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disk_name: ::Value<String>,
        /// Property [`IOPS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`IsSystemDisk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-issystemdisk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_system_disk: Option<::Value<bool>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
        /// Property [`SizeInGb`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html#cfn-lightsail-instance-disk-sizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_gb: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Disk {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attached_to) = self.attached_to {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachedTo", attached_to)?;
            }
            if let Some(ref attachment_state) = self.attachment_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentState", attachment_state)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiskName", &self.disk_name)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IOPS", iops)?;
            }
            if let Some(ref is_system_disk) = self.is_system_disk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsSystemDisk", is_system_disk)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            if let Some(ref size_in_gb) = self.size_in_gb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGb", size_in_gb)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Disk {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Disk, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Disk;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Disk")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attached_to: Option<::Value<String>> = None;
                    let mut attachment_state: Option<::Value<String>> = None;
                    let mut disk_name: Option<::Value<String>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut is_system_disk: Option<::Value<bool>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut size_in_gb: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachedTo" => {
                                attached_to = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttachmentState" => {
                                attachment_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiskName" => {
                                disk_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IOPS" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsSystemDisk" => {
                                is_system_disk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInGb" => {
                                size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Disk {
                        attached_to: attached_to,
                        attachment_state: attachment_state,
                        disk_name: disk_name.ok_or(::serde::de::Error::missing_field("DiskName"))?,
                        iops: iops,
                        is_system_disk: is_system_disk,
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                        size_in_gb: size_in_gb,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.Hardware`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-hardware.html) property type.
    #[derive(Debug, Default)]
    pub struct Hardware {
        /// Property [`CpuCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-hardware.html#cfn-lightsail-instance-hardware-cpucount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu_count: Option<::Value<u32>>,
        /// Property [`Disks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-hardware.html#cfn-lightsail-instance-hardware-disks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disks: Option<::ValueList<Disk>>,
        /// Property [`RamSizeInGb`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-hardware.html#cfn-lightsail-instance-hardware-ramsizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ram_size_in_gb: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Hardware {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cpu_count) = self.cpu_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuCount", cpu_count)?;
            }
            if let Some(ref disks) = self.disks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Disks", disks)?;
            }
            if let Some(ref ram_size_in_gb) = self.ram_size_in_gb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RamSizeInGb", ram_size_in_gb)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Hardware {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Hardware, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Hardware;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Hardware")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_count: Option<::Value<u32>> = None;
                    let mut disks: Option<::ValueList<Disk>> = None;
                    let mut ram_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuCount" => {
                                cpu_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Disks" => {
                                disks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RamSizeInGb" => {
                                ram_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Hardware {
                        cpu_count: cpu_count,
                        disks: disks,
                        ram_size_in_gb: ram_size_in_gb,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-location.html) property type.
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-location.html#cfn-lightsail-instance-location-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`RegionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-location.html#cfn-lightsail-instance-location-regionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref region_name) = self.region_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionName", region_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut region_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionName" => {
                                region_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        availability_zone: availability_zone,
                        region_name: region_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.MonthlyTransfer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-monthlytransfer.html) property type.
    #[derive(Debug, Default)]
    pub struct MonthlyTransfer {
        /// Property [`GbPerMonthAllocated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-monthlytransfer.html#cfn-lightsail-instance-monthlytransfer-gbpermonthallocated).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gb_per_month_allocated: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MonthlyTransfer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref gb_per_month_allocated) = self.gb_per_month_allocated {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GbPerMonthAllocated", gb_per_month_allocated)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonthlyTransfer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonthlyTransfer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonthlyTransfer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonthlyTransfer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gb_per_month_allocated: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GbPerMonthAllocated" => {
                                gb_per_month_allocated = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonthlyTransfer {
                        gb_per_month_allocated: gb_per_month_allocated,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.Networking`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-networking.html) property type.
    #[derive(Debug, Default)]
    pub struct Networking {
        /// Property [`MonthlyTransfer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-networking.html#cfn-lightsail-instance-networking-monthlytransfer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monthly_transfer: Option<::Value<MonthlyTransfer>>,
        /// Property [`Ports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-networking.html#cfn-lightsail-instance-networking-ports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ports: ::ValueList<Port>,
    }

    impl ::codec::SerializeValue for Networking {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref monthly_transfer) = self.monthly_transfer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonthlyTransfer", monthly_transfer)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ports", &self.ports)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Networking {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Networking, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Networking;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Networking")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut monthly_transfer: Option<::Value<MonthlyTransfer>> = None;
                    let mut ports: Option<::ValueList<Port>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MonthlyTransfer" => {
                                monthly_transfer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ports" => {
                                ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Networking {
                        monthly_transfer: monthly_transfer,
                        ports: ports.ok_or(::serde::de::Error::missing_field("Ports"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html) property type.
    #[derive(Debug, Default)]
    pub struct Port {
        /// Property [`AccessDirection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-accessdirection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_direction: Option<::Value<String>>,
        /// Property [`AccessFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-accessfrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_from: Option<::Value<String>>,
        /// Property [`AccessType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-accesstype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_type: Option<::Value<String>>,
        /// Property [`CidrListAliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-cidrlistaliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidr_list_aliases: Option<::ValueList<String>>,
        /// Property [`Cidrs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-cidrs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidrs: Option<::ValueList<String>>,
        /// Property [`CommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-commonname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub common_name: Option<::Value<String>>,
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: Option<::Value<u32>>,
        /// Property [`Ipv6Cidrs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-ipv6cidrs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6_cidrs: Option<::ValueList<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html#cfn-lightsail-instance-port-toport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub to_port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Port {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_direction) = self.access_direction {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessDirection", access_direction)?;
            }
            if let Some(ref access_from) = self.access_from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessFrom", access_from)?;
            }
            if let Some(ref access_type) = self.access_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessType", access_type)?;
            }
            if let Some(ref cidr_list_aliases) = self.cidr_list_aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrListAliases", cidr_list_aliases)?;
            }
            if let Some(ref cidrs) = self.cidrs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cidrs", cidrs)?;
            }
            if let Some(ref common_name) = self.common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommonName", common_name)?;
            }
            if let Some(ref from_port) = self.from_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", from_port)?;
            }
            if let Some(ref ipv6_cidrs) = self.ipv6_cidrs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Cidrs", ipv6_cidrs)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref to_port) = self.to_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", to_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Port {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Port, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Port;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Port")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_direction: Option<::Value<String>> = None;
                    let mut access_from: Option<::Value<String>> = None;
                    let mut access_type: Option<::Value<String>> = None;
                    let mut cidr_list_aliases: Option<::ValueList<String>> = None;
                    let mut cidrs: Option<::ValueList<String>> = None;
                    let mut common_name: Option<::Value<String>> = None;
                    let mut from_port: Option<::Value<u32>> = None;
                    let mut ipv6_cidrs: Option<::ValueList<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut to_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessDirection" => {
                                access_direction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessFrom" => {
                                access_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessType" => {
                                access_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CidrListAliases" => {
                                cidr_list_aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cidrs" => {
                                cidrs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CommonName" => {
                                common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FromPort" => {
                                from_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ipv6Cidrs" => {
                                ipv6_cidrs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ToPort" => {
                                to_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Port {
                        access_direction: access_direction,
                        access_from: access_from,
                        access_type: access_type,
                        cidr_list_aliases: cidr_list_aliases,
                        cidrs: cidrs,
                        common_name: common_name,
                        from_port: from_port,
                        ipv6_cidrs: ipv6_cidrs,
                        protocol: protocol,
                        to_port: to_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lightsail::Instance.State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-state.html) property type.
    #[derive(Debug, Default)]
    pub struct State {
        /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-state.html#cfn-lightsail-instance-state-code).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-state.html#cfn-lightsail-instance-state-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for State {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code) = self.code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", code)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for State {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<State, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = State;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type State")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Code" => {
                                code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(State {
                        code: code,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
