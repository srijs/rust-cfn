//! Types for the `ECR` service.

/// The [`AWS::ECR::PublicRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html) resource type.
#[derive(Debug, Default)]
pub struct PublicRepository {
    properties: PublicRepositoryProperties
}

/// Properties for the `PublicRepository` resource.
#[derive(Debug, Default)]
pub struct PublicRepositoryProperties {
    /// Property [`RepositoryCatalogData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html#cfn-ecr-publicrepository-repositorycatalogdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_catalog_data: Option<::Value<::json::Value>>,
    /// Property [`RepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html#cfn-ecr-publicrepository-repositoryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub repository_name: Option<::Value<String>>,
    /// Property [`RepositoryPolicyText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html#cfn-ecr-publicrepository-repositorypolicytext).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_policy_text: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html#cfn-ecr-publicrepository-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PublicRepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref repository_catalog_data) = self.repository_catalog_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryCatalogData", repository_catalog_data)?;
        }
        if let Some(ref repository_name) = self.repository_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", repository_name)?;
        }
        if let Some(ref repository_policy_text) = self.repository_policy_text {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryPolicyText", repository_policy_text)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublicRepositoryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicRepositoryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublicRepositoryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublicRepositoryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut repository_catalog_data: Option<::Value<::json::Value>> = None;
                let mut repository_name: Option<::Value<String>> = None;
                let mut repository_policy_text: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RepositoryCatalogData" => {
                            repository_catalog_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryName" => {
                            repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryPolicyText" => {
                            repository_policy_text = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublicRepositoryProperties {
                    repository_catalog_data: repository_catalog_data,
                    repository_name: repository_name,
                    repository_policy_text: repository_policy_text,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PublicRepository {
    type Properties = PublicRepositoryProperties;
    const TYPE: &'static str = "AWS::ECR::PublicRepository";
    fn properties(&self) -> &PublicRepositoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublicRepositoryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PublicRepository {}

impl From<PublicRepositoryProperties> for PublicRepository {
    fn from(properties: PublicRepositoryProperties) -> PublicRepository {
        PublicRepository { properties }
    }
}

/// The [`AWS::ECR::RegistryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-registrypolicy.html) resource type.
#[derive(Debug, Default)]
pub struct RegistryPolicy {
    properties: RegistryPolicyProperties
}

/// Properties for the `RegistryPolicy` resource.
#[derive(Debug, Default)]
pub struct RegistryPolicyProperties {
    /// Property [`PolicyText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-registrypolicy.html#cfn-ecr-registrypolicy-policytext).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_text: ::Value<::json::Value>,
}

impl ::serde::Serialize for RegistryPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyText", &self.policy_text)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RegistryPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RegistryPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RegistryPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RegistryPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_text: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyText" => {
                            policy_text = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RegistryPolicyProperties {
                    policy_text: policy_text.ok_or(::serde::de::Error::missing_field("PolicyText"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RegistryPolicy {
    type Properties = RegistryPolicyProperties;
    const TYPE: &'static str = "AWS::ECR::RegistryPolicy";
    fn properties(&self) -> &RegistryPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RegistryPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RegistryPolicy {}

impl From<RegistryPolicyProperties> for RegistryPolicy {
    fn from(properties: RegistryPolicyProperties) -> RegistryPolicy {
        RegistryPolicy { properties }
    }
}

/// The [`AWS::ECR::ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-replicationconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationConfiguration {
    properties: ReplicationConfigurationProperties
}

/// Properties for the `ReplicationConfiguration` resource.
#[derive(Debug, Default)]
pub struct ReplicationConfigurationProperties {
    /// Property [`ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-replicationconfiguration.html#cfn-ecr-replicationconfiguration-replicationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_configuration: ::Value<self::replication_configuration::ReplicationConfiguration>,
}

impl ::serde::Serialize for ReplicationConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationConfiguration", &self.replication_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut replication_configuration: Option<::Value<self::replication_configuration::ReplicationConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ReplicationConfiguration" => {
                            replication_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationConfigurationProperties {
                    replication_configuration: replication_configuration.ok_or(::serde::de::Error::missing_field("ReplicationConfiguration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationConfiguration {
    type Properties = ReplicationConfigurationProperties;
    const TYPE: &'static str = "AWS::ECR::ReplicationConfiguration";
    fn properties(&self) -> &ReplicationConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationConfiguration {}

impl From<ReplicationConfigurationProperties> for ReplicationConfiguration {
    fn from(properties: ReplicationConfigurationProperties) -> ReplicationConfiguration {
        ReplicationConfiguration { properties }
    }
}

/// The [`AWS::ECR::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html) resource type.
#[derive(Debug, Default)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Debug, Default)]
pub struct RepositoryProperties {
    /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-encryptionconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_configuration: Option<::Value<self::repository::EncryptionConfiguration>>,
    /// Property [`ImageScanningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-imagescanningconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_scanning_configuration: Option<::Value<self::repository::ImageScanningConfiguration>>,
    /// Property [`ImageTagMutability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-imagetagmutability).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_tag_mutability: Option<::Value<String>>,
    /// Property [`LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-lifecyclepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_policy: Option<::Value<self::repository::LifecyclePolicy>>,
    /// Property [`RepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-repositoryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub repository_name: Option<::Value<String>>,
    /// Property [`RepositoryPolicyText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-repositorypolicytext).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_policy_text: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html#cfn-ecr-repository-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref encryption_configuration) = self.encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
        }
        if let Some(ref image_scanning_configuration) = self.image_scanning_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageScanningConfiguration", image_scanning_configuration)?;
        }
        if let Some(ref image_tag_mutability) = self.image_tag_mutability {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageTagMutability", image_tag_mutability)?;
        }
        if let Some(ref lifecycle_policy) = self.lifecycle_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecyclePolicy", lifecycle_policy)?;
        }
        if let Some(ref repository_name) = self.repository_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", repository_name)?;
        }
        if let Some(ref repository_policy_text) = self.repository_policy_text {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryPolicyText", repository_policy_text)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RepositoryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RepositoryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RepositoryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RepositoryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut encryption_configuration: Option<::Value<self::repository::EncryptionConfiguration>> = None;
                let mut image_scanning_configuration: Option<::Value<self::repository::ImageScanningConfiguration>> = None;
                let mut image_tag_mutability: Option<::Value<String>> = None;
                let mut lifecycle_policy: Option<::Value<self::repository::LifecyclePolicy>> = None;
                let mut repository_name: Option<::Value<String>> = None;
                let mut repository_policy_text: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EncryptionConfiguration" => {
                            encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageScanningConfiguration" => {
                            image_scanning_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageTagMutability" => {
                            image_tag_mutability = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecyclePolicy" => {
                            lifecycle_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryName" => {
                            repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryPolicyText" => {
                            repository_policy_text = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RepositoryProperties {
                    encryption_configuration: encryption_configuration,
                    image_scanning_configuration: image_scanning_configuration,
                    image_tag_mutability: image_tag_mutability,
                    lifecycle_policy: lifecycle_policy,
                    repository_name: repository_name,
                    repository_policy_text: repository_policy_text,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Repository {
    type Properties = RepositoryProperties;
    const TYPE: &'static str = "AWS::ECR::Repository";
    fn properties(&self) -> &RepositoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RepositoryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Repository {}

impl From<RepositoryProperties> for Repository {
    fn from(properties: RepositoryProperties) -> Repository {
        Repository { properties }
    }
}

pub mod replication_configuration {
    //! Property types for the `ReplicationConfiguration` resource.

    /// The [`AWS::ECR::ReplicationConfiguration.ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationConfiguration {
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationconfiguration.html#cfn-ecr-replicationconfiguration-replicationconfiguration-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<ReplicationRule>,
    }

    impl ::codec::SerializeValue for ReplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rules: Option<::ValueList<ReplicationRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationConfiguration {
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECR::ReplicationConfiguration.ReplicationDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationDestination {
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationdestination.html#cfn-ecr-replicationconfiguration-replicationdestination-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
        /// Property [`RegistryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationdestination.html#cfn-ecr-replicationconfiguration-replicationdestination-registryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReplicationDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryId", &self.registry_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut region: Option<::Value<String>> = None;
                    let mut registry_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistryId" => {
                                registry_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationDestination {
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                        registry_id: registry_id.ok_or(::serde::de::Error::missing_field("RegistryId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECR::ReplicationConfiguration.ReplicationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationrule.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationRule {
        /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationrule.html#cfn-ecr-replicationconfiguration-replicationrule-destinations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destinations: ::ValueList<ReplicationDestination>,
    }

    impl ::codec::SerializeValue for ReplicationRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", &self.destinations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destinations: Option<::ValueList<ReplicationDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destinations" => {
                                destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationRule {
                        destinations: destinations.ok_or(::serde::de::Error::missing_field("Destinations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod repository {
    //! Property types for the `Repository` resource.

    /// The [`AWS::ECR::Repository.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfiguration {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-encryptionconfiguration.html#cfn-ecr-repository-encryptionconfiguration-encryptiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encryption_type: ::Value<String>,
        /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-encryptionconfiguration.html#cfn-ecr-repository-encryptionconfiguration-kmskey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            if let Some(ref kms_key) = self.kms_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", kms_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut kms_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKey" => {
                                kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                        kms_key: kms_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECR::Repository.ImageScanningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-imagescanningconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageScanningConfiguration {
        /// Property [`ScanOnPush`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-imagescanningconfiguration.html#cfn-ecr-repository-imagescanningconfiguration-scanonpush).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scan_on_push: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ImageScanningConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref scan_on_push) = self.scan_on_push {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanOnPush", scan_on_push)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageScanningConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageScanningConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageScanningConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageScanningConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scan_on_push: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScanOnPush" => {
                                scan_on_push = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageScanningConfiguration {
                        scan_on_push: scan_on_push,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECR::Repository.LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct LifecyclePolicy {
        /// Property [`LifecyclePolicyText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html#cfn-ecr-repository-lifecyclepolicy-lifecyclepolicytext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle_policy_text: Option<::Value<String>>,
        /// Property [`RegistryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html#cfn-ecr-repository-lifecyclepolicy-registryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LifecyclePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lifecycle_policy_text) = self.lifecycle_policy_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecyclePolicyText", lifecycle_policy_text)?;
            }
            if let Some(ref registry_id) = self.registry_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryId", registry_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecyclePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecyclePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecyclePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecyclePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lifecycle_policy_text: Option<::Value<String>> = None;
                    let mut registry_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LifecyclePolicyText" => {
                                lifecycle_policy_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistryId" => {
                                registry_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecyclePolicy {
                        lifecycle_policy_text: lifecycle_policy_text,
                        registry_id: registry_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
