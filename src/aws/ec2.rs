//! Types for the `EC2` service.

/// The [`AWS::EC2::CustomerGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-customer-gateway.html) resource type.
#[derive(Debug)]
pub struct CustomerGateway {
    properties: CustomerGatewayProperties
}

/// Properties for the `CustomerGateway` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerGatewayProperties {
    /// Property `BgpAsn`.
    #[serde(rename = "BgpAsn")]
    pub bgp_asn: ::Value<u32>,
    /// Property `IpAddress`.
    #[serde(rename = "IpAddress")]
    pub ip_address: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
}

impl<'a> ::Resource<'a> for CustomerGateway {
    type Properties = CustomerGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::CustomerGateway";
    fn properties(&self) -> &CustomerGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomerGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomerGateway {}

impl From<CustomerGatewayProperties> for CustomerGateway {
    fn from(properties: CustomerGatewayProperties) -> CustomerGateway {
        CustomerGateway { properties }
    }
}

/// The [`AWS::EC2::DHCPOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-dhcp-options.html) resource type.
#[derive(Debug)]
pub struct DHCPOptions {
    properties: DHCPOptionsProperties
}

/// Properties for the `DHCPOptions` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DHCPOptionsProperties {
    /// Property `DomainName`.
    #[serde(rename = "DomainName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<::Value<String>>,
    /// Property `DomainNameServers`.
    #[serde(rename = "DomainNameServers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_name_servers: Option<::ValueList<String>>,
    /// Property `NetbiosNameServers`.
    #[serde(rename = "NetbiosNameServers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netbios_name_servers: Option<::ValueList<String>>,
    /// Property `NetbiosNodeType`.
    #[serde(rename = "NetbiosNodeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netbios_node_type: Option<::Value<u32>>,
    /// Property `NtpServers`.
    #[serde(rename = "NtpServers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<::ValueList<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for DHCPOptions {
    type Properties = DHCPOptionsProperties;
    const TYPE: &'static str = "AWS::EC2::DHCPOptions";
    fn properties(&self) -> &DHCPOptionsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DHCPOptionsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DHCPOptions {}

impl From<DHCPOptionsProperties> for DHCPOptions {
    fn from(properties: DHCPOptionsProperties) -> DHCPOptions {
        DHCPOptions { properties }
    }
}

/// The [`AWS::EC2::EIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-eip.html) resource type.
#[derive(Debug)]
pub struct EIP {
    properties: EIPProperties
}

/// Properties for the `EIP` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EIPProperties {
    /// Property `Domain`.
    #[serde(rename = "Domain")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<::Value<String>>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for EIP {
    type Properties = EIPProperties;
    const TYPE: &'static str = "AWS::EC2::EIP";
    fn properties(&self) -> &EIPProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EIPProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EIP {}

impl From<EIPProperties> for EIP {
    fn from(properties: EIPProperties) -> EIP {
        EIP { properties }
    }
}

/// The [`AWS::EC2::EIPAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-eip-association.html) resource type.
#[derive(Debug)]
pub struct EIPAssociation {
    properties: EIPAssociationProperties
}

/// Properties for the `EIPAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EIPAssociationProperties {
    /// Property `AllocationId`.
    #[serde(rename = "AllocationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<::Value<String>>,
    /// Property `EIP`.
    #[serde(rename = "EIP")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eip: Option<::Value<String>>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `NetworkInterfaceId`.
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<::Value<String>>,
    /// Property `PrivateIpAddress`.
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for EIPAssociation {
    type Properties = EIPAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::EIPAssociation";
    fn properties(&self) -> &EIPAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EIPAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EIPAssociation {}

impl From<EIPAssociationProperties> for EIPAssociation {
    fn from(properties: EIPAssociationProperties) -> EIPAssociation {
        EIPAssociation { properties }
    }
}

/// The [`AWS::EC2::EgressOnlyInternetGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-egressonlyinternetgateway.html) resource type.
#[derive(Debug)]
pub struct EgressOnlyInternetGateway {
    properties: EgressOnlyInternetGatewayProperties
}

/// Properties for the `EgressOnlyInternetGateway` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EgressOnlyInternetGatewayProperties {
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for EgressOnlyInternetGateway {
    type Properties = EgressOnlyInternetGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::EgressOnlyInternetGateway";
    fn properties(&self) -> &EgressOnlyInternetGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EgressOnlyInternetGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EgressOnlyInternetGateway {}

impl From<EgressOnlyInternetGatewayProperties> for EgressOnlyInternetGateway {
    fn from(properties: EgressOnlyInternetGatewayProperties) -> EgressOnlyInternetGateway {
        EgressOnlyInternetGateway { properties }
    }
}

/// The [`AWS::EC2::FlowLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-flowlog.html) resource type.
#[derive(Debug)]
pub struct FlowLog {
    properties: FlowLogProperties
}

/// Properties for the `FlowLog` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowLogProperties {
    /// Property `DeliverLogsPermissionArn`.
    #[serde(rename = "DeliverLogsPermissionArn")]
    pub deliver_logs_permission_arn: ::Value<String>,
    /// Property `LogGroupName`.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: ::Value<String>,
    /// Property `ResourceId`.
    #[serde(rename = "ResourceId")]
    pub resource_id: ::Value<String>,
    /// Property `ResourceType`.
    #[serde(rename = "ResourceType")]
    pub resource_type: ::Value<String>,
    /// Property `TrafficType`.
    #[serde(rename = "TrafficType")]
    pub traffic_type: ::Value<String>,
}

impl<'a> ::Resource<'a> for FlowLog {
    type Properties = FlowLogProperties;
    const TYPE: &'static str = "AWS::EC2::FlowLog";
    fn properties(&self) -> &FlowLogProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowLogProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowLog {}

impl From<FlowLogProperties> for FlowLog {
    fn from(properties: FlowLogProperties) -> FlowLog {
        FlowLog { properties }
    }
}

/// The [`AWS::EC2::Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-host.html) resource type.
#[derive(Debug)]
pub struct Host {
    properties: HostProperties
}

/// Properties for the `Host` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct HostProperties {
    /// Property `AutoPlacement`.
    #[serde(rename = "AutoPlacement")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_placement: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: ::Value<String>,
    /// Property `InstanceType`.
    #[serde(rename = "InstanceType")]
    pub instance_type: ::Value<String>,
}

impl<'a> ::Resource<'a> for Host {
    type Properties = HostProperties;
    const TYPE: &'static str = "AWS::EC2::Host";
    fn properties(&self) -> &HostProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HostProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Host {}

impl From<HostProperties> for Host {
    fn from(properties: HostProperties) -> Host {
        Host { properties }
    }
}

/// The [`AWS::EC2::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance.html) resource type.
#[derive(Debug)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceProperties {
    /// Property `AdditionalInfo`.
    #[serde(rename = "AdditionalInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<::Value<String>>,
    /// Property `Affinity`.
    #[serde(rename = "Affinity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<::Value<String>>,
    /// Property `BlockDeviceMappings`.
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>>,
    /// Property `CreditSpecification`.
    #[serde(rename = "CreditSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_specification: Option<::Value<self::instance::CreditSpecification>>,
    /// Property `DisableApiTermination`.
    #[serde(rename = "DisableApiTermination")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_api_termination: Option<::Value<bool>>,
    /// Property `EbsOptimized`.
    #[serde(rename = "EbsOptimized")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `ElasticGpuSpecifications`.
    #[serde(rename = "ElasticGpuSpecifications")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elastic_gpu_specifications: Option<::ValueList<self::instance::ElasticGpuSpecification>>,
    /// Property `HostId`.
    #[serde(rename = "HostId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<::Value<String>>,
    /// Property `IamInstanceProfile`.
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<::Value<String>>,
    /// Property `ImageId`.
    #[serde(rename = "ImageId")]
    pub image_id: ::Value<String>,
    /// Property `InstanceInitiatedShutdownBehavior`.
    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_initiated_shutdown_behavior: Option<::Value<String>>,
    /// Property `InstanceType`.
    #[serde(rename = "InstanceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<::Value<String>>,
    /// Property `Ipv6AddressCount`.
    #[serde(rename = "Ipv6AddressCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<::Value<u32>>,
    /// Property `Ipv6Addresses`.
    #[serde(rename = "Ipv6Addresses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<::ValueList<self::instance::InstanceIpv6Address>>,
    /// Property `KernelId`.
    #[serde(rename = "KernelId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<::Value<String>>,
    /// Property `KeyName`.
    #[serde(rename = "KeyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<::Value<String>>,
    /// Property `Monitoring`.
    #[serde(rename = "Monitoring")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<::Value<bool>>,
    /// Property `NetworkInterfaces`.
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<::ValueList<self::instance::NetworkInterface>>,
    /// Property `PlacementGroupName`.
    #[serde(rename = "PlacementGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_group_name: Option<::Value<String>>,
    /// Property `PrivateIpAddress`.
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<::Value<String>>,
    /// Property `RamdiskId`.
    #[serde(rename = "RamdiskId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramdisk_id: Option<::Value<String>>,
    /// Property `SecurityGroupIds`.
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SecurityGroups`.
    #[serde(rename = "SecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<::ValueList<String>>,
    /// Property `SourceDestCheck`.
    #[serde(rename = "SourceDestCheck")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<::Value<bool>>,
    /// Property `SsmAssociations`.
    #[serde(rename = "SsmAssociations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssm_associations: Option<::ValueList<self::instance::SsmAssociation>>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Tenancy`.
    #[serde(rename = "Tenancy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<::Value<String>>,
    /// Property `UserData`.
    #[serde(rename = "UserData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<::Value<String>>,
    /// Property `Volumes`.
    #[serde(rename = "Volumes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<::ValueList<self::instance::Volume>>,
}

impl<'a> ::Resource<'a> for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::EC2::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::EC2::InternetGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-internetgateway.html) resource type.
#[derive(Debug)]
pub struct InternetGateway {
    properties: InternetGatewayProperties
}

/// Properties for the `InternetGateway` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InternetGatewayProperties {
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for InternetGateway {
    type Properties = InternetGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::InternetGateway";
    fn properties(&self) -> &InternetGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InternetGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InternetGateway {}

impl From<InternetGatewayProperties> for InternetGateway {
    fn from(properties: InternetGatewayProperties) -> InternetGateway {
        InternetGateway { properties }
    }
}

/// The [`AWS::EC2::NatGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-natgateway.html) resource type.
#[derive(Debug)]
pub struct NatGateway {
    properties: NatGatewayProperties
}

/// Properties for the `NatGateway` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NatGatewayProperties {
    /// Property `AllocationId`.
    #[serde(rename = "AllocationId")]
    pub allocation_id: ::Value<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for NatGateway {
    type Properties = NatGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::NatGateway";
    fn properties(&self) -> &NatGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NatGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NatGateway {}

impl From<NatGatewayProperties> for NatGateway {
    fn from(properties: NatGatewayProperties) -> NatGateway {
        NatGateway { properties }
    }
}

/// The [`AWS::EC2::NetworkAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-acl.html) resource type.
#[derive(Debug)]
pub struct NetworkAcl {
    properties: NetworkAclProperties
}

/// Properties for the `NetworkAcl` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAclProperties {
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for NetworkAcl {
    type Properties = NetworkAclProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkAcl";
    fn properties(&self) -> &NetworkAclProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkAclProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkAcl {}

impl From<NetworkAclProperties> for NetworkAcl {
    fn from(properties: NetworkAclProperties) -> NetworkAcl {
        NetworkAcl { properties }
    }
}

/// The [`AWS::EC2::NetworkAclEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-acl-entry.html) resource type.
#[derive(Debug)]
pub struct NetworkAclEntry {
    properties: NetworkAclEntryProperties
}

/// Properties for the `NetworkAclEntry` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAclEntryProperties {
    /// Property `CidrBlock`.
    #[serde(rename = "CidrBlock")]
    pub cidr_block: ::Value<String>,
    /// Property `Egress`.
    #[serde(rename = "Egress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<::Value<bool>>,
    /// Property `Icmp`.
    #[serde(rename = "Icmp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icmp: Option<::Value<self::network_acl_entry::Icmp>>,
    /// Property `Ipv6CidrBlock`.
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<::Value<String>>,
    /// Property `NetworkAclId`.
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: ::Value<String>,
    /// Property `PortRange`.
    #[serde(rename = "PortRange")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_range: Option<::Value<self::network_acl_entry::PortRange>>,
    /// Property `Protocol`.
    #[serde(rename = "Protocol")]
    pub protocol: ::Value<u32>,
    /// Property `RuleAction`.
    #[serde(rename = "RuleAction")]
    pub rule_action: ::Value<String>,
    /// Property `RuleNumber`.
    #[serde(rename = "RuleNumber")]
    pub rule_number: ::Value<u32>,
}

impl<'a> ::Resource<'a> for NetworkAclEntry {
    type Properties = NetworkAclEntryProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkAclEntry";
    fn properties(&self) -> &NetworkAclEntryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkAclEntryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkAclEntry {}

impl From<NetworkAclEntryProperties> for NetworkAclEntry {
    fn from(properties: NetworkAclEntryProperties) -> NetworkAclEntry {
        NetworkAclEntry { properties }
    }
}

/// The [`AWS::EC2::NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-interface.html) resource type.
#[derive(Debug)]
pub struct NetworkInterface {
    properties: NetworkInterfaceProperties
}

/// Properties for the `NetworkInterface` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterfaceProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `GroupSet`.
    #[serde(rename = "GroupSet")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_set: Option<::ValueList<String>>,
    /// Property `InterfaceType`.
    #[serde(rename = "InterfaceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<::Value<String>>,
    /// Property `Ipv6AddressCount`.
    #[serde(rename = "Ipv6AddressCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<::Value<u32>>,
    /// Property `Ipv6Addresses`.
    #[serde(rename = "Ipv6Addresses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<::Value<self::network_interface::InstanceIpv6Address>>,
    /// Property `PrivateIpAddress`.
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<::Value<String>>,
    /// Property `PrivateIpAddresses`.
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<::ValueList<self::network_interface::PrivateIpAddressSpecification>>,
    /// Property `SecondaryPrivateIpAddressCount`.
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_private_ip_address_count: Option<::Value<u32>>,
    /// Property `SourceDestCheck`.
    #[serde(rename = "SourceDestCheck")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<::Value<bool>>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for NetworkInterface {
    type Properties = NetworkInterfaceProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterface";
    fn properties(&self) -> &NetworkInterfaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterface {}

impl From<NetworkInterfaceProperties> for NetworkInterface {
    fn from(properties: NetworkInterfaceProperties) -> NetworkInterface {
        NetworkInterface { properties }
    }
}

/// The [`AWS::EC2::NetworkInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-interface-attachment.html) resource type.
#[derive(Debug)]
pub struct NetworkInterfaceAttachment {
    properties: NetworkInterfaceAttachmentProperties
}

/// Properties for the `NetworkInterfaceAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterfaceAttachmentProperties {
    /// Property `DeleteOnTermination`.
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<::Value<bool>>,
    /// Property `DeviceIndex`.
    #[serde(rename = "DeviceIndex")]
    pub device_index: ::Value<String>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    pub instance_id: ::Value<String>,
    /// Property `NetworkInterfaceId`.
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for NetworkInterfaceAttachment {
    type Properties = NetworkInterfaceAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterfaceAttachment";
    fn properties(&self) -> &NetworkInterfaceAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfaceAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterfaceAttachment {}

impl From<NetworkInterfaceAttachmentProperties> for NetworkInterfaceAttachment {
    fn from(properties: NetworkInterfaceAttachmentProperties) -> NetworkInterfaceAttachment {
        NetworkInterfaceAttachment { properties }
    }
}

/// The [`AWS::EC2::NetworkInterfacePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinterfacepermission.html) resource type.
#[derive(Debug)]
pub struct NetworkInterfacePermission {
    properties: NetworkInterfacePermissionProperties
}

/// Properties for the `NetworkInterfacePermission` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterfacePermissionProperties {
    /// Property `AwsAccountId`.
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: ::Value<String>,
    /// Property `NetworkInterfaceId`.
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: ::Value<String>,
    /// Property `Permission`.
    #[serde(rename = "Permission")]
    pub permission: ::Value<String>,
}

impl<'a> ::Resource<'a> for NetworkInterfacePermission {
    type Properties = NetworkInterfacePermissionProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterfacePermission";
    fn properties(&self) -> &NetworkInterfacePermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfacePermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterfacePermission {}

impl From<NetworkInterfacePermissionProperties> for NetworkInterfacePermission {
    fn from(properties: NetworkInterfacePermissionProperties) -> NetworkInterfacePermission {
        NetworkInterfacePermission { properties }
    }
}

/// The [`AWS::EC2::PlacementGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-placementgroup.html) resource type.
#[derive(Debug)]
pub struct PlacementGroup {
    properties: PlacementGroupProperties
}

/// Properties for the `PlacementGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementGroupProperties {
    /// Property `Strategy`.
    #[serde(rename = "Strategy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for PlacementGroup {
    type Properties = PlacementGroupProperties;
    const TYPE: &'static str = "AWS::EC2::PlacementGroup";
    fn properties(&self) -> &PlacementGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlacementGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PlacementGroup {}

impl From<PlacementGroupProperties> for PlacementGroup {
    fn from(properties: PlacementGroupProperties) -> PlacementGroup {
        PlacementGroup { properties }
    }
}

/// The [`AWS::EC2::Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-route.html) resource type.
#[derive(Debug)]
pub struct Route {
    properties: RouteProperties
}

/// Properties for the `Route` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteProperties {
    /// Property `DestinationCidrBlock`.
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<::Value<String>>,
    /// Property `DestinationIpv6CidrBlock`.
    #[serde(rename = "DestinationIpv6CidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_ipv6_cidr_block: Option<::Value<String>>,
    /// Property `EgressOnlyInternetGatewayId`.
    #[serde(rename = "EgressOnlyInternetGatewayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress_only_internet_gateway_id: Option<::Value<String>>,
    /// Property `GatewayId`.
    #[serde(rename = "GatewayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<::Value<String>>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `NatGatewayId`.
    #[serde(rename = "NatGatewayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<::Value<String>>,
    /// Property `NetworkInterfaceId`.
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<::Value<String>>,
    /// Property `RouteTableId`.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: ::Value<String>,
    /// Property `VpcPeeringConnectionId`.
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Route {
    type Properties = RouteProperties;
    const TYPE: &'static str = "AWS::EC2::Route";
    fn properties(&self) -> &RouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Route {}

impl From<RouteProperties> for Route {
    fn from(properties: RouteProperties) -> Route {
        Route { properties }
    }
}

/// The [`AWS::EC2::RouteTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-route-table.html) resource type.
#[derive(Debug)]
pub struct RouteTable {
    properties: RouteTableProperties
}

/// Properties for the `RouteTable` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteTableProperties {
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for RouteTable {
    type Properties = RouteTableProperties;
    const TYPE: &'static str = "AWS::EC2::RouteTable";
    fn properties(&self) -> &RouteTableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteTableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RouteTable {}

impl From<RouteTableProperties> for RouteTable {
    fn from(properties: RouteTableProperties) -> RouteTable {
        RouteTable { properties }
    }
}

/// The [`AWS::EC2::SecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group.html) resource type.
#[derive(Debug)]
pub struct SecurityGroup {
    properties: SecurityGroupProperties
}

/// Properties for the `SecurityGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupProperties {
    /// Property `GroupDescription`.
    #[serde(rename = "GroupDescription")]
    pub group_description: ::Value<String>,
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<::Value<String>>,
    /// Property `SecurityGroupEgress`.
    #[serde(rename = "SecurityGroupEgress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_egress: Option<::ValueList<self::security_group::Egress>>,
    /// Property `SecurityGroupIngress`.
    #[serde(rename = "SecurityGroupIngress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_ingress: Option<::ValueList<self::security_group::Ingress>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for SecurityGroup {
    type Properties = SecurityGroupProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroup";
    fn properties(&self) -> &SecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroup {}

impl From<SecurityGroupProperties> for SecurityGroup {
    fn from(properties: SecurityGroupProperties) -> SecurityGroup {
        SecurityGroup { properties }
    }
}

/// The [`AWS::EC2::SecurityGroupEgress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-security-group-egress.html) resource type.
#[derive(Debug)]
pub struct SecurityGroupEgress {
    properties: SecurityGroupEgressProperties
}

/// Properties for the `SecurityGroupEgress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupEgressProperties {
    /// Property `CidrIp`.
    #[serde(rename = "CidrIp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<::Value<String>>,
    /// Property `CidrIpv6`.
    #[serde(rename = "CidrIpv6")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `DestinationPrefixListId`.
    #[serde(rename = "DestinationPrefixListId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_prefix_list_id: Option<::Value<String>>,
    /// Property `DestinationSecurityGroupId`.
    #[serde(rename = "DestinationSecurityGroupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_security_group_id: Option<::Value<String>>,
    /// Property `FromPort`.
    #[serde(rename = "FromPort")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_port: Option<::Value<u32>>,
    /// Property `GroupId`.
    #[serde(rename = "GroupId")]
    pub group_id: ::Value<String>,
    /// Property `IpProtocol`.
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: ::Value<String>,
    /// Property `ToPort`.
    #[serde(rename = "ToPort")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_port: Option<::Value<u32>>,
}

impl<'a> ::Resource<'a> for SecurityGroupEgress {
    type Properties = SecurityGroupEgressProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroupEgress";
    fn properties(&self) -> &SecurityGroupEgressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupEgressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroupEgress {}

impl From<SecurityGroupEgressProperties> for SecurityGroupEgress {
    fn from(properties: SecurityGroupEgressProperties) -> SecurityGroupEgress {
        SecurityGroupEgress { properties }
    }
}

/// The [`AWS::EC2::SecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-ingress.html) resource type.
#[derive(Debug)]
pub struct SecurityGroupIngress {
    properties: SecurityGroupIngressProperties
}

/// Properties for the `SecurityGroupIngress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupIngressProperties {
    /// Property `CidrIp`.
    #[serde(rename = "CidrIp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<::Value<String>>,
    /// Property `CidrIpv6`.
    #[serde(rename = "CidrIpv6")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `FromPort`.
    #[serde(rename = "FromPort")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_port: Option<::Value<u32>>,
    /// Property `GroupId`.
    #[serde(rename = "GroupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<::Value<String>>,
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<::Value<String>>,
    /// Property `IpProtocol`.
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: ::Value<String>,
    /// Property `SourceSecurityGroupId`.
    #[serde(rename = "SourceSecurityGroupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_security_group_id: Option<::Value<String>>,
    /// Property `SourceSecurityGroupName`.
    #[serde(rename = "SourceSecurityGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_security_group_name: Option<::Value<String>>,
    /// Property `SourceSecurityGroupOwnerId`.
    #[serde(rename = "SourceSecurityGroupOwnerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_security_group_owner_id: Option<::Value<String>>,
    /// Property `ToPort`.
    #[serde(rename = "ToPort")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_port: Option<::Value<u32>>,
}

impl<'a> ::Resource<'a> for SecurityGroupIngress {
    type Properties = SecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroupIngress";
    fn properties(&self) -> &SecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroupIngress {}

impl From<SecurityGroupIngressProperties> for SecurityGroupIngress {
    fn from(properties: SecurityGroupIngressProperties) -> SecurityGroupIngress {
        SecurityGroupIngress { properties }
    }
}

/// The [`AWS::EC2::SpotFleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-spotfleet.html) resource type.
#[derive(Debug)]
pub struct SpotFleet {
    properties: SpotFleetProperties
}

/// Properties for the `SpotFleet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpotFleetProperties {
    /// Property `SpotFleetRequestConfigData`.
    #[serde(rename = "SpotFleetRequestConfigData")]
    pub spot_fleet_request_config_data: ::Value<self::spot_fleet::SpotFleetRequestConfigData>,
}

impl<'a> ::Resource<'a> for SpotFleet {
    type Properties = SpotFleetProperties;
    const TYPE: &'static str = "AWS::EC2::SpotFleet";
    fn properties(&self) -> &SpotFleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SpotFleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SpotFleet {}

impl From<SpotFleetProperties> for SpotFleet {
    fn from(properties: SpotFleetProperties) -> SpotFleet {
        SpotFleet { properties }
    }
}

/// The [`AWS::EC2::Subnet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet.html) resource type.
#[derive(Debug)]
pub struct Subnet {
    properties: SubnetProperties
}

/// Properties for the `Subnet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetProperties {
    /// Property `AssignIpv6AddressOnCreation`.
    #[serde(rename = "AssignIpv6AddressOnCreation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_ipv6_address_on_creation: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<::Value<String>>,
    /// Property `CidrBlock`.
    #[serde(rename = "CidrBlock")]
    pub cidr_block: ::Value<String>,
    /// Property `Ipv6CidrBlock`.
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<::Value<String>>,
    /// Property `MapPublicIpOnLaunch`.
    #[serde(rename = "MapPublicIpOnLaunch")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_public_ip_on_launch: Option<::Value<bool>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for Subnet {
    type Properties = SubnetProperties;
    const TYPE: &'static str = "AWS::EC2::Subnet";
    fn properties(&self) -> &SubnetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Subnet {}

impl From<SubnetProperties> for Subnet {
    fn from(properties: SubnetProperties) -> Subnet {
        Subnet { properties }
    }
}

/// The [`AWS::EC2::SubnetCidrBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnetcidrblock.html) resource type.
#[derive(Debug)]
pub struct SubnetCidrBlock {
    properties: SubnetCidrBlockProperties
}

/// Properties for the `SubnetCidrBlock` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetCidrBlockProperties {
    /// Property `Ipv6CidrBlock`.
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: ::Value<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for SubnetCidrBlock {
    type Properties = SubnetCidrBlockProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetCidrBlock";
    fn properties(&self) -> &SubnetCidrBlockProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetCidrBlockProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetCidrBlock {}

impl From<SubnetCidrBlockProperties> for SubnetCidrBlock {
    fn from(properties: SubnetCidrBlockProperties) -> SubnetCidrBlock {
        SubnetCidrBlock { properties }
    }
}

/// The [`AWS::EC2::SubnetNetworkAclAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet-network-acl-assoc.html) resource type.
#[derive(Debug)]
pub struct SubnetNetworkAclAssociation {
    properties: SubnetNetworkAclAssociationProperties
}

/// Properties for the `SubnetNetworkAclAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetNetworkAclAssociationProperties {
    /// Property `NetworkAclId`.
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: ::Value<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for SubnetNetworkAclAssociation {
    type Properties = SubnetNetworkAclAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetNetworkAclAssociation";
    fn properties(&self) -> &SubnetNetworkAclAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetNetworkAclAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetNetworkAclAssociation {}

impl From<SubnetNetworkAclAssociationProperties> for SubnetNetworkAclAssociation {
    fn from(properties: SubnetNetworkAclAssociationProperties) -> SubnetNetworkAclAssociation {
        SubnetNetworkAclAssociation { properties }
    }
}

/// The [`AWS::EC2::SubnetRouteTableAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet-route-table-assoc.html) resource type.
#[derive(Debug)]
pub struct SubnetRouteTableAssociation {
    properties: SubnetRouteTableAssociationProperties
}

/// Properties for the `SubnetRouteTableAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetRouteTableAssociationProperties {
    /// Property `RouteTableId`.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: ::Value<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for SubnetRouteTableAssociation {
    type Properties = SubnetRouteTableAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetRouteTableAssociation";
    fn properties(&self) -> &SubnetRouteTableAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetRouteTableAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetRouteTableAssociation {}

impl From<SubnetRouteTableAssociationProperties> for SubnetRouteTableAssociation {
    fn from(properties: SubnetRouteTableAssociationProperties) -> SubnetRouteTableAssociation {
        SubnetRouteTableAssociation { properties }
    }
}

/// The [`AWS::EC2::TrunkInterfaceAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trunkinterfaceassociation.html) resource type.
#[derive(Debug)]
pub struct TrunkInterfaceAssociation {
    properties: TrunkInterfaceAssociationProperties
}

/// Properties for the `TrunkInterfaceAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrunkInterfaceAssociationProperties {
    /// Property `BranchInterfaceId`.
    #[serde(rename = "BranchInterfaceId")]
    pub branch_interface_id: ::Value<String>,
    /// Property `GREKey`.
    #[serde(rename = "GREKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gre_key: Option<::Value<u32>>,
    /// Property `TrunkInterfaceId`.
    #[serde(rename = "TrunkInterfaceId")]
    pub trunk_interface_id: ::Value<String>,
    /// Property `VLANId`.
    #[serde(rename = "VLANId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<::Value<u32>>,
}

impl<'a> ::Resource<'a> for TrunkInterfaceAssociation {
    type Properties = TrunkInterfaceAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::TrunkInterfaceAssociation";
    fn properties(&self) -> &TrunkInterfaceAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrunkInterfaceAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrunkInterfaceAssociation {}

impl From<TrunkInterfaceAssociationProperties> for TrunkInterfaceAssociation {
    fn from(properties: TrunkInterfaceAssociationProperties) -> TrunkInterfaceAssociation {
        TrunkInterfaceAssociation { properties }
    }
}

/// The [`AWS::EC2::VPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc.html) resource type.
#[derive(Debug)]
pub struct VPC {
    properties: VPCProperties
}

/// Properties for the `VPC` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCProperties {
    /// Property `CidrBlock`.
    #[serde(rename = "CidrBlock")]
    pub cidr_block: ::Value<String>,
    /// Property `EnableDnsHostnames`.
    #[serde(rename = "EnableDnsHostnames")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_dns_hostnames: Option<::Value<bool>>,
    /// Property `EnableDnsSupport`.
    #[serde(rename = "EnableDnsSupport")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_dns_support: Option<::Value<bool>>,
    /// Property `InstanceTenancy`.
    #[serde(rename = "InstanceTenancy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_tenancy: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for VPC {
    type Properties = VPCProperties;
    const TYPE: &'static str = "AWS::EC2::VPC";
    fn properties(&self) -> &VPCProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPC {}

impl From<VPCProperties> for VPC {
    fn from(properties: VPCProperties) -> VPC {
        VPC { properties }
    }
}

/// The [`AWS::EC2::VPCCidrBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpccidrblock.html) resource type.
#[derive(Debug)]
pub struct VPCCidrBlock {
    properties: VPCCidrBlockProperties
}

/// Properties for the `VPCCidrBlock` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCCidrBlockProperties {
    /// Property `AmazonProvidedIpv6CidrBlock`.
    #[serde(rename = "AmazonProvidedIpv6CidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amazon_provided_ipv6_cidr_block: Option<::Value<bool>>,
    /// Property `CidrBlock`.
    #[serde(rename = "CidrBlock")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<::Value<String>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPCCidrBlock {
    type Properties = VPCCidrBlockProperties;
    const TYPE: &'static str = "AWS::EC2::VPCCidrBlock";
    fn properties(&self) -> &VPCCidrBlockProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCCidrBlockProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCCidrBlock {}

impl From<VPCCidrBlockProperties> for VPCCidrBlock {
    fn from(properties: VPCCidrBlockProperties) -> VPCCidrBlock {
        VPCCidrBlock { properties }
    }
}

/// The [`AWS::EC2::VPCDHCPOptionsAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc-dhcp-options-assoc.html) resource type.
#[derive(Debug)]
pub struct VPCDHCPOptionsAssociation {
    properties: VPCDHCPOptionsAssociationProperties
}

/// Properties for the `VPCDHCPOptionsAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCDHCPOptionsAssociationProperties {
    /// Property `DhcpOptionsId`.
    #[serde(rename = "DhcpOptionsId")]
    pub dhcp_options_id: ::Value<String>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPCDHCPOptionsAssociation {
    type Properties = VPCDHCPOptionsAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::VPCDHCPOptionsAssociation";
    fn properties(&self) -> &VPCDHCPOptionsAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCDHCPOptionsAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCDHCPOptionsAssociation {}

impl From<VPCDHCPOptionsAssociationProperties> for VPCDHCPOptionsAssociation {
    fn from(properties: VPCDHCPOptionsAssociationProperties) -> VPCDHCPOptionsAssociation {
        VPCDHCPOptionsAssociation { properties }
    }
}

/// The [`AWS::EC2::VPCEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpoint.html) resource type.
#[derive(Debug)]
pub struct VPCEndpoint {
    properties: VPCEndpointProperties
}

/// Properties for the `VPCEndpoint` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCEndpointProperties {
    /// Property `PolicyDocument`.
    #[serde(rename = "PolicyDocument")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<::Value<::json::Value>>,
    /// Property `RouteTableIds`.
    #[serde(rename = "RouteTableIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<::ValueList<String>>,
    /// Property `ServiceName`.
    #[serde(rename = "ServiceName")]
    pub service_name: ::Value<String>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPCEndpoint {
    type Properties = VPCEndpointProperties;
    const TYPE: &'static str = "AWS::EC2::VPCEndpoint";
    fn properties(&self) -> &VPCEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCEndpoint {}

impl From<VPCEndpointProperties> for VPCEndpoint {
    fn from(properties: VPCEndpointProperties) -> VPCEndpoint {
        VPCEndpoint { properties }
    }
}

/// The [`AWS::EC2::VPCGatewayAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc-gateway-attachment.html) resource type.
#[derive(Debug)]
pub struct VPCGatewayAttachment {
    properties: VPCGatewayAttachmentProperties
}

/// Properties for the `VPCGatewayAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCGatewayAttachmentProperties {
    /// Property `InternetGatewayId`.
    #[serde(rename = "InternetGatewayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet_gateway_id: Option<::Value<String>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
    /// Property `VpnGatewayId`.
    #[serde(rename = "VpnGatewayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpn_gateway_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for VPCGatewayAttachment {
    type Properties = VPCGatewayAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::VPCGatewayAttachment";
    fn properties(&self) -> &VPCGatewayAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCGatewayAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCGatewayAttachment {}

impl From<VPCGatewayAttachmentProperties> for VPCGatewayAttachment {
    fn from(properties: VPCGatewayAttachmentProperties) -> VPCGatewayAttachment {
        VPCGatewayAttachment { properties }
    }
}

/// The [`AWS::EC2::VPCPeeringConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcpeeringconnection.html) resource type.
#[derive(Debug)]
pub struct VPCPeeringConnection {
    properties: VPCPeeringConnectionProperties
}

/// Properties for the `VPCPeeringConnection` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPCPeeringConnectionProperties {
    /// Property `PeerOwnerId`.
    #[serde(rename = "PeerOwnerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_owner_id: Option<::Value<String>>,
    /// Property `PeerRoleArn`.
    #[serde(rename = "PeerRoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_role_arn: Option<::Value<String>>,
    /// Property `PeerVpcId`.
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    pub vpc_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPCPeeringConnection {
    type Properties = VPCPeeringConnectionProperties;
    const TYPE: &'static str = "AWS::EC2::VPCPeeringConnection";
    fn properties(&self) -> &VPCPeeringConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCPeeringConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCPeeringConnection {}

impl From<VPCPeeringConnectionProperties> for VPCPeeringConnection {
    fn from(properties: VPCPeeringConnectionProperties) -> VPCPeeringConnection {
        VPCPeeringConnection { properties }
    }
}

/// The [`AWS::EC2::VPNConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-connection.html) resource type.
#[derive(Debug)]
pub struct VPNConnection {
    properties: VPNConnectionProperties
}

/// Properties for the `VPNConnection` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPNConnectionProperties {
    /// Property `CustomerGatewayId`.
    #[serde(rename = "CustomerGatewayId")]
    pub customer_gateway_id: ::Value<String>,
    /// Property `StaticRoutesOnly`.
    #[serde(rename = "StaticRoutesOnly")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_routes_only: Option<::Value<bool>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
    /// Property `VpnGatewayId`.
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: ::Value<String>,
    /// Property `VpnTunnelOptionsSpecifications`.
    #[serde(rename = "VpnTunnelOptionsSpecifications")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpn_tunnel_options_specifications: Option<::ValueList<self::vpn_connection::VpnTunnelOptionsSpecification>>,
}

impl<'a> ::Resource<'a> for VPNConnection {
    type Properties = VPNConnectionProperties;
    const TYPE: &'static str = "AWS::EC2::VPNConnection";
    fn properties(&self) -> &VPNConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNConnection {}

impl From<VPNConnectionProperties> for VPNConnection {
    fn from(properties: VPNConnectionProperties) -> VPNConnection {
        VPNConnection { properties }
    }
}

/// The [`AWS::EC2::VPNConnectionRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-connection-route.html) resource type.
#[derive(Debug)]
pub struct VPNConnectionRoute {
    properties: VPNConnectionRouteProperties
}

/// Properties for the `VPNConnectionRoute` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPNConnectionRouteProperties {
    /// Property `DestinationCidrBlock`.
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: ::Value<String>,
    /// Property `VpnConnectionId`.
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPNConnectionRoute {
    type Properties = VPNConnectionRouteProperties;
    const TYPE: &'static str = "AWS::EC2::VPNConnectionRoute";
    fn properties(&self) -> &VPNConnectionRouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNConnectionRouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNConnectionRoute {}

impl From<VPNConnectionRouteProperties> for VPNConnectionRoute {
    fn from(properties: VPNConnectionRouteProperties) -> VPNConnectionRoute {
        VPNConnectionRoute { properties }
    }
}

/// The [`AWS::EC2::VPNGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-gateway.html) resource type.
#[derive(Debug)]
pub struct VPNGateway {
    properties: VPNGatewayProperties
}

/// Properties for the `VPNGateway` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPNGatewayProperties {
    /// Property `AmazonSideAsn`.
    #[serde(rename = "AmazonSideAsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<::Value<u64>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPNGateway {
    type Properties = VPNGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::VPNGateway";
    fn properties(&self) -> &VPNGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNGateway {}

impl From<VPNGatewayProperties> for VPNGateway {
    fn from(properties: VPNGatewayProperties) -> VPNGateway {
        VPNGateway { properties }
    }
}

/// The [`AWS::EC2::VPNGatewayRoutePropagation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-gatewayrouteprop.html) resource type.
#[derive(Debug)]
pub struct VPNGatewayRoutePropagation {
    properties: VPNGatewayRoutePropagationProperties
}

/// Properties for the `VPNGatewayRoutePropagation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VPNGatewayRoutePropagationProperties {
    /// Property `RouteTableIds`.
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: ::ValueList<String>,
    /// Property `VpnGatewayId`.
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VPNGatewayRoutePropagation {
    type Properties = VPNGatewayRoutePropagationProperties;
    const TYPE: &'static str = "AWS::EC2::VPNGatewayRoutePropagation";
    fn properties(&self) -> &VPNGatewayRoutePropagationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNGatewayRoutePropagationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNGatewayRoutePropagation {}

impl From<VPNGatewayRoutePropagationProperties> for VPNGatewayRoutePropagation {
    fn from(properties: VPNGatewayRoutePropagationProperties) -> VPNGatewayRoutePropagation {
        VPNGatewayRoutePropagation { properties }
    }
}

/// The [`AWS::EC2::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ebs-volume.html) resource type.
#[derive(Debug)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeProperties {
    /// Property `AutoEnableIO`.
    #[serde(rename = "AutoEnableIO")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_enable_io: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: ::Value<String>,
    /// Property `Encrypted`.
    #[serde(rename = "Encrypted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<::Value<bool>>,
    /// Property `Iops`.
    #[serde(rename = "Iops")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<::Value<u32>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `Size`.
    #[serde(rename = "Size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<::Value<u32>>,
    /// Property `SnapshotId`.
    #[serde(rename = "SnapshotId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VolumeType`.
    #[serde(rename = "VolumeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Volume {
    type Properties = VolumeProperties;
    const TYPE: &'static str = "AWS::EC2::Volume";
    fn properties(&self) -> &VolumeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Volume {}

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

/// The [`AWS::EC2::VolumeAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ebs-volumeattachment.html) resource type.
#[derive(Debug)]
pub struct VolumeAttachment {
    properties: VolumeAttachmentProperties
}

/// Properties for the `VolumeAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeAttachmentProperties {
    /// Property `Device`.
    #[serde(rename = "Device")]
    pub device: ::Value<String>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    pub instance_id: ::Value<String>,
    /// Property `VolumeId`.
    #[serde(rename = "VolumeId")]
    pub volume_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for VolumeAttachment {
    type Properties = VolumeAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::VolumeAttachment";
    fn properties(&self) -> &VolumeAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VolumeAttachment {}

impl From<VolumeAttachmentProperties> for VolumeAttachment {
    fn from(properties: VolumeAttachmentProperties) -> VolumeAttachment {
        VolumeAttachment { properties }
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::EC2::Instance.AssociationParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociations-associationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssociationParameter {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(AssociationParameter);

    /// The [`AWS::EC2::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-mapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename = "DeviceName")]
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        #[serde(rename = "Ebs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs: Option<::Value<Ebs>>,
        /// Property `NoDevice`.
        #[serde(rename = "NoDevice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub no_device: Option<::Value<NoDevice>>,
        /// Property `VirtualName`.
        #[serde(rename = "VirtualName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(BlockDeviceMapping);

    /// The [`AWS::EC2::Instance.CreditSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-creditspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreditSpecification {
        /// Property `CPUCredits`.
        #[serde(rename = "CPUCredits")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu_credits: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CreditSpecification);

    /// The [`AWS::EC2::Instance.Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-template.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ebs {
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        #[serde(rename = "Encrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        #[serde(rename = "SnapshotId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        #[serde(rename = "VolumeSize")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Ebs);

    /// The [`AWS::EC2::Instance.ElasticGpuSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-elasticgpuspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticGpuSpecification {
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ElasticGpuSpecification);

    /// The [`AWS::EC2::Instance.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-instanceipv6address.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        #[serde(rename = "Ipv6Address")]
        pub ipv6_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InstanceIpv6Address);

    /// The [`AWS::EC2::Instance.NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-iface-embedded.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NetworkInterface {
        /// Property `AssociatePublicIpAddress`.
        #[serde(rename = "AssociatePublicIpAddress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub associate_public_ip_address: Option<::Value<bool>>,
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `DeviceIndex`.
        #[serde(rename = "DeviceIndex")]
        pub device_index: ::Value<String>,
        /// Property `GroupSet`.
        #[serde(rename = "GroupSet")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group_set: Option<::ValueList<String>>,
        /// Property `Ipv6AddressCount`.
        #[serde(rename = "Ipv6AddressCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_address_count: Option<::Value<u32>>,
        /// Property `Ipv6Addresses`.
        #[serde(rename = "Ipv6Addresses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_addresses: Option<::ValueList<InstanceIpv6Address>>,
        /// Property `NetworkInterfaceId`.
        #[serde(rename = "NetworkInterfaceId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub network_interface_id: Option<::Value<String>>,
        /// Property `PrivateIpAddress`.
        #[serde(rename = "PrivateIpAddress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_ip_address: Option<::Value<String>>,
        /// Property `PrivateIpAddresses`.
        #[serde(rename = "PrivateIpAddresses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_ip_addresses: Option<::ValueList<PrivateIpAddressSpecification>>,
        /// Property `SecondaryPrivateIpAddressCount`.
        #[serde(rename = "SecondaryPrivateIpAddressCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secondary_private_ip_address_count: Option<::Value<u32>>,
        /// Property `SubnetId`.
        #[serde(rename = "SubnetId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(NetworkInterface);

    /// The [`AWS::EC2::Instance.NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-nodevice.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NoDevice {
    }

    cfn_internal__inherit_codec_impls!(NoDevice);

    /// The [`AWS::EC2::Instance.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        #[serde(rename = "Primary")]
        pub primary: ::Value<bool>,
        /// Property `PrivateIpAddress`.
        #[serde(rename = "PrivateIpAddress")]
        pub private_ip_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PrivateIpAddressSpecification);

    /// The [`AWS::EC2::Instance.SsmAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SsmAssociation {
        /// Property `AssociationParameters`.
        #[serde(rename = "AssociationParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub association_parameters: Option<::ValueList<AssociationParameter>>,
        /// Property `DocumentName`.
        #[serde(rename = "DocumentName")]
        pub document_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SsmAssociation);

    /// The [`AWS::EC2::Instance.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-mount-point.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Volume {
        /// Property `Device`.
        #[serde(rename = "Device")]
        pub device: ::Value<String>,
        /// Property `VolumeId`.
        #[serde(rename = "VolumeId")]
        pub volume_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Volume);
}

pub mod network_acl_entry {
    //! Property types for the `NetworkAclEntry` resource.

    /// The [`AWS::EC2::NetworkAclEntry.Icmp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-icmp.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Icmp {
        /// Property `Code`.
        #[serde(rename = "Code")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<::Value<u32>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(Icmp);

    /// The [`AWS::EC2::NetworkAclEntry.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-portrange.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PortRange {
        /// Property `From`.
        #[serde(rename = "From")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<::Value<u32>>,
        /// Property `To`.
        #[serde(rename = "To")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(PortRange);
}

pub mod network_interface {
    //! Property types for the `NetworkInterface` resource.

    /// The [`AWS::EC2::NetworkInterface.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-instanceipv6address.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        #[serde(rename = "Ipv6Address")]
        pub ipv6_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InstanceIpv6Address);

    /// The [`AWS::EC2::NetworkInterface.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        #[serde(rename = "Primary")]
        pub primary: ::Value<bool>,
        /// Property `PrivateIpAddress`.
        #[serde(rename = "PrivateIpAddress")]
        pub private_ip_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PrivateIpAddressSpecification);
}

pub mod security_group {
    //! Property types for the `SecurityGroup` resource.

    /// The [`AWS::EC2::SecurityGroup.Egress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Egress {
        /// Property `CidrIp`.
        #[serde(rename = "CidrIp")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cidr_ip: Option<::Value<String>>,
        /// Property `CidrIpv6`.
        #[serde(rename = "CidrIpv6")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cidr_ipv6: Option<::Value<String>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `DestinationPrefixListId`.
        #[serde(rename = "DestinationPrefixListId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub destination_prefix_list_id: Option<::Value<String>>,
        /// Property `DestinationSecurityGroupId`.
        #[serde(rename = "DestinationSecurityGroupId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub destination_security_group_id: Option<::Value<String>>,
        /// Property `FromPort`.
        #[serde(rename = "FromPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from_port: Option<::Value<u32>>,
        /// Property `IpProtocol`.
        #[serde(rename = "IpProtocol")]
        pub ip_protocol: ::Value<String>,
        /// Property `ToPort`.
        #[serde(rename = "ToPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to_port: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(Egress);

    /// The [`AWS::EC2::SecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ingress {
        /// Property `CidrIp`.
        #[serde(rename = "CidrIp")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cidr_ip: Option<::Value<String>>,
        /// Property `CidrIpv6`.
        #[serde(rename = "CidrIpv6")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cidr_ipv6: Option<::Value<String>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `FromPort`.
        #[serde(rename = "FromPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from_port: Option<::Value<u32>>,
        /// Property `IpProtocol`.
        #[serde(rename = "IpProtocol")]
        pub ip_protocol: ::Value<String>,
        /// Property `SourceSecurityGroupId`.
        #[serde(rename = "SourceSecurityGroupId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_security_group_id: Option<::Value<String>>,
        /// Property `SourceSecurityGroupName`.
        #[serde(rename = "SourceSecurityGroupName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_security_group_name: Option<::Value<String>>,
        /// Property `SourceSecurityGroupOwnerId`.
        #[serde(rename = "SourceSecurityGroupOwnerId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_security_group_owner_id: Option<::Value<String>>,
        /// Property `ToPort`.
        #[serde(rename = "ToPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to_port: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(Ingress);
}

pub mod spot_fleet {
    //! Property types for the `SpotFleet` resource.

    /// The [`AWS::EC2::SpotFleet.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename = "DeviceName")]
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        #[serde(rename = "Ebs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs: Option<::Value<EbsBlockDevice>>,
        /// Property `NoDevice`.
        #[serde(rename = "NoDevice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub no_device: Option<::Value<String>>,
        /// Property `VirtualName`.
        #[serde(rename = "VirtualName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(BlockDeviceMapping);

    /// The [`AWS::EC2::SpotFleet.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings-ebs.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        #[serde(rename = "Encrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        #[serde(rename = "SnapshotId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        #[serde(rename = "VolumeSize")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(EbsBlockDevice);

    /// The [`AWS::EC2::SpotFleet.GroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-securitygroups.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GroupIdentifier {
        /// Property `GroupId`.
        #[serde(rename = "GroupId")]
        pub group_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(GroupIdentifier);

    /// The [`AWS::EC2::SpotFleet.IamInstanceProfileSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-iaminstanceprofile.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IamInstanceProfileSpecification {
        /// Property `Arn`.
        #[serde(rename = "Arn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(IamInstanceProfileSpecification);

    /// The [`AWS::EC2::SpotFleet.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instanceipv6address.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        #[serde(rename = "Ipv6Address")]
        pub ipv6_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InstanceIpv6Address);

    /// The [`AWS::EC2::SpotFleet.InstanceNetworkInterfaceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceNetworkInterfaceSpecification {
        /// Property `AssociatePublicIpAddress`.
        #[serde(rename = "AssociatePublicIpAddress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub associate_public_ip_address: Option<::Value<bool>>,
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `DeviceIndex`.
        #[serde(rename = "DeviceIndex")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub device_index: Option<::Value<u32>>,
        /// Property `Groups`.
        #[serde(rename = "Groups")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub groups: Option<::ValueList<String>>,
        /// Property `Ipv6AddressCount`.
        #[serde(rename = "Ipv6AddressCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_address_count: Option<::Value<u32>>,
        /// Property `Ipv6Addresses`.
        #[serde(rename = "Ipv6Addresses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_addresses: Option<::ValueList<InstanceIpv6Address>>,
        /// Property `NetworkInterfaceId`.
        #[serde(rename = "NetworkInterfaceId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub network_interface_id: Option<::Value<String>>,
        /// Property `PrivateIpAddresses`.
        #[serde(rename = "PrivateIpAddresses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_ip_addresses: Option<::ValueList<PrivateIpAddressSpecification>>,
        /// Property `SecondaryPrivateIpAddressCount`.
        #[serde(rename = "SecondaryPrivateIpAddressCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secondary_private_ip_address_count: Option<::Value<u32>>,
        /// Property `SubnetId`.
        #[serde(rename = "SubnetId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(InstanceNetworkInterfaceSpecification);

    /// The [`AWS::EC2::SpotFleet.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces-privateipaddresses.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        #[serde(rename = "Primary")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub primary: Option<::Value<bool>>,
        /// Property `PrivateIpAddress`.
        #[serde(rename = "PrivateIpAddress")]
        pub private_ip_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PrivateIpAddressSpecification);

    /// The [`AWS::EC2::SpotFleet.SpotFleetLaunchSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotFleetLaunchSpecification {
        /// Property `BlockDeviceMappings`.
        #[serde(rename = "BlockDeviceMappings")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block_device_mappings: Option<::ValueList<BlockDeviceMapping>>,
        /// Property `EbsOptimized`.
        #[serde(rename = "EbsOptimized")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_optimized: Option<::Value<bool>>,
        /// Property `IamInstanceProfile`.
        #[serde(rename = "IamInstanceProfile")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iam_instance_profile: Option<::Value<IamInstanceProfileSpecification>>,
        /// Property `ImageId`.
        #[serde(rename = "ImageId")]
        pub image_id: ::Value<String>,
        /// Property `InstanceType`.
        #[serde(rename = "InstanceType")]
        pub instance_type: ::Value<String>,
        /// Property `KernelId`.
        #[serde(rename = "KernelId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kernel_id: Option<::Value<String>>,
        /// Property `KeyName`.
        #[serde(rename = "KeyName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_name: Option<::Value<String>>,
        /// Property `Monitoring`.
        #[serde(rename = "Monitoring")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub monitoring: Option<::Value<SpotFleetMonitoring>>,
        /// Property `NetworkInterfaces`.
        #[serde(rename = "NetworkInterfaces")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub network_interfaces: Option<::ValueList<InstanceNetworkInterfaceSpecification>>,
        /// Property `Placement`.
        #[serde(rename = "Placement")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub placement: Option<::Value<SpotPlacement>>,
        /// Property `RamdiskId`.
        #[serde(rename = "RamdiskId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ramdisk_id: Option<::Value<String>>,
        /// Property `SecurityGroups`.
        #[serde(rename = "SecurityGroups")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub security_groups: Option<::ValueList<GroupIdentifier>>,
        /// Property `SpotPrice`.
        #[serde(rename = "SpotPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spot_price: Option<::Value<String>>,
        /// Property `SubnetId`.
        #[serde(rename = "SubnetId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet_id: Option<::Value<String>>,
        /// Property `TagSpecifications`.
        #[serde(rename = "TagSpecifications")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_specifications: Option<::ValueList<SpotFleetTagSpecification>>,
        /// Property `UserData`.
        #[serde(rename = "UserData")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_data: Option<::Value<String>>,
        /// Property `WeightedCapacity`.
        #[serde(rename = "WeightedCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub weighted_capacity: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(SpotFleetLaunchSpecification);

    /// The [`AWS::EC2::SpotFleet.SpotFleetMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-monitoring.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotFleetMonitoring {
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(SpotFleetMonitoring);

    /// The [`AWS::EC2::SpotFleet.SpotFleetRequestConfigData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotFleetRequestConfigData {
        /// Property `AllocationStrategy`.
        #[serde(rename = "AllocationStrategy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allocation_strategy: Option<::Value<String>>,
        /// Property `ExcessCapacityTerminationPolicy`.
        #[serde(rename = "ExcessCapacityTerminationPolicy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub excess_capacity_termination_policy: Option<::Value<String>>,
        /// Property `IamFleetRole`.
        #[serde(rename = "IamFleetRole")]
        pub iam_fleet_role: ::Value<String>,
        /// Property `LaunchSpecifications`.
        #[serde(rename = "LaunchSpecifications")]
        pub launch_specifications: ::ValueList<SpotFleetLaunchSpecification>,
        /// Property `ReplaceUnhealthyInstances`.
        #[serde(rename = "ReplaceUnhealthyInstances")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replace_unhealthy_instances: Option<::Value<bool>>,
        /// Property `SpotPrice`.
        #[serde(rename = "SpotPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spot_price: Option<::Value<String>>,
        /// Property `TargetCapacity`.
        #[serde(rename = "TargetCapacity")]
        pub target_capacity: ::Value<u32>,
        /// Property `TerminateInstancesWithExpiration`.
        #[serde(rename = "TerminateInstancesWithExpiration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub terminate_instances_with_expiration: Option<::Value<bool>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `ValidFrom`.
        #[serde(rename = "ValidFrom")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid_from: Option<::Value<String>>,
        /// Property `ValidUntil`.
        #[serde(rename = "ValidUntil")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid_until: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SpotFleetRequestConfigData);

    /// The [`AWS::EC2::SpotFleet.SpotFleetTagSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-tagspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotFleetTagSpecification {
        /// Property `ResourceType`.
        #[serde(rename = "ResourceType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SpotFleetTagSpecification);

    /// The [`AWS::EC2::SpotFleet.SpotPlacement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-placement.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotPlacement {
        /// Property `AvailabilityZone`.
        #[serde(rename = "AvailabilityZone")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub availability_zone: Option<::Value<String>>,
        /// Property `GroupName`.
        #[serde(rename = "GroupName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SpotPlacement);
}

pub mod vpn_connection {
    //! Property types for the `VPNConnection` resource.

    /// The [`AWS::EC2::VPNConnection.VpnTunnelOptionsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-vpntunneloptionsspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpnTunnelOptionsSpecification {
        /// Property `PreSharedKey`.
        #[serde(rename = "PreSharedKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pre_shared_key: Option<::Value<String>>,
        /// Property `TunnelInsideCidr`.
        #[serde(rename = "TunnelInsideCidr")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tunnel_inside_cidr: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(VpnTunnelOptionsSpecification);
}
