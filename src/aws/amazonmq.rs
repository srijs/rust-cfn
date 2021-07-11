//! Types for the `AmazonMQ` service.

/// The [`AWS::AmazonMQ::Broker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html) resource type.
#[derive(Debug, Default)]
pub struct Broker {
    properties: BrokerProperties
}

/// Properties for the `Broker` resource.
#[derive(Debug, Default)]
pub struct BrokerProperties {
    /// Property [`AuthenticationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-authenticationstrategy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authentication_strategy: Option<::Value<String>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: ::Value<bool>,
    /// Property [`BrokerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-brokername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub broker_name: ::Value<String>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<self::broker::ConfigurationId>>,
    /// Property [`DeploymentMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-deploymentmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub deployment_mode: ::Value<String>,
    /// Property [`EncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-encryptionoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_options: Option<::Value<self::broker::EncryptionOptions>>,
    /// Property [`EngineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-enginetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_type: ::Value<String>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: ::Value<String>,
    /// Property [`HostInstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-hostinstancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub host_instance_type: ::Value<String>,
    /// Property [`LdapServerMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-ldapservermetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ldap_server_metadata: Option<::Value<self::broker::LdapServerMetadata>>,
    /// Property [`Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-logs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logs: Option<::Value<self::broker::LogList>>,
    /// Property [`MaintenanceWindowStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-maintenancewindowstarttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maintenance_window_start_time: Option<::Value<self::broker::MaintenanceWindow>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-publiclyaccessible).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub publicly_accessible: ::Value<bool>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-securitygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_groups: Option<::ValueList<String>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-storagetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_type: Option<::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::broker::TagsEntry>>,
    /// Property [`Users`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html#cfn-amazonmq-broker-users).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub users: ::ValueList<self::broker::User>,
}

