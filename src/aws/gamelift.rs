/// The [`AWS::GameLift::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RoutingStrategy")]
    pub routing_strategy: (),
}

impl<'a> ::Resource<'a> for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::GameLift::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::GameLift::Build`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Build {
    properties: BuildProperties
}

/// Properties for the `Build` resource.
#[derive(Serialize, Deserialize)]
pub struct BuildProperties {
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="StorageLocation")]
    pub storage_location: (),
    #[serde(rename="Version")]
    pub version: (),
}

impl<'a> ::Resource<'a> for Build {
    type Properties = BuildProperties;
    const TYPE: &'static str = "AWS::GameLift::Build";
    fn properties(&self) -> &BuildProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BuildProperties {
        &mut self.properties
    }
}

impl From<BuildProperties> for Build {
    fn from(properties: BuildProperties) -> Build {
        Build { properties }
    }
}

/// The [`AWS::GameLift::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Serialize, Deserialize)]
pub struct FleetProperties {
    #[serde(rename="BuildId")]
    pub build_id: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="DesiredEC2Instances")]
    pub desired_ec2_instances: (),
    #[serde(rename="EC2InboundPermissions")]
    pub ec2_inbound_permissions: (),
    #[serde(rename="EC2InstanceType")]
    pub ec2_instance_type: (),
    #[serde(rename="LogPaths")]
    pub log_paths: (),
    #[serde(rename="MaxSize")]
    pub max_size: (),
    #[serde(rename="MinSize")]
    pub min_size: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="ServerLaunchParameters")]
    pub server_launch_parameters: (),
    #[serde(rename="ServerLaunchPath")]
    pub server_launch_path: (),
}

impl<'a> ::Resource<'a> for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::GameLift::Fleet";
    fn properties(&self) -> &FleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FleetProperties {
        &mut self.properties
    }
}

impl From<FleetProperties> for Fleet {
    fn from(properties: FleetProperties) -> Fleet {
        Fleet { properties }
    }
}

