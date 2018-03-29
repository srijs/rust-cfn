/// The [`AWS::GameLift::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RoutingStrategy")]
    pub routing_strategy: self::alias::RoutingStrategy,
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
    pub name: String,
    #[serde(rename="StorageLocation")]
    pub storage_location: self::build::S3Location,
    #[serde(rename="Version")]
    pub version: String,
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
    pub build_id: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="DesiredEC2Instances")]
    pub desired_ec2_instances: u32,
    #[serde(rename="EC2InboundPermissions")]
    pub ec2_inbound_permissions: Vec<self::fleet::IpPermission>,
    #[serde(rename="EC2InstanceType")]
    pub ec2_instance_type: String,
    #[serde(rename="LogPaths")]
    pub log_paths: Vec<String>,
    #[serde(rename="MaxSize")]
    pub max_size: u32,
    #[serde(rename="MinSize")]
    pub min_size: u32,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ServerLaunchParameters")]
    pub server_launch_parameters: String,
    #[serde(rename="ServerLaunchPath")]
    pub server_launch_path: String,
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

pub mod alias {
    #[derive(Serialize, Deserialize)]
    pub struct RoutingStrategy {
        #[serde(rename="FleetId")]
        pub fleet_id: String,
        #[serde(rename="Message")]
        pub message: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod build {
    #[derive(Serialize, Deserialize)]
    pub struct S3Location {
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

}

pub mod fleet {
    #[derive(Serialize, Deserialize)]
    pub struct IpPermission {
        #[serde(rename="FromPort")]
        pub from_port: u32,
        #[serde(rename="IpRange")]
        pub ip_range: String,
        #[serde(rename="Protocol")]
        pub protocol: String,
        #[serde(rename="ToPort")]
        pub to_port: u32,
    }

}

