//! Types for the `Rekognition` service.

/// The [`AWS::Rekognition::Collection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html) resource type.
#[derive(Debug, Default)]
pub struct Collection {
    properties: CollectionProperties
}

/// Properties for the `Collection` resource.
#[derive(Debug, Default)]
pub struct CollectionProperties {
    /// Property [`CollectionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html#cfn-rekognition-collection-collectionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub collection_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html#cfn-rekognition-collection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CollectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionId", &self.collection_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CollectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CollectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CollectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CollectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collection_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CollectionId" => {
                            collection_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CollectionProperties {
                    collection_id: collection_id.ok_or(::serde::de::Error::missing_field("CollectionId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Collection {
    type Properties = CollectionProperties;
    const TYPE: &'static str = "AWS::Rekognition::Collection";
    fn properties(&self) -> &CollectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CollectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Collection {}

impl From<CollectionProperties> for Collection {
    fn from(properties: CollectionProperties) -> Collection {
        Collection { properties }
    }
}

/// The [`AWS::Rekognition::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-project.html#cfn-rekognition-project-projectname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_name: ::Value<String>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", &self.project_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut project_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    project_name: project_name.ok_or(::serde::de::Error::missing_field("ProjectName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::Rekognition::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}
