//! Types for the `Elasticsearch` service.

/// The [`AWS::Elasticsearch::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`AccessPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-accesspolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policies: Option<::Value<::json::Value>>,
    /// Property [`AdvancedOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-advancedoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub advanced_options: Option<::ValueMap<String>>,
    /// Property [`AdvancedSecurityOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-advancedsecurityoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub advanced_security_options: Option<::Value<self::domain::AdvancedSecurityOptionsInput>>,
    /// Property [`CognitoOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-cognitooptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cognito_options: Option<::Value<self::domain::CognitoOptions>>,
    /// Property [`DomainEndpointOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-domainendpointoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_endpoint_options: Option<::Value<self::domain::DomainEndpointOptions>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: Option<::Value<String>>,
    /// Property [`EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-ebsoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ebs_options: Option<::Value<self::domain::EBSOptions>>,
    /// Property [`ElasticsearchClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-elasticsearchclusterconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elasticsearch_cluster_config: Option<::Value<self::domain::ElasticsearchClusterConfig>>,
    /// Property [`ElasticsearchVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-elasticsearchversion).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub elasticsearch_version: Option<::Value<String>>,
    /// Property [`EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-encryptionatrestoptions).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub encryption_at_rest_options: Option<::Value<self::domain::EncryptionAtRestOptions>>,
    /// Property [`LogPublishingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-logpublishingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_publishing_options: Option<::ValueMap<self::domain::LogPublishingOption>>,
    /// Property [`NodeToNodeEncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-nodetonodeencryptionoptions).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub node_to_node_encryption_options: Option<::Value<self::domain::NodeToNodeEncryptionOptions>>,
    /// Property [`SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-snapshotoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_options: Option<::Value<self::domain::SnapshotOptions>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html#cfn-elasticsearch-domain-vpcoptions).
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
        if let Some(ref elasticsearch_cluster_config) = self.elasticsearch_cluster_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchClusterConfig", elasticsearch_cluster_config)?;
        }
        if let Some(ref elasticsearch_version) = self.elasticsearch_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchVersion", elasticsearch_version)?;
        }
        if let Some(ref encryption_at_rest_options) = self.encryption_at_rest_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAtRestOptions", encryption_at_rest_options)?;
        }
        if let Some(ref log_publishing_options) = self.log_publishing_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPublishingOptions", log_publishing_options)?;
        }
        if let Some(ref node_to_node_encryption_options) = self.node_to_node_encryption_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeToNodeEncryptionOptions", node_to_node_encryption_options)?;
        }
        if let Some(ref snapshot_options) = self.snapshot_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotOptions", snapshot_options)?;
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
                let mut cognito_options: Option<::Value<self::domain::CognitoOptions>> = None;
                let mut domain_endpoint_options: Option<::Value<self::domain::DomainEndpointOptions>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut ebs_options: Option<::Value<self::domain::EBSOptions>> = None;
                let mut elasticsearch_cluster_config: Option<::Value<self::domain::ElasticsearchClusterConfig>> = None;
                let mut elasticsearch_version: Option<::Value<String>> = None;
                let mut encryption_at_rest_options: Option<::Value<self::domain::EncryptionAtRestOptions>> = None;
                let mut log_publishing_options: Option<::ValueMap<self::domain::LogPublishingOption>> = None;
                let mut node_to_node_encryption_options: Option<::Value<self::domain::NodeToNodeEncryptionOptions>> = None;
                let mut snapshot_options: Option<::Value<self::domain::SnapshotOptions>> = None;
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
                        "ElasticsearchClusterConfig" => {
                            elasticsearch_cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticsearchVersion" => {
                            elasticsearch_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionAtRestOptions" => {
                            encryption_at_rest_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogPublishingOptions" => {
                            log_publishing_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeToNodeEncryptionOptions" => {
                            node_to_node_encryption_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotOptions" => {
                            snapshot_options = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    cognito_options: cognito_options,
                    domain_endpoint_options: domain_endpoint_options,
                    domain_name: domain_name,
                    ebs_options: ebs_options,
                    elasticsearch_cluster_config: elasticsearch_cluster_config,
                    elasticsearch_version: elasticsearch_version,
                    encryption_at_rest_options: encryption_at_rest_options,
                    log_publishing_options: log_publishing_options,
                    node_to_node_encryption_options: node_to_node_encryption_options,
                    snapshot_options: snapshot_options,
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
    const TYPE: &'static str = "AWS::Elasticsearch::Domain";
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

    /// The [`AWS::Elasticsearch::Domain.AdvancedSecurityOptionsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-advancedsecurityoptionsinput.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedSecurityOptionsInput {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-advancedsecurityoptionsinput.html#cfn-elasticsearch-domain-advancedsecurityoptionsinput-enabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`InternalUserDatabaseEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-advancedsecurityoptionsinput.html#cfn-elasticsearch-domain-advancedsecurityoptionsinput-internaluserdatabaseenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub internal_user_database_enabled: Option<::Value<bool>>,
        /// Property [`MasterUserOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-advancedsecurityoptionsinput.html#cfn-elasticsearch-domain-advancedsecurityoptionsinput-masteruseroptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_options: Option<::Value<MasterUserOptions>>,
    }

    impl ::codec::SerializeValue for AdvancedSecurityOptionsInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref internal_user_database_enabled) = self.internal_user_database_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InternalUserDatabaseEnabled", internal_user_database_enabled)?;
            }
            if let Some(ref master_user_options) = self.master_user_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserOptions", master_user_options)?;
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
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut internal_user_database_enabled: Option<::Value<bool>> = None;
                    let mut master_user_options: Option<::Value<MasterUserOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InternalUserDatabaseEnabled" => {
                                internal_user_database_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterUserOptions" => {
                                master_user_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedSecurityOptionsInput {
                        enabled: enabled,
                        internal_user_database_enabled: internal_user_database_enabled,
                        master_user_options: master_user_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Elasticsearch::Domain.CognitoOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html#cfn-elasticsearch-domain-cognitooptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`IdentityPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html#cfn-elasticsearch-domain-cognitooptions-identitypoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identity_pool_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html#cfn-elasticsearch-domain-cognitooptions-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html#cfn-elasticsearch-domain-cognitooptions-userpoolid).
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

    /// The [`AWS::Elasticsearch::Domain.DomainEndpointOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainEndpointOptions {
        /// Property [`CustomEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html#cfn-elasticsearch-domain-domainendpointoptions-customendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint: Option<::Value<String>>,
        /// Property [`CustomEndpointCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html#cfn-elasticsearch-domain-domainendpointoptions-customendpointcertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint_certificate_arn: Option<::Value<String>>,
        /// Property [`CustomEndpointEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html#cfn-elasticsearch-domain-domainendpointoptions-customendpointenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_endpoint_enabled: Option<::Value<bool>>,
        /// Property [`EnforceHTTPS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html#cfn-elasticsearch-domain-domainendpointoptions-enforcehttps).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce_https: Option<::Value<bool>>,
        /// Property [`TLSSecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html#cfn-elasticsearch-domain-domainendpointoptions-tlssecuritypolicy).
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

    /// The [`AWS::Elasticsearch::Domain.EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EBSOptions {
        /// Property [`EBSEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html#cfn-elasticsearch-domain-ebsoptions-ebsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_enabled: Option<::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html#cfn-elasticsearch-domain-ebsoptions-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html#cfn-elasticsearch-domain-ebsoptions-volumesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html#cfn-elasticsearch-domain-ebsoptions-volumetype).
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
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Elasticsearch::Domain.ElasticsearchClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchClusterConfig {
        /// Property [`DedicatedMasterCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-dedicatedmastercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_count: Option<::Value<u32>>,
        /// Property [`DedicatedMasterEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-dedicatedmasterenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_enabled: Option<::Value<bool>>,
        /// Property [`DedicatedMasterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-dedicatedmastertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedicated_master_type: Option<::Value<String>>,
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-instancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_count: Option<::Value<u32>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-instnacetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`WarmCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticsearchclusterconfig-warmcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_count: Option<::Value<u32>>,
        /// Property [`WarmEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticsearchclusterconfig-warmenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_enabled: Option<::Value<bool>>,
        /// Property [`WarmType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticsearchclusterconfig-warmtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warm_type: Option<::Value<String>>,
        /// Property [`ZoneAwarenessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticsearchclusterconfig-zoneawarenessconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zone_awareness_config: Option<::Value<ZoneAwarenessConfig>>,
        /// Property [`ZoneAwarenessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html#cfn-elasticsearch-domain-elasticseachclusterconfig-zoneawarenessenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zone_awareness_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ElasticsearchClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
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

    impl ::codec::DeserializeValue for ElasticsearchClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dedicated_master_count: Option<::Value<u32>> = None;
                    let mut dedicated_master_enabled: Option<::Value<bool>> = None;
                    let mut dedicated_master_type: Option<::Value<String>> = None;
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut warm_count: Option<::Value<u32>> = None;
                    let mut warm_enabled: Option<::Value<bool>> = None;
                    let mut warm_type: Option<::Value<String>> = None;
                    let mut zone_awareness_config: Option<::Value<ZoneAwarenessConfig>> = None;
                    let mut zone_awareness_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
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

                    Ok(ElasticsearchClusterConfig {
                        dedicated_master_count: dedicated_master_count,
                        dedicated_master_enabled: dedicated_master_enabled,
                        dedicated_master_type: dedicated_master_type,
                        instance_count: instance_count,
                        instance_type: instance_type,
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

    /// The [`AWS::Elasticsearch::Domain.EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-encryptionatrestoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionAtRestOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-encryptionatrestoptions.html#cfn-elasticsearch-domain-encryptionatrestoptions-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub enabled: Option<::Value<bool>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-encryptionatrestoptions.html#cfn-elasticsearch-domain-encryptionatrestoptions-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
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

    /// The [`AWS::Elasticsearch::Domain.LogPublishingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-logpublishingoption.html) property type.
    #[derive(Debug, Default)]
    pub struct LogPublishingOption {
        /// Property [`CloudWatchLogsLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-logpublishingoption.html#cfn-elasticsearch-domain-logpublishingoption-cloudwatchlogsloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_log_group_arn: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-logpublishingoption.html#cfn-elasticsearch-domain-logpublishingoption-enabled).
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

    /// The [`AWS::Elasticsearch::Domain.MasterUserOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-masteruseroptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MasterUserOptions {
        /// Property [`MasterUserARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-masteruseroptions.html#cfn-elasticsearch-domain-masteruseroptions-masteruserarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_arn: Option<::Value<String>>,
        /// Property [`MasterUserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-masteruseroptions.html#cfn-elasticsearch-domain-masteruseroptions-masterusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_user_name: Option<::Value<String>>,
        /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-masteruseroptions.html#cfn-elasticsearch-domain-masteruseroptions-masteruserpassword).
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

    /// The [`AWS::Elasticsearch::Domain.NodeToNodeEncryptionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-nodetonodeencryptionoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeToNodeEncryptionOptions {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-nodetonodeencryptionoptions.html#cfn-elasticsearch-domain-nodetonodeencryptionoptions-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
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

    /// The [`AWS::Elasticsearch::Domain.SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SnapshotOptions {
        /// Property [`AutomatedSnapshotStartHour`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html#cfn-elasticsearch-domain-snapshotoptions-automatedsnapshotstarthour).
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

    /// The [`AWS::Elasticsearch::Domain.VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct VPCOptions {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html#cfn-elasticsearch-domain-vpcoptions-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html#cfn-elasticsearch-domain-vpcoptions-subnetids).
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

    /// The [`AWS::Elasticsearch::Domain.ZoneAwarenessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-zoneawarenessconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ZoneAwarenessConfig {
        /// Property [`AvailabilityZoneCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-zoneawarenessconfig.html#cfn-elasticsearch-domain-zoneawarenessconfig-availabilityzonecount).
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
