//! Types for the `OpsWorksCM` service.

/// The [`AWS::OpsWorksCM::Server`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html) resource type.
#[derive(Debug, Default)]
pub struct Server {
    properties: ServerProperties
}

/// Properties for the `Server` resource.
#[derive(Debug, Default)]
pub struct ServerProperties {
    /// Property [`AssociatePublicIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-associatepublicipaddress).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub associate_public_ip_address: Option<::Value<bool>>,
    /// Property [`BackupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-backupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_id: Option<::Value<String>>,
    /// Property [`BackupRetentionCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-backupretentioncount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_retention_count: Option<::Value<u32>>,
    /// Property [`CustomCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-customcertificate).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub custom_certificate: Option<::Value<String>>,
    /// Property [`CustomDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-customdomain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub custom_domain: Option<::Value<String>>,
    /// Property [`CustomPrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-customprivatekey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub custom_private_key: Option<::Value<String>>,
    /// Property [`DisableAutomatedBackup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-disableautomatedbackup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disable_automated_backup: Option<::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: Option<::Value<String>>,
    /// Property [`EngineAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-engineattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_attributes: Option<::ValueList<self::server::EngineAttribute>>,
    /// Property [`EngineModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-enginemodel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_model: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-engineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`InstanceProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-instanceprofilearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_profile_arn: ::Value<String>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-instancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`KeyPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-keypair).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_pair: Option<::Value<String>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-servername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_name: Option<::Value<String>>,
    /// Property [`ServiceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-servicerolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_role_arn: ::Value<String>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html#cfn-opsworkscm-server-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ServerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associate_public_ip_address) = self.associate_public_ip_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatePublicIpAddress", associate_public_ip_address)?;
        }
        if let Some(ref backup_id) = self.backup_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupId", backup_id)?;
        }
        if let Some(ref backup_retention_count) = self.backup_retention_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionCount", backup_retention_count)?;
        }
        if let Some(ref custom_certificate) = self.custom_certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomCertificate", custom_certificate)?;
        }
        if let Some(ref custom_domain) = self.custom_domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomDomain", custom_domain)?;
        }
        if let Some(ref custom_private_key) = self.custom_private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomPrivateKey", custom_private_key)?;
        }
        if let Some(ref disable_automated_backup) = self.disable_automated_backup {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableAutomatedBackup", disable_automated_backup)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_attributes) = self.engine_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineAttributes", engine_attributes)?;
        }
        if let Some(ref engine_model) = self.engine_model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineModel", engine_model)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileArn", &self.instance_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        if let Some(ref key_pair) = self.key_pair {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPair", key_pair)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref server_name) = self.server_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associate_public_ip_address: Option<::Value<bool>> = None;
                let mut backup_id: Option<::Value<String>> = None;
                let mut backup_retention_count: Option<::Value<u32>> = None;
                let mut custom_certificate: Option<::Value<String>> = None;
                let mut custom_domain: Option<::Value<String>> = None;
                let mut custom_private_key: Option<::Value<String>> = None;
                let mut disable_automated_backup: Option<::Value<bool>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_attributes: Option<::ValueList<self::server::EngineAttribute>> = None;
                let mut engine_model: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut instance_profile_arn: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut key_pair: Option<::Value<String>> = None;
                let mut preferred_backup_window: Option<::Value<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut server_name: Option<::Value<String>> = None;
                let mut service_role_arn: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatePublicIpAddress" => {
                            associate_public_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupId" => {
                            backup_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionCount" => {
                            backup_retention_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomCertificate" => {
                            custom_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomDomain" => {
                            custom_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomPrivateKey" => {
                            custom_private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisableAutomatedBackup" => {
                            disable_automated_backup = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineAttributes" => {
                            engine_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineModel" => {
                            engine_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileArn" => {
                            instance_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyPair" => {
                            key_pair = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerName" => {
                            server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServerProperties {
                    associate_public_ip_address: associate_public_ip_address,
                    backup_id: backup_id,
                    backup_retention_count: backup_retention_count,
                    custom_certificate: custom_certificate,
                    custom_domain: custom_domain,
                    custom_private_key: custom_private_key,
                    disable_automated_backup: disable_automated_backup,
                    engine: engine,
                    engine_attributes: engine_attributes,
                    engine_model: engine_model,
                    engine_version: engine_version,
                    instance_profile_arn: instance_profile_arn.ok_or(::serde::de::Error::missing_field("InstanceProfileArn"))?,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    key_pair: key_pair,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    security_group_ids: security_group_ids,
                    server_name: server_name,
                    service_role_arn: service_role_arn.ok_or(::serde::de::Error::missing_field("ServiceRoleArn"))?,
                    subnet_ids: subnet_ids,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Server {
    type Properties = ServerProperties;
    const TYPE: &'static str = "AWS::OpsWorksCM::Server";
    fn properties(&self) -> &ServerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Server {}

impl From<ServerProperties> for Server {
    fn from(properties: ServerProperties) -> Server {
        Server { properties }
    }
}

pub mod server {
    //! Property types for the `Server` resource.

    /// The [`AWS::OpsWorksCM::Server.EngineAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworkscm-server-engineattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct EngineAttribute {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworkscm-server-engineattribute.html#cfn-opsworkscm-server-engineattribute-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworkscm-server-engineattribute.html#cfn-opsworkscm-server-engineattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EngineAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EngineAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EngineAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EngineAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EngineAttribute")
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

                    Ok(EngineAttribute {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
