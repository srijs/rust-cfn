//! Types for the `Cloud9` service.

/// The [`AWS::Cloud9::EnvironmentEC2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloud9-environmentec2.html) resource type.
#[derive(Debug)]
pub struct EnvironmentEC2 {
    properties: EnvironmentEC2Properties
}

/// Properties for the `EnvironmentEC2` resource.
#[derive(Debug)]
pub struct EnvironmentEC2Properties {
    /// Property `AutomaticStopTimeMinutes`.
    pub automatic_stop_time_minutes: Option<::Value<u32>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `InstanceType`.
    pub instance_type: ::Value<String>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `OwnerArn`.
    pub owner_arn: Option<::Value<String>>,
    /// Property `Repositories`.
    pub repositories: Option<::ValueList<self::environment_ec2::Repository>>,
    /// Property `SubnetId`.
    pub subnet_id: Option<::Value<String>>,
}

impl ::serde::Serialize for EnvironmentEC2Properties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref automatic_stop_time_minutes) = self.automatic_stop_time_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticStopTimeMinutes", automatic_stop_time_minutes)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref owner_arn) = self.owner_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerArn", owner_arn)?;
        }
        if let Some(ref repositories) = self.repositories {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Repositories", repositories)?;
        }
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentEC2Properties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentEC2Properties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentEC2Properties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentEC2Properties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut automatic_stop_time_minutes: Option<::Value<u32>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut owner_arn: Option<::Value<String>> = None;
                let mut repositories: Option<::ValueList<self::environment_ec2::Repository>> = None;
                let mut subnet_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutomaticStopTimeMinutes" => {
                            automatic_stop_time_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerArn" => {
                            owner_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Repositories" => {
                            repositories = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentEC2Properties {
                    automatic_stop_time_minutes: automatic_stop_time_minutes,
                    description: description,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    name: name,
                    owner_arn: owner_arn,
                    repositories: repositories,
                    subnet_id: subnet_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnvironmentEC2 {
    type Properties = EnvironmentEC2Properties;
    const TYPE: &'static str = "AWS::Cloud9::EnvironmentEC2";
    fn properties(&self) -> &EnvironmentEC2Properties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentEC2Properties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentEC2 {}

impl From<EnvironmentEC2Properties> for EnvironmentEC2 {
    fn from(properties: EnvironmentEC2Properties) -> EnvironmentEC2 {
        EnvironmentEC2 { properties }
    }
}

pub mod environment_ec2 {
    //! Property types for the `EnvironmentEC2` resource.

    /// The [`AWS::Cloud9::EnvironmentEC2.Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloud9-environmentec2-repository.html) property type.
    #[derive(Debug)]
    pub struct Repository {
        /// Property `PathComponent`.
        pub path_component: ::Value<String>,
        /// Property `RepositoryUrl`.
        pub repository_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for Repository {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathComponent", &self.path_component)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryUrl", &self.repository_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Repository {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Repository, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Repository;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Repository")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path_component: Option<::Value<String>> = None;
                    let mut repository_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PathComponent" => {
                                path_component = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepositoryUrl" => {
                                repository_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Repository {
                        path_component: path_component.ok_or(::serde::de::Error::missing_field("PathComponent"))?,
                        repository_url: repository_url.ok_or(::serde::de::Error::missing_field("RepositoryUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
