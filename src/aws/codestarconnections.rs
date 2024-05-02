//! Types for the `CodeStarConnections` service.

/// The [`AWS::CodeStarConnections::Connection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html) resource type.
#[derive(Debug, Default)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Debug, Default)]
pub struct ConnectionProperties {
    /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html#cfn-codestarconnections-connection-connectionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connection_name: ::Value<String>,
    /// Property [`HostArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html#cfn-codestarconnections-connection-hostarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub host_arn: Option<::Value<String>>,
    /// Property [`ProviderType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html#cfn-codestarconnections-connection-providertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provider_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html#cfn-codestarconnections-connection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", &self.connection_name)?;
        if let Some(ref host_arn) = self.host_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostArn", host_arn)?;
        }
        if let Some(ref provider_type) = self.provider_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderType", provider_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_name: Option<::Value<String>> = None;
                let mut host_arn: Option<::Value<String>> = None;
                let mut provider_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionName" => {
                            connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostArn" => {
                            host_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderType" => {
                            provider_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectionProperties {
                    connection_name: connection_name.ok_or(::serde::de::Error::missing_field("ConnectionName"))?,
                    host_arn: host_arn,
                    provider_type: provider_type,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connection {
    type Properties = ConnectionProperties;
    const TYPE: &'static str = "AWS::CodeStarConnections::Connection";
    fn properties(&self) -> &ConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connection {}

impl From<ConnectionProperties> for Connection {
    fn from(properties: ConnectionProperties) -> Connection {
        Connection { properties }
    }
}

/// The [`AWS::CodeStarConnections::RepositoryLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html) resource type.
#[derive(Debug, Default)]
pub struct RepositoryLink {
    properties: RepositoryLinkProperties
}

/// Properties for the `RepositoryLink` resource.
#[derive(Debug, Default)]
pub struct RepositoryLinkProperties {
    /// Property [`ConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html#cfn-codestarconnections-repositorylink-connectionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_arn: ::Value<String>,
    /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html#cfn-codestarconnections-repositorylink-encryptionkeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_key_arn: Option<::Value<String>>,
    /// Property [`OwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html#cfn-codestarconnections-repositorylink-ownerid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub owner_id: ::Value<String>,
    /// Property [`RepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html#cfn-codestarconnections-repositorylink-repositoryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub repository_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html#cfn-codestarconnections-repositorylink-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RepositoryLinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionArn", &self.connection_arn)?;
        if let Some(ref encryption_key_arn) = self.encryption_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerId", &self.owner_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", &self.repository_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RepositoryLinkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RepositoryLinkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RepositoryLinkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RepositoryLinkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_arn: Option<::Value<String>> = None;
                let mut encryption_key_arn: Option<::Value<String>> = None;
                let mut owner_id: Option<::Value<String>> = None;
                let mut repository_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionArn" => {
                            connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKeyArn" => {
                            encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerId" => {
                            owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryName" => {
                            repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RepositoryLinkProperties {
                    connection_arn: connection_arn.ok_or(::serde::de::Error::missing_field("ConnectionArn"))?,
                    encryption_key_arn: encryption_key_arn,
                    owner_id: owner_id.ok_or(::serde::de::Error::missing_field("OwnerId"))?,
                    repository_name: repository_name.ok_or(::serde::de::Error::missing_field("RepositoryName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RepositoryLink {
    type Properties = RepositoryLinkProperties;
    const TYPE: &'static str = "AWS::CodeStarConnections::RepositoryLink";
    fn properties(&self) -> &RepositoryLinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RepositoryLinkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RepositoryLink {}

impl From<RepositoryLinkProperties> for RepositoryLink {
    fn from(properties: RepositoryLinkProperties) -> RepositoryLink {
        RepositoryLink { properties }
    }
}

/// The [`AWS::CodeStarConnections::SyncConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct SyncConfiguration {
    properties: SyncConfigurationProperties
}

/// Properties for the `SyncConfiguration` resource.
#[derive(Debug, Default)]
pub struct SyncConfigurationProperties {
    /// Property [`Branch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-branch).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub branch: ::Value<String>,
    /// Property [`ConfigFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-configfile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config_file: ::Value<String>,
    /// Property [`RepositoryLinkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-repositorylinkid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_link_id: ::Value<String>,
    /// Property [`ResourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-resourcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`SyncType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html#cfn-codestarconnections-syncconfiguration-synctype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_type: ::Value<String>,
}

impl ::serde::Serialize for SyncConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Branch", &self.branch)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigFile", &self.config_file)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryLinkId", &self.repository_link_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceName", &self.resource_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncType", &self.sync_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SyncConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SyncConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SyncConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SyncConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut branch: Option<::Value<String>> = None;
                let mut config_file: Option<::Value<String>> = None;
                let mut repository_link_id: Option<::Value<String>> = None;
                let mut resource_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut sync_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Branch" => {
                            branch = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigFile" => {
                            config_file = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryLinkId" => {
                            repository_link_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceName" => {
                            resource_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncType" => {
                            sync_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SyncConfigurationProperties {
                    branch: branch.ok_or(::serde::de::Error::missing_field("Branch"))?,
                    config_file: config_file.ok_or(::serde::de::Error::missing_field("ConfigFile"))?,
                    repository_link_id: repository_link_id.ok_or(::serde::de::Error::missing_field("RepositoryLinkId"))?,
                    resource_name: resource_name.ok_or(::serde::de::Error::missing_field("ResourceName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    sync_type: sync_type.ok_or(::serde::de::Error::missing_field("SyncType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SyncConfiguration {
    type Properties = SyncConfigurationProperties;
    const TYPE: &'static str = "AWS::CodeStarConnections::SyncConfiguration";
    fn properties(&self) -> &SyncConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SyncConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SyncConfiguration {}

impl From<SyncConfigurationProperties> for SyncConfiguration {
    fn from(properties: SyncConfigurationProperties) -> SyncConfiguration {
        SyncConfiguration { properties }
    }
}
