//! Types for the `DMS` service.

/// The [`AWS::DMS::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html) resource type.
#[derive(Debug)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug)]
pub struct CertificateProperties {
    /// Property `CertificateIdentifier`.
    pub certificate_identifier: Option<::Value<String>>,
    /// Property `CertificatePem`.
    pub certificate_pem: Option<::Value<String>>,
    /// Property `CertificateWallet`.
    pub certificate_wallet: Option<::Value<String>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateIdentifier", &self.certificate_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificatePem", &self.certificate_pem)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateWallet", &self.certificate_wallet)?;
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
                let mut certificate_identifier = None;
                let mut certificate_pem = None;
                let mut certificate_wallet = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateIdentifier" => {
                            certificate_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CertificatePem" => {
                            certificate_pem = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CertificateWallet" => {
                            certificate_wallet = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    certificate_identifier: certificate_identifier,
                    certificate_pem: certificate_pem,
                    certificate_wallet: certificate_wallet,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::DMS::Certificate";
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

/// The [`AWS::DMS::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html) resource type.
#[derive(Debug)]
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Debug)]
pub struct EndpointProperties {
    /// Property `CertificateArn`.
    pub certificate_arn: Option<::Value<String>>,
    /// Property `DatabaseName`.
    pub database_name: Option<::Value<String>>,
    /// Property `DynamoDbSettings`.
    pub dynamo_db_settings: Option<::Value<self::endpoint::DynamoDbSettings>>,
    /// Property `EndpointIdentifier`.
    pub endpoint_identifier: Option<::Value<String>>,
    /// Property `EndpointType`.
    pub endpoint_type: ::Value<String>,
    /// Property `EngineName`.
    pub engine_name: ::Value<String>,
    /// Property `ExtraConnectionAttributes`.
    pub extra_connection_attributes: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MongoDbSettings`.
    pub mongo_db_settings: Option<::Value<self::endpoint::MongoDbSettings>>,
    /// Property `Password`.
    pub password: Option<::Value<String>>,
    /// Property `Port`.
    pub port: Option<::Value<u32>>,
    /// Property `S3Settings`.
    pub s3_settings: Option<::Value<self::endpoint::S3Settings>>,
    /// Property `ServerName`.
    pub server_name: Option<::Value<String>>,
    /// Property `SslMode`.
    pub ssl_mode: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Username`.
    pub username: Option<::Value<String>>,
}

