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
    /// Property [`OperatingSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html#cfn-gamelift-build-operatingsystem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub operating_system: Option<::Value<String>>,
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
        if let Some(ref operating_system) = self.operating_system {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatingSystem", operating_system)?;
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
                let mut operating_system: Option<::Value<String>> = None;
                let mut storage_location: Option<::Value<self::build::S3Location>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OperatingSystem" => {
                            operating_system = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    operating_system: operating_system,
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
    pub build_id: Option<::Value<String>>,
    /// Property [`CertificateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-certificateconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_configuration: Option<::Value<self::fleet::CertificateConfiguration>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DesiredEC2Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-desiredec2instances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_ec2_instances: Option<::Value<u32>>,
    /// Property [`EC2InboundPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-ec2inboundpermissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_inbound_permissions: Option<::ValueList<self::fleet::IpPermission>>,
    /// Property [`EC2InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-ec2instancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_instance_type: Option<::Value<String>>,
    /// Property [`FleetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-fleettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fleet_type: Option<::Value<String>>,
    /// Property [`InstanceRoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-instancerolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_role_arn: Option<::Value<String>>,
    /// Property [`Locations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-locations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub locations: Option<::ValueList<self::fleet::LocationConfiguration>>,
    /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-maxsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_size: Option<::Value<u32>>,
    /// Property [`MetricGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-metricgroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_groups: Option<::ValueList<String>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NewGameSessionProtectionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-newgamesessionprotectionpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub new_game_session_protection_policy: Option<::Value<String>>,
    /// Property [`PeerVpcAwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-peervpcawsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub peer_vpc_aws_account_id: Option<::Value<String>>,
    /// Property [`PeerVpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-peervpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub peer_vpc_id: Option<::Value<String>>,
    /// Property [`ResourceCreationLimitPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-resourcecreationlimitpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_creation_limit_policy: Option<::Value<self::fleet::ResourceCreationLimitPolicy>>,
    /// Property [`RuntimeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-runtimeconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub runtime_configuration: Option<::Value<self::fleet::RuntimeConfiguration>>,
    /// Property [`ScriptId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html#cfn-gamelift-fleet-scriptid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub script_id: Option<::Value<String>>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref build_id) = self.build_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildId", build_id)?;
        }
        if let Some(ref certificate_configuration) = self.certificate_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateConfiguration", certificate_configuration)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref desired_ec2_instances) = self.desired_ec2_instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredEC2Instances", desired_ec2_instances)?;
        }
        if let Some(ref ec2_inbound_permissions) = self.ec2_inbound_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InboundPermissions", ec2_inbound_permissions)?;
        }
        if let Some(ref ec2_instance_type) = self.ec2_instance_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InstanceType", ec2_instance_type)?;
        }
        if let Some(ref fleet_type) = self.fleet_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetType", fleet_type)?;
        }
        if let Some(ref instance_role_arn) = self.instance_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRoleARN", instance_role_arn)?;
        }
        if let Some(ref locations) = self.locations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Locations", locations)?;
        }
        if let Some(ref max_size) = self.max_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
        }
        if let Some(ref metric_groups) = self.metric_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricGroups", metric_groups)?;
        }
        if let Some(ref min_size) = self.min_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref new_game_session_protection_policy) = self.new_game_session_protection_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewGameSessionProtectionPolicy", new_game_session_protection_policy)?;
        }
        if let Some(ref peer_vpc_aws_account_id) = self.peer_vpc_aws_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerVpcAwsAccountId", peer_vpc_aws_account_id)?;
        }
        if let Some(ref peer_vpc_id) = self.peer_vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerVpcId", peer_vpc_id)?;
        }
        if let Some(ref resource_creation_limit_policy) = self.resource_creation_limit_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceCreationLimitPolicy", resource_creation_limit_policy)?;
        }
        if let Some(ref runtime_configuration) = self.runtime_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeConfiguration", runtime_configuration)?;
        }
        if let Some(ref script_id) = self.script_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScriptId", script_id)?;
        }
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
                let mut certificate_configuration: Option<::Value<self::fleet::CertificateConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut desired_ec2_instances: Option<::Value<u32>> = None;
                let mut ec2_inbound_permissions: Option<::ValueList<self::fleet::IpPermission>> = None;
                let mut ec2_instance_type: Option<::Value<String>> = None;
                let mut fleet_type: Option<::Value<String>> = None;
                let mut instance_role_arn: Option<::Value<String>> = None;
                let mut locations: Option<::ValueList<self::fleet::LocationConfiguration>> = None;
                let mut max_size: Option<::Value<u32>> = None;
                let mut metric_groups: Option<::ValueList<String>> = None;
                let mut min_size: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut new_game_session_protection_policy: Option<::Value<String>> = None;
                let mut peer_vpc_aws_account_id: Option<::Value<String>> = None;
                let mut peer_vpc_id: Option<::Value<String>> = None;
                let mut resource_creation_limit_policy: Option<::Value<self::fleet::ResourceCreationLimitPolicy>> = None;
                let mut runtime_configuration: Option<::Value<self::fleet::RuntimeConfiguration>> = None;
                let mut script_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BuildId" => {
                            build_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateConfiguration" => {
                            certificate_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "FleetType" => {
                            fleet_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceRoleARN" => {
                            instance_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Locations" => {
                            locations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSize" => {
                            max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricGroups" => {
                            metric_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NewGameSessionProtectionPolicy" => {
                            new_game_session_protection_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PeerVpcAwsAccountId" => {
                            peer_vpc_aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PeerVpcId" => {
                            peer_vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceCreationLimitPolicy" => {
                            resource_creation_limit_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimeConfiguration" => {
                            runtime_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScriptId" => {
                            script_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    build_id: build_id,
                    certificate_configuration: certificate_configuration,
                    description: description,
                    desired_ec2_instances: desired_ec2_instances,
                    ec2_inbound_permissions: ec2_inbound_permissions,
                    ec2_instance_type: ec2_instance_type,
                    fleet_type: fleet_type,
                    instance_role_arn: instance_role_arn,
                    locations: locations,
                    max_size: max_size,
                    metric_groups: metric_groups,
                    min_size: min_size,
                    name: name,
                    new_game_session_protection_policy: new_game_session_protection_policy,
                    peer_vpc_aws_account_id: peer_vpc_aws_account_id,
                    peer_vpc_id: peer_vpc_id,
                    resource_creation_limit_policy: resource_creation_limit_policy,
                    runtime_configuration: runtime_configuration,
                    script_id: script_id,
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

/// The [`AWS::GameLift::GameServerGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html) resource type.
#[derive(Debug, Default)]
pub struct GameServerGroup {
    properties: GameServerGroupProperties
}

/// Properties for the `GameServerGroup` resource.
#[derive(Debug, Default)]
pub struct GameServerGroupProperties {
    /// Property [`AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-autoscalingpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_scaling_policy: Option<::Value<self::game_server_group::AutoScalingPolicy>>,
    /// Property [`BalancingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-balancingstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub balancing_strategy: Option<::Value<String>>,
    /// Property [`DeleteOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-deleteoption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_option: Option<::Value<String>>,
    /// Property [`GameServerGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-gameservergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub game_server_group_name: ::Value<String>,
    /// Property [`GameServerProtectionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-gameserverprotectionpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub game_server_protection_policy: Option<::Value<String>>,
    /// Property [`InstanceDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-instancedefinitions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_definitions: ::ValueList<self::game_server_group::InstanceDefinition>,
    /// Property [`LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-launchtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_template: ::Value<self::game_server_group::LaunchTemplate>,
    /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-maxsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_size: Option<::Value<f64>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: Option<::Value<f64>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcSubnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html#cfn-gamelift-gameservergroup-vpcsubnets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_subnets: Option<::ValueList<String>>,
}

impl ::serde::Serialize for GameServerGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_scaling_policy) = self.auto_scaling_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingPolicy", auto_scaling_policy)?;
        }
        if let Some(ref balancing_strategy) = self.balancing_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BalancingStrategy", balancing_strategy)?;
        }
        if let Some(ref delete_option) = self.delete_option {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOption", delete_option)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameServerGroupName", &self.game_server_group_name)?;
        if let Some(ref game_server_protection_policy) = self.game_server_protection_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameServerProtectionPolicy", game_server_protection_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceDefinitions", &self.instance_definitions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplate", &self.launch_template)?;
        if let Some(ref max_size) = self.max_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
        }
        if let Some(ref min_size) = self.min_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_subnets) = self.vpc_subnets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnets", vpc_subnets)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GameServerGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GameServerGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GameServerGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GameServerGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_policy: Option<::Value<self::game_server_group::AutoScalingPolicy>> = None;
                let mut balancing_strategy: Option<::Value<String>> = None;
                let mut delete_option: Option<::Value<String>> = None;
                let mut game_server_group_name: Option<::Value<String>> = None;
                let mut game_server_protection_policy: Option<::Value<String>> = None;
                let mut instance_definitions: Option<::ValueList<self::game_server_group::InstanceDefinition>> = None;
                let mut launch_template: Option<::Value<self::game_server_group::LaunchTemplate>> = None;
                let mut max_size: Option<::Value<f64>> = None;
                let mut min_size: Option<::Value<f64>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_subnets: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingPolicy" => {
                            auto_scaling_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BalancingStrategy" => {
                            balancing_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeleteOption" => {
                            delete_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GameServerGroupName" => {
                            game_server_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GameServerProtectionPolicy" => {
                            game_server_protection_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceDefinitions" => {
                            instance_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchTemplate" => {
                            launch_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSize" => {
                            max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSubnets" => {
                            vpc_subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GameServerGroupProperties {
                    auto_scaling_policy: auto_scaling_policy,
                    balancing_strategy: balancing_strategy,
                    delete_option: delete_option,
                    game_server_group_name: game_server_group_name.ok_or(::serde::de::Error::missing_field("GameServerGroupName"))?,
                    game_server_protection_policy: game_server_protection_policy,
                    instance_definitions: instance_definitions.ok_or(::serde::de::Error::missing_field("InstanceDefinitions"))?,
                    launch_template: launch_template.ok_or(::serde::de::Error::missing_field("LaunchTemplate"))?,
                    max_size: max_size,
                    min_size: min_size,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    vpc_subnets: vpc_subnets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GameServerGroup {
    type Properties = GameServerGroupProperties;
    const TYPE: &'static str = "AWS::GameLift::GameServerGroup";
    fn properties(&self) -> &GameServerGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GameServerGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GameServerGroup {}

impl From<GameServerGroupProperties> for GameServerGroup {
    fn from(properties: GameServerGroupProperties) -> GameServerGroup {
        GameServerGroup { properties }
    }
}

/// The [`AWS::GameLift::GameSessionQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html) resource type.
#[derive(Debug, Default)]
pub struct GameSessionQueue {
    properties: GameSessionQueueProperties
}

/// Properties for the `GameSessionQueue` resource.
#[derive(Debug, Default)]
pub struct GameSessionQueueProperties {
    /// Property [`CustomEventData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-customeventdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_event_data: Option<::Value<String>>,
    /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-destinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destinations: Option<::ValueList<self::game_session_queue::Destination>>,
    /// Property [`FilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-filterconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_configuration: Option<::Value<self::game_session_queue::FilterConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NotificationTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-notificationtarget).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_target: Option<::Value<String>>,
    /// Property [`PlayerLatencyPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-playerlatencypolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub player_latency_policies: Option<::ValueList<self::game_session_queue::PlayerLatencyPolicy>>,
    /// Property [`PriorityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-priorityconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority_configuration: Option<::Value<self::game_session_queue::PriorityConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html#cfn-gamelift-gamesessionqueue-timeoutinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout_in_seconds: Option<::Value<u32>>,
}

impl ::serde::Serialize for GameSessionQueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref custom_event_data) = self.custom_event_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEventData", custom_event_data)?;
        }
        if let Some(ref destinations) = self.destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
        }
        if let Some(ref filter_configuration) = self.filter_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterConfiguration", filter_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref notification_target) = self.notification_target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTarget", notification_target)?;
        }
        if let Some(ref player_latency_policies) = self.player_latency_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlayerLatencyPolicies", player_latency_policies)?;
        }
        if let Some(ref priority_configuration) = self.priority_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriorityConfiguration", priority_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout_in_seconds) = self.timeout_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", timeout_in_seconds)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GameSessionQueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GameSessionQueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GameSessionQueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GameSessionQueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut custom_event_data: Option<::Value<String>> = None;
                let mut destinations: Option<::ValueList<self::game_session_queue::Destination>> = None;
                let mut filter_configuration: Option<::Value<self::game_session_queue::FilterConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut notification_target: Option<::Value<String>> = None;
                let mut player_latency_policies: Option<::ValueList<self::game_session_queue::PlayerLatencyPolicy>> = None;
                let mut priority_configuration: Option<::Value<self::game_session_queue::PriorityConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout_in_seconds: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomEventData" => {
                            custom_event_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Destinations" => {
                            destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FilterConfiguration" => {
                            filter_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTarget" => {
                            notification_target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlayerLatencyPolicies" => {
                            player_latency_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PriorityConfiguration" => {
                            priority_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeoutInSeconds" => {
                            timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GameSessionQueueProperties {
                    custom_event_data: custom_event_data,
                    destinations: destinations,
                    filter_configuration: filter_configuration,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    notification_target: notification_target,
                    player_latency_policies: player_latency_policies,
                    priority_configuration: priority_configuration,
                    tags: tags,
                    timeout_in_seconds: timeout_in_seconds,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GameSessionQueue {
    type Properties = GameSessionQueueProperties;
    const TYPE: &'static str = "AWS::GameLift::GameSessionQueue";
    fn properties(&self) -> &GameSessionQueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GameSessionQueueProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GameSessionQueue {}

impl From<GameSessionQueueProperties> for GameSessionQueue {
    fn from(properties: GameSessionQueueProperties) -> GameSessionQueue {
        GameSessionQueue { properties }
    }
}

/// The [`AWS::GameLift::MatchmakingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct MatchmakingConfiguration {
    properties: MatchmakingConfigurationProperties
}

/// Properties for the `MatchmakingConfiguration` resource.
#[derive(Debug, Default)]
pub struct MatchmakingConfigurationProperties {
    /// Property [`AcceptanceRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-acceptancerequired).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub acceptance_required: ::Value<bool>,
    /// Property [`AcceptanceTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-acceptancetimeoutseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub acceptance_timeout_seconds: Option<::Value<u32>>,
    /// Property [`AdditionalPlayerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-additionalplayercount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_player_count: Option<::Value<u32>>,
    /// Property [`BackfillMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-backfillmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backfill_mode: Option<::Value<String>>,
    /// Property [`CustomEventData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-customeventdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_event_data: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FlexMatchMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-flexmatchmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flex_match_mode: Option<::Value<String>>,
    /// Property [`GameProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-gameproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub game_properties: Option<::ValueList<self::matchmaking_configuration::GameProperty>>,
    /// Property [`GameSessionData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-gamesessiondata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub game_session_data: Option<::Value<String>>,
    /// Property [`GameSessionQueueArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-gamesessionqueuearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub game_session_queue_arns: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NotificationTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-notificationtarget).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_target: Option<::Value<String>>,
    /// Property [`RequestTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-requesttimeoutseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_timeout_seconds: ::Value<u32>,
    /// Property [`RuleSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-rulesetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_set_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html#cfn-gamelift-matchmakingconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MatchmakingConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptanceRequired", &self.acceptance_required)?;
        if let Some(ref acceptance_timeout_seconds) = self.acceptance_timeout_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptanceTimeoutSeconds", acceptance_timeout_seconds)?;
        }
        if let Some(ref additional_player_count) = self.additional_player_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalPlayerCount", additional_player_count)?;
        }
        if let Some(ref backfill_mode) = self.backfill_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackfillMode", backfill_mode)?;
        }
        if let Some(ref custom_event_data) = self.custom_event_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEventData", custom_event_data)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref flex_match_mode) = self.flex_match_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlexMatchMode", flex_match_mode)?;
        }
        if let Some(ref game_properties) = self.game_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameProperties", game_properties)?;
        }
        if let Some(ref game_session_data) = self.game_session_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameSessionData", game_session_data)?;
        }
        if let Some(ref game_session_queue_arns) = self.game_session_queue_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameSessionQueueArns", game_session_queue_arns)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref notification_target) = self.notification_target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTarget", notification_target)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestTimeoutSeconds", &self.request_timeout_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetName", &self.rule_set_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MatchmakingConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchmakingConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MatchmakingConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MatchmakingConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut acceptance_required: Option<::Value<bool>> = None;
                let mut acceptance_timeout_seconds: Option<::Value<u32>> = None;
                let mut additional_player_count: Option<::Value<u32>> = None;
                let mut backfill_mode: Option<::Value<String>> = None;
                let mut custom_event_data: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut flex_match_mode: Option<::Value<String>> = None;
                let mut game_properties: Option<::ValueList<self::matchmaking_configuration::GameProperty>> = None;
                let mut game_session_data: Option<::Value<String>> = None;
                let mut game_session_queue_arns: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut notification_target: Option<::Value<String>> = None;
                let mut request_timeout_seconds: Option<::Value<u32>> = None;
                let mut rule_set_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptanceRequired" => {
                            acceptance_required = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AcceptanceTimeoutSeconds" => {
                            acceptance_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdditionalPlayerCount" => {
                            additional_player_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackfillMode" => {
                            backfill_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomEventData" => {
                            custom_event_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlexMatchMode" => {
                            flex_match_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GameProperties" => {
                            game_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GameSessionData" => {
                            game_session_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GameSessionQueueArns" => {
                            game_session_queue_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTarget" => {
                            notification_target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestTimeoutSeconds" => {
                            request_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleSetName" => {
                            rule_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MatchmakingConfigurationProperties {
                    acceptance_required: acceptance_required.ok_or(::serde::de::Error::missing_field("AcceptanceRequired"))?,
                    acceptance_timeout_seconds: acceptance_timeout_seconds,
                    additional_player_count: additional_player_count,
                    backfill_mode: backfill_mode,
                    custom_event_data: custom_event_data,
                    description: description,
                    flex_match_mode: flex_match_mode,
                    game_properties: game_properties,
                    game_session_data: game_session_data,
                    game_session_queue_arns: game_session_queue_arns,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    notification_target: notification_target,
                    request_timeout_seconds: request_timeout_seconds.ok_or(::serde::de::Error::missing_field("RequestTimeoutSeconds"))?,
                    rule_set_name: rule_set_name.ok_or(::serde::de::Error::missing_field("RuleSetName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MatchmakingConfiguration {
    type Properties = MatchmakingConfigurationProperties;
    const TYPE: &'static str = "AWS::GameLift::MatchmakingConfiguration";
    fn properties(&self) -> &MatchmakingConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MatchmakingConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MatchmakingConfiguration {}

impl From<MatchmakingConfigurationProperties> for MatchmakingConfiguration {
    fn from(properties: MatchmakingConfigurationProperties) -> MatchmakingConfiguration {
        MatchmakingConfiguration { properties }
    }
}

/// The [`AWS::GameLift::MatchmakingRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingruleset.html) resource type.
#[derive(Debug, Default)]
pub struct MatchmakingRuleSet {
    properties: MatchmakingRuleSetProperties
}

/// Properties for the `MatchmakingRuleSet` resource.
#[derive(Debug, Default)]
pub struct MatchmakingRuleSetProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingruleset.html#cfn-gamelift-matchmakingruleset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RuleSetBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingruleset.html#cfn-gamelift-matchmakingruleset-rulesetbody).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_set_body: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingruleset.html#cfn-gamelift-matchmakingruleset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MatchmakingRuleSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetBody", &self.rule_set_body)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MatchmakingRuleSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchmakingRuleSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MatchmakingRuleSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MatchmakingRuleSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut rule_set_body: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleSetBody" => {
                            rule_set_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MatchmakingRuleSetProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rule_set_body: rule_set_body.ok_or(::serde::de::Error::missing_field("RuleSetBody"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MatchmakingRuleSet {
    type Properties = MatchmakingRuleSetProperties;
    const TYPE: &'static str = "AWS::GameLift::MatchmakingRuleSet";
    fn properties(&self) -> &MatchmakingRuleSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MatchmakingRuleSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MatchmakingRuleSet {}

impl From<MatchmakingRuleSetProperties> for MatchmakingRuleSet {
    fn from(properties: MatchmakingRuleSetProperties) -> MatchmakingRuleSet {
        MatchmakingRuleSet { properties }
    }
}

/// The [`AWS::GameLift::Script`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html) resource type.
#[derive(Debug, Default)]
pub struct Script {
    properties: ScriptProperties
}

/// Properties for the `Script` resource.
#[derive(Debug, Default)]
pub struct ScriptProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html#cfn-gamelift-script-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`StorageLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html#cfn-gamelift-script-storagelocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_location: ::Value<self::script::S3Location>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html#cfn-gamelift-script-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html#cfn-gamelift-script-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for ScriptProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageLocation", &self.storage_location)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref version) = self.version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScriptProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScriptProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScriptProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScriptProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut storage_location: Option<::Value<self::script::S3Location>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageLocation" => {
                            storage_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScriptProperties {
                    name: name,
                    storage_location: storage_location.ok_or(::serde::de::Error::missing_field("StorageLocation"))?,
                    tags: tags,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Script {
    type Properties = ScriptProperties;
    const TYPE: &'static str = "AWS::GameLift::Script";
    fn properties(&self) -> &ScriptProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScriptProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Script {}

impl From<ScriptProperties> for Script {
    fn from(properties: ScriptProperties) -> Script {
        Script { properties }
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
        pub r#type: ::Value<String>,
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
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FleetId" => {
                                fleet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingStrategy {
                        fleet_id: fleet_id,
                        message: message,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
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
        /// Property [`ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html#cfn-gamelift-build-object-verison).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_version: Option<::Value<String>>,
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
            if let Some(ref object_version) = self.object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersion", object_version)?;
            }
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
                    let mut object_version: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectVersion" => {
                                object_version = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        object_version: object_version,
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

    /// The [`AWS::GameLift::Fleet.CertificateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-certificateconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CertificateConfiguration {
        /// Property [`CertificateType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-certificateconfiguration.html#cfn-gamelift-fleet-certificateconfiguration-certificatetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CertificateConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateType", &self.certificate_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CertificateConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CertificateConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CertificateConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateType" => {
                                certificate_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CertificateConfiguration {
                        certificate_type: certificate_type.ok_or(::serde::de::Error::missing_field("CertificateType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::Fleet.IpPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html) property type.
    #[derive(Debug, Default)]
    pub struct IpPermission {
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html#cfn-gamelift-fleet-ippermission-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: ::Value<u32>,
        /// Property [`IpRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html#cfn-gamelift-fleet-ippermission-iprange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_range: ::Value<String>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html#cfn-gamelift-fleet-ippermission-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html#cfn-gamelift-fleet-ippermission-toport).
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

    /// The [`AWS::GameLift::Fleet.LocationCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationcapacity.html) property type.
    #[derive(Debug, Default)]
    pub struct LocationCapacity {
        /// Property [`DesiredEC2Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationcapacity.html#cfn-gamelift-fleet-locationcapacity-desiredec2instances).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_ec2_instances: ::Value<u32>,
        /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationcapacity.html#cfn-gamelift-fleet-locationcapacity-maxsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_size: ::Value<u32>,
        /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationcapacity.html#cfn-gamelift-fleet-locationcapacity-minsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_size: ::Value<u32>,
    }

    impl ::codec::SerializeValue for LocationCapacity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredEC2Instances", &self.desired_ec2_instances)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", &self.max_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", &self.min_size)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocationCapacity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationCapacity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocationCapacity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocationCapacity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut desired_ec2_instances: Option<::Value<u32>> = None;
                    let mut max_size: Option<::Value<u32>> = None;
                    let mut min_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredEC2Instances" => {
                                desired_ec2_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSize" => {
                                max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinSize" => {
                                min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocationCapacity {
                        desired_ec2_instances: desired_ec2_instances.ok_or(::serde::de::Error::missing_field("DesiredEC2Instances"))?,
                        max_size: max_size.ok_or(::serde::de::Error::missing_field("MaxSize"))?,
                        min_size: min_size.ok_or(::serde::de::Error::missing_field("MinSize"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::Fleet.LocationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LocationConfiguration {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationconfiguration.html#cfn-gamelift-fleet-locationconfiguration-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: ::Value<String>,
        /// Property [`LocationCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationconfiguration.html#cfn-gamelift-fleet-locationconfiguration-locationcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location_capacity: Option<::Value<LocationCapacity>>,
    }

    impl ::codec::SerializeValue for LocationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            if let Some(ref location_capacity) = self.location_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocationCapacity", location_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut location: Option<::Value<String>> = None;
                    let mut location_capacity: Option<::Value<LocationCapacity>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocationCapacity" => {
                                location_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocationConfiguration {
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                        location_capacity: location_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::Fleet.ResourceCreationLimitPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-resourcecreationlimitpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceCreationLimitPolicy {
        /// Property [`NewGameSessionsPerCreator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-resourcecreationlimitpolicy.html#cfn-gamelift-fleet-resourcecreationlimitpolicy-newgamesessionspercreator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub new_game_sessions_per_creator: Option<::Value<u32>>,
        /// Property [`PolicyPeriodInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-resourcecreationlimitpolicy.html#cfn-gamelift-fleet-resourcecreationlimitpolicy-policyperiodinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_period_in_minutes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ResourceCreationLimitPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref new_game_sessions_per_creator) = self.new_game_sessions_per_creator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewGameSessionsPerCreator", new_game_sessions_per_creator)?;
            }
            if let Some(ref policy_period_in_minutes) = self.policy_period_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyPeriodInMinutes", policy_period_in_minutes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceCreationLimitPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceCreationLimitPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceCreationLimitPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceCreationLimitPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut new_game_sessions_per_creator: Option<::Value<u32>> = None;
                    let mut policy_period_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NewGameSessionsPerCreator" => {
                                new_game_sessions_per_creator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyPeriodInMinutes" => {
                                policy_period_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceCreationLimitPolicy {
                        new_game_sessions_per_creator: new_game_sessions_per_creator,
                        policy_period_in_minutes: policy_period_in_minutes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::Fleet.RuntimeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-runtimeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RuntimeConfiguration {
        /// Property [`GameSessionActivationTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-runtimeconfiguration.html#cfn-gamelift-fleet-runtimeconfiguration-gamesessionactivationtimeoutseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub game_session_activation_timeout_seconds: Option<::Value<u32>>,
        /// Property [`MaxConcurrentGameSessionActivations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-runtimeconfiguration.html#cfn-gamelift-fleet-runtimeconfiguration-maxconcurrentgamesessionactivations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_concurrent_game_session_activations: Option<::Value<u32>>,
        /// Property [`ServerProcesses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-runtimeconfiguration.html#cfn-gamelift-fleet-runtimeconfiguration-serverprocesses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_processes: Option<::ValueList<ServerProcess>>,
    }

    impl ::codec::SerializeValue for RuntimeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref game_session_activation_timeout_seconds) = self.game_session_activation_timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GameSessionActivationTimeoutSeconds", game_session_activation_timeout_seconds)?;
            }
            if let Some(ref max_concurrent_game_session_activations) = self.max_concurrent_game_session_activations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentGameSessionActivations", max_concurrent_game_session_activations)?;
            }
            if let Some(ref server_processes) = self.server_processes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerProcesses", server_processes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuntimeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuntimeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuntimeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuntimeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut game_session_activation_timeout_seconds: Option<::Value<u32>> = None;
                    let mut max_concurrent_game_session_activations: Option<::Value<u32>> = None;
                    let mut server_processes: Option<::ValueList<ServerProcess>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GameSessionActivationTimeoutSeconds" => {
                                game_session_activation_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConcurrentGameSessionActivations" => {
                                max_concurrent_game_session_activations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerProcesses" => {
                                server_processes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuntimeConfiguration {
                        game_session_activation_timeout_seconds: game_session_activation_timeout_seconds,
                        max_concurrent_game_session_activations: max_concurrent_game_session_activations,
                        server_processes: server_processes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::Fleet.ServerProcess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-serverprocess.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerProcess {
        /// Property [`ConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-serverprocess.html#cfn-gamelift-fleet-serverprocess-concurrentexecutions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub concurrent_executions: ::Value<u32>,
        /// Property [`LaunchPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-serverprocess.html#cfn-gamelift-fleet-serverprocess-launchpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_path: ::Value<String>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-serverprocess.html#cfn-gamelift-fleet-serverprocess-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServerProcess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConcurrentExecutions", &self.concurrent_executions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchPath", &self.launch_path)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerProcess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerProcess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerProcess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerProcess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut concurrent_executions: Option<::Value<u32>> = None;
                    let mut launch_path: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConcurrentExecutions" => {
                                concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchPath" => {
                                launch_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerProcess {
                        concurrent_executions: concurrent_executions.ok_or(::serde::de::Error::missing_field("ConcurrentExecutions"))?,
                        launch_path: launch_path.ok_or(::serde::de::Error::missing_field("LaunchPath"))?,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod game_server_group {
    //! Property types for the `GameServerGroup` resource.

    /// The [`AWS::GameLift::GameServerGroup.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-autoscalingpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoScalingPolicy {
        /// Property [`EstimatedInstanceWarmup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-autoscalingpolicy.html#cfn-gamelift-gameservergroup-autoscalingpolicy-estimatedinstancewarmup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub estimated_instance_warmup: Option<::Value<f64>>,
        /// Property [`TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-autoscalingpolicy.html#cfn-gamelift-gameservergroup-autoscalingpolicy-targettrackingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_tracking_configuration: ::Value<TargetTrackingConfiguration>,
    }

    impl ::codec::SerializeValue for AutoScalingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref estimated_instance_warmup) = self.estimated_instance_warmup {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EstimatedInstanceWarmup", estimated_instance_warmup)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingConfiguration", &self.target_tracking_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScalingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScalingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScalingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut estimated_instance_warmup: Option<::Value<f64>> = None;
                    let mut target_tracking_configuration: Option<::Value<TargetTrackingConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EstimatedInstanceWarmup" => {
                                estimated_instance_warmup = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTrackingConfiguration" => {
                                target_tracking_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScalingPolicy {
                        estimated_instance_warmup: estimated_instance_warmup,
                        target_tracking_configuration: target_tracking_configuration.ok_or(::serde::de::Error::missing_field("TargetTrackingConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameServerGroup.InstanceDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-instancedefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceDefinition {
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-instancedefinition.html#cfn-gamelift-gameservergroup-instancedefinition-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`WeightedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-instancedefinition.html#cfn-gamelift-gameservergroup-instancedefinition-weightedcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weighted_capacity: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref weighted_capacity) = self.weighted_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", weighted_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut weighted_capacity: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceDefinition {
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        weighted_capacity: weighted_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameServerGroup.LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-launchtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplate {
        /// Property [`LaunchTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-launchtemplate.html#cfn-gamelift-gameservergroup-launchtemplate-launchtemplateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_id: Option<::Value<String>>,
        /// Property [`LaunchTemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-launchtemplate.html#cfn-gamelift-gameservergroup-launchtemplate-launchtemplatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_name: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-launchtemplate.html#cfn-gamelift-gameservergroup-launchtemplate-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LaunchTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref launch_template_id) = self.launch_template_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateId", launch_template_id)?;
            }
            if let Some(ref launch_template_name) = self.launch_template_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateName", launch_template_name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LaunchTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut launch_template_id: Option<::Value<String>> = None;
                    let mut launch_template_name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LaunchTemplateId" => {
                                launch_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplateName" => {
                                launch_template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplate {
                        launch_template_id: launch_template_id,
                        launch_template_name: launch_template_name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameServerGroup.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-targettrackingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetTrackingConfiguration {
        /// Property [`TargetValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-targettrackingconfiguration.html#cfn-gamelift-gameservergroup-targettrackingconfiguration-targetvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TargetTrackingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetValue", &self.target_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetTrackingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetValue" => {
                                target_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetTrackingConfiguration {
                        target_value: target_value.ok_or(::serde::de::Error::missing_field("TargetValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod game_session_queue {
    //! Property types for the `GameSessionQueue` resource.

    /// The [`AWS::GameLift::GameSessionQueue.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-destination.html) property type.
    #[derive(Debug, Default)]
    pub struct Destination {
        /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-destination.html#cfn-gamelift-gamesessionqueue-destination-destinationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_arn) = self.destination_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", destination_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Destination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Destination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Destination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Destination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationArn" => {
                                destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Destination {
                        destination_arn: destination_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameSessionQueue.FilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-filterconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterConfiguration {
        /// Property [`AllowedLocations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-filterconfiguration.html#cfn-gamelift-gamesessionqueue-filterconfiguration-allowedlocations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_locations: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for FilterConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_locations) = self.allowed_locations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedLocations", allowed_locations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_locations: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedLocations" => {
                                allowed_locations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterConfiguration {
                        allowed_locations: allowed_locations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameSessionQueue.PlayerLatencyPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-playerlatencypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct PlayerLatencyPolicy {
        /// Property [`MaximumIndividualPlayerLatencyMilliseconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-playerlatencypolicy.html#cfn-gamelift-gamesessionqueue-playerlatencypolicy-maximumindividualplayerlatencymilliseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_individual_player_latency_milliseconds: Option<::Value<u32>>,
        /// Property [`PolicyDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-playerlatencypolicy.html#cfn-gamelift-gamesessionqueue-playerlatencypolicy-policydurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_duration_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for PlayerLatencyPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_individual_player_latency_milliseconds) = self.maximum_individual_player_latency_milliseconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumIndividualPlayerLatencyMilliseconds", maximum_individual_player_latency_milliseconds)?;
            }
            if let Some(ref policy_duration_seconds) = self.policy_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDurationSeconds", policy_duration_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlayerLatencyPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlayerLatencyPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlayerLatencyPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlayerLatencyPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_individual_player_latency_milliseconds: Option<::Value<u32>> = None;
                    let mut policy_duration_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumIndividualPlayerLatencyMilliseconds" => {
                                maximum_individual_player_latency_milliseconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyDurationSeconds" => {
                                policy_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlayerLatencyPolicy {
                        maximum_individual_player_latency_milliseconds: maximum_individual_player_latency_milliseconds,
                        policy_duration_seconds: policy_duration_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GameLift::GameSessionQueue.PriorityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-priorityconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct PriorityConfiguration {
        /// Property [`LocationOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-priorityconfiguration.html#cfn-gamelift-gamesessionqueue-priorityconfiguration-locationorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location_order: Option<::ValueList<String>>,
        /// Property [`PriorityOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-priorityconfiguration.html#cfn-gamelift-gamesessionqueue-priorityconfiguration-priorityorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority_order: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PriorityConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref location_order) = self.location_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocationOrder", location_order)?;
            }
            if let Some(ref priority_order) = self.priority_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriorityOrder", priority_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PriorityConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PriorityConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PriorityConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PriorityConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut location_order: Option<::ValueList<String>> = None;
                    let mut priority_order: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocationOrder" => {
                                location_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PriorityOrder" => {
                                priority_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PriorityConfiguration {
                        location_order: location_order,
                        priority_order: priority_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod matchmaking_configuration {
    //! Property types for the `MatchmakingConfiguration` resource.

    /// The [`AWS::GameLift::MatchmakingConfiguration.GameProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-matchmakingconfiguration-gameproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct GameProperty {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-matchmakingconfiguration-gameproperty.html#cfn-gamelift-matchmakingconfiguration-gameproperty-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-matchmakingconfiguration-gameproperty.html#cfn-gamelift-matchmakingconfiguration-gameproperty-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for GameProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GameProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GameProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GameProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GameProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GameProperty {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod script {
    //! Property types for the `Script` resource.

    /// The [`AWS::GameLift::Script.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html#cfn-gamelift-script-s3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html#cfn-gamelift-script-s3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html#cfn-gamelift-script-s3location-objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_version: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html#cfn-gamelift-script-s3location-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref object_version) = self.object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersion", object_version)?;
            }
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
                    let mut object_version: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectVersion" => {
                                object_version = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        object_version: object_version,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
