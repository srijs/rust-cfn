//! Types for the `HealthImaging` service.

/// The [`AWS::HealthImaging::Datastore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthimaging-datastore.html) resource type.
#[derive(Debug, Default)]
pub struct Datastore {
    properties: DatastoreProperties
}

/// Properties for the `Datastore` resource.
#[derive(Debug, Default)]
pub struct DatastoreProperties {
    /// Property [`DatastoreName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthimaging-datastore.html#cfn-healthimaging-datastore-datastorename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub datastore_name: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthimaging-datastore.html#cfn-healthimaging-datastore-kmskeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthimaging-datastore.html#cfn-healthimaging-datastore-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for DatastoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref datastore_name) = self.datastore_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreName", datastore_name)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatastoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatastoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatastoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatastoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut datastore_name: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatastoreName" => {
                            datastore_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatastoreProperties {
                    datastore_name: datastore_name,
                    kms_key_arn: kms_key_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Datastore {
    type Properties = DatastoreProperties;
    const TYPE: &'static str = "AWS::HealthImaging::Datastore";
    fn properties(&self) -> &DatastoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatastoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Datastore {}

impl From<DatastoreProperties> for Datastore {
    fn from(properties: DatastoreProperties) -> Datastore {
        Datastore { properties }
    }
}
