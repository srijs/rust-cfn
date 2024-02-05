//! Types for the `OpenSearchService` service.

/// The [`AWS::OpenSearchService::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`AccessPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-accesspolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policies: Option<::Value<::json::Value>>,
    /// Property [`AdvancedOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-advancedoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub advanced_options: Option<::ValueMap<String>>,
    /// Property [`AdvancedSecurityOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-advancedsecurityoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub advanced_security_options: Option<::Value<self::domain::AdvancedSecurityOptionsInput>>,
    /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-clusterconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_config: Option<::Value<self::domain::ClusterConfig>>,
    /// Property [`CognitoOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-cognitooptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cognito_options: Option<::Value<self::domain::CognitoOptions>>,
    /// Property [`DomainEndpointOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-domainendpointoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_endpoint_options: Option<::Value<self::domain::DomainEndpointOptions>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: Option<::Value<String>>,
    /// Property [`EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-ebsoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ebs_options: Option<::Value<self::domain::EBSOptions>>,
    /// Property [`EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-encryptionatrestoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_at_rest_options: Option<::Value<self::domain::EncryptionAtRestOptions>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`IPAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-ipaddresstype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_address_type: Option<::Value<String>>,
    /// Property [`LogPublishingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-logpublishingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_publishing_options: Option<::ValueMap<self::domain::LogPublishingOption>>,
    /// Property [`NodeToNodeEncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-nodetonodeencryptionoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_to_node_encryption_options: Option<::Value<self::domain::NodeToNodeEncryptionOptions>>,
    /// Property [`OffPeakWindowOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-offpeakwindowoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub off_peak_window_options: Option<::Value<self::domain::OffPeakWindowOptions>>,
    /// Property [`SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-snapshotoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_options: Option<::Value<self::domain::SnapshotOptions>>,
    /// Property [`SoftwareUpdateOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-softwareupdateoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub software_update_options: Option<::Value<self::domain::SoftwareUpdateOptions>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html#cfn-opensearchservice-domain-vpcoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_options: Option<::Value<self::domain::VPCOptions>>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_policies) = self.access_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicies", access_policies)?;
        }
        if let Some(ref advanced_options) = self.advanced_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedOptions", advanced_options)?;
        }
        if let Some(ref advanced_security_options) = self.advanced_security_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedSecurityOptions", advanced_security_options)?;
        }
        if let Some(ref cluster_config) = self.cluster_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", cluster_config)?;
        }
        if let Some(ref cognito_options) = self.cognito_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoOptions", cognito_options)?;
        }
        if let Some(ref domain_endpoint_options) = self.domain_endpoint_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainEndpointOptions", domain_endpoint_options)?;
        }
        if let Some(ref domain_name) = self.domain_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
        }
        if let Some(ref ebs_options) = self.ebs_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EBSOptions", ebs_options)?;
        }
        if let Some(ref encryption_at_rest_options) = self.encryption_at_rest_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAtRestOptions", encryption_at_rest_options)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref ip_address_type) = self.ip_address_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPAddressType", ip_address_type)?;
        }
        if let Some(ref log_publishing_options) = self.log_publishing_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPublishingOptions", log_publishing_options)?;
        }
        if let Some(ref node_to_node_encryption_options) = self.node_to_node_encryption_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeToNodeEncryptionOptions", node_to_node_encryption_options)?;
        }
        if let Some(ref off_peak_window_options) = self.off_peak_window_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OffPeakWindowOptions", off_peak_window_options)?;
        }
        if let Some(ref snapshot_options) = self.snapshot_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotOptions", snapshot_options)?;
        }
        if let Some(ref software_update_options) = self.software_update_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftwareUpdateOptions", software_update_options)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_options) = self.vpc_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCOptions", vpc_options)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_policies: Option<::Value<::json::Value>> = None;
                let mut advanced_options: Option<::ValueMap<String>> = None;
                let mut advanced_security_options: Option<::Value<self::domain::AdvancedSecurityOptionsInput>> = None;
                let mut cluster_config: Option<::Value<self::domain::ClusterConfig>> = None;
                let mut cognito_options: Option<::Value<self::domain::CognitoOptions>> = None;
                let mut domain_endpoint_options: Option<::Value<self::domain::DomainEndpointOptions>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut ebs_options: Option<::Value<self::domain::EBSOptions>> = None;
                let mut encryption_at_rest_options: Option<::Value<self::domain::EncryptionAtRestOptions>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut ip_address_type: Option<::Value<String>> = None;
                let mut log_publishing_options: Option<::ValueMap<self::domain::LogPublishingOption>> = None;
                let mut node_to_node_encryption_options: Option<::Value<self::domain::NodeToNodeEncryptionOptions>> = None;
                let mut off_peak_window_options: Option<::Value<self::domain::OffPeakWindowOptions>> = None;
                let mut snapshot_options: Option<::Value<self::domain::SnapshotOptions>> = None;
                let mut software_update_options: Option<::Value<self::domain::SoftwareUpdateOptions>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_options: Option<::Value<self::domain::VPCOptions>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessPolicies" => {
                            access_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdvancedOptions" => {
                            advanced_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdvancedSecurityOptions" => {
                            advanced_security_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterConfig" => {
                            cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CognitoOptions" => {
                            cognito_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainEndpointOptions" => {
                            domain_endpoint_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EBSOptions" => {
                            ebs_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionAtRestOptions" => {
                            encryption_at_rest_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IPAddressType" => {
                            ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogPublishingOptions" => {
                            log_publishing_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeToNodeEncryptionOptions" => {
                            node_to_node_encryption_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OffPeakWindowOptions" => {
                            off_peak_window_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotOptions" => {
                            snapshot_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SoftwareUpdateOptions" => {
                            software_update_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCOptions" => {
                            vpc_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    access_policies: access_policies,
                    advanced_options: advanced_options,
                    advanced_security_options: advanced_security_options,
                    cluster_config: cluster_config,
                    cognito_options: cognito_options,
                    domain_endpoint_options: domain_endpoint_options,
                    domain_name: domain_name,
                    ebs_options: ebs_options,
                    encryption_at_rest_options: encryption_at_rest_options,
                    engine_version: engine_version,
                    ip_address_type: ip_address_type,
                    log_publishing_options: log_publishing_options,
                    node_to_node_encryption_options: node_to_node_encryption_options,
                    off_peak_window_options: off_peak_window_options,
                    snapshot_options: snapshot_options,
                    software_update_options: software_update_options,
                    tags: tags,
                    vpc_options: vpc_options,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::OpenSearchService::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::OpenSearchService::Domain.AdvancedSecurityOptionsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedSecurityOptionsInput {
        /// Property [`AnonymousAuthDisableDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-anonymousauthdisabledate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub anonymous_auth_disable_date: Option<::Value<String>>,
        /// Property [`AnonymousAuthEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-anonymousauthenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub anonymous_auth_enabled: Option<::Value<bool>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`InternalUserDatabaseEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-internaluserdatabaseenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub internal_user_database_enabled: Option<::Value<bool>>,
        /// Property [`MasterUserOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-masteruseroptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_options: Option<::Value<MasterUserOptions>>,
        /// Property [`SAMLOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html#cfn-opensearchservice-domain-advancedsecurityoptionsinput-samloptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub saml_options: Option<::Value<SAMLOptions>>,
    }

    impl ::codec::SerializeValue for AdvancedSecurityOptionsInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref anonymous_auth_disable_date) = self.anonymous_auth_disable_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnonymousAuthDisableDate", anonymous_auth_disable_date)?;
            }
            if let Some(ref anonymous_auth_enabled) = self.anonymous_auth_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnonymousAuthEnabled", anonymous_auth_enabled)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref internal_user_database_enabled) = self.internal_user_database_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InternalUserDatabaseEnabled", internal_user_database_enabled)?;
            }
            if let Some(ref master_user_options) = self.master_user_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserOptions", master_user_options)?;
            }
            if let Some(ref saml_options) = self.saml_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAMLOptions", saml_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedSecurityOptionsInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedSecurityOptionsInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedSecurityOptionsInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedSecurityOptionsInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut anonymous_auth_disable_date: Option<::Value<String>> = None;
                    let mut anonymous_auth_enabled: Option<::Value<bool>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut internal_user_database_enabled: Option<::Value<bool>> = None;
                    let mut master_user_options: Option<::Value<MasterUserOptions>> = None;
                    let mut saml_options: Option<::Value<SAMLOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AnonymousAuthDisableDate" => {
                                anonymous_auth_disable_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AnonymousAuthEnabled" => {
                                anonymous_auth_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InternalUserDatabaseEnabled" => {
                                internal_user_database_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterUserOptions" => {
                                master_user_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAMLOptions" => {
                                saml_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedSecurityOptionsInput {
                        anonymous_auth_disable_date: anonymous_auth_disable_date,
                        anonymous_auth_enabled: anonymous_auth_enabled,
                        enabled: enabled,
                        internal_user_database_enabled: internal_user_database_enabled,
                        master_user_options: master_user_options,
                        saml_options: saml_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`ColdStorageOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-coldstorageoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cold_storage_options: Option<::Value<ColdStorageOptions>>,
        /// Property [`DedicatedMasterCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-dedicatedmastercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_count: Option<::Value<u32>>,
        /// Property [`DedicatedMasterEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-dedicatedmasterenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_enabled: Option<::Value<bool>>,
        /// Property [`DedicatedMasterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-dedicatedmastertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_type: Option<::Value<String>>,
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-instancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_count: Option<::Value<u32>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`MultiAZWithStandbyEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-multiazwithstandbyenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multi_az_with_standby_enabled: Option<::Value<bool>>,
        /// Property [`WarmCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-warmcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_count: Option<::Value<u32>>,
        /// Property [`WarmEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-warmenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_enabled: Option<::Value<bool>>,
        /// Property [`WarmType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-warmtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_type: Option<::Value<String>>,
        /// Property [`ZoneAwarenessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-zoneawarenessconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zone_awareness_config: Option<::Value<ZoneAwarenessConfig>>,
        /// Property [`ZoneAwarenessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html#cfn-opensearchservice-domain-clusterconfig-zoneawarenessenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zone_awareness_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cold_storage_options) = self.cold_storage_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColdStorageOptions", cold_storage_options)?;
            }
            if let Some(ref dedicated_master_count) = self.dedicated_master_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DedicatedMasterCount", dedicated_master_count)?;
            }
            if let Some(ref dedicated_master_enabled) = self.dedicated_master_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DedicatedMasterEnabled", dedicated_master_enabled)?;
            }
            if let Some(ref dedicated_master_type) = self.dedicated_master_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DedicatedMasterType", dedicated_master_type)?;
            }
            if let Some(ref instance_count) = self.instance_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", instance_count)?;
            }
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref multi_az_with_standby_enabled) = self.multi_az_with_standby_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZWithStandbyEnabled", multi_az_with_standby_enabled)?;
            }
            if let Some(ref warm_count) = self.warm_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WarmCount", warm_count)?;
            }
            if let Some(ref warm_enabled) = self.warm_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WarmEnabled", warm_enabled)?;
            }
            if let Some(ref warm_type) = self.warm_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WarmType", warm_type)?;
            }
            if let Some(ref zone_awareness_config) = self.zone_awareness_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZoneAwarenessConfig", zone_awareness_config)?;
            }
            if let Some(ref zone_awareness_enabled) = self.zone_awareness_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZoneAwarenessEnabled", zone_awareness_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cold_storage_options: Option<::Value<ColdStorageOptions>> = None;
                    let mut dedicated_master_count: Option<::Value<u32>> = None;
                    let mut dedicated_master_enabled: Option<::Value<bool>> = None;
                    let mut dedicated_master_type: Option<::Value<String>> = None;
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut multi_az_with_standby_enabled: Option<::Value<bool>> = None;
                    let mut warm_count: Option<::Value<u32>> = None;
                    let mut warm_enabled: Option<::Value<bool>> = None;
                    let mut warm_type: Option<::Value<String>> = None;
                    let mut zone_awareness_config: Option<::Value<ZoneAwarenessConfig>> = None;
                    let mut zone_awareness_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColdStorageOptions" => {
                                cold_storage_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DedicatedMasterCount" => {
                                dedicated_master_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DedicatedMasterEnabled" => {
                                dedicated_master_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DedicatedMasterType" => {
                                dedicated_master_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiAZWithStandbyEnabled" => {
                                multi_az_with_standby_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WarmCount" => {
                                warm_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WarmEnabled" => {
                                warm_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WarmType" => {
                                warm_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZoneAwarenessConfig" => {
                                zone_awareness_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZoneAwarenessEnabled" => {
                                zone_awareness_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        cold_storage_options: cold_storage_options,
                        dedicated_master_count: dedicated_master_count,
                        dedicated_master_enabled: dedicated_master_enabled,
                        dedicated_master_type: dedicated_master_type,
                        instance_count: instance_count,
                        instance_type: instance_type,
                        multi_az_with_standby_enabled: multi_az_with_standby_enabled,
                        warm_count: warm_count,
                        warm_enabled: warm_enabled,
                        warm_type: warm_type,
                        zone_awareness_config: zone_awareness_config,
                        zone_awareness_enabled: zone_awareness_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.CognitoOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html#cfn-opensearchservice-domain-cognitooptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`IdentityPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html#cfn-opensearchservice-domain-cognitooptions-identitypoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identity_pool_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html#cfn-opensearchservice-domain-cognitooptions-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html#cfn-opensearchservice-domain-cognitooptions-userpoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CognitoOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref identity_pool_id) = self.identity_pool_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolId", identity_pool_id)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref user_pool_id) = self.user_pool_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", user_pool_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut identity_pool_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut user_pool_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdentityPoolId" => {
                                identity_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolId" => {
                                user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoOptions {
                        enabled: enabled,
                        identity_pool_id: identity_pool_id,
                        role_arn: role_arn,
                        user_pool_id: user_pool_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.ColdStorageOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-coldstorageoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ColdStorageOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-coldstorageoptions.html#cfn-opensearchservice-domain-coldstorageoptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ColdStorageOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColdStorageOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColdStorageOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColdStorageOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColdStorageOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColdStorageOptions {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.DomainEndpointOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainEndpointOptions {
        /// Property [`CustomEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html#cfn-opensearchservice-domain-domainendpointoptions-customendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint: Option<::Value<String>>,
        /// Property [`CustomEndpointCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html#cfn-opensearchservice-domain-domainendpointoptions-customendpointcertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint_certificate_arn: Option<::Value<String>>,
        /// Property [`CustomEndpointEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html#cfn-opensearchservice-domain-domainendpointoptions-customendpointenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint_enabled: Option<::Value<bool>>,
        /// Property [`EnforceHTTPS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html#cfn-opensearchservice-domain-domainendpointoptions-enforcehttps).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce_https: Option<::Value<bool>>,
        /// Property [`TLSSecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html#cfn-opensearchservice-domain-domainendpointoptions-tlssecuritypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_security_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DomainEndpointOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_endpoint) = self.custom_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEndpoint", custom_endpoint)?;
            }
            if let Some(ref custom_endpoint_certificate_arn) = self.custom_endpoint_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEndpointCertificateArn", custom_endpoint_certificate_arn)?;
            }
            if let Some(ref custom_endpoint_enabled) = self.custom_endpoint_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEndpointEnabled", custom_endpoint_enabled)?;
            }
            if let Some(ref enforce_https) = self.enforce_https {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnforceHTTPS", enforce_https)?;
            }
            if let Some(ref tls_security_policy) = self.tls_security_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLSSecurityPolicy", tls_security_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainEndpointOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainEndpointOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainEndpointOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainEndpointOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_endpoint: Option<::Value<String>> = None;
                    let mut custom_endpoint_certificate_arn: Option<::Value<String>> = None;
                    let mut custom_endpoint_enabled: Option<::Value<bool>> = None;
                    let mut enforce_https: Option<::Value<bool>> = None;
                    let mut tls_security_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomEndpoint" => {
                                custom_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomEndpointCertificateArn" => {
                                custom_endpoint_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomEndpointEnabled" => {
                                custom_endpoint_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnforceHTTPS" => {
                                enforce_https = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TLSSecurityPolicy" => {
                                tls_security_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainEndpointOptions {
                        custom_endpoint: custom_endpoint,
                        custom_endpoint_certificate_arn: custom_endpoint_certificate_arn,
                        custom_endpoint_enabled: custom_endpoint_enabled,
                        enforce_https: enforce_https,
                        tls_security_policy: tls_security_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EBSOptions {
        /// Property [`EBSEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html#cfn-opensearchservice-domain-ebsoptions-ebsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_enabled: Option<::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html#cfn-opensearchservice-domain-ebsoptions-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`Throughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html#cfn-opensearchservice-domain-ebsoptions-throughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput: Option<::Value<u32>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html#cfn-opensearchservice-domain-ebsoptions-volumesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html#cfn-opensearchservice-domain-ebsoptions-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EBSOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ebs_enabled) = self.ebs_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EBSEnabled", ebs_enabled)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref throughput) = self.throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throughput", throughput)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EBSOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EBSOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EBSOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EBSOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ebs_enabled: Option<::Value<bool>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut throughput: Option<::Value<u32>> = None;
                    let mut volume_size: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EBSEnabled" => {
                                ebs_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Throughput" => {
                                throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EBSOptions {
                        ebs_enabled: ebs_enabled,
                        iops: iops,
                        throughput: throughput,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-encryptionatrestoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionAtRestOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-encryptionatrestoptions.html#cfn-opensearchservice-domain-encryptionatrestoptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-encryptionatrestoptions.html#cfn-opensearchservice-domain-encryptionatrestoptions-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionAtRestOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionAtRestOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionAtRestOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionAtRestOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionAtRestOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionAtRestOptions {
                        enabled: enabled,
                        kms_key_id: kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.Idp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-idp.html) property type.
    #[derive(Debug, Default)]
    pub struct Idp {
        /// Property [`EntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-idp.html#cfn-opensearchservice-domain-idp-entityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id: ::Value<String>,
        /// Property [`MetadataContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-idp.html#cfn-opensearchservice-domain-idp-metadatacontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata_content: ::Value<String>,
    }

    impl ::codec::SerializeValue for Idp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityId", &self.entity_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataContent", &self.metadata_content)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Idp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Idp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Idp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Idp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_id: Option<::Value<String>> = None;
                    let mut metadata_content: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityId" => {
                                entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetadataContent" => {
                                metadata_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Idp {
                        entity_id: entity_id.ok_or(::serde::de::Error::missing_field("EntityId"))?,
                        metadata_content: metadata_content.ok_or(::serde::de::Error::missing_field("MetadataContent"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.LogPublishingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-logpublishingoption.html) property type.
    #[derive(Debug, Default)]
    pub struct LogPublishingOption {
        /// Property [`CloudWatchLogsLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-logpublishingoption.html#cfn-opensearchservice-domain-logpublishingoption-cloudwatchlogsloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_log_group_arn: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-logpublishingoption.html#cfn-opensearchservice-domain-logpublishingoption-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LogPublishingOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs_log_group_arn) = self.cloud_watch_logs_log_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", cloud_watch_logs_log_group_arn)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogPublishingOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogPublishingOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogPublishingOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogPublishingOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_log_group_arn: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsLogGroupArn" => {
                                cloud_watch_logs_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogPublishingOption {
                        cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn,
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.MasterUserOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-masteruseroptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MasterUserOptions {
        /// Property [`MasterUserARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-masteruseroptions.html#cfn-opensearchservice-domain-masteruseroptions-masteruserarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_arn: Option<::Value<String>>,
        /// Property [`MasterUserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-masteruseroptions.html#cfn-opensearchservice-domain-masteruseroptions-masterusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_name: Option<::Value<String>>,
        /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-masteruseroptions.html#cfn-opensearchservice-domain-masteruseroptions-masteruserpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_password: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MasterUserOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref master_user_arn) = self.master_user_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserARN", master_user_arn)?;
            }
            if let Some(ref master_user_name) = self.master_user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserName", master_user_name)?;
            }
            if let Some(ref master_user_password) = self.master_user_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MasterUserOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MasterUserOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MasterUserOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MasterUserOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut master_user_arn: Option<::Value<String>> = None;
                    let mut master_user_name: Option<::Value<String>> = None;
                    let mut master_user_password: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MasterUserARN" => {
                                master_user_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterUserName" => {
                                master_user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterUserPassword" => {
                                master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MasterUserOptions {
                        master_user_arn: master_user_arn,
                        master_user_name: master_user_name,
                        master_user_password: master_user_password,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.NodeToNodeEncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-nodetonodeencryptionoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeToNodeEncryptionOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-nodetonodeencryptionoptions.html#cfn-opensearchservice-domain-nodetonodeencryptionoptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for NodeToNodeEncryptionOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NodeToNodeEncryptionOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeToNodeEncryptionOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeToNodeEncryptionOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeToNodeEncryptionOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeToNodeEncryptionOptions {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.OffPeakWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindow.html) property type.
    #[derive(Debug, Default)]
    pub struct OffPeakWindow {
        /// Property [`WindowStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindow.html#cfn-opensearchservice-domain-offpeakwindow-windowstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub window_start_time: Option<::Value<WindowStartTime>>,
    }

    impl ::codec::SerializeValue for OffPeakWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref window_start_time) = self.window_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowStartTime", window_start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OffPeakWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OffPeakWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OffPeakWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OffPeakWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut window_start_time: Option<::Value<WindowStartTime>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WindowStartTime" => {
                                window_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OffPeakWindow {
                        window_start_time: window_start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.OffPeakWindowOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindowoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct OffPeakWindowOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindowoptions.html#cfn-opensearchservice-domain-offpeakwindowoptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`OffPeakWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindowoptions.html#cfn-opensearchservice-domain-offpeakwindowoptions-offpeakwindow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub off_peak_window: Option<::Value<OffPeakWindow>>,
    }

    impl ::codec::SerializeValue for OffPeakWindowOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref off_peak_window) = self.off_peak_window {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OffPeakWindow", off_peak_window)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OffPeakWindowOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OffPeakWindowOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OffPeakWindowOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OffPeakWindowOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut off_peak_window: Option<::Value<OffPeakWindow>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OffPeakWindow" => {
                                off_peak_window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OffPeakWindowOptions {
                        enabled: enabled,
                        off_peak_window: off_peak_window,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.SAMLOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SAMLOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`Idp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-idp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idp: Option<::Value<Idp>>,
        /// Property [`MasterBackendRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-masterbackendrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_backend_role: Option<::Value<String>>,
        /// Property [`MasterUserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-masterusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_name: Option<::Value<String>>,
        /// Property [`RolesKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-roleskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub roles_key: Option<::Value<String>>,
        /// Property [`SessionTimeoutMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-sessiontimeoutminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_timeout_minutes: Option<::Value<u32>>,
        /// Property [`SubjectKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html#cfn-opensearchservice-domain-samloptions-subjectkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SAMLOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref idp) = self.idp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idp", idp)?;
            }
            if let Some(ref master_backend_role) = self.master_backend_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterBackendRole", master_backend_role)?;
            }
            if let Some(ref master_user_name) = self.master_user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserName", master_user_name)?;
            }
            if let Some(ref roles_key) = self.roles_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RolesKey", roles_key)?;
            }
            if let Some(ref session_timeout_minutes) = self.session_timeout_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionTimeoutMinutes", session_timeout_minutes)?;
            }
            if let Some(ref subject_key) = self.subject_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectKey", subject_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAMLOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAMLOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAMLOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAMLOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut idp: Option<::Value<Idp>> = None;
                    let mut master_backend_role: Option<::Value<String>> = None;
                    let mut master_user_name: Option<::Value<String>> = None;
                    let mut roles_key: Option<::Value<String>> = None;
                    let mut session_timeout_minutes: Option<::Value<u32>> = None;
                    let mut subject_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Idp" => {
                                idp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterBackendRole" => {
                                master_backend_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterUserName" => {
                                master_user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RolesKey" => {
                                roles_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionTimeoutMinutes" => {
                                session_timeout_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectKey" => {
                                subject_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAMLOptions {
                        enabled: enabled,
                        idp: idp,
                        master_backend_role: master_backend_role,
                        master_user_name: master_user_name,
                        roles_key: roles_key,
                        session_timeout_minutes: session_timeout_minutes,
                        subject_key: subject_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.ServiceSoftwareOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceSoftwareOptions {
        /// Property [`AutomatedUpdateDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-automatedupdatedate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automated_update_date: Option<::Value<String>>,
        /// Property [`Cancellable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-cancellable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cancellable: Option<::Value<bool>>,
        /// Property [`CurrentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-currentversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub current_version: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`NewVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-newversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub new_version: Option<::Value<String>>,
        /// Property [`OptionalDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-optionaldeployment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub optional_deployment: Option<::Value<bool>>,
        /// Property [`UpdateAvailable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-updateavailable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_available: Option<::Value<bool>>,
        /// Property [`UpdateStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html#cfn-opensearchservice-domain-servicesoftwareoptions-updatestatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceSoftwareOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automated_update_date) = self.automated_update_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomatedUpdateDate", automated_update_date)?;
            }
            if let Some(ref cancellable) = self.cancellable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cancellable", cancellable)?;
            }
            if let Some(ref current_version) = self.current_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentVersion", current_version)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref new_version) = self.new_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewVersion", new_version)?;
            }
            if let Some(ref optional_deployment) = self.optional_deployment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionalDeployment", optional_deployment)?;
            }
            if let Some(ref update_available) = self.update_available {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateAvailable", update_available)?;
            }
            if let Some(ref update_status) = self.update_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateStatus", update_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceSoftwareOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceSoftwareOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceSoftwareOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceSoftwareOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automated_update_date: Option<::Value<String>> = None;
                    let mut cancellable: Option<::Value<bool>> = None;
                    let mut current_version: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut new_version: Option<::Value<String>> = None;
                    let mut optional_deployment: Option<::Value<bool>> = None;
                    let mut update_available: Option<::Value<bool>> = None;
                    let mut update_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomatedUpdateDate" => {
                                automated_update_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cancellable" => {
                                cancellable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CurrentVersion" => {
                                current_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NewVersion" => {
                                new_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionalDeployment" => {
                                optional_deployment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateAvailable" => {
                                update_available = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateStatus" => {
                                update_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceSoftwareOptions {
                        automated_update_date: automated_update_date,
                        cancellable: cancellable,
                        current_version: current_version,
                        description: description,
                        new_version: new_version,
                        optional_deployment: optional_deployment,
                        update_available: update_available,
                        update_status: update_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-snapshotoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SnapshotOptions {
        /// Property [`AutomatedSnapshotStartHour`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-snapshotoptions.html#cfn-opensearchservice-domain-snapshotoptions-automatedsnapshotstarthour).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automated_snapshot_start_hour: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SnapshotOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automated_snapshot_start_hour) = self.automated_snapshot_start_hour {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomatedSnapshotStartHour", automated_snapshot_start_hour)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnapshotOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnapshotOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnapshotOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnapshotOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automated_snapshot_start_hour: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomatedSnapshotStartHour" => {
                                automated_snapshot_start_hour = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnapshotOptions {
                        automated_snapshot_start_hour: automated_snapshot_start_hour,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.SoftwareUpdateOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-softwareupdateoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SoftwareUpdateOptions {
        /// Property [`AutoSoftwareUpdateEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-softwareupdateoptions.html#cfn-opensearchservice-domain-softwareupdateoptions-autosoftwareupdateenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_software_update_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SoftwareUpdateOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_software_update_enabled) = self.auto_software_update_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoSoftwareUpdateEnabled", auto_software_update_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SoftwareUpdateOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SoftwareUpdateOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SoftwareUpdateOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SoftwareUpdateOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_software_update_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoSoftwareUpdateEnabled" => {
                                auto_software_update_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SoftwareUpdateOptions {
                        auto_software_update_enabled: auto_software_update_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-vpcoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct VPCOptions {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-vpcoptions.html#cfn-opensearchservice-domain-vpcoptions-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-vpcoptions.html#cfn-opensearchservice-domain-vpcoptions-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for VPCOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VPCOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VPCOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VPCOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VPCOptions {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.WindowStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-windowstarttime.html) property type.
    #[derive(Debug, Default)]
    pub struct WindowStartTime {
        /// Property [`Hours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-windowstarttime.html#cfn-opensearchservice-domain-windowstarttime-hours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hours: ::Value<u32>,
        /// Property [`Minutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-windowstarttime.html#cfn-opensearchservice-domain-windowstarttime-minutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for WindowStartTime {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hours", &self.hours)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Minutes", &self.minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WindowStartTime {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WindowStartTime, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WindowStartTime;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WindowStartTime")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hours: Option<::Value<u32>> = None;
                    let mut minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hours" => {
                                hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Minutes" => {
                                minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WindowStartTime {
                        hours: hours.ok_or(::serde::de::Error::missing_field("Hours"))?,
                        minutes: minutes.ok_or(::serde::de::Error::missing_field("Minutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpenSearchService::Domain.ZoneAwarenessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-zoneawarenessconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ZoneAwarenessConfig {
        /// Property [`AvailabilityZoneCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-zoneawarenessconfig.html#cfn-opensearchservice-domain-zoneawarenessconfig-availabilityzonecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone_count: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ZoneAwarenessConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone_count) = self.availability_zone_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZoneCount", availability_zone_count)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZoneAwarenessConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZoneAwarenessConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZoneAwarenessConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZoneAwarenessConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone_count: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZoneCount" => {
                                availability_zone_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZoneAwarenessConfig {
                        availability_zone_count: availability_zone_count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
