//! Types for the `GameLift` service.

/// The [`AWS::GameLift::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AliasProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RoutingStrategy`.
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

impl ::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::GameLift::Build`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html) resource type.
#[derive(Debug)]
pub struct Build {
    properties: BuildProperties
}

/// Properties for the `Build` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `StorageLocation`.
    #[serde(rename="StorageLocation")]
    pub storage_location: self::build::S3Location,
    /// Property `Version`.
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

impl ::private::Sealed for Build {}

impl From<BuildProperties> for Build {
    fn from(properties: BuildProperties) -> Build {
        Build { properties }
    }
}

/// The [`AWS::GameLift::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html) resource type.
#[derive(Debug)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct FleetProperties {
    /// Property `BuildId`.
    #[serde(rename="BuildId")]
    pub build_id: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `DesiredEC2Instances`.
    #[serde(rename="DesiredEC2Instances")]
    pub desired_ec2_instances: u32,
    /// Property `EC2InboundPermissions`.
    #[serde(rename="EC2InboundPermissions")]
    pub ec2_inbound_permissions: Vec<self::fleet::IpPermission>,
    /// Property `EC2InstanceType`.
    #[serde(rename="EC2InstanceType")]
    pub ec2_instance_type: String,
    /// Property `LogPaths`.
    #[serde(rename="LogPaths")]
    pub log_paths: Vec<String>,
    /// Property `MaxSize`.
    #[serde(rename="MaxSize")]
    pub max_size: u32,
    /// Property `MinSize`.
    #[serde(rename="MinSize")]
    pub min_size: u32,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `ServerLaunchParameters`.
    #[serde(rename="ServerLaunchParameters")]
    pub server_launch_parameters: String,
    /// Property `ServerLaunchPath`.
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

impl ::private::Sealed for Fleet {}

impl From<FleetProperties> for Fleet {
    fn from(properties: FleetProperties) -> Fleet {
        Fleet { properties }
    }
}

pub mod alias {
    //! Property types for the `Alias` resource.

    /// The [`AWS::GameLift::Alias.RoutingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoutingStrategy {
        /// Property `FleetId`.
        #[serde(rename="FleetId")]
        pub fleet_id: String,
        /// Property `Message`.
        #[serde(rename="Message")]
        pub message: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod build {
    //! Property types for the `Build` resource.

    /// The [`AWS::GameLift::Build.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Location {
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }
}

pub mod fleet {
    //! Property types for the `Fleet` resource.

    /// The [`AWS::GameLift::Fleet.IpPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IpPermission {
        /// Property `FromPort`.
        #[serde(rename="FromPort")]
        pub from_port: u32,
        /// Property `IpRange`.
        #[serde(rename="IpRange")]
        pub ip_range: String,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
        /// Property `ToPort`.
        #[serde(rename="ToPort")]
        pub to_port: u32,
    }
}
