//! Types for the `Athena` service.

/// The [`AWS::Athena::NamedQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html) resource type.
#[derive(Debug)]
pub struct NamedQuery {
    properties: NamedQueryProperties
}

/// Properties for the `NamedQuery` resource.
#[derive(Debug)]
pub struct NamedQueryProperties {
    /// Property `Database`.
    pub database: ::Value<String>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `QueryString`.
    pub query_string: ::Value<String>,
}

impl ::serde::Serialize for NamedQueryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", &self.query_string)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NamedQueryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NamedQueryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NamedQueryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NamedQueryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut database = None;
                let mut description = None;
                let mut name = None;
                let mut query_string = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Database" => {
                            database = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "QueryString" => {
                            query_string = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NamedQueryProperties {
                    database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                    description: description,
                    name: name,
                    query_string: query_string.ok_or(::serde::de::Error::missing_field("QueryString"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for NamedQuery {
    type Properties = NamedQueryProperties;
    const TYPE: &'static str = "AWS::Athena::NamedQuery";
    fn properties(&self) -> &NamedQueryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NamedQueryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NamedQuery {}

impl From<NamedQueryProperties> for NamedQuery {
    fn from(properties: NamedQueryProperties) -> NamedQuery {
        NamedQuery { properties }
    }
}