impl ::serde::Serialize for BrokerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref authentication_strategy) = self.authentication_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationStrategy", authentication_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", &self.auto_minor_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrokerName", &self.broker_name)?;
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentMode", &self.deployment_mode)?;
        if let Some(ref encryption_options) = self.encryption_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionOptions", encryption_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineType", &self.engine_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostInstanceType", &self.host_instance_type)?;
        if let Some(ref ldap_server_metadata) = self.ldap_server_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LdapServerMetadata", ldap_server_metadata)?;
        }
        if let Some(ref logs) = self.logs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logs", logs)?;
        }
        if let Some(ref maintenance_window_start_time) = self.maintenance_window_start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowStartTime", maintenance_window_start_time)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", &self.publicly_accessible)?;
        if let Some(ref security_groups) = self.security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
        }
        if let Some(ref storage_type) = self.storage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", storage_type)?;
        }
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", &self.users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BrokerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BrokerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BrokerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BrokerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authentication_strategy: Option<::Value<String>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut broker_name: Option<::Value<String>> = None;
                let mut configuration: Option<::Value<self::broker::ConfigurationId>> = None;
                let mut deployment_mode: Option<::Value<String>> = None;
                let mut encryption_options: Option<::Value<self::broker::EncryptionOptions>> = None;
                let mut engine_type: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut host_instance_type: Option<::Value<String>> = None;
                let mut ldap_server_metadata: Option<::Value<self::broker::LdapServerMetadata>> = None;
                let mut logs: Option<::Value<self::broker::LogList>> = None;
                let mut maintenance_window_start_time: Option<::Value<self::broker::MaintenanceWindow>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut security_groups: Option<::ValueList<String>> = None;
                let mut storage_type: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<self::broker::TagsEntry>> = None;
                let mut users: Option<::ValueList<self::broker::User>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthenticationStrategy" => {
                            authentication_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BrokerName" => {
                            broker_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeploymentMode" => {
                            deployment_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionOptions" => {
                            encryption_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineType" => {
                            engine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostInstanceType" => {
                            host_instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LdapServerMetadata" => {
                            ldap_server_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Logs" => {
                            logs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaintenanceWindowStartTime" => {
                            maintenance_window_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Users" => {
                            users = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BrokerProperties {
                    authentication_strategy: authentication_strategy,
                    auto_minor_version_upgrade: auto_minor_version_upgrade.ok_or(::serde::de::Error::missing_field("AutoMinorVersionUpgrade"))?,
                    broker_name: broker_name.ok_or(::serde::de::Error::missing_field("BrokerName"))?,
                    configuration: configuration,
                    deployment_mode: deployment_mode.ok_or(::serde::de::Error::missing_field("DeploymentMode"))?,
                    encryption_options: encryption_options,
                    engine_type: engine_type.ok_or(::serde::de::Error::missing_field("EngineType"))?,
                    engine_version: engine_version.ok_or(::serde::de::Error::missing_field("EngineVersion"))?,
                    host_instance_type: host_instance_type.ok_or(::serde::de::Error::missing_field("HostInstanceType"))?,
                    ldap_server_metadata: ldap_server_metadata,
                    logs: logs,
                    maintenance_window_start_time: maintenance_window_start_time,
                    publicly_accessible: publicly_accessible.ok_or(::serde::de::Error::missing_field("PubliclyAccessible"))?,
                    security_groups: security_groups,
                    storage_type: storage_type,
                    subnet_ids: subnet_ids,
                    tags: tags,
                    users: users.ok_or(::serde::de::Error::missing_field("Users"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Broker {
    type Properties = BrokerProperties;
    const TYPE: &'static str = "AWS::AmazonMQ::Broker";
    fn properties(&self) -> &BrokerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BrokerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Broker {}

impl From<BrokerProperties> for Broker {
    fn from(properties: BrokerProperties) -> Broker {
        Broker { properties }
    }
}

/// The [`AWS::AmazonMQ::Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html) resource type.
#[derive(Debug, Default)]
pub struct Configuration {
    properties: ConfigurationProperties
}

/// Properties for the `Configuration` resource.
#[derive(Debug, Default)]
pub struct ConfigurationProperties {
    /// Property [`AuthenticationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-authenticationstrategy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authentication_strategy: Option<::Value<String>>,
    /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-data).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EngineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-enginetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_type: ::Value<String>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-engineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_version: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html#cfn-amazonmq-configuration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::configuration::TagsEntry>>,
}

impl ::serde::Serialize for ConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref authentication_strategy) = self.authentication_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationStrategy", authentication_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", &self.data)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineType", &self.engine_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authentication_strategy: Option<::Value<String>> = None;
                let mut data: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut engine_type: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<self::configuration::TagsEntry>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthenticationStrategy" => {
                            authentication_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Data" => {
                            data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineType" => {
                            engine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationProperties {
                    authentication_strategy: authentication_strategy,
                    data: data.ok_or(::serde::de::Error::missing_field("Data"))?,
                    description: description,
                    engine_type: engine_type.ok_or(::serde::de::Error::missing_field("EngineType"))?,
                    engine_version: engine_version.ok_or(::serde::de::Error::missing_field("EngineVersion"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Configuration {
    type Properties = ConfigurationProperties;
    const TYPE: &'static str = "AWS::AmazonMQ::Configuration";
    fn properties(&self) -> &ConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Configuration {}

impl From<ConfigurationProperties> for Configuration {
    fn from(properties: ConfigurationProperties) -> Configuration {
        Configuration { properties }
    }
}

/// The [`AWS::AmazonMQ::ConfigurationAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configurationassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationAssociation {
    properties: ConfigurationAssociationProperties
}

/// Properties for the `ConfigurationAssociation` resource.
#[derive(Debug, Default)]
pub struct ConfigurationAssociationProperties {
    /// Property [`Broker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configurationassociation.html#cfn-amazonmq-configurationassociation-broker).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub broker: ::Value<String>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configurationassociation.html#cfn-amazonmq-configurationassociation-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: ::Value<self::configuration_association::ConfigurationId>,
}

impl ::serde::Serialize for ConfigurationAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Broker", &self.broker)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut broker: Option<::Value<String>> = None;
                let mut configuration: Option<::Value<self::configuration_association::ConfigurationId>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Broker" => {
                            broker = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationAssociationProperties {
                    broker: broker.ok_or(::serde::de::Error::missing_field("Broker"))?,
                    configuration: configuration.ok_or(::serde::de::Error::missing_field("Configuration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationAssociation {
    type Properties = ConfigurationAssociationProperties;
    const TYPE: &'static str = "AWS::AmazonMQ::ConfigurationAssociation";
    fn properties(&self) -> &ConfigurationAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationAssociation {}

impl From<ConfigurationAssociationProperties> for ConfigurationAssociation {
    fn from(properties: ConfigurationAssociationProperties) -> ConfigurationAssociation {
        ConfigurationAssociation { properties }
    }
}

pub mod broker {
    //! Property types for the `Broker` resource.

    /// The [`AWS::AmazonMQ::Broker.ConfigurationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-configurationid.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationId {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-configurationid.html#cfn-amazonmq-broker-configurationid-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-configurationid.html#cfn-amazonmq-broker-configurationid-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ConfigurationId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationId {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.EncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-encryptionoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionOptions {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-encryptionoptions.html#cfn-amazonmq-broker-encryptionoptions-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`UseAwsOwnedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-encryptionoptions.html#cfn-amazonmq-broker-encryptionoptions-useawsownedkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_aws_owned_key: ::Value<bool>,
    }

    impl ::codec::SerializeValue for EncryptionOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseAwsOwnedKey", &self.use_aws_owned_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut use_aws_owned_key: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseAwsOwnedKey" => {
                                use_aws_owned_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionOptions {
                        kms_key_id: kms_key_id,
                        use_aws_owned_key: use_aws_owned_key.ok_or(::serde::de::Error::missing_field("UseAwsOwnedKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.LdapServerMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct LdapServerMetadata {
        /// Property [`Hosts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-hosts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosts: ::ValueList<String>,
        /// Property [`RoleBase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-rolebase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_base: ::Value<String>,
        /// Property [`RoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-rolename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_name: Option<::Value<String>>,
        /// Property [`RoleSearchMatching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-rolesearchmatching).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_search_matching: ::Value<String>,
        /// Property [`RoleSearchSubtree`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-rolesearchsubtree).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_search_subtree: Option<::Value<bool>>,
        /// Property [`ServiceAccountPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-serviceaccountpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_account_password: ::Value<String>,
        /// Property [`ServiceAccountUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-serviceaccountusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_account_username: ::Value<String>,
        /// Property [`UserBase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-userbase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_base: ::Value<String>,
        /// Property [`UserRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-userrolename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_role_name: Option<::Value<String>>,
        /// Property [`UserSearchMatching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-usersearchmatching).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_search_matching: ::Value<String>,
        /// Property [`UserSearchSubtree`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html#cfn-amazonmq-broker-ldapservermetadata-usersearchsubtree).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_search_subtree: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LdapServerMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hosts", &self.hosts)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleBase", &self.role_base)?;
            if let Some(ref role_name) = self.role_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleName", role_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleSearchMatching", &self.role_search_matching)?;
            if let Some(ref role_search_subtree) = self.role_search_subtree {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleSearchSubtree", role_search_subtree)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccountPassword", &self.service_account_password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccountUsername", &self.service_account_username)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserBase", &self.user_base)?;
            if let Some(ref user_role_name) = self.user_role_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserRoleName", user_role_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSearchMatching", &self.user_search_matching)?;
            if let Some(ref user_search_subtree) = self.user_search_subtree {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSearchSubtree", user_search_subtree)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LdapServerMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LdapServerMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LdapServerMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LdapServerMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hosts: Option<::ValueList<String>> = None;
                    let mut role_base: Option<::Value<String>> = None;
                    let mut role_name: Option<::Value<String>> = None;
                    let mut role_search_matching: Option<::Value<String>> = None;
                    let mut role_search_subtree: Option<::Value<bool>> = None;
                    let mut service_account_password: Option<::Value<String>> = None;
                    let mut service_account_username: Option<::Value<String>> = None;
                    let mut user_base: Option<::Value<String>> = None;
                    let mut user_role_name: Option<::Value<String>> = None;
                    let mut user_search_matching: Option<::Value<String>> = None;
                    let mut user_search_subtree: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hosts" => {
                                hosts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleBase" => {
                                role_base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleName" => {
                                role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleSearchMatching" => {
                                role_search_matching = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleSearchSubtree" => {
                                role_search_subtree = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccountPassword" => {
                                service_account_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccountUsername" => {
                                service_account_username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserBase" => {
                                user_base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserRoleName" => {
                                user_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserSearchMatching" => {
                                user_search_matching = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserSearchSubtree" => {
                                user_search_subtree = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LdapServerMetadata {
                        hosts: hosts.ok_or(::serde::de::Error::missing_field("Hosts"))?,
                        role_base: role_base.ok_or(::serde::de::Error::missing_field("RoleBase"))?,
                        role_name: role_name,
                        role_search_matching: role_search_matching.ok_or(::serde::de::Error::missing_field("RoleSearchMatching"))?,
                        role_search_subtree: role_search_subtree,
                        service_account_password: service_account_password.ok_or(::serde::de::Error::missing_field("ServiceAccountPassword"))?,
                        service_account_username: service_account_username.ok_or(::serde::de::Error::missing_field("ServiceAccountUsername"))?,
                        user_base: user_base.ok_or(::serde::de::Error::missing_field("UserBase"))?,
                        user_role_name: user_role_name,
                        user_search_matching: user_search_matching.ok_or(::serde::de::Error::missing_field("UserSearchMatching"))?,
                        user_search_subtree: user_search_subtree,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.LogList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-loglist.html) property type.
    #[derive(Debug, Default)]
    pub struct LogList {
        /// Property [`Audit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-loglist.html#cfn-amazonmq-broker-loglist-audit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audit: Option<::Value<bool>>,
        /// Property [`General`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-loglist.html#cfn-amazonmq-broker-loglist-general).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub general: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LogList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audit) = self.audit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Audit", audit)?;
            }
            if let Some(ref general) = self.general {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "General", general)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audit: Option<::Value<bool>> = None;
                    let mut general: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Audit" => {
                                audit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "General" => {
                                general = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogList {
                        audit: audit,
                        general: general,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.MaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-maintenancewindow.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindow {
        /// Property [`DayOfWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-maintenancewindow.html#cfn-amazonmq-broker-maintenancewindow-dayofweek).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day_of_week: ::Value<String>,
        /// Property [`TimeOfDay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-maintenancewindow.html#cfn-amazonmq-broker-maintenancewindow-timeofday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_of_day: ::Value<String>,
        /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-maintenancewindow.html#cfn-amazonmq-broker-maintenancewindow-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_zone: ::Value<String>,
    }

    impl ::codec::SerializeValue for MaintenanceWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfWeek", &self.day_of_week)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeOfDay", &self.time_of_day)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", &self.time_zone)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day_of_week: Option<::Value<String>> = None;
                    let mut time_of_day: Option<::Value<String>> = None;
                    let mut time_zone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DayOfWeek" => {
                                day_of_week = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeOfDay" => {
                                time_of_day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeZone" => {
                                time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindow {
                        day_of_week: day_of_week.ok_or(::serde::de::Error::missing_field("DayOfWeek"))?,
                        time_of_day: time_of_day.ok_or(::serde::de::Error::missing_field("TimeOfDay"))?,
                        time_zone: time_zone.ok_or(::serde::de::Error::missing_field("TimeZone"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.TagsEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-tagsentry.html) property type.
    #[derive(Debug, Default)]
    pub struct TagsEntry {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-tagsentry.html#cfn-amazonmq-broker-tagsentry-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-tagsentry.html#cfn-amazonmq-broker-tagsentry-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TagsEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagsEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagsEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagsEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagsEntry")
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

                    Ok(TagsEntry {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmazonMQ::Broker.User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html) property type.
    #[derive(Debug, Default)]
    pub struct User {
        /// Property [`ConsoleAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html#cfn-amazonmq-broker-user-consoleaccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub console_access: Option<::Value<bool>>,
        /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html#cfn-amazonmq-broker-user-groups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub groups: Option<::ValueList<String>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html#cfn-amazonmq-broker-user-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html#cfn-amazonmq-broker-user-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for User {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref console_access) = self.console_access {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsoleAccess", console_access)?;
            }
            if let Some(ref groups) = self.groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for User {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<User, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = User;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type User")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut console_access: Option<::Value<bool>> = None;
                    let mut groups: Option<::ValueList<String>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConsoleAccess" => {
                                console_access = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Groups" => {
                                groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(User {
                        console_access: console_access,
                        groups: groups,
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration {
    //! Property types for the `Configuration` resource.

    /// The [`AWS::AmazonMQ::Configuration.TagsEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configuration-tagsentry.html) property type.
    #[derive(Debug, Default)]
    pub struct TagsEntry {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configuration-tagsentry.html#cfn-amazonmq-configuration-tagsentry-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configuration-tagsentry.html#cfn-amazonmq-configuration-tagsentry-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TagsEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagsEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagsEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagsEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagsEntry")
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

                    Ok(TagsEntry {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_association {
    //! Property types for the `ConfigurationAssociation` resource.

    /// The [`AWS::AmazonMQ::ConfigurationAssociation.ConfigurationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configurationassociation-configurationid.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationId {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configurationassociation-configurationid.html#cfn-amazonmq-configurationassociation-configurationid-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configurationassociation-configurationid.html#cfn-amazonmq-configurationassociation-configurationid-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ConfigurationId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationId {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
