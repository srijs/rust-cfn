//! Types for the `RoboMaker` service.

/// The [`AWS::RoboMaker::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-fleet.html) resource type.
#[derive(Debug, Default)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug, Default)]
pub struct FleetProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-fleet.html#cfn-robomaker-fleet-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-fleet.html#cfn-robomaker-fleet-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::RoboMaker::Fleet";
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

/// The [`AWS::RoboMaker::Robot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html) resource type.
#[derive(Debug, Default)]
pub struct Robot {
    properties: RobotProperties
}

/// Properties for the `Robot` resource.
#[derive(Debug, Default)]
pub struct RobotProperties {
    /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html#cfn-robomaker-robot-architecture).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub architecture: ::Value<String>,
    /// Property [`Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html#cfn-robomaker-robot-fleet).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fleet: Option<::Value<String>>,
    /// Property [`GreengrassGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html#cfn-robomaker-robot-greengrassgroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub greengrass_group_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html#cfn-robomaker-robot-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html#cfn-robomaker-robot-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for RobotProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", &self.architecture)?;
        if let Some(ref fleet) = self.fleet {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fleet", fleet)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GreengrassGroupId", &self.greengrass_group_id)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RobotProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RobotProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RobotProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RobotProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut architecture: Option<::Value<String>> = None;
                let mut fleet: Option<::Value<String>> = None;
                let mut greengrass_group_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Architecture" => {
                            architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fleet" => {
                            fleet = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GreengrassGroupId" => {
                            greengrass_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RobotProperties {
                    architecture: architecture.ok_or(::serde::de::Error::missing_field("Architecture"))?,
                    fleet: fleet,
                    greengrass_group_id: greengrass_group_id.ok_or(::serde::de::Error::missing_field("GreengrassGroupId"))?,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Robot {
    type Properties = RobotProperties;
    const TYPE: &'static str = "AWS::RoboMaker::Robot";
    fn properties(&self) -> &RobotProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RobotProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Robot {}

impl From<RobotProperties> for Robot {
    fn from(properties: RobotProperties) -> Robot {
        Robot { properties }
    }
}

/// The [`AWS::RoboMaker::RobotApplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html) resource type.
#[derive(Debug, Default)]
pub struct RobotApplication {
    properties: RobotApplicationProperties
}

/// Properties for the `RobotApplication` resource.
#[derive(Debug, Default)]
pub struct RobotApplicationProperties {
    /// Property [`CurrentRevisionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-currentrevisionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub current_revision_id: Option<::Value<String>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RobotSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-robotsoftwaresuite).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub robot_software_suite: ::Value<self::robot_application::RobotSoftwareSuite>,
    /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-sources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sources: Option<::ValueList<self::robot_application::SourceConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html#cfn-robomaker-robotapplication-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for RobotApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref current_revision_id) = self.current_revision_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentRevisionId", current_revision_id)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RobotSoftwareSuite", &self.robot_software_suite)?;
        if let Some(ref sources) = self.sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RobotApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RobotApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RobotApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RobotApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut current_revision_id: Option<::Value<String>> = None;
                let mut environment: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut robot_software_suite: Option<::Value<self::robot_application::RobotSoftwareSuite>> = None;
                let mut sources: Option<::ValueList<self::robot_application::SourceConfig>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CurrentRevisionId" => {
                            current_revision_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RobotSoftwareSuite" => {
                            robot_software_suite = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sources" => {
                            sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RobotApplicationProperties {
                    current_revision_id: current_revision_id,
                    environment: environment,
                    name: name,
                    robot_software_suite: robot_software_suite.ok_or(::serde::de::Error::missing_field("RobotSoftwareSuite"))?,
                    sources: sources,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RobotApplication {
    type Properties = RobotApplicationProperties;
    const TYPE: &'static str = "AWS::RoboMaker::RobotApplication";
    fn properties(&self) -> &RobotApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RobotApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RobotApplication {}

impl From<RobotApplicationProperties> for RobotApplication {
    fn from(properties: RobotApplicationProperties) -> RobotApplication {
        RobotApplication { properties }
    }
}

/// The [`AWS::RoboMaker::RobotApplicationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplicationversion.html) resource type.
#[derive(Debug, Default)]
pub struct RobotApplicationVersion {
    properties: RobotApplicationVersionProperties
}

/// Properties for the `RobotApplicationVersion` resource.
#[derive(Debug, Default)]
pub struct RobotApplicationVersionProperties {
    /// Property [`Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplicationversion.html#cfn-robomaker-robotapplicationversion-application).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application: ::Value<String>,
    /// Property [`CurrentRevisionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplicationversion.html#cfn-robomaker-robotapplicationversion-currentrevisionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub current_revision_id: Option<::Value<String>>,
}

impl ::serde::Serialize for RobotApplicationVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Application", &self.application)?;
        if let Some(ref current_revision_id) = self.current_revision_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentRevisionId", current_revision_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RobotApplicationVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RobotApplicationVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RobotApplicationVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RobotApplicationVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application: Option<::Value<String>> = None;
                let mut current_revision_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Application" => {
                            application = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CurrentRevisionId" => {
                            current_revision_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RobotApplicationVersionProperties {
                    application: application.ok_or(::serde::de::Error::missing_field("Application"))?,
                    current_revision_id: current_revision_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RobotApplicationVersion {
    type Properties = RobotApplicationVersionProperties;
    const TYPE: &'static str = "AWS::RoboMaker::RobotApplicationVersion";
    fn properties(&self) -> &RobotApplicationVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RobotApplicationVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RobotApplicationVersion {}

impl From<RobotApplicationVersionProperties> for RobotApplicationVersion {
    fn from(properties: RobotApplicationVersionProperties) -> RobotApplicationVersion {
        RobotApplicationVersion { properties }
    }
}

/// The [`AWS::RoboMaker::SimulationApplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html) resource type.
#[derive(Debug, Default)]
pub struct SimulationApplication {
    properties: SimulationApplicationProperties
}

/// Properties for the `SimulationApplication` resource.
#[derive(Debug, Default)]
pub struct SimulationApplicationProperties {
    /// Property [`CurrentRevisionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-currentrevisionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub current_revision_id: Option<::Value<String>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RenderingEngine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-renderingengine).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rendering_engine: Option<::Value<self::simulation_application::RenderingEngine>>,
    /// Property [`RobotSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-robotsoftwaresuite).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub robot_software_suite: ::Value<self::simulation_application::RobotSoftwareSuite>,
    /// Property [`SimulationSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-simulationsoftwaresuite).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub simulation_software_suite: ::Value<self::simulation_application::SimulationSoftwareSuite>,
    /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-sources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sources: Option<::ValueList<self::simulation_application::SourceConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html#cfn-robomaker-simulationapplication-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for SimulationApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref current_revision_id) = self.current_revision_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentRevisionId", current_revision_id)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref rendering_engine) = self.rendering_engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RenderingEngine", rendering_engine)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RobotSoftwareSuite", &self.robot_software_suite)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SimulationSoftwareSuite", &self.simulation_software_suite)?;
        if let Some(ref sources) = self.sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SimulationApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SimulationApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SimulationApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SimulationApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut current_revision_id: Option<::Value<String>> = None;
                let mut environment: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rendering_engine: Option<::Value<self::simulation_application::RenderingEngine>> = None;
                let mut robot_software_suite: Option<::Value<self::simulation_application::RobotSoftwareSuite>> = None;
                let mut simulation_software_suite: Option<::Value<self::simulation_application::SimulationSoftwareSuite>> = None;
                let mut sources: Option<::ValueList<self::simulation_application::SourceConfig>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CurrentRevisionId" => {
                            current_revision_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RenderingEngine" => {
                            rendering_engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RobotSoftwareSuite" => {
                            robot_software_suite = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SimulationSoftwareSuite" => {
                            simulation_software_suite = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sources" => {
                            sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SimulationApplicationProperties {
                    current_revision_id: current_revision_id,
                    environment: environment,
                    name: name,
                    rendering_engine: rendering_engine,
                    robot_software_suite: robot_software_suite.ok_or(::serde::de::Error::missing_field("RobotSoftwareSuite"))?,
                    simulation_software_suite: simulation_software_suite.ok_or(::serde::de::Error::missing_field("SimulationSoftwareSuite"))?,
                    sources: sources,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SimulationApplication {
    type Properties = SimulationApplicationProperties;
    const TYPE: &'static str = "AWS::RoboMaker::SimulationApplication";
    fn properties(&self) -> &SimulationApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SimulationApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SimulationApplication {}

impl From<SimulationApplicationProperties> for SimulationApplication {
    fn from(properties: SimulationApplicationProperties) -> SimulationApplication {
        SimulationApplication { properties }
    }
}

/// The [`AWS::RoboMaker::SimulationApplicationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplicationversion.html) resource type.
#[derive(Debug, Default)]
pub struct SimulationApplicationVersion {
    properties: SimulationApplicationVersionProperties
}

/// Properties for the `SimulationApplicationVersion` resource.
#[derive(Debug, Default)]
pub struct SimulationApplicationVersionProperties {
    /// Property [`Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplicationversion.html#cfn-robomaker-simulationapplicationversion-application).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application: ::Value<String>,
    /// Property [`CurrentRevisionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplicationversion.html#cfn-robomaker-simulationapplicationversion-currentrevisionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub current_revision_id: Option<::Value<String>>,
}

impl ::serde::Serialize for SimulationApplicationVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Application", &self.application)?;
        if let Some(ref current_revision_id) = self.current_revision_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentRevisionId", current_revision_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SimulationApplicationVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SimulationApplicationVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SimulationApplicationVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SimulationApplicationVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application: Option<::Value<String>> = None;
                let mut current_revision_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Application" => {
                            application = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CurrentRevisionId" => {
                            current_revision_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SimulationApplicationVersionProperties {
                    application: application.ok_or(::serde::de::Error::missing_field("Application"))?,
                    current_revision_id: current_revision_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SimulationApplicationVersion {
    type Properties = SimulationApplicationVersionProperties;
    const TYPE: &'static str = "AWS::RoboMaker::SimulationApplicationVersion";
    fn properties(&self) -> &SimulationApplicationVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SimulationApplicationVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SimulationApplicationVersion {}

impl From<SimulationApplicationVersionProperties> for SimulationApplicationVersion {
    fn from(properties: SimulationApplicationVersionProperties) -> SimulationApplicationVersion {
        SimulationApplicationVersion { properties }
    }
}

pub mod robot_application {
    //! Property types for the `RobotApplication` resource.

    /// The [`AWS::RoboMaker::RobotApplication.RobotSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-robotsoftwaresuite.html) property type.
    #[derive(Debug, Default)]
    pub struct RobotSoftwareSuite {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-robotsoftwaresuite.html#cfn-robomaker-robotapplication-robotsoftwaresuite-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-robotsoftwaresuite.html#cfn-robomaker-robotapplication-robotsoftwaresuite-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RobotSoftwareSuite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RobotSoftwareSuite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RobotSoftwareSuite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RobotSoftwareSuite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RobotSoftwareSuite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RobotSoftwareSuite {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RoboMaker::RobotApplication.SourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-sourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConfig {
        /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-sourceconfig.html#cfn-robomaker-robotapplication-sourceconfig-architecture).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub architecture: ::Value<String>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-sourceconfig.html#cfn-robomaker-robotapplication-sourceconfig-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-sourceconfig.html#cfn-robomaker-robotapplication-sourceconfig-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", &self.architecture)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut architecture: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Architecture" => {
                                architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConfig {
                        architecture: architecture.ok_or(::serde::de::Error::missing_field("Architecture"))?,
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod simulation_application {
    //! Property types for the `SimulationApplication` resource.

    /// The [`AWS::RoboMaker::SimulationApplication.RenderingEngine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-renderingengine.html) property type.
    #[derive(Debug, Default)]
    pub struct RenderingEngine {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-renderingengine.html#cfn-robomaker-simulationapplication-renderingengine-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-renderingengine.html#cfn-robomaker-simulationapplication-renderingengine-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: ::Value<String>,
    }

    impl ::codec::SerializeValue for RenderingEngine {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RenderingEngine {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RenderingEngine, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RenderingEngine;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RenderingEngine")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RenderingEngine {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RoboMaker::SimulationApplication.RobotSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-robotsoftwaresuite.html) property type.
    #[derive(Debug, Default)]
    pub struct RobotSoftwareSuite {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-robotsoftwaresuite.html#cfn-robomaker-simulationapplication-robotsoftwaresuite-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-robotsoftwaresuite.html#cfn-robomaker-simulationapplication-robotsoftwaresuite-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RobotSoftwareSuite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RobotSoftwareSuite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RobotSoftwareSuite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RobotSoftwareSuite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RobotSoftwareSuite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RobotSoftwareSuite {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RoboMaker::SimulationApplication.SimulationSoftwareSuite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-simulationsoftwaresuite.html) property type.
    #[derive(Debug, Default)]
    pub struct SimulationSoftwareSuite {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-simulationsoftwaresuite.html#cfn-robomaker-simulationapplication-simulationsoftwaresuite-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-simulationsoftwaresuite.html#cfn-robomaker-simulationapplication-simulationsoftwaresuite-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SimulationSoftwareSuite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SimulationSoftwareSuite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SimulationSoftwareSuite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SimulationSoftwareSuite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SimulationSoftwareSuite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SimulationSoftwareSuite {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RoboMaker::SimulationApplication.SourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-sourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConfig {
        /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-sourceconfig.html#cfn-robomaker-simulationapplication-sourceconfig-architecture).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub architecture: ::Value<String>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-sourceconfig.html#cfn-robomaker-simulationapplication-sourceconfig-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-sourceconfig.html#cfn-robomaker-simulationapplication-sourceconfig-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", &self.architecture)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut architecture: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Architecture" => {
                                architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConfig {
                        architecture: architecture.ok_or(::serde::de::Error::missing_field("Architecture"))?,
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
