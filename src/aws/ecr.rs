//! Types for the `ECR` service.

/// The [`AWS::ECR::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html) resource type.
#[derive(Debug)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Debug)]
pub struct RepositoryProperties {
    /// Property `LifecyclePolicy`.
    pub lifecycle_policy: Option<::Value<self::repository::LifecyclePolicy>>,
    /// Property `RepositoryName`.
    pub repository_name: Option<::Value<String>>,
    /// Property `RepositoryPolicyText`.
    pub repository_policy_text: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for RepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref lifecycle_policy) = self.lifecycle_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecyclePolicy", lifecycle_policy)?;
        }
        if let Some(ref repository_name) = self.repository_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", repository_name)?;
        }
        if let Some(ref repository_policy_text) = self.repository_policy_text {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryPolicyText", repository_policy_text)?;
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
                let mut lifecycle_policy = None;
                let mut repository_name = None;
                let mut repository_policy_text = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LifecyclePolicy" => {
                            lifecycle_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RepositoryName" => {
                            repository_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RepositoryPolicyText" => {
                            repository_policy_text = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RepositoryProperties {
                    lifecycle_policy: lifecycle_policy,
                    repository_name: repository_name,
                    repository_policy_text: repository_policy_text,
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

pub mod repository {
    //! Property types for the `Repository` resource.

    /// The [`AWS::ECR::Repository.LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html) property type.
    #[derive(Debug)]
    pub struct LifecyclePolicy {
        /// Property `LifecyclePolicyText`.
        pub lifecycle_policy_text: Option<::Value<String>>,
        /// Property `RegistryId`.
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
                    let mut lifecycle_policy_text = None;
                    let mut registry_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LifecyclePolicyText" => {
                                lifecycle_policy_text = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RegistryId" => {
                                registry_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