impl ::serde::Serialize for EndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDbSettings", &self.dynamo_db_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointIdentifier", &self.endpoint_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", &self.endpoint_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineName", &self.engine_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraConnectionAttributes", &self.extra_connection_attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MongoDbSettings", &self.mongo_db_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Settings", &self.s3_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", &self.server_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", &self.ssl_mode)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_arn = None;
                let mut database_name = None;
                let mut dynamo_db_settings = None;
                let mut endpoint_identifier = None;
                let mut endpoint_type = None;
                let mut engine_name = None;
                let mut extra_connection_attributes = None;
                let mut kms_key_id = None;
                let mut mongo_db_settings = None;
                let mut password = None;
                let mut port = None;
                let mut s3_settings = None;
                let mut server_name = None;
                let mut ssl_mode = None;
                let mut tags = None;
                let mut username = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateArn" => {
                            certificate_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DatabaseName" => {
                            database_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DynamoDbSettings" => {
                            dynamo_db_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EndpointIdentifier" => {
                            endpoint_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EndpointType" => {
                            endpoint_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineName" => {
                            engine_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ExtraConnectionAttributes" => {
                            extra_connection_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MongoDbSettings" => {
                            mongo_db_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Password" => {
                            password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3Settings" => {
                            s3_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServerName" => {
                            server_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SslMode" => {
                            ssl_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Username" => {
                            username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EndpointProperties {
                    certificate_arn: certificate_arn,
                    database_name: database_name,
                    dynamo_db_settings: dynamo_db_settings,
                    endpoint_identifier: endpoint_identifier,
                    endpoint_type: endpoint_type.ok_or(::serde::de::Error::missing_field("EndpointType"))?,
                    engine_name: engine_name.ok_or(::serde::de::Error::missing_field("EngineName"))?,
                    extra_connection_attributes: extra_connection_attributes,
                    kms_key_id: kms_key_id,
                    mongo_db_settings: mongo_db_settings,
                    password: password,
                    port: port,
                    s3_settings: s3_settings,
                    server_name: server_name,
                    ssl_mode: ssl_mode,
                    tags: tags,
                    username: username,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Endpoint {
    type Properties = EndpointProperties;
    const TYPE: &'static str = "AWS::DMS::Endpoint";
    fn properties(&self) -> &EndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Endpoint {}

impl From<EndpointProperties> for Endpoint {
    fn from(properties: EndpointProperties) -> Endpoint {
        Endpoint { properties }
    }
}

/// The [`AWS::DMS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html) resource type.
#[derive(Debug)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug)]
pub struct EventSubscriptionProperties {
    /// Property `Enabled`.
    pub enabled: Option<::Value<bool>>,
    /// Property `EventCategories`.
    pub event_categories: Option<::ValueList<String>>,
    /// Property `SnsTopicArn`.
    pub sns_topic_arn: ::Value<String>,
    /// Property `SourceIds`.
    pub source_ids: Option<::ValueList<String>>,
    /// Property `SourceType`.
    pub source_type: Option<::Value<String>>,
    /// Property `SubscriptionName`.
    pub subscription_name: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EventSubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventCategories", &self.event_categories)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", &self.source_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", &self.source_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionName", &self.subscription_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled = None;
                let mut event_categories = None;
                let mut sns_topic_arn = None;
                let mut source_ids = None;
                let mut source_type = None;
                let mut subscription_name = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventCategories" => {
                            event_categories = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceIds" => {
                            source_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceType" => {
                            source_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubscriptionName" => {
                            subscription_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EventSubscriptionProperties {
                    enabled: enabled,
                    event_categories: event_categories,
                    sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                    source_ids: source_ids,
                    source_type: source_type,
                    subscription_name: subscription_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::DMS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::DMS::ReplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html) resource type.
#[derive(Debug)]
pub struct ReplicationInstance {
    properties: ReplicationInstanceProperties
}

/// Properties for the `ReplicationInstance` resource.
#[derive(Debug)]
pub struct ReplicationInstanceProperties {
    /// Property `AllocatedStorage`.
    pub allocated_storage: Option<::Value<u32>>,
    /// Property `AllowMajorVersionUpgrade`.
    pub allow_major_version_upgrade: Option<::Value<bool>>,
    /// Property `AutoMinorVersionUpgrade`.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `EngineVersion`.
    pub engine_version: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MultiAZ`.
    pub multi_az: Option<::Value<bool>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `ReplicationInstanceClass`.
    pub replication_instance_class: ::Value<String>,
    /// Property `ReplicationInstanceIdentifier`.
    pub replication_instance_identifier: Option<::Value<String>>,
    /// Property `ReplicationSubnetGroupIdentifier`.
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ReplicationInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", &self.allocated_storage)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMajorVersionUpgrade", &self.allow_major_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", &self.auto_minor_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", &self.multi_az)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", &self.publicly_accessible)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceClass", &self.replication_instance_class)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceIdentifier", &self.replication_instance_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupIdentifier", &self.replication_subnet_group_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", &self.vpc_security_group_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_storage = None;
                let mut allow_major_version_upgrade = None;
                let mut auto_minor_version_upgrade = None;
                let mut availability_zone = None;
                let mut engine_version = None;
                let mut kms_key_id = None;
                let mut multi_az = None;
                let mut preferred_maintenance_window = None;
                let mut publicly_accessible = None;
                let mut replication_instance_class = None;
                let mut replication_instance_identifier = None;
                let mut replication_subnet_group_identifier = None;
                let mut tags = None;
                let mut vpc_security_group_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AllowMajorVersionUpgrade" => {
                            allow_major_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineVersion" => {
                            engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MultiAZ" => {
                            multi_az = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationInstanceClass" => {
                            replication_instance_class = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationInstanceIdentifier" => {
                            replication_instance_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationSubnetGroupIdentifier" => {
                            replication_subnet_group_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationInstanceProperties {
                    allocated_storage: allocated_storage,
                    allow_major_version_upgrade: allow_major_version_upgrade,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zone: availability_zone,
                    engine_version: engine_version,
                    kms_key_id: kms_key_id,
                    multi_az: multi_az,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    replication_instance_class: replication_instance_class.ok_or(::serde::de::Error::missing_field("ReplicationInstanceClass"))?,
                    replication_instance_identifier: replication_instance_identifier,
                    replication_subnet_group_identifier: replication_subnet_group_identifier,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReplicationInstance {
    type Properties = ReplicationInstanceProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationInstance";
    fn properties(&self) -> &ReplicationInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationInstance {}

impl From<ReplicationInstanceProperties> for ReplicationInstance {
    fn from(properties: ReplicationInstanceProperties) -> ReplicationInstance {
        ReplicationInstance { properties }
    }
}

/// The [`AWS::DMS::ReplicationSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html) resource type.
#[derive(Debug)]
pub struct ReplicationSubnetGroup {
    properties: ReplicationSubnetGroupProperties
}

/// Properties for the `ReplicationSubnetGroup` resource.
#[derive(Debug)]
pub struct ReplicationSubnetGroupProperties {
    /// Property `ReplicationSubnetGroupDescription`.
    pub replication_subnet_group_description: ::Value<String>,
    /// Property `ReplicationSubnetGroupIdentifier`.
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property `SubnetIds`.
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ReplicationSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupDescription", &self.replication_subnet_group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupIdentifier", &self.replication_subnet_group_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut replication_subnet_group_description = None;
                let mut replication_subnet_group_identifier = None;
                let mut subnet_ids = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ReplicationSubnetGroupDescription" => {
                            replication_subnet_group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationSubnetGroupIdentifier" => {
                            replication_subnet_group_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetIds" => {
                            subnet_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationSubnetGroupProperties {
                    replication_subnet_group_description: replication_subnet_group_description.ok_or(::serde::de::Error::missing_field("ReplicationSubnetGroupDescription"))?,
                    replication_subnet_group_identifier: replication_subnet_group_identifier,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReplicationSubnetGroup {
    type Properties = ReplicationSubnetGroupProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationSubnetGroup";
    fn properties(&self) -> &ReplicationSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationSubnetGroup {}

impl From<ReplicationSubnetGroupProperties> for ReplicationSubnetGroup {
    fn from(properties: ReplicationSubnetGroupProperties) -> ReplicationSubnetGroup {
        ReplicationSubnetGroup { properties }
    }
}

/// The [`AWS::DMS::ReplicationTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html) resource type.
#[derive(Debug)]
pub struct ReplicationTask {
    properties: ReplicationTaskProperties
}

/// Properties for the `ReplicationTask` resource.
#[derive(Debug)]
pub struct ReplicationTaskProperties {
    /// Property `CdcStartTime`.
    pub cdc_start_time: Option<::Value<f64>>,
    /// Property `MigrationType`.
    pub migration_type: ::Value<String>,
    /// Property `ReplicationInstanceArn`.
    pub replication_instance_arn: ::Value<String>,
    /// Property `ReplicationTaskIdentifier`.
    pub replication_task_identifier: Option<::Value<String>>,
    /// Property `ReplicationTaskSettings`.
    pub replication_task_settings: Option<::Value<String>>,
    /// Property `SourceEndpointArn`.
    pub source_endpoint_arn: ::Value<String>,
    /// Property `TableMappings`.
    pub table_mappings: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TargetEndpointArn`.
    pub target_endpoint_arn: ::Value<String>,
}

impl ::serde::Serialize for ReplicationTaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcStartTime", &self.cdc_start_time)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MigrationType", &self.migration_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceArn", &self.replication_instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationTaskIdentifier", &self.replication_task_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationTaskSettings", &self.replication_task_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEndpointArn", &self.source_endpoint_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableMappings", &self.table_mappings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEndpointArn", &self.target_endpoint_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationTaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationTaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationTaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationTaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cdc_start_time = None;
                let mut migration_type = None;
                let mut replication_instance_arn = None;
                let mut replication_task_identifier = None;
                let mut replication_task_settings = None;
                let mut source_endpoint_arn = None;
                let mut table_mappings = None;
                let mut tags = None;
                let mut target_endpoint_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CdcStartTime" => {
                            cdc_start_time = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MigrationType" => {
                            migration_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationInstanceArn" => {
                            replication_instance_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationTaskIdentifier" => {
                            replication_task_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationTaskSettings" => {
                            replication_task_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceEndpointArn" => {
                            source_endpoint_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TableMappings" => {
                            table_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetEndpointArn" => {
                            target_endpoint_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationTaskProperties {
                    cdc_start_time: cdc_start_time,
                    migration_type: migration_type.ok_or(::serde::de::Error::missing_field("MigrationType"))?,
                    replication_instance_arn: replication_instance_arn.ok_or(::serde::de::Error::missing_field("ReplicationInstanceArn"))?,
                    replication_task_identifier: replication_task_identifier,
                    replication_task_settings: replication_task_settings,
                    source_endpoint_arn: source_endpoint_arn.ok_or(::serde::de::Error::missing_field("SourceEndpointArn"))?,
                    table_mappings: table_mappings.ok_or(::serde::de::Error::missing_field("TableMappings"))?,
                    tags: tags,
                    target_endpoint_arn: target_endpoint_arn.ok_or(::serde::de::Error::missing_field("TargetEndpointArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReplicationTask {
    type Properties = ReplicationTaskProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationTask";
    fn properties(&self) -> &ReplicationTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationTask {}

impl From<ReplicationTaskProperties> for ReplicationTask {
    fn from(properties: ReplicationTaskProperties) -> ReplicationTask {
        ReplicationTask { properties }
    }
}

pub mod endpoint {
    //! Property types for the `Endpoint` resource.

    /// The [`AWS::DMS::Endpoint.DynamoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html) property type.
    #[derive(Debug)]
    pub struct DynamoDbSettings {
        /// Property `ServiceAccessRoleArn`.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DynamoDbSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", &self.service_access_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDbSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDbSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDbSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDbSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut service_access_role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDbSettings {
                        service_access_role_arn: service_access_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Debug)]
    pub struct MongoDbSettings {
        /// Property `AuthMechanism`.
        pub auth_mechanism: Option<::Value<String>>,
        /// Property `AuthSource`.
        pub auth_source: Option<::Value<String>>,
        /// Property `AuthType`.
        pub auth_type: Option<::Value<String>>,
        /// Property `DatabaseName`.
        pub database_name: Option<::Value<String>>,
        /// Property `DocsToInvestigate`.
        pub docs_to_investigate: Option<::Value<String>>,
        /// Property `ExtractDocId`.
        pub extract_doc_id: Option<::Value<String>>,
        /// Property `NestingLevel`.
        pub nesting_level: Option<::Value<String>>,
        /// Property `Password`.
        pub password: Option<::Value<String>>,
        /// Property `Port`.
        pub port: Option<::Value<u32>>,
        /// Property `ServerName`.
        pub server_name: Option<::Value<String>>,
        /// Property `Username`.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MongoDbSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthMechanism", &self.auth_mechanism)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthSource", &self.auth_source)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", &self.auth_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocsToInvestigate", &self.docs_to_investigate)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtractDocId", &self.extract_doc_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NestingLevel", &self.nesting_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", &self.server_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MongoDbSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MongoDbSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MongoDbSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MongoDbSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_mechanism = None;
                    let mut auth_source = None;
                    let mut auth_type = None;
                    let mut database_name = None;
                    let mut docs_to_investigate = None;
                    let mut extract_doc_id = None;
                    let mut nesting_level = None;
                    let mut password = None;
                    let mut port = None;
                    let mut server_name = None;
                    let mut username = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthMechanism" => {
                                auth_mechanism = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "AuthSource" => {
                                auth_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "AuthType" => {
                                auth_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DatabaseName" => {
                                database_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DocsToInvestigate" => {
                                docs_to_investigate = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExtractDocId" => {
                                extract_doc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NestingLevel" => {
                                nesting_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Password" => {
                                password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Port" => {
                                port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ServerName" => {
                                server_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Username" => {
                                username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MongoDbSettings {
                        auth_mechanism: auth_mechanism,
                        auth_source: auth_source,
                        auth_type: auth_type,
                        database_name: database_name,
                        docs_to_investigate: docs_to_investigate,
                        extract_doc_id: extract_doc_id,
                        nesting_level: nesting_level,
                        password: password,
                        port: port,
                        server_name: server_name,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug)]
    pub struct S3Settings {
        /// Property `BucketFolder`.
        pub bucket_folder: Option<::Value<String>>,
        /// Property `BucketName`.
        pub bucket_name: Option<::Value<String>>,
        /// Property `CompressionType`.
        pub compression_type: Option<::Value<String>>,
        /// Property `CsvDelimiter`.
        pub csv_delimiter: Option<::Value<String>>,
        /// Property `CsvRowDelimiter`.
        pub csv_row_delimiter: Option<::Value<String>>,
        /// Property `ExternalTableDefinition`.
        pub external_table_definition: Option<::Value<String>>,
        /// Property `ServiceAccessRoleArn`.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketFolder", &self.bucket_folder)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionType", &self.compression_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvDelimiter", &self.csv_delimiter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvRowDelimiter", &self.csv_row_delimiter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalTableDefinition", &self.external_table_definition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", &self.service_access_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_folder = None;
                    let mut bucket_name = None;
                    let mut compression_type = None;
                    let mut csv_delimiter = None;
                    let mut csv_row_delimiter = None;
                    let mut external_table_definition = None;
                    let mut service_access_role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketFolder" => {
                                bucket_folder = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BucketName" => {
                                bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CompressionType" => {
                                compression_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CsvDelimiter" => {
                                csv_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CsvRowDelimiter" => {
                                csv_row_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExternalTableDefinition" => {
                                external_table_definition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Settings {
                        bucket_folder: bucket_folder,
                        bucket_name: bucket_name,
                        compression_type: compression_type,
                        csv_delimiter: csv_delimiter,
                        csv_row_delimiter: csv_row_delimiter,
                        external_table_definition: external_table_definition,
                        service_access_role_arn: service_access_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
