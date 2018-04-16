//! Types for the `GameLift` service.

/// The [`AWS::GameLift::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html) resource type.
#[derive(Debug, Default)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug, Default)]
pub struct AliasProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html#cfn-gamelift-alias-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html#cfn-gamelift-alias-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoutingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html#cfn-gamelift-alias-routingstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_strategy: ::Value<self::alias::RoutingStrategy>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingStrategy", &self.routing_strategy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut routing_strategy: Option<::Value<self::alias::RoutingStrategy>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingStrategy" => {
                            routing_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    routing_strategy: routing_strategy.ok_or(::serde::de::Error::missing_field("RoutingStrategy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::GameLift::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::GameLift::Build`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html) resource type.
#[derive(Debug, Default)]
pub struct Build {
    properties: BuildProperties
}

/// Properties for the `Build` resource.
#[derive(Debug, Default)]
pub struct BuildProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html#cfn-gamelift-build-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`StorageLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html#cfn-gamelift-build-storagelocation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_location: Option<::Value<self::build::S3Location>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html#cfn-gamelift-build-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for BuildProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref storage_location) = self.storage_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageLocation", storage_location)?;
        }
        if let Some(ref version) = self.version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BuildProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BuildProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BuildProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BuildProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut storage_location: Option<::Value<self::build::S3Location>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageLocation" => {
                            storage_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BuildProperties {
                    name: name,
                    storage_location: storage_location,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Build {
    type Properties = BuildProperties;
    const TYPE: &'static str = "AWS::GameLift::Build";
    fn properties(&self) -> &BuildProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BuildProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Build {}

impl From<BuildProperties> for Build {
    fn from(properties: BuildProperties) -> Build {
        Build { properties }
    }
}

/// The [`AWS::GameLift::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html) resource type.
#[derive(Debug, Default)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug, Default)]
pub struct FleetProperties {
    /// Property [`BuildId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-buildid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub build_id: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DesiredEC2Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-desiredec2instances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_ec2_instances: ::Value<u32>,
    /// Property [`EC2InboundPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-ec2inboundpermissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_inbound_permissions: Option<::ValueList<self::fleet::IpPermission>>,
    /// Property [`EC2InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-ec2instancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_instance_type: ::Value<String>,
    /// Property [`LogPaths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-logpaths).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_paths: Option<::ValueList<String>>,
    /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-maxsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_size: Option<::Value<u32>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ServerLaunchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-serverlaunchparameters).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_launch_parameters: Option<::Value<String>>,
    /// Property [`ServerLaunchPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-serverlaunchpath).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_launch_path: ::Value<String>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildId", &self.build_id)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredEC2Instances", &self.desired_ec2_instances)?;
        if let Some(ref ec2_inbound_permissions) = self.ec2_inbound_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InboundPermissions", ec2_inbound_permissions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InstanceType", &self.ec2_instance_type)?;
        if let Some(ref log_paths) = self.log_paths {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPaths", log_paths)?;
        }
        if let Some(ref max_size) = self.max_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
        }
        if let Some(ref min_size) = self.min_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref server_launch_parameters) = self.server_launch_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerLaunchParameters", server_launch_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerLaunchPath", &self.server_launch_path)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FleetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FleetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FleetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FleetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut build_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut desired_ec2_instances: Option<::Value<u32>> = None;
                let mut ec2_inbound_permissions: Option<::ValueList<self::fleet::IpPermission>> = None;
                let mut ec2_instance_type: Option<::Value<String>> = None;
                let mut log_paths: Option<::ValueList<String>> = None;
                let mut max_size: Option<::Value<u32>> = None;
                let mut min_size: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut server_launch_parameters: Option<::Value<String>> = None;
                let mut server_launch_path: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BuildId" => {
                            build_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredEC2Instances" => {
                            desired_ec2_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2InboundPermissions" => {
                            ec2_inbound_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2InstanceType" => {
                            ec2_instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogPaths" => {
                            log_paths = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSize" => {
                            max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerLaunchParameters" => {
                            server_launch_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerLaunchPath" => {
                            server_launch_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    build_id: build_id.ok_or(::serde::de::Error::missing_field("BuildId"))?,
                    description: description,
                    desired_ec2_instances: desired_ec2_instances.ok_or(::serde::de::Error::missing_field("DesiredEC2Instances"))?,
                    ec2_inbound_permissions: ec2_inbound_permissions,
                    ec2_instance_type: ec2_instance_type.ok_or(::serde::de::Error::missing_field("EC2InstanceType"))?,
                    log_paths: log_paths,
                    max_size: max_size,
                    min_size: min_size,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    server_launch_parameters: server_launch_parameters,
                    server_launch_path: server_launch_path.ok_or(::serde::de::Error::missing_field("ServerLaunchPath"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::GameLift::Fleet";
    fn properties(&self) -> &FleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Fleet {}

impl From<FleetProperties> for Fleet {
    fn from(properties: FleetProperties) -> Fleet {
        Fleet { properties }
    }
}

pub mod alias {
    //! Property types for the `Alias` resource.

    /// The [`AWS::GameLift::Alias.RoutingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct RoutingStrategy {
        /// Property [`FleetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html#cfn-gamelift-alias-routingstrategy-fleetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fleet_id: Option<::Value<String>>,
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html#cfn-gamelift-alias-routingstrategy-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html#cfn-gamelift-alias-routingstrategy-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for RoutingStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref fleet_id) = self.fleet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetId", fleet_id)?;
            }
            if let Some(ref message) = self.message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fleet_id: Option<::Value<String>> = None;
                    let mut message: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FleetId" => {
                                fleet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingStrategy {
                        fleet_id: fleet_id,
                        message: message,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod build {
    //! Property types for the `Build` resource.

    /// The [`AWS::GameLift::Build.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html#cfn-gamelift-build-storage-bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html#cfn-gamelift-build-storage-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html#cfn-gamelift-build-storage-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
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
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fleet {
    //! Property types for the `Fleet` resource.

    /// The [`AWS::GameLift::Fleet.IpPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html) property type.
    #[derive(Debug, Default)]
    pub struct IpPermission {
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html#cfn-gamelift-fleet-ec2inboundpermissions-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: ::Value<u32>,
        /// Property [`IpRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html#cfn-gamelift-fleet-ec2inboundpermissions-iprange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_range: ::Value<String>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html#cfn-gamelift-fleet-ec2inboundpermissions-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html#cfn-gamelift-fleet-ec2inboundpermissions-toport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub to_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for IpPermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpRange", &self.ip_range)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpPermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpPermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpPermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpPermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from_port: Option<::Value<u32>> = None;
                    let mut ip_range: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut to_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FromPort" => {
                                from_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IpRange" => {
                                ip_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ToPort" => {
                                to_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IpPermission {
                        from_port: from_port.ok_or(::serde::de::Error::missing_field("FromPort"))?,
                        ip_range: ip_range.ok_or(::serde::de::Error::missing_field("IpRange"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        to_port: to_port.ok_or(::serde::de::Error::missing_field("ToPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
