//! Types for the `Elasticsearch` service.

/// The [`AWS::Elasticsearch::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html) resource type.
#[derive(Debug)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug)]
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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub elasticsearch_version: Option<::Value<String>>,
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
                let mut domain_name: Option<::Value<String>> = None;
                let mut ebs_options: Option<::Value<self::domain::EBSOptions>> = None;
                let mut elasticsearch_cluster_config: Option<::Value<self::domain::ElasticsearchClusterConfig>> = None;
                let mut elasticsearch_version: Option<::Value<String>> = None;
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
                    domain_name: domain_name,
                    ebs_options: ebs_options,
                    elasticsearch_cluster_config: elasticsearch_cluster_config,
                    elasticsearch_version: elasticsearch_version,
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

    /// The [`AWS::Elasticsearch::Domain.EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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
                        zone_awareness_enabled: zone_awareness_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Elasticsearch::Domain.SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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
}
