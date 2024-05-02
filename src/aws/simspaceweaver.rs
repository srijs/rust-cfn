//! Types for the `SimSpaceWeaver` service.

/// The [`AWS::SimSpaceWeaver::Simulation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html) resource type.
#[derive(Debug, Default)]
pub struct Simulation {
    properties: SimulationProperties
}

/// Properties for the `Simulation` resource.
#[derive(Debug, Default)]
pub struct SimulationProperties {
    /// Property [`MaximumDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html#cfn-simspaceweaver-simulation-maximumduration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub maximum_duration: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html#cfn-simspaceweaver-simulation-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html#cfn-simspaceweaver-simulation-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`SchemaS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html#cfn-simspaceweaver-simulation-schemas3location).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_s3_location: Option<::Value<self::simulation::S3Location>>,
    /// Property [`SnapshotS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html#cfn-simspaceweaver-simulation-snapshots3location).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_s3_location: Option<::Value<self::simulation::S3Location>>,
}

impl ::serde::Serialize for SimulationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref maximum_duration) = self.maximum_duration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumDuration", maximum_duration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref schema_s3_location) = self.schema_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaS3Location", schema_s3_location)?;
        }
        if let Some(ref snapshot_s3_location) = self.snapshot_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotS3Location", snapshot_s3_location)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SimulationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SimulationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SimulationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SimulationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut maximum_duration: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut schema_s3_location: Option<::Value<self::simulation::S3Location>> = None;
                let mut snapshot_s3_location: Option<::Value<self::simulation::S3Location>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MaximumDuration" => {
                            maximum_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaS3Location" => {
                            schema_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotS3Location" => {
                            snapshot_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SimulationProperties {
                    maximum_duration: maximum_duration,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    schema_s3_location: schema_s3_location,
                    snapshot_s3_location: snapshot_s3_location,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Simulation {
    type Properties = SimulationProperties;
    const TYPE: &'static str = "AWS::SimSpaceWeaver::Simulation";
    fn properties(&self) -> &SimulationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SimulationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Simulation {}

impl From<SimulationProperties> for Simulation {
    fn from(properties: SimulationProperties) -> Simulation {
        Simulation { properties }
    }
}

pub mod simulation {
    //! Property types for the `Simulation` resource.

    /// The [`AWS::SimSpaceWeaver::Simulation.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simspaceweaver-simulation-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simspaceweaver-simulation-s3location.html#cfn-simspaceweaver-simulation-s3location-bucketname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`ObjectKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simspaceweaver-simulation-s3location.html#cfn-simspaceweaver-simulation-s3location-objectkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectKey", &self.object_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut object_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectKey" => {
                                object_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        object_key: object_key.ok_or(::serde::de::Error::missing_field("ObjectKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
