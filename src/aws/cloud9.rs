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
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticStopTimeMinutes", &self.automatic_stop_time_minutes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerArn", &self.owner_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Repositories", &self.repositories)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
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
                let mut automatic_stop_time_minutes = None;
                let mut description = None;
                let mut instance_type = None;
                let mut name = None;
                let mut owner_arn = None;
                let mut repositories = None;
                let mut subnet_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutomaticStopTimeMinutes" => {
                            automatic_stop_time_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OwnerArn" => {
                            owner_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Repositories" => {
                            repositories = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for EnvironmentEC2 {
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
            #[allow(unused_mut)]
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
                    let mut path_component = None;
                    let mut repository_url = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PathComponent" => {
                                path_component = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RepositoryUrl" => {
                                repository_url = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
