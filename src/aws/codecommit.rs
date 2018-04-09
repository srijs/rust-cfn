//! Types for the `CodeCommit` service.

/// The [`AWS::CodeCommit::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codecommit-repository.html) resource type.
#[derive(Debug)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Debug)]
pub struct RepositoryProperties {
    /// Property `RepositoryDescription`.
    pub repository_description: Option<::Value<String>>,
    /// Property `RepositoryName`.
    pub repository_name: ::Value<String>,
    /// Property `Triggers`.
    pub triggers: Option<::ValueList<self::repository::RepositoryTrigger>>,
}

impl ::serde::Serialize for RepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryDescription", &self.repository_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", &self.repository_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Triggers", &self.triggers)?;
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
                let mut repository_description = None;
                let mut repository_name = None;
                let mut triggers = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RepositoryDescription" => {
                            repository_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RepositoryName" => {
                            repository_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Triggers" => {
                            triggers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RepositoryProperties {
                    repository_description: repository_description,
                    repository_name: repository_name.ok_or(::serde::de::Error::missing_field("RepositoryName"))?,
                    triggers: triggers,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Repository {
    type Properties = RepositoryProperties;
    const TYPE: &'static str = "AWS::CodeCommit::Repository";
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

    /// The [`AWS::CodeCommit::Repository.RepositoryTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-repositorytrigger.html) property type.
    #[derive(Debug)]
    pub struct RepositoryTrigger {
        /// Property `Branches`.
        pub branches: Option<::ValueList<String>>,
        /// Property `CustomData`.
        pub custom_data: Option<::Value<String>>,
        /// Property `DestinationArn`.
        pub destination_arn: Option<::Value<String>>,
        /// Property `Events`.
        pub events: Option<::ValueList<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RepositoryTrigger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Branches", &self.branches)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomData", &self.custom_data)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", &self.destination_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", &self.events)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepositoryTrigger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepositoryTrigger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepositoryTrigger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepositoryTrigger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut branches = None;
                    let mut custom_data = None;
                    let mut destination_arn = None;
                    let mut events = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Branches" => {
                                branches = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CustomData" => {
                                custom_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DestinationArn" => {
                                destination_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Events" => {
                                events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RepositoryTrigger {
                        branches: branches,
                        custom_data: custom_data,
                        destination_arn: destination_arn,
                        events: events,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
