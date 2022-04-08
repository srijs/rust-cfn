//! Types for the `EKS` service.

/// The [`AWS::EKS::Addon`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html) resource type.
#[derive(Debug, Default)]
pub struct Addon {
    properties: AddonProperties
}

/// Properties for the `Addon` resource.
#[derive(Debug, Default)]
pub struct AddonProperties {
    /// Property [`AddonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-addonname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub addon_name: ::Value<String>,
    /// Property [`AddonVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-addonversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub addon_version: Option<::Value<String>>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`ResolveConflicts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-resolveconflicts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resolve_conflicts: Option<::Value<String>>,
    /// Property [`ServiceAccountRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-serviceaccountrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_account_role_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html#cfn-eks-addon-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AddonProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddonName", &self.addon_name)?;
        if let Some(ref addon_version) = self.addon_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddonVersion", addon_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref resolve_conflicts) = self.resolve_conflicts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolveConflicts", resolve_conflicts)?;
        }
        if let Some(ref service_account_role_arn) = self.service_account_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccountRoleArn", service_account_role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AddonProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AddonProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AddonProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AddonProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut addon_name: Option<::Value<String>> = None;
                let mut addon_version: Option<::Value<String>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut resolve_conflicts: Option<::Value<String>> = None;
                let mut service_account_role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AddonName" => {
                            addon_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AddonVersion" => {
                            addon_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResolveConflicts" => {
                            resolve_conflicts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceAccountRoleArn" => {
                            service_account_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AddonProperties {
                    addon_name: addon_name.ok_or(::serde::de::Error::missing_field("AddonName"))?,
                    addon_version: addon_version,
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    resolve_conflicts: resolve_conflicts,
                    service_account_role_arn: service_account_role_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Addon {
    type Properties = AddonProperties;
    const TYPE: &'static str = "AWS::EKS::Addon";
    fn properties(&self) -> &AddonProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AddonProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Addon {}

impl From<AddonProperties> for Addon {
    fn from(properties: AddonProperties) -> Addon {
        Addon { properties }
    }
}

/// The [`AWS::EKS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-encryptionconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_config: Option<::ValueList<self::cluster::EncryptionConfig>>,
    /// Property [`KubernetesNetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-kubernetesnetworkconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kubernetes_network_config: Option<::Value<self::cluster::KubernetesNetworkConfig>>,
    /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-logging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging: Option<::Value<self::cluster::Logging>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ResourcesVpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-resourcesvpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources_vpc_config: ::Value<self::cluster::ResourcesVpcConfig>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html#cfn-eks-cluster-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref encryption_config) = self.encryption_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", encryption_config)?;
        }
        if let Some(ref kubernetes_network_config) = self.kubernetes_network_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KubernetesNetworkConfig", kubernetes_network_config)?;
        }
        if let Some(ref logging) = self.logging {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcesVpcConfig", &self.resources_vpc_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref version) = self.version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut encryption_config: Option<::ValueList<self::cluster::EncryptionConfig>> = None;
                let mut kubernetes_network_config: Option<::Value<self::cluster::KubernetesNetworkConfig>> = None;
                let mut logging: Option<::Value<self::cluster::Logging>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut resources_vpc_config: Option<::Value<self::cluster::ResourcesVpcConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EncryptionConfig" => {
                            encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KubernetesNetworkConfig" => {
                            kubernetes_network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Logging" => {
                            logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcesVpcConfig" => {
                            resources_vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    encryption_config: encryption_config,
                    kubernetes_network_config: kubernetes_network_config,
                    logging: logging,
                    name: name,
                    resources_vpc_config: resources_vpc_config.ok_or(::serde::de::Error::missing_field("ResourcesVpcConfig"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::EKS::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::EKS::FargateProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html) resource type.
#[derive(Debug, Default)]
pub struct FargateProfile {
    properties: FargateProfileProperties
}

/// Properties for the `FargateProfile` resource.
#[derive(Debug, Default)]
pub struct FargateProfileProperties {
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`FargateProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-fargateprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fargate_profile_name: Option<::Value<String>>,
    /// Property [`PodExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-podexecutionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pod_execution_role_arn: ::Value<String>,
    /// Property [`Selectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-selectors).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub selectors: ::ValueList<self::fargate_profile::Selector>,
    /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-subnets).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnets: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html#cfn-eks-fargateprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FargateProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref fargate_profile_name) = self.fargate_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FargateProfileName", fargate_profile_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PodExecutionRoleArn", &self.pod_execution_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Selectors", &self.selectors)?;
        if let Some(ref subnets) = self.subnets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FargateProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FargateProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FargateProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FargateProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_name: Option<::Value<String>> = None;
                let mut fargate_profile_name: Option<::Value<String>> = None;
                let mut pod_execution_role_arn: Option<::Value<String>> = None;
                let mut selectors: Option<::ValueList<self::fargate_profile::Selector>> = None;
                let mut subnets: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FargateProfileName" => {
                            fargate_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PodExecutionRoleArn" => {
                            pod_execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Selectors" => {
                            selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subnets" => {
                            subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FargateProfileProperties {
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    fargate_profile_name: fargate_profile_name,
                    pod_execution_role_arn: pod_execution_role_arn.ok_or(::serde::de::Error::missing_field("PodExecutionRoleArn"))?,
                    selectors: selectors.ok_or(::serde::de::Error::missing_field("Selectors"))?,
                    subnets: subnets,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FargateProfile {
    type Properties = FargateProfileProperties;
    const TYPE: &'static str = "AWS::EKS::FargateProfile";
    fn properties(&self) -> &FargateProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FargateProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FargateProfile {}

impl From<FargateProfileProperties> for FargateProfile {
    fn from(properties: FargateProfileProperties) -> FargateProfile {
        FargateProfile { properties }
    }
}

/// The [`AWS::EKS::IdentityProviderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html) resource type.
#[derive(Debug, Default)]
pub struct IdentityProviderConfig {
    properties: IdentityProviderConfigProperties
}

/// Properties for the `IdentityProviderConfig` resource.
#[derive(Debug, Default)]
pub struct IdentityProviderConfigProperties {
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html#cfn-eks-identityproviderconfig-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`IdentityProviderConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html#cfn-eks-identityproviderconfig-identityproviderconfigname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_provider_config_name: Option<::Value<String>>,
    /// Property [`Oidc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html#cfn-eks-identityproviderconfig-oidc).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub oidc: Option<::Value<self::identity_provider_config::OidcIdentityProviderConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html#cfn-eks-identityproviderconfig-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html#cfn-eks-identityproviderconfig-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for IdentityProviderConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref identity_provider_config_name) = self.identity_provider_config_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderConfigName", identity_provider_config_name)?;
        }
        if let Some(ref oidc) = self.oidc {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Oidc", oidc)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentityProviderConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityProviderConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentityProviderConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentityProviderConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_name: Option<::Value<String>> = None;
                let mut identity_provider_config_name: Option<::Value<String>> = None;
                let mut oidc: Option<::Value<self::identity_provider_config::OidcIdentityProviderConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderConfigName" => {
                            identity_provider_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Oidc" => {
                            oidc = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdentityProviderConfigProperties {
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    identity_provider_config_name: identity_provider_config_name,
                    oidc: oidc,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdentityProviderConfig {
    type Properties = IdentityProviderConfigProperties;
    const TYPE: &'static str = "AWS::EKS::IdentityProviderConfig";
    fn properties(&self) -> &IdentityProviderConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentityProviderConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdentityProviderConfig {}

impl From<IdentityProviderConfigProperties> for IdentityProviderConfig {
    fn from(properties: IdentityProviderConfigProperties) -> IdentityProviderConfig {
        IdentityProviderConfig { properties }
    }
}

/// The [`AWS::EKS::Nodegroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html) resource type.
#[derive(Debug, Default)]
pub struct Nodegroup {
    properties: NodegroupProperties
}

/// Properties for the `Nodegroup` resource.
#[derive(Debug, Default)]
pub struct NodegroupProperties {
    /// Property [`AmiType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-amitype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ami_type: Option<::Value<String>>,
    /// Property [`CapacityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-capacitytype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub capacity_type: Option<::Value<String>>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`DiskSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-disksize).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub disk_size: Option<::Value<u32>>,
    /// Property [`ForceUpdateEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-forceupdateenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub force_update_enabled: Option<::Value<bool>>,
    /// Property [`InstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-instancetypes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_types: Option<::ValueList<String>>,
    /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-labels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub labels: Option<::Value<::json::Value>>,
    /// Property [`LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-launchtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_template: Option<::Value<self::nodegroup::LaunchTemplateSpecification>>,
    /// Property [`NodeRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-noderole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub node_role: ::Value<String>,
    /// Property [`NodegroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-nodegroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub nodegroup_name: Option<::Value<String>>,
    /// Property [`ReleaseVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-releaseversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub release_version: Option<::Value<String>>,
    /// Property [`RemoteAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-remoteaccess).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub remote_access: Option<::Value<self::nodegroup::RemoteAccess>>,
    /// Property [`ScalingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-scalingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_config: Option<::Value<self::nodegroup::ScalingConfig>>,
    /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-subnets).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnets: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Taints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-taints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub taints: Option<::ValueList<self::nodegroup::Taint>>,
    /// Property [`UpdateConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-updateconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub update_config: Option<::Value<self::nodegroup::UpdateConfig>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html#cfn-eks-nodegroup-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for NodegroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref ami_type) = self.ami_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmiType", ami_type)?;
        }
        if let Some(ref capacity_type) = self.capacity_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityType", capacity_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref disk_size) = self.disk_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiskSize", disk_size)?;
        }
        if let Some(ref force_update_enabled) = self.force_update_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceUpdateEnabled", force_update_enabled)?;
        }
        if let Some(ref instance_types) = self.instance_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypes", instance_types)?;
        }
        if let Some(ref labels) = self.labels {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", labels)?;
        }
        if let Some(ref launch_template) = self.launch_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplate", launch_template)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeRole", &self.node_role)?;
        if let Some(ref nodegroup_name) = self.nodegroup_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodegroupName", nodegroup_name)?;
        }
        if let Some(ref release_version) = self.release_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReleaseVersion", release_version)?;
        }
        if let Some(ref remote_access) = self.remote_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoteAccess", remote_access)?;
        }
        if let Some(ref scaling_config) = self.scaling_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingConfig", scaling_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref taints) = self.taints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Taints", taints)?;
        }
        if let Some(ref update_config) = self.update_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateConfig", update_config)?;
        }
        if let Some(ref version) = self.version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NodegroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NodegroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodegroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NodegroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ami_type: Option<::Value<String>> = None;
                let mut capacity_type: Option<::Value<String>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut disk_size: Option<::Value<u32>> = None;
                let mut force_update_enabled: Option<::Value<bool>> = None;
                let mut instance_types: Option<::ValueList<String>> = None;
                let mut labels: Option<::Value<::json::Value>> = None;
                let mut launch_template: Option<::Value<self::nodegroup::LaunchTemplateSpecification>> = None;
                let mut node_role: Option<::Value<String>> = None;
                let mut nodegroup_name: Option<::Value<String>> = None;
                let mut release_version: Option<::Value<String>> = None;
                let mut remote_access: Option<::Value<self::nodegroup::RemoteAccess>> = None;
                let mut scaling_config: Option<::Value<self::nodegroup::ScalingConfig>> = None;
                let mut subnets: Option<::ValueList<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut taints: Option<::ValueList<self::nodegroup::Taint>> = None;
                let mut update_config: Option<::Value<self::nodegroup::UpdateConfig>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AmiType" => {
                            ami_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CapacityType" => {
                            capacity_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DiskSize" => {
                            disk_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ForceUpdateEnabled" => {
                            force_update_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceTypes" => {
                            instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Labels" => {
                            labels = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchTemplate" => {
                            launch_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeRole" => {
                            node_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodegroupName" => {
                            nodegroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReleaseVersion" => {
                            release_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RemoteAccess" => {
                            remote_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingConfig" => {
                            scaling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subnets" => {
                            subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Taints" => {
                            taints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UpdateConfig" => {
                            update_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NodegroupProperties {
                    ami_type: ami_type,
                    capacity_type: capacity_type,
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    disk_size: disk_size,
                    force_update_enabled: force_update_enabled,
                    instance_types: instance_types,
                    labels: labels,
                    launch_template: launch_template,
                    node_role: node_role.ok_or(::serde::de::Error::missing_field("NodeRole"))?,
                    nodegroup_name: nodegroup_name,
                    release_version: release_version,
                    remote_access: remote_access,
                    scaling_config: scaling_config,
                    subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    tags: tags,
                    taints: taints,
                    update_config: update_config,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Nodegroup {
    type Properties = NodegroupProperties;
    const TYPE: &'static str = "AWS::EKS::Nodegroup";
    fn properties(&self) -> &NodegroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NodegroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Nodegroup {}

impl From<NodegroupProperties> for Nodegroup {
    fn from(properties: NodegroupProperties) -> Nodegroup {
        Nodegroup { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::EKS::Cluster.ClusterLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-clusterlogging.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterLogging {
        /// Property [`EnabledTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-clusterlogging.html#cfn-eks-cluster-clusterlogging-enabledtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled_types: Option<::ValueList<LoggingTypeConfig>>,
    }

    impl ::codec::SerializeValue for ClusterLogging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled_types) = self.enabled_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnabledTypes", enabled_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterLogging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterLogging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterLogging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterLogging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled_types: Option<::ValueList<LoggingTypeConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnabledTypes" => {
                                enabled_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterLogging {
                        enabled_types: enabled_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-encryptionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfig {
        /// Property [`Provider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-encryptionconfig.html#cfn-eks-cluster-encryptionconfig-provider).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub provider: Option<::Value<Provider>>,
        /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-encryptionconfig.html#cfn-eks-cluster-encryptionconfig-resources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resources: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref provider) = self.provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", provider)?;
            }
            if let Some(ref resources) = self.resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", resources)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provider: Option<::Value<Provider>> = None;
                    let mut resources: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Provider" => {
                                provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Resources" => {
                                resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfig {
                        provider: provider,
                        resources: resources,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.KubernetesNetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-kubernetesnetworkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KubernetesNetworkConfig {
        /// Property [`IpFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-kubernetesnetworkconfig.html#cfn-eks-cluster-kubernetesnetworkconfig-ipfamily).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ip_family: Option<::Value<String>>,
        /// Property [`ServiceIpv4Cidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-kubernetesnetworkconfig.html#cfn-eks-cluster-kubernetesnetworkconfig-serviceipv4cidr).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub service_ipv4_cidr: Option<::Value<String>>,
        /// Property [`ServiceIpv6Cidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-kubernetesnetworkconfig.html#cfn-eks-cluster-kubernetesnetworkconfig-serviceipv6cidr).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub service_ipv6_cidr: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KubernetesNetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip_family) = self.ip_family {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpFamily", ip_family)?;
            }
            if let Some(ref service_ipv4_cidr) = self.service_ipv4_cidr {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceIpv4Cidr", service_ipv4_cidr)?;
            }
            if let Some(ref service_ipv6_cidr) = self.service_ipv6_cidr {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceIpv6Cidr", service_ipv6_cidr)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KubernetesNetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KubernetesNetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KubernetesNetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KubernetesNetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_family: Option<::Value<String>> = None;
                    let mut service_ipv4_cidr: Option<::Value<String>> = None;
                    let mut service_ipv6_cidr: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IpFamily" => {
                                ip_family = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceIpv4Cidr" => {
                                service_ipv4_cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceIpv6Cidr" => {
                                service_ipv6_cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KubernetesNetworkConfig {
                        ip_family: ip_family,
                        service_ipv4_cidr: service_ipv4_cidr,
                        service_ipv6_cidr: service_ipv6_cidr,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`ClusterLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-logging.html#cfn-eks-cluster-logging-clusterlogging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_logging: Option<::Value<ClusterLogging>>,
    }

    impl ::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cluster_logging) = self.cluster_logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterLogging", cluster_logging)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_logging: Option<::Value<ClusterLogging>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterLogging" => {
                                cluster_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        cluster_logging: cluster_logging,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.LoggingTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-loggingtypeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingTypeConfig {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-loggingtypeconfig.html#cfn-eks-cluster-loggingtypeconfig-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingTypeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingTypeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingTypeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingTypeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingTypeConfig {
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.Provider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-provider.html) property type.
    #[derive(Debug, Default)]
    pub struct Provider {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-provider.html#cfn-eks-cluster-provider-keyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Provider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Provider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Provider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Provider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Provider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Provider {
                        key_arn: key_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Cluster.ResourcesVpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcesVpcConfig {
        /// Property [`EndpointPrivateAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html#cfn-eks-cluster-resourcesvpcconfig-endpointprivateaccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_private_access: Option<::Value<bool>>,
        /// Property [`EndpointPublicAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html#cfn-eks-cluster-resourcesvpcconfig-endpointpublicaccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_public_access: Option<::Value<bool>>,
        /// Property [`PublicAccessCidrs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html#cfn-eks-cluster-resourcesvpcconfig-publicaccesscidrs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub public_access_cidrs: Option<::ValueList<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html#cfn-eks-cluster-resourcesvpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html#cfn-eks-cluster-resourcesvpcconfig-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ResourcesVpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint_private_access) = self.endpoint_private_access {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointPrivateAccess", endpoint_private_access)?;
            }
            if let Some(ref endpoint_public_access) = self.endpoint_public_access {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointPublicAccess", endpoint_public_access)?;
            }
            if let Some(ref public_access_cidrs) = self.public_access_cidrs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicAccessCidrs", public_access_cidrs)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcesVpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcesVpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcesVpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcesVpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_private_access: Option<::Value<bool>> = None;
                    let mut endpoint_public_access: Option<::Value<bool>> = None;
                    let mut public_access_cidrs: Option<::ValueList<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointPrivateAccess" => {
                                endpoint_private_access = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointPublicAccess" => {
                                endpoint_public_access = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PublicAccessCidrs" => {
                                public_access_cidrs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcesVpcConfig {
                        endpoint_private_access: endpoint_private_access,
                        endpoint_public_access: endpoint_public_access,
                        public_access_cidrs: public_access_cidrs,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fargate_profile {
    //! Property types for the `FargateProfile` resource.

    /// The [`AWS::EKS::FargateProfile.Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-label.html) property type.
    #[derive(Debug, Default)]
    pub struct Label {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-label.html#cfn-eks-fargateprofile-label-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-label.html#cfn-eks-fargateprofile-label-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Label {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Label {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Label, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Label;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Label")
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

                    Ok(Label {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::FargateProfile.Selector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-selector.html) property type.
    #[derive(Debug, Default)]
    pub struct Selector {
        /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-selector.html#cfn-eks-fargateprofile-selector-labels).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub labels: Option<::ValueList<Label>>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-selector.html#cfn-eks-fargateprofile-selector-namespace).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub namespace: ::Value<String>,
    }

    impl ::codec::SerializeValue for Selector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref labels) = self.labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", labels)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Selector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Selector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Selector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Selector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut labels: Option<::ValueList<Label>> = None;
                    let mut namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Labels" => {
                                labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Selector {
                        labels: labels,
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod identity_provider_config {
    //! Property types for the `IdentityProviderConfig` resource.

    /// The [`AWS::EKS::IdentityProviderConfig.OidcIdentityProviderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OidcIdentityProviderConfig {
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-clientid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`GroupsClaim`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-groupsclaim).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub groups_claim: Option<::Value<String>>,
        /// Property [`GroupsPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-groupsprefix).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub groups_prefix: Option<::Value<String>>,
        /// Property [`IssuerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-issuerurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub issuer_url: ::Value<String>,
        /// Property [`RequiredClaims`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-requiredclaims).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub required_claims: Option<::ValueList<RequiredClaim>>,
        /// Property [`UsernameClaim`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-usernameclaim).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub username_claim: Option<::Value<String>>,
        /// Property [`UsernamePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html#cfn-eks-identityproviderconfig-oidcidentityproviderconfig-usernameprefix).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub username_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OidcIdentityProviderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            if let Some(ref groups_claim) = self.groups_claim {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupsClaim", groups_claim)?;
            }
            if let Some(ref groups_prefix) = self.groups_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupsPrefix", groups_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IssuerUrl", &self.issuer_url)?;
            if let Some(ref required_claims) = self.required_claims {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequiredClaims", required_claims)?;
            }
            if let Some(ref username_claim) = self.username_claim {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameClaim", username_claim)?;
            }
            if let Some(ref username_prefix) = self.username_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernamePrefix", username_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OidcIdentityProviderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OidcIdentityProviderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OidcIdentityProviderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OidcIdentityProviderConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_id: Option<::Value<String>> = None;
                    let mut groups_claim: Option<::Value<String>> = None;
                    let mut groups_prefix: Option<::Value<String>> = None;
                    let mut issuer_url: Option<::Value<String>> = None;
                    let mut required_claims: Option<::ValueList<RequiredClaim>> = None;
                    let mut username_claim: Option<::Value<String>> = None;
                    let mut username_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupsClaim" => {
                                groups_claim = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupsPrefix" => {
                                groups_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IssuerUrl" => {
                                issuer_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequiredClaims" => {
                                required_claims = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsernameClaim" => {
                                username_claim = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsernamePrefix" => {
                                username_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OidcIdentityProviderConfig {
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        groups_claim: groups_claim,
                        groups_prefix: groups_prefix,
                        issuer_url: issuer_url.ok_or(::serde::de::Error::missing_field("IssuerUrl"))?,
                        required_claims: required_claims,
                        username_claim: username_claim,
                        username_prefix: username_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::IdentityProviderConfig.RequiredClaim`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-requiredclaim.html) property type.
    #[derive(Debug, Default)]
    pub struct RequiredClaim {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-requiredclaim.html#cfn-eks-identityproviderconfig-requiredclaim-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-requiredclaim.html#cfn-eks-identityproviderconfig-requiredclaim-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for RequiredClaim {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequiredClaim {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequiredClaim, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequiredClaim;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequiredClaim")
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

                    Ok(RequiredClaim {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod nodegroup {
    //! Property types for the `Nodegroup` resource.

    /// The [`AWS::EKS::Nodegroup.LaunchTemplateSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-launchtemplatespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplateSpecification {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-launchtemplatespecification.html#cfn-eks-nodegroup-launchtemplatespecification-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-launchtemplatespecification.html#cfn-eks-nodegroup-launchtemplatespecification-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-launchtemplatespecification.html#cfn-eks-nodegroup-launchtemplatespecification-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LaunchTemplateSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LaunchTemplateSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplateSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplateSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplateSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplateSpecification {
                        id: id,
                        name: name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Nodegroup.RemoteAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-remoteaccess.html) property type.
    #[derive(Debug, Default)]
    pub struct RemoteAccess {
        /// Property [`Ec2SshKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-remoteaccess.html#cfn-eks-nodegroup-remoteaccess-ec2sshkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_ssh_key: ::Value<String>,
        /// Property [`SourceSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-remoteaccess.html#cfn-eks-nodegroup-remoteaccess-sourcesecuritygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_security_groups: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RemoteAccess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2SshKey", &self.ec2_ssh_key)?;
            if let Some(ref source_security_groups) = self.source_security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroups", source_security_groups)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RemoteAccess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RemoteAccess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RemoteAccess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RemoteAccess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ec2_ssh_key: Option<::Value<String>> = None;
                    let mut source_security_groups: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ec2SshKey" => {
                                ec2_ssh_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceSecurityGroups" => {
                                source_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RemoteAccess {
                        ec2_ssh_key: ec2_ssh_key.ok_or(::serde::de::Error::missing_field("Ec2SshKey"))?,
                        source_security_groups: source_security_groups,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Nodegroup.ScalingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-scalingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ScalingConfig {
        /// Property [`DesiredSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-scalingconfig.html#cfn-eks-nodegroup-scalingconfig-desiredsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_size: Option<::Value<u32>>,
        /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-scalingconfig.html#cfn-eks-nodegroup-scalingconfig-maxsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_size: Option<::Value<u32>>,
        /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-scalingconfig.html#cfn-eks-nodegroup-scalingconfig-minsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ScalingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref desired_size) = self.desired_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredSize", desired_size)?;
            }
            if let Some(ref max_size) = self.max_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
            }
            if let Some(ref min_size) = self.min_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut desired_size: Option<::Value<u32>> = None;
                    let mut max_size: Option<::Value<u32>> = None;
                    let mut min_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredSize" => {
                                desired_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSize" => {
                                max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinSize" => {
                                min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConfig {
                        desired_size: desired_size,
                        max_size: max_size,
                        min_size: min_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Nodegroup.Taint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-taint.html) property type.
    #[derive(Debug, Default)]
    pub struct Taint {
        /// Property [`Effect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-taint.html#cfn-eks-nodegroup-taint-effect).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub effect: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-taint.html#cfn-eks-nodegroup-taint-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-taint.html#cfn-eks-nodegroup-taint-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Taint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref effect) = self.effect {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Effect", effect)?;
            }
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Taint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Taint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Taint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Taint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut effect: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Effect" => {
                                effect = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Taint {
                        effect: effect,
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EKS::Nodegroup.UpdateConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-updateconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UpdateConfig {
        /// Property [`MaxUnavailable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-updateconfig.html#cfn-eks-nodegroup-updateconfig-maxunavailable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_unavailable: Option<::Value<f64>>,
        /// Property [`MaxUnavailablePercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-updateconfig.html#cfn-eks-nodegroup-updateconfig-maxunavailablepercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_unavailable_percentage: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for UpdateConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_unavailable) = self.max_unavailable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUnavailable", max_unavailable)?;
            }
            if let Some(ref max_unavailable_percentage) = self.max_unavailable_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUnavailablePercentage", max_unavailable_percentage)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpdateConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpdateConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpdateConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpdateConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_unavailable: Option<::Value<f64>> = None;
                    let mut max_unavailable_percentage: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxUnavailable" => {
                                max_unavailable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxUnavailablePercentage" => {
                                max_unavailable_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpdateConfig {
                        max_unavailable: max_unavailable,
                        max_unavailable_percentage: max_unavailable_percentage,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
