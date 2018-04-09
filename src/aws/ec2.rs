//! Types for the `EC2` service.

/// The [`AWS::EC2::CustomerGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-customer-gateway.html) resource type.
#[derive(Debug)]
pub struct CustomerGateway {
    properties: CustomerGatewayProperties
}

/// Properties for the `CustomerGateway` resource.
#[derive(Debug)]
pub struct CustomerGatewayProperties {
    /// Property `BgpAsn`.
    pub bgp_asn: ::Value<u32>,
    /// Property `IpAddress`.
    pub ip_address: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for CustomerGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BgpAsn", &self.bgp_asn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", &self.ip_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomerGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomerGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomerGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomerGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bgp_asn = None;
                let mut ip_address = None;
                let mut tags = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BgpAsn" => {
                            bgp_asn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IpAddress" => {
                            ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(CustomerGatewayProperties {
                    bgp_asn: bgp_asn.ok_or(::serde::de::Error::missing_field("BgpAsn"))?,
                    ip_address: ip_address.ok_or(::serde::de::Error::missing_field("IpAddress"))?,
                    tags: tags,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct DHCPOptionsProperties {
    /// Property `DomainName`.
    pub domain_name: Option<::Value<String>>,
    /// Property `DomainNameServers`.
    pub domain_name_servers: Option<::ValueList<String>>,
    /// Property `NetbiosNameServers`.
    pub netbios_name_servers: Option<::ValueList<String>>,
    /// Property `NetbiosNodeType`.
    pub netbios_node_type: Option<::Value<u32>>,
    /// Property `NtpServers`.
    pub ntp_servers: Option<::ValueList<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DHCPOptionsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainNameServers", &self.domain_name_servers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetbiosNameServers", &self.netbios_name_servers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetbiosNodeType", &self.netbios_node_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NtpServers", &self.ntp_servers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DHCPOptionsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DHCPOptionsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DHCPOptionsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DHCPOptionsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_name = None;
                let mut domain_name_servers = None;
                let mut netbios_name_servers = None;
                let mut netbios_node_type = None;
                let mut ntp_servers = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DomainNameServers" => {
                            domain_name_servers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetbiosNameServers" => {
                            netbios_name_servers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetbiosNodeType" => {
                            netbios_node_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NtpServers" => {
                            ntp_servers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DHCPOptionsProperties {
                    domain_name: domain_name,
                    domain_name_servers: domain_name_servers,
                    netbios_name_servers: netbios_name_servers,
                    netbios_node_type: netbios_node_type,
                    ntp_servers: ntp_servers,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct EIPProperties {
    /// Property `Domain`.
    pub domain: Option<::Value<String>>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
}

impl ::serde::Serialize for EIPProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EIPProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EIPProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EIPProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EIPProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain = None;
                let mut instance_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Domain" => {
                            domain = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EIPProperties {
                    domain: domain,
                    instance_id: instance_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct EIPAssociationProperties {
    /// Property `AllocationId`.
    pub allocation_id: Option<::Value<String>>,
    /// Property `EIP`.
    pub eip: Option<::Value<String>>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `NetworkInterfaceId`.
    pub network_interface_id: Option<::Value<String>>,
    /// Property `PrivateIpAddress`.
    pub private_ip_address: Option<::Value<String>>,
}

impl ::serde::Serialize for EIPAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationId", &self.allocation_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EIP", &self.eip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EIPAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EIPAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EIPAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EIPAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocation_id = None;
                let mut eip = None;
                let mut instance_id = None;
                let mut network_interface_id = None;
                let mut private_ip_address = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocationId" => {
                            allocation_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EIP" => {
                            eip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkInterfaceId" => {
                            network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PrivateIpAddress" => {
                            private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EIPAssociationProperties {
                    allocation_id: allocation_id,
                    eip: eip,
                    instance_id: instance_id,
                    network_interface_id: network_interface_id,
                    private_ip_address: private_ip_address,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct EgressOnlyInternetGatewayProperties {
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for EgressOnlyInternetGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EgressOnlyInternetGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EgressOnlyInternetGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EgressOnlyInternetGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EgressOnlyInternetGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EgressOnlyInternetGatewayProperties {
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct FlowLogProperties {
    /// Property `DeliverLogsPermissionArn`.
    pub deliver_logs_permission_arn: ::Value<String>,
    /// Property `LogGroupName`.
    pub log_group_name: ::Value<String>,
    /// Property `ResourceId`.
    pub resource_id: ::Value<String>,
    /// Property `ResourceType`.
    pub resource_type: ::Value<String>,
    /// Property `TrafficType`.
    pub traffic_type: ::Value<String>,
}

impl ::serde::Serialize for FlowLogProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliverLogsPermissionArn", &self.deliver_logs_permission_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrafficType", &self.traffic_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowLogProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowLogProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowLogProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowLogProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deliver_logs_permission_arn = None;
                let mut log_group_name = None;
                let mut resource_id = None;
                let mut resource_type = None;
                let mut traffic_type = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeliverLogsPermissionArn" => {
                            deliver_logs_permission_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LogGroupName" => {
                            log_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResourceId" => {
                            resource_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResourceType" => {
                            resource_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TrafficType" => {
                            traffic_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(FlowLogProperties {
                    deliver_logs_permission_arn: deliver_logs_permission_arn.ok_or(::serde::de::Error::missing_field("DeliverLogsPermissionArn"))?,
                    log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                    resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    traffic_type: traffic_type.ok_or(::serde::de::Error::missing_field("TrafficType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct HostProperties {
    /// Property `AutoPlacement`.
    pub auto_placement: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: ::Value<String>,
    /// Property `InstanceType`.
    pub instance_type: ::Value<String>,
}

impl ::serde::Serialize for HostProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoPlacement", &self.auto_placement)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HostProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HostProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HostProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HostProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_placement = None;
                let mut availability_zone = None;
                let mut instance_type = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoPlacement" => {
                            auto_placement = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(HostProperties {
                    auto_placement: auto_placement,
                    availability_zone: availability_zone.ok_or(::serde::de::Error::missing_field("AvailabilityZone"))?,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct InstanceProperties {
    /// Property `AdditionalInfo`.
    pub additional_info: Option<::Value<String>>,
    /// Property `Affinity`.
    pub affinity: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `BlockDeviceMappings`.
    pub block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>>,
    /// Property `CreditSpecification`.
    pub credit_specification: Option<::Value<self::instance::CreditSpecification>>,
    /// Property `DisableApiTermination`.
    pub disable_api_termination: Option<::Value<bool>>,
    /// Property `EbsOptimized`.
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `ElasticGpuSpecifications`.
    pub elastic_gpu_specifications: Option<::ValueList<self::instance::ElasticGpuSpecification>>,
    /// Property `HostId`.
    pub host_id: Option<::Value<String>>,
    /// Property `IamInstanceProfile`.
    pub iam_instance_profile: Option<::Value<String>>,
    /// Property `ImageId`.
    pub image_id: ::Value<String>,
    /// Property `InstanceInitiatedShutdownBehavior`.
    pub instance_initiated_shutdown_behavior: Option<::Value<String>>,
    /// Property `InstanceType`.
    pub instance_type: Option<::Value<String>>,
    /// Property `Ipv6AddressCount`.
    pub ipv6_address_count: Option<::Value<u32>>,
    /// Property `Ipv6Addresses`.
    pub ipv6_addresses: Option<::ValueList<self::instance::InstanceIpv6Address>>,
    /// Property `KernelId`.
    pub kernel_id: Option<::Value<String>>,
    /// Property `KeyName`.
    pub key_name: Option<::Value<String>>,
    /// Property `Monitoring`.
    pub monitoring: Option<::Value<bool>>,
    /// Property `NetworkInterfaces`.
    pub network_interfaces: Option<::ValueList<self::instance::NetworkInterface>>,
    /// Property `PlacementGroupName`.
    pub placement_group_name: Option<::Value<String>>,
    /// Property `PrivateIpAddress`.
    pub private_ip_address: Option<::Value<String>>,
    /// Property `RamdiskId`.
    pub ramdisk_id: Option<::Value<String>>,
    /// Property `SecurityGroupIds`.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SecurityGroups`.
    pub security_groups: Option<::ValueList<String>>,
    /// Property `SourceDestCheck`.
    pub source_dest_check: Option<::Value<bool>>,
    /// Property `SsmAssociations`.
    pub ssm_associations: Option<::ValueList<self::instance::SsmAssociation>>,
    /// Property `SubnetId`.
    pub subnet_id: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Tenancy`.
    pub tenancy: Option<::Value<String>>,
    /// Property `UserData`.
    pub user_data: Option<::Value<String>>,
    /// Property `Volumes`.
    pub volumes: Option<::ValueList<self::instance::Volume>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInfo", &self.additional_info)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Affinity", &self.affinity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", &self.block_device_mappings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreditSpecification", &self.credit_specification)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableApiTermination", &self.disable_api_termination)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticGpuSpecifications", &self.elastic_gpu_specifications)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostId", &self.host_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamInstanceProfile", &self.iam_instance_profile)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", &self.image_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceInitiatedShutdownBehavior", &self.instance_initiated_shutdown_behavior)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6AddressCount", &self.ipv6_address_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Addresses", &self.ipv6_addresses)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelId", &self.kernel_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyName", &self.key_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Monitoring", &self.monitoring)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaces", &self.network_interfaces)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementGroupName", &self.placement_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RamdiskId", &self.ramdisk_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDestCheck", &self.source_dest_check)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SsmAssociations", &self.ssm_associations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tenancy", &self.tenancy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserData", &self.user_data)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", &self.volumes)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_info = None;
                let mut affinity = None;
                let mut availability_zone = None;
                let mut block_device_mappings = None;
                let mut credit_specification = None;
                let mut disable_api_termination = None;
                let mut ebs_optimized = None;
                let mut elastic_gpu_specifications = None;
                let mut host_id = None;
                let mut iam_instance_profile = None;
                let mut image_id = None;
                let mut instance_initiated_shutdown_behavior = None;
                let mut instance_type = None;
                let mut ipv6_address_count = None;
                let mut ipv6_addresses = None;
                let mut kernel_id = None;
                let mut key_name = None;
                let mut monitoring = None;
                let mut network_interfaces = None;
                let mut placement_group_name = None;
                let mut private_ip_address = None;
                let mut ramdisk_id = None;
                let mut security_group_ids = None;
                let mut security_groups = None;
                let mut source_dest_check = None;
                let mut ssm_associations = None;
                let mut subnet_id = None;
                let mut tags = None;
                let mut tenancy = None;
                let mut user_data = None;
                let mut volumes = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalInfo" => {
                            additional_info = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Affinity" => {
                            affinity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CreditSpecification" => {
                            credit_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DisableApiTermination" => {
                            disable_api_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EbsOptimized" => {
                            ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ElasticGpuSpecifications" => {
                            elastic_gpu_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostId" => {
                            host_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IamInstanceProfile" => {
                            iam_instance_profile = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ImageId" => {
                            image_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceInitiatedShutdownBehavior" => {
                            instance_initiated_shutdown_behavior = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6AddressCount" => {
                            ipv6_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6Addresses" => {
                            ipv6_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KernelId" => {
                            kernel_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeyName" => {
                            key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Monitoring" => {
                            monitoring = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkInterfaces" => {
                            network_interfaces = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementGroupName" => {
                            placement_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PrivateIpAddress" => {
                            private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RamdiskId" => {
                            ramdisk_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroups" => {
                            security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceDestCheck" => {
                            source_dest_check = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SsmAssociations" => {
                            ssm_associations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tenancy" => {
                            tenancy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserData" => {
                            user_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Volumes" => {
                            volumes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    additional_info: additional_info,
                    affinity: affinity,
                    availability_zone: availability_zone,
                    block_device_mappings: block_device_mappings,
                    credit_specification: credit_specification,
                    disable_api_termination: disable_api_termination,
                    ebs_optimized: ebs_optimized,
                    elastic_gpu_specifications: elastic_gpu_specifications,
                    host_id: host_id,
                    iam_instance_profile: iam_instance_profile,
                    image_id: image_id.ok_or(::serde::de::Error::missing_field("ImageId"))?,
                    instance_initiated_shutdown_behavior: instance_initiated_shutdown_behavior,
                    instance_type: instance_type,
                    ipv6_address_count: ipv6_address_count,
                    ipv6_addresses: ipv6_addresses,
                    kernel_id: kernel_id,
                    key_name: key_name,
                    monitoring: monitoring,
                    network_interfaces: network_interfaces,
                    placement_group_name: placement_group_name,
                    private_ip_address: private_ip_address,
                    ramdisk_id: ramdisk_id,
                    security_group_ids: security_group_ids,
                    security_groups: security_groups,
                    source_dest_check: source_dest_check,
                    ssm_associations: ssm_associations,
                    subnet_id: subnet_id,
                    tags: tags,
                    tenancy: tenancy,
                    user_data: user_data,
                    volumes: volumes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct InternetGatewayProperties {
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for InternetGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InternetGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InternetGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InternetGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InternetGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InternetGatewayProperties {
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NatGatewayProperties {
    /// Property `AllocationId`.
    pub allocation_id: ::Value<String>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for NatGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationId", &self.allocation_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NatGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NatGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NatGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NatGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocation_id = None;
                let mut subnet_id = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocationId" => {
                            allocation_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NatGatewayProperties {
                    allocation_id: allocation_id.ok_or(::serde::de::Error::missing_field("AllocationId"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NetworkAclProperties {
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for NetworkAclProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkAclProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkAclProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkAclProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkAclProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NetworkAclProperties {
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NetworkAclEntryProperties {
    /// Property `CidrBlock`.
    pub cidr_block: ::Value<String>,
    /// Property `Egress`.
    pub egress: Option<::Value<bool>>,
    /// Property `Icmp`.
    pub icmp: Option<::Value<self::network_acl_entry::Icmp>>,
    /// Property `Ipv6CidrBlock`.
    pub ipv6_cidr_block: Option<::Value<String>>,
    /// Property `NetworkAclId`.
    pub network_acl_id: ::Value<String>,
    /// Property `PortRange`.
    pub port_range: Option<::Value<self::network_acl_entry::PortRange>>,
    /// Property `Protocol`.
    pub protocol: ::Value<u32>,
    /// Property `RuleAction`.
    pub rule_action: ::Value<String>,
    /// Property `RuleNumber`.
    pub rule_number: ::Value<u32>,
}

impl ::serde::Serialize for NetworkAclEntryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrBlock", &self.cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Egress", &self.egress)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Icmp", &self.icmp)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6CidrBlock", &self.ipv6_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkAclId", &self.network_acl_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortRange", &self.port_range)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleAction", &self.rule_action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleNumber", &self.rule_number)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkAclEntryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkAclEntryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkAclEntryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkAclEntryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidr_block = None;
                let mut egress = None;
                let mut icmp = None;
                let mut ipv6_cidr_block = None;
                let mut network_acl_id = None;
                let mut port_range = None;
                let mut protocol = None;
                let mut rule_action = None;
                let mut rule_number = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CidrBlock" => {
                            cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Egress" => {
                            egress = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Icmp" => {
                            icmp = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6CidrBlock" => {
                            ipv6_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkAclId" => {
                            network_acl_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PortRange" => {
                            port_range = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Protocol" => {
                            protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RuleAction" => {
                            rule_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RuleNumber" => {
                            rule_number = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NetworkAclEntryProperties {
                    cidr_block: cidr_block.ok_or(::serde::de::Error::missing_field("CidrBlock"))?,
                    egress: egress,
                    icmp: icmp,
                    ipv6_cidr_block: ipv6_cidr_block,
                    network_acl_id: network_acl_id.ok_or(::serde::de::Error::missing_field("NetworkAclId"))?,
                    port_range: port_range,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    rule_action: rule_action.ok_or(::serde::de::Error::missing_field("RuleAction"))?,
                    rule_number: rule_number.ok_or(::serde::de::Error::missing_field("RuleNumber"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NetworkInterfaceProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `GroupSet`.
    pub group_set: Option<::ValueList<String>>,
    /// Property `InterfaceType`.
    pub interface_type: Option<::Value<String>>,
    /// Property `Ipv6AddressCount`.
    pub ipv6_address_count: Option<::Value<u32>>,
    /// Property `Ipv6Addresses`.
    pub ipv6_addresses: Option<::Value<self::network_interface::InstanceIpv6Address>>,
    /// Property `PrivateIpAddress`.
    pub private_ip_address: Option<::Value<String>>,
    /// Property `PrivateIpAddresses`.
    pub private_ip_addresses: Option<::ValueList<self::network_interface::PrivateIpAddressSpecification>>,
    /// Property `SecondaryPrivateIpAddressCount`.
    pub secondary_private_ip_address_count: Option<::Value<u32>>,
    /// Property `SourceDestCheck`.
    pub source_dest_check: Option<::Value<bool>>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for NetworkInterfaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupSet", &self.group_set)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InterfaceType", &self.interface_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6AddressCount", &self.ipv6_address_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Addresses", &self.ipv6_addresses)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddresses", &self.private_ip_addresses)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryPrivateIpAddressCount", &self.secondary_private_ip_address_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDestCheck", &self.source_dest_check)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkInterfaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterfaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkInterfaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkInterfaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut group_set = None;
                let mut interface_type = None;
                let mut ipv6_address_count = None;
                let mut ipv6_addresses = None;
                let mut private_ip_address = None;
                let mut private_ip_addresses = None;
                let mut secondary_private_ip_address_count = None;
                let mut source_dest_check = None;
                let mut subnet_id = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupSet" => {
                            group_set = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InterfaceType" => {
                            interface_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6AddressCount" => {
                            ipv6_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6Addresses" => {
                            ipv6_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PrivateIpAddress" => {
                            private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PrivateIpAddresses" => {
                            private_ip_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecondaryPrivateIpAddressCount" => {
                            secondary_private_ip_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceDestCheck" => {
                            source_dest_check = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NetworkInterfaceProperties {
                    description: description,
                    group_set: group_set,
                    interface_type: interface_type,
                    ipv6_address_count: ipv6_address_count,
                    ipv6_addresses: ipv6_addresses,
                    private_ip_address: private_ip_address,
                    private_ip_addresses: private_ip_addresses,
                    secondary_private_ip_address_count: secondary_private_ip_address_count,
                    source_dest_check: source_dest_check,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NetworkInterfaceAttachmentProperties {
    /// Property `DeleteOnTermination`.
    pub delete_on_termination: Option<::Value<bool>>,
    /// Property `DeviceIndex`.
    pub device_index: ::Value<String>,
    /// Property `InstanceId`.
    pub instance_id: ::Value<String>,
    /// Property `NetworkInterfaceId`.
    pub network_interface_id: ::Value<String>,
}

impl ::serde::Serialize for NetworkInterfaceAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceIndex", &self.device_index)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkInterfaceAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterfaceAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkInterfaceAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkInterfaceAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut delete_on_termination = None;
                let mut device_index = None;
                let mut instance_id = None;
                let mut network_interface_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeleteOnTermination" => {
                            delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeviceIndex" => {
                            device_index = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkInterfaceId" => {
                            network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NetworkInterfaceAttachmentProperties {
                    delete_on_termination: delete_on_termination,
                    device_index: device_index.ok_or(::serde::de::Error::missing_field("DeviceIndex"))?,
                    instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    network_interface_id: network_interface_id.ok_or(::serde::de::Error::missing_field("NetworkInterfaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct NetworkInterfacePermissionProperties {
    /// Property `AwsAccountId`.
    pub aws_account_id: ::Value<String>,
    /// Property `NetworkInterfaceId`.
    pub network_interface_id: ::Value<String>,
    /// Property `Permission`.
    pub permission: ::Value<String>,
}

impl ::serde::Serialize for NetworkInterfacePermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", &self.permission)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkInterfacePermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterfacePermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkInterfacePermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkInterfacePermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id = None;
                let mut network_interface_id = None;
                let mut permission = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkInterfaceId" => {
                            network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Permission" => {
                            permission = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(NetworkInterfacePermissionProperties {
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    network_interface_id: network_interface_id.ok_or(::serde::de::Error::missing_field("NetworkInterfaceId"))?,
                    permission: permission.ok_or(::serde::de::Error::missing_field("Permission"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct PlacementGroupProperties {
    /// Property `Strategy`.
    pub strategy: Option<::Value<String>>,
}

impl ::serde::Serialize for PlacementGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Strategy", &self.strategy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PlacementGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PlacementGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PlacementGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut strategy = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Strategy" => {
                            strategy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(PlacementGroupProperties {
                    strategy: strategy,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct RouteProperties {
    /// Property `DestinationCidrBlock`.
    pub destination_cidr_block: Option<::Value<String>>,
    /// Property `DestinationIpv6CidrBlock`.
    pub destination_ipv6_cidr_block: Option<::Value<String>>,
    /// Property `EgressOnlyInternetGatewayId`.
    pub egress_only_internet_gateway_id: Option<::Value<String>>,
    /// Property `GatewayId`.
    pub gateway_id: Option<::Value<String>>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `NatGatewayId`.
    pub nat_gateway_id: Option<::Value<String>>,
    /// Property `NetworkInterfaceId`.
    pub network_interface_id: Option<::Value<String>>,
    /// Property `RouteTableId`.
    pub route_table_id: ::Value<String>,
    /// Property `VpcPeeringConnectionId`.
    pub vpc_peering_connection_id: Option<::Value<String>>,
}

impl ::serde::Serialize for RouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationCidrBlock", &self.destination_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationIpv6CidrBlock", &self.destination_ipv6_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressOnlyInternetGatewayId", &self.egress_only_internet_gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayId", &self.gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NatGatewayId", &self.nat_gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteTableId", &self.route_table_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcPeeringConnectionId", &self.vpc_peering_connection_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_cidr_block = None;
                let mut destination_ipv6_cidr_block = None;
                let mut egress_only_internet_gateway_id = None;
                let mut gateway_id = None;
                let mut instance_id = None;
                let mut nat_gateway_id = None;
                let mut network_interface_id = None;
                let mut route_table_id = None;
                let mut vpc_peering_connection_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationCidrBlock" => {
                            destination_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DestinationIpv6CidrBlock" => {
                            destination_ipv6_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EgressOnlyInternetGatewayId" => {
                            egress_only_internet_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GatewayId" => {
                            gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NatGatewayId" => {
                            nat_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkInterfaceId" => {
                            network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RouteTableId" => {
                            route_table_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcPeeringConnectionId" => {
                            vpc_peering_connection_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RouteProperties {
                    destination_cidr_block: destination_cidr_block,
                    destination_ipv6_cidr_block: destination_ipv6_cidr_block,
                    egress_only_internet_gateway_id: egress_only_internet_gateway_id,
                    gateway_id: gateway_id,
                    instance_id: instance_id,
                    nat_gateway_id: nat_gateway_id,
                    network_interface_id: network_interface_id,
                    route_table_id: route_table_id.ok_or(::serde::de::Error::missing_field("RouteTableId"))?,
                    vpc_peering_connection_id: vpc_peering_connection_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct RouteTableProperties {
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for RouteTableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteTableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteTableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteTableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteTableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RouteTableProperties {
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SecurityGroupProperties {
    /// Property `GroupDescription`.
    pub group_description: ::Value<String>,
    /// Property `GroupName`.
    pub group_name: Option<::Value<String>>,
    /// Property `SecurityGroupEgress`.
    pub security_group_egress: Option<::ValueList<self::security_group::Egress>>,
    /// Property `SecurityGroupIngress`.
    pub security_group_ingress: Option<::ValueList<self::security_group::Ingress>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for SecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupDescription", &self.group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupEgress", &self.security_group_egress)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIngress", &self.security_group_ingress)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_description = None;
                let mut group_name = None;
                let mut security_group_egress = None;
                let mut security_group_ingress = None;
                let mut tags = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupDescription" => {
                            group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroupEgress" => {
                            security_group_egress = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroupIngress" => {
                            security_group_ingress = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SecurityGroupProperties {
                    group_description: group_description.ok_or(::serde::de::Error::missing_field("GroupDescription"))?,
                    group_name: group_name,
                    security_group_egress: security_group_egress,
                    security_group_ingress: security_group_ingress,
                    tags: tags,
                    vpc_id: vpc_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SecurityGroupEgressProperties {
    /// Property `CidrIp`.
    pub cidr_ip: Option<::Value<String>>,
    /// Property `CidrIpv6`.
    pub cidr_ipv6: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `DestinationPrefixListId`.
    pub destination_prefix_list_id: Option<::Value<String>>,
    /// Property `DestinationSecurityGroupId`.
    pub destination_security_group_id: Option<::Value<String>>,
    /// Property `FromPort`.
    pub from_port: Option<::Value<u32>>,
    /// Property `GroupId`.
    pub group_id: ::Value<String>,
    /// Property `IpProtocol`.
    pub ip_protocol: ::Value<String>,
    /// Property `ToPort`.
    pub to_port: Option<::Value<u32>>,
}

impl ::serde::Serialize for SecurityGroupEgressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIp", &self.cidr_ip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIpv6", &self.cidr_ipv6)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPrefixListId", &self.destination_prefix_list_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationSecurityGroupId", &self.destination_security_group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpProtocol", &self.ip_protocol)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityGroupEgressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityGroupEgressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityGroupEgressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityGroupEgressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidr_ip = None;
                let mut cidr_ipv6 = None;
                let mut description = None;
                let mut destination_prefix_list_id = None;
                let mut destination_security_group_id = None;
                let mut from_port = None;
                let mut group_id = None;
                let mut ip_protocol = None;
                let mut to_port = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CidrIp" => {
                            cidr_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CidrIpv6" => {
                            cidr_ipv6 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DestinationPrefixListId" => {
                            destination_prefix_list_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DestinationSecurityGroupId" => {
                            destination_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FromPort" => {
                            from_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupId" => {
                            group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IpProtocol" => {
                            ip_protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ToPort" => {
                            to_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SecurityGroupEgressProperties {
                    cidr_ip: cidr_ip,
                    cidr_ipv6: cidr_ipv6,
                    description: description,
                    destination_prefix_list_id: destination_prefix_list_id,
                    destination_security_group_id: destination_security_group_id,
                    from_port: from_port,
                    group_id: group_id.ok_or(::serde::de::Error::missing_field("GroupId"))?,
                    ip_protocol: ip_protocol.ok_or(::serde::de::Error::missing_field("IpProtocol"))?,
                    to_port: to_port,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SecurityGroupIngressProperties {
    /// Property `CidrIp`.
    pub cidr_ip: Option<::Value<String>>,
    /// Property `CidrIpv6`.
    pub cidr_ipv6: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `FromPort`.
    pub from_port: Option<::Value<u32>>,
    /// Property `GroupId`.
    pub group_id: Option<::Value<String>>,
    /// Property `GroupName`.
    pub group_name: Option<::Value<String>>,
    /// Property `IpProtocol`.
    pub ip_protocol: ::Value<String>,
    /// Property `SourceSecurityGroupId`.
    pub source_security_group_id: Option<::Value<String>>,
    /// Property `SourceSecurityGroupName`.
    pub source_security_group_name: Option<::Value<String>>,
    /// Property `SourceSecurityGroupOwnerId`.
    pub source_security_group_owner_id: Option<::Value<String>>,
    /// Property `ToPort`.
    pub to_port: Option<::Value<u32>>,
}

impl ::serde::Serialize for SecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIp", &self.cidr_ip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIpv6", &self.cidr_ipv6)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpProtocol", &self.ip_protocol)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupId", &self.source_security_group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupName", &self.source_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupOwnerId", &self.source_security_group_owner_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidr_ip = None;
                let mut cidr_ipv6 = None;
                let mut description = None;
                let mut from_port = None;
                let mut group_id = None;
                let mut group_name = None;
                let mut ip_protocol = None;
                let mut source_security_group_id = None;
                let mut source_security_group_name = None;
                let mut source_security_group_owner_id = None;
                let mut to_port = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CidrIp" => {
                            cidr_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CidrIpv6" => {
                            cidr_ipv6 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FromPort" => {
                            from_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupId" => {
                            group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IpProtocol" => {
                            ip_protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceSecurityGroupId" => {
                            source_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceSecurityGroupName" => {
                            source_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceSecurityGroupOwnerId" => {
                            source_security_group_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ToPort" => {
                            to_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SecurityGroupIngressProperties {
                    cidr_ip: cidr_ip,
                    cidr_ipv6: cidr_ipv6,
                    description: description,
                    from_port: from_port,
                    group_id: group_id,
                    group_name: group_name,
                    ip_protocol: ip_protocol.ok_or(::serde::de::Error::missing_field("IpProtocol"))?,
                    source_security_group_id: source_security_group_id,
                    source_security_group_name: source_security_group_name,
                    source_security_group_owner_id: source_security_group_owner_id,
                    to_port: to_port,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SpotFleetProperties {
    /// Property `SpotFleetRequestConfigData`.
    pub spot_fleet_request_config_data: ::Value<self::spot_fleet::SpotFleetRequestConfigData>,
}

impl ::serde::Serialize for SpotFleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotFleetRequestConfigData", &self.spot_fleet_request_config_data)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SpotFleetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotFleetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SpotFleetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SpotFleetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut spot_fleet_request_config_data = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SpotFleetRequestConfigData" => {
                            spot_fleet_request_config_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SpotFleetProperties {
                    spot_fleet_request_config_data: spot_fleet_request_config_data.ok_or(::serde::de::Error::missing_field("SpotFleetRequestConfigData"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SubnetProperties {
    /// Property `AssignIpv6AddressOnCreation`.
    pub assign_ipv6_address_on_creation: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `CidrBlock`.
    pub cidr_block: ::Value<String>,
    /// Property `Ipv6CidrBlock`.
    pub ipv6_cidr_block: Option<::Value<String>>,
    /// Property `MapPublicIpOnLaunch`.
    pub map_public_ip_on_launch: Option<::Value<bool>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for SubnetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignIpv6AddressOnCreation", &self.assign_ipv6_address_on_creation)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrBlock", &self.cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6CidrBlock", &self.ipv6_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapPublicIpOnLaunch", &self.map_public_ip_on_launch)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assign_ipv6_address_on_creation = None;
                let mut availability_zone = None;
                let mut cidr_block = None;
                let mut ipv6_cidr_block = None;
                let mut map_public_ip_on_launch = None;
                let mut tags = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssignIpv6AddressOnCreation" => {
                            assign_ipv6_address_on_creation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CidrBlock" => {
                            cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ipv6CidrBlock" => {
                            ipv6_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MapPublicIpOnLaunch" => {
                            map_public_ip_on_launch = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SubnetProperties {
                    assign_ipv6_address_on_creation: assign_ipv6_address_on_creation,
                    availability_zone: availability_zone,
                    cidr_block: cidr_block.ok_or(::serde::de::Error::missing_field("CidrBlock"))?,
                    ipv6_cidr_block: ipv6_cidr_block,
                    map_public_ip_on_launch: map_public_ip_on_launch,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SubnetCidrBlockProperties {
    /// Property `Ipv6CidrBlock`.
    pub ipv6_cidr_block: ::Value<String>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for SubnetCidrBlockProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6CidrBlock", &self.ipv6_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetCidrBlockProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetCidrBlockProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetCidrBlockProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetCidrBlockProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ipv6_cidr_block = None;
                let mut subnet_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Ipv6CidrBlock" => {
                            ipv6_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SubnetCidrBlockProperties {
                    ipv6_cidr_block: ipv6_cidr_block.ok_or(::serde::de::Error::missing_field("Ipv6CidrBlock"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SubnetNetworkAclAssociationProperties {
    /// Property `NetworkAclId`.
    pub network_acl_id: ::Value<String>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for SubnetNetworkAclAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkAclId", &self.network_acl_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetNetworkAclAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetNetworkAclAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetNetworkAclAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetNetworkAclAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut network_acl_id = None;
                let mut subnet_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "NetworkAclId" => {
                            network_acl_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SubnetNetworkAclAssociationProperties {
                    network_acl_id: network_acl_id.ok_or(::serde::de::Error::missing_field("NetworkAclId"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct SubnetRouteTableAssociationProperties {
    /// Property `RouteTableId`.
    pub route_table_id: ::Value<String>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for SubnetRouteTableAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteTableId", &self.route_table_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetRouteTableAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetRouteTableAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetRouteTableAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetRouteTableAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut route_table_id = None;
                let mut subnet_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RouteTableId" => {
                            route_table_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SubnetRouteTableAssociationProperties {
                    route_table_id: route_table_id.ok_or(::serde::de::Error::missing_field("RouteTableId"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct TrunkInterfaceAssociationProperties {
    /// Property `BranchInterfaceId`.
    pub branch_interface_id: ::Value<String>,
    /// Property `GREKey`.
    pub gre_key: Option<::Value<u32>>,
    /// Property `TrunkInterfaceId`.
    pub trunk_interface_id: ::Value<String>,
    /// Property `VLANId`.
    pub vlan_id: Option<::Value<u32>>,
}

impl ::serde::Serialize for TrunkInterfaceAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BranchInterfaceId", &self.branch_interface_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GREKey", &self.gre_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrunkInterfaceId", &self.trunk_interface_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VLANId", &self.vlan_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrunkInterfaceAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrunkInterfaceAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrunkInterfaceAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrunkInterfaceAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut branch_interface_id = None;
                let mut gre_key = None;
                let mut trunk_interface_id = None;
                let mut vlan_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BranchInterfaceId" => {
                            branch_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GREKey" => {
                            gre_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TrunkInterfaceId" => {
                            trunk_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VLANId" => {
                            vlan_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(TrunkInterfaceAssociationProperties {
                    branch_interface_id: branch_interface_id.ok_or(::serde::de::Error::missing_field("BranchInterfaceId"))?,
                    gre_key: gre_key,
                    trunk_interface_id: trunk_interface_id.ok_or(::serde::de::Error::missing_field("TrunkInterfaceId"))?,
                    vlan_id: vlan_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCProperties {
    /// Property `CidrBlock`.
    pub cidr_block: ::Value<String>,
    /// Property `EnableDnsHostnames`.
    pub enable_dns_hostnames: Option<::Value<bool>>,
    /// Property `EnableDnsSupport`.
    pub enable_dns_support: Option<::Value<bool>>,
    /// Property `InstanceTenancy`.
    pub instance_tenancy: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for VPCProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrBlock", &self.cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDnsHostnames", &self.enable_dns_hostnames)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDnsSupport", &self.enable_dns_support)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTenancy", &self.instance_tenancy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidr_block = None;
                let mut enable_dns_hostnames = None;
                let mut enable_dns_support = None;
                let mut instance_tenancy = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CidrBlock" => {
                            cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableDnsHostnames" => {
                            enable_dns_hostnames = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableDnsSupport" => {
                            enable_dns_support = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceTenancy" => {
                            instance_tenancy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCProperties {
                    cidr_block: cidr_block.ok_or(::serde::de::Error::missing_field("CidrBlock"))?,
                    enable_dns_hostnames: enable_dns_hostnames,
                    enable_dns_support: enable_dns_support,
                    instance_tenancy: instance_tenancy,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCCidrBlockProperties {
    /// Property `AmazonProvidedIpv6CidrBlock`.
    pub amazon_provided_ipv6_cidr_block: Option<::Value<bool>>,
    /// Property `CidrBlock`.
    pub cidr_block: Option<::Value<String>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for VPCCidrBlockProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonProvidedIpv6CidrBlock", &self.amazon_provided_ipv6_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrBlock", &self.cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCCidrBlockProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCCidrBlockProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCCidrBlockProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCCidrBlockProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut amazon_provided_ipv6_cidr_block = None;
                let mut cidr_block = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AmazonProvidedIpv6CidrBlock" => {
                            amazon_provided_ipv6_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CidrBlock" => {
                            cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCCidrBlockProperties {
                    amazon_provided_ipv6_cidr_block: amazon_provided_ipv6_cidr_block,
                    cidr_block: cidr_block,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCDHCPOptionsAssociationProperties {
    /// Property `DhcpOptionsId`.
    pub dhcp_options_id: ::Value<String>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for VPCDHCPOptionsAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DhcpOptionsId", &self.dhcp_options_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCDHCPOptionsAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCDHCPOptionsAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCDHCPOptionsAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCDHCPOptionsAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dhcp_options_id = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DhcpOptionsId" => {
                            dhcp_options_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCDHCPOptionsAssociationProperties {
                    dhcp_options_id: dhcp_options_id.ok_or(::serde::de::Error::missing_field("DhcpOptionsId"))?,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCEndpointProperties {
    /// Property `PolicyDocument`.
    pub policy_document: Option<::Value<::json::Value>>,
    /// Property `RouteTableIds`.
    pub route_table_ids: Option<::ValueList<String>>,
    /// Property `ServiceName`.
    pub service_name: ::Value<String>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for VPCEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteTableIds", &self.route_table_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", &self.service_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_document = None;
                let mut route_table_ids = None;
                let mut service_name = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RouteTableIds" => {
                            route_table_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceName" => {
                            service_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCEndpointProperties {
                    policy_document: policy_document,
                    route_table_ids: route_table_ids,
                    service_name: service_name.ok_or(::serde::de::Error::missing_field("ServiceName"))?,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCGatewayAttachmentProperties {
    /// Property `InternetGatewayId`.
    pub internet_gateway_id: Option<::Value<String>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
    /// Property `VpnGatewayId`.
    pub vpn_gateway_id: Option<::Value<String>>,
}

impl ::serde::Serialize for VPCGatewayAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InternetGatewayId", &self.internet_gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnGatewayId", &self.vpn_gateway_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCGatewayAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCGatewayAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCGatewayAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCGatewayAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut internet_gateway_id = None;
                let mut vpc_id = None;
                let mut vpn_gateway_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InternetGatewayId" => {
                            internet_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpnGatewayId" => {
                            vpn_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCGatewayAttachmentProperties {
                    internet_gateway_id: internet_gateway_id,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                    vpn_gateway_id: vpn_gateway_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPCPeeringConnectionProperties {
    /// Property `PeerOwnerId`.
    pub peer_owner_id: Option<::Value<String>>,
    /// Property `PeerRoleArn`.
    pub peer_role_arn: Option<::Value<String>>,
    /// Property `PeerVpcId`.
    pub peer_vpc_id: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for VPCPeeringConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerOwnerId", &self.peer_owner_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerRoleArn", &self.peer_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerVpcId", &self.peer_vpc_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPCPeeringConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCPeeringConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPCPeeringConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPCPeeringConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut peer_owner_id = None;
                let mut peer_role_arn = None;
                let mut peer_vpc_id = None;
                let mut tags = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PeerOwnerId" => {
                            peer_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PeerRoleArn" => {
                            peer_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PeerVpcId" => {
                            peer_vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPCPeeringConnectionProperties {
                    peer_owner_id: peer_owner_id,
                    peer_role_arn: peer_role_arn,
                    peer_vpc_id: peer_vpc_id.ok_or(::serde::de::Error::missing_field("PeerVpcId"))?,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPNConnectionProperties {
    /// Property `CustomerGatewayId`.
    pub customer_gateway_id: ::Value<String>,
    /// Property `StaticRoutesOnly`.
    pub static_routes_only: Option<::Value<bool>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
    /// Property `VpnGatewayId`.
    pub vpn_gateway_id: ::Value<String>,
    /// Property `VpnTunnelOptionsSpecifications`.
    pub vpn_tunnel_options_specifications: Option<::ValueList<self::vpn_connection::VpnTunnelOptionsSpecification>>,
}

impl ::serde::Serialize for VPNConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerGatewayId", &self.customer_gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StaticRoutesOnly", &self.static_routes_only)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnGatewayId", &self.vpn_gateway_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnTunnelOptionsSpecifications", &self.vpn_tunnel_options_specifications)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPNConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPNConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPNConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPNConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut customer_gateway_id = None;
                let mut static_routes_only = None;
                let mut tags = None;
                let mut type_ = None;
                let mut vpn_gateway_id = None;
                let mut vpn_tunnel_options_specifications = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomerGatewayId" => {
                            customer_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StaticRoutesOnly" => {
                            static_routes_only = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpnGatewayId" => {
                            vpn_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpnTunnelOptionsSpecifications" => {
                            vpn_tunnel_options_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPNConnectionProperties {
                    customer_gateway_id: customer_gateway_id.ok_or(::serde::de::Error::missing_field("CustomerGatewayId"))?,
                    static_routes_only: static_routes_only,
                    tags: tags,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    vpn_gateway_id: vpn_gateway_id.ok_or(::serde::de::Error::missing_field("VpnGatewayId"))?,
                    vpn_tunnel_options_specifications: vpn_tunnel_options_specifications,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPNConnectionRouteProperties {
    /// Property `DestinationCidrBlock`.
    pub destination_cidr_block: ::Value<String>,
    /// Property `VpnConnectionId`.
    pub vpn_connection_id: ::Value<String>,
}

impl ::serde::Serialize for VPNConnectionRouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationCidrBlock", &self.destination_cidr_block)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnConnectionId", &self.vpn_connection_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPNConnectionRouteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPNConnectionRouteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPNConnectionRouteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPNConnectionRouteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_cidr_block = None;
                let mut vpn_connection_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationCidrBlock" => {
                            destination_cidr_block = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpnConnectionId" => {
                            vpn_connection_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPNConnectionRouteProperties {
                    destination_cidr_block: destination_cidr_block.ok_or(::serde::de::Error::missing_field("DestinationCidrBlock"))?,
                    vpn_connection_id: vpn_connection_id.ok_or(::serde::de::Error::missing_field("VpnConnectionId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPNGatewayProperties {
    /// Property `AmazonSideAsn`.
    pub amazon_side_asn: Option<::Value<u64>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for VPNGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonSideAsn", &self.amazon_side_asn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPNGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPNGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPNGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPNGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut amazon_side_asn = None;
                let mut tags = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AmazonSideAsn" => {
                            amazon_side_asn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPNGatewayProperties {
                    amazon_side_asn: amazon_side_asn,
                    tags: tags,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VPNGatewayRoutePropagationProperties {
    /// Property `RouteTableIds`.
    pub route_table_ids: ::ValueList<String>,
    /// Property `VpnGatewayId`.
    pub vpn_gateway_id: ::Value<String>,
}

impl ::serde::Serialize for VPNGatewayRoutePropagationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteTableIds", &self.route_table_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnGatewayId", &self.vpn_gateway_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VPNGatewayRoutePropagationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VPNGatewayRoutePropagationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VPNGatewayRoutePropagationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VPNGatewayRoutePropagationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut route_table_ids = None;
                let mut vpn_gateway_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RouteTableIds" => {
                            route_table_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpnGatewayId" => {
                            vpn_gateway_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VPNGatewayRoutePropagationProperties {
                    route_table_ids: route_table_ids.ok_or(::serde::de::Error::missing_field("RouteTableIds"))?,
                    vpn_gateway_id: vpn_gateway_id.ok_or(::serde::de::Error::missing_field("VpnGatewayId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VolumeProperties {
    /// Property `AutoEnableIO`.
    pub auto_enable_io: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: ::Value<String>,
    /// Property `Encrypted`.
    pub encrypted: Option<::Value<bool>>,
    /// Property `Iops`.
    pub iops: Option<::Value<u32>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `Size`.
    pub size: Option<::Value<u32>>,
    /// Property `SnapshotId`.
    pub snapshot_id: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VolumeType`.
    pub volume_type: Option<::Value<String>>,
}

impl ::serde::Serialize for VolumeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnableIO", &self.auto_enable_io)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", &self.snapshot_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VolumeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VolumeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_enable_io = None;
                let mut availability_zone = None;
                let mut encrypted = None;
                let mut iops = None;
                let mut kms_key_id = None;
                let mut size = None;
                let mut snapshot_id = None;
                let mut tags = None;
                let mut volume_type = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoEnableIO" => {
                            auto_enable_io = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Encrypted" => {
                            encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Iops" => {
                            iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Size" => {
                            size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotId" => {
                            snapshot_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VolumeType" => {
                            volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VolumeProperties {
                    auto_enable_io: auto_enable_io,
                    availability_zone: availability_zone.ok_or(::serde::de::Error::missing_field("AvailabilityZone"))?,
                    encrypted: encrypted,
                    iops: iops,
                    kms_key_id: kms_key_id,
                    size: size,
                    snapshot_id: snapshot_id,
                    tags: tags,
                    volume_type: volume_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VolumeAttachmentProperties {
    /// Property `Device`.
    pub device: ::Value<String>,
    /// Property `InstanceId`.
    pub instance_id: ::Value<String>,
    /// Property `VolumeId`.
    pub volume_id: ::Value<String>,
}

impl ::serde::Serialize for VolumeAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Device", &self.device)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeId", &self.volume_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VolumeAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VolumeAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut device = None;
                let mut instance_id = None;
                let mut volume_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Device" => {
                            device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VolumeId" => {
                            volume_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VolumeAttachmentProperties {
                    device: device.ok_or(::serde::de::Error::missing_field("Device"))?,
                    instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    volume_id: volume_id.ok_or(::serde::de::Error::missing_field("VolumeId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
    #[derive(Debug)]
    pub struct AssociationParameter {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AssociationParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssociationParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssociationParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssociationParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssociationParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AssociationParameter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-mapping.html) property type.
    #[derive(Debug)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        pub ebs: Option<::Value<Ebs>>,
        /// Property `NoDevice`.
        pub no_device: Option<::Value<NoDevice>>,
        /// Property `VirtualName`.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", &self.ebs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", &self.no_device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", &self.virtual_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name = None;
                    let mut ebs = None;
                    let mut no_device = None;
                    let mut virtual_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ebs" => {
                                ebs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NoDevice" => {
                                no_device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VirtualName" => {
                                virtual_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDeviceMapping {
                        device_name: device_name.ok_or(::serde::de::Error::missing_field("DeviceName"))?,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.CreditSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-creditspecification.html) property type.
    #[derive(Debug)]
    pub struct CreditSpecification {
        /// Property `CPUCredits`.
        pub cpu_credits: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CreditSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CPUCredits", &self.cpu_credits)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreditSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreditSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreditSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreditSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_credits = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CPUCredits" => {
                                cpu_credits = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CreditSpecification {
                        cpu_credits: cpu_credits,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-template.html) property type.
    #[derive(Debug)]
    pub struct Ebs {
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Ebs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", &self.snapshot_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", &self.volume_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ebs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ebs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ebs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ebs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination = None;
                    let mut encrypted = None;
                    let mut iops = None;
                    let mut snapshot_id = None;
                    let mut volume_size = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Encrypted" => {
                                encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SnapshotId" => {
                                snapshot_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeSize" => {
                                volume_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Ebs {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        snapshot_id: snapshot_id,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.ElasticGpuSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-elasticgpuspecification.html) property type.
    #[derive(Debug)]
    pub struct ElasticGpuSpecification {
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticGpuSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticGpuSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticGpuSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticGpuSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticGpuSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticGpuSpecification {
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-instanceipv6address.html) property type.
    #[derive(Debug)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        pub ipv6_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for InstanceIpv6Address {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Address", &self.ipv6_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceIpv6Address {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceIpv6Address, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceIpv6Address;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceIpv6Address")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ipv6_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ipv6Address" => {
                                ipv6_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceIpv6Address {
                        ipv6_address: ipv6_address.ok_or(::serde::de::Error::missing_field("Ipv6Address"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-iface-embedded.html) property type.
    #[derive(Debug)]
    pub struct NetworkInterface {
        /// Property `AssociatePublicIpAddress`.
        pub associate_public_ip_address: Option<::Value<bool>>,
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `DeviceIndex`.
        pub device_index: ::Value<String>,
        /// Property `GroupSet`.
        pub group_set: Option<::ValueList<String>>,
        /// Property `Ipv6AddressCount`.
        pub ipv6_address_count: Option<::Value<u32>>,
        /// Property `Ipv6Addresses`.
        pub ipv6_addresses: Option<::ValueList<InstanceIpv6Address>>,
        /// Property `NetworkInterfaceId`.
        pub network_interface_id: Option<::Value<String>>,
        /// Property `PrivateIpAddress`.
        pub private_ip_address: Option<::Value<String>>,
        /// Property `PrivateIpAddresses`.
        pub private_ip_addresses: Option<::ValueList<PrivateIpAddressSpecification>>,
        /// Property `SecondaryPrivateIpAddressCount`.
        pub secondary_private_ip_address_count: Option<::Value<u32>>,
        /// Property `SubnetId`.
        pub subnet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NetworkInterface {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatePublicIpAddress", &self.associate_public_ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceIndex", &self.device_index)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupSet", &self.group_set)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6AddressCount", &self.ipv6_address_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Addresses", &self.ipv6_addresses)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddresses", &self.private_ip_addresses)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryPrivateIpAddressCount", &self.secondary_private_ip_address_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkInterface {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterface, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkInterface;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkInterface")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut associate_public_ip_address = None;
                    let mut delete_on_termination = None;
                    let mut description = None;
                    let mut device_index = None;
                    let mut group_set = None;
                    let mut ipv6_address_count = None;
                    let mut ipv6_addresses = None;
                    let mut network_interface_id = None;
                    let mut private_ip_address = None;
                    let mut private_ip_addresses = None;
                    let mut secondary_private_ip_address_count = None;
                    let mut subnet_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssociatePublicIpAddress" => {
                                associate_public_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeviceIndex" => {
                                device_index = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "GroupSet" => {
                                group_set = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ipv6AddressCount" => {
                                ipv6_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ipv6Addresses" => {
                                ipv6_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NetworkInterfaceId" => {
                                network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddresses" => {
                                private_ip_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SecondaryPrivateIpAddressCount" => {
                                secondary_private_ip_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubnetId" => {
                                subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkInterface {
                        associate_public_ip_address: associate_public_ip_address,
                        delete_on_termination: delete_on_termination,
                        description: description,
                        device_index: device_index.ok_or(::serde::de::Error::missing_field("DeviceIndex"))?,
                        group_set: group_set,
                        ipv6_address_count: ipv6_address_count,
                        ipv6_addresses: ipv6_addresses,
                        network_interface_id: network_interface_id,
                        private_ip_address: private_ip_address,
                        private_ip_addresses: private_ip_addresses,
                        secondary_private_ip_address_count: secondary_private_ip_address_count,
                        subnet_id: subnet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-nodevice.html) property type.
    #[derive(Debug)]
    pub struct NoDevice {
    }

    impl ::codec::SerializeValue for NoDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NoDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NoDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NoDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NoDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(NoDevice {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Debug)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        pub primary: ::Value<bool>,
        /// Property `PrivateIpAddress`.
        pub private_ip_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for PrivateIpAddressSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Primary", &self.primary)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateIpAddressSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateIpAddressSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateIpAddressSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateIpAddressSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary = None;
                    let mut private_ip_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Primary" => {
                                primary = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateIpAddressSpecification {
                        primary: primary.ok_or(::serde::de::Error::missing_field("Primary"))?,
                        private_ip_address: private_ip_address.ok_or(::serde::de::Error::missing_field("PrivateIpAddress"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.SsmAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociations.html) property type.
    #[derive(Debug)]
    pub struct SsmAssociation {
        /// Property `AssociationParameters`.
        pub association_parameters: Option<::ValueList<AssociationParameter>>,
        /// Property `DocumentName`.
        pub document_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SsmAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationParameters", &self.association_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentName", &self.document_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SsmAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SsmAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SsmAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SsmAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut association_parameters = None;
                    let mut document_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssociationParameters" => {
                                association_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DocumentName" => {
                                document_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SsmAssociation {
                        association_parameters: association_parameters,
                        document_name: document_name.ok_or(::serde::de::Error::missing_field("DocumentName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::Instance.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-mount-point.html) property type.
    #[derive(Debug)]
    pub struct Volume {
        /// Property `Device`.
        pub device: ::Value<String>,
        /// Property `VolumeId`.
        pub volume_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for Volume {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Device", &self.device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeId", &self.volume_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Volume {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Volume, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Volume;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Volume")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device = None;
                    let mut volume_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Device" => {
                                device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeId" => {
                                volume_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Volume {
                        device: device.ok_or(::serde::de::Error::missing_field("Device"))?,
                        volume_id: volume_id.ok_or(::serde::de::Error::missing_field("VolumeId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod network_acl_entry {
    //! Property types for the `NetworkAclEntry` resource.

    /// The [`AWS::EC2::NetworkAclEntry.Icmp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-icmp.html) property type.
    #[derive(Debug)]
    pub struct Icmp {
        /// Property `Code`.
        pub code: Option<::Value<u32>>,
        /// Property `Type`.
        pub type_: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Icmp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Icmp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Icmp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Icmp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Icmp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Code" => {
                                code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Icmp {
                        code: code,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::NetworkAclEntry.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-portrange.html) property type.
    #[derive(Debug)]
    pub struct PortRange {
        /// Property `From`.
        pub from: Option<::Value<u32>>,
        /// Property `To`.
        pub to: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for PortRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "From", &self.from)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "To", &self.to)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from = None;
                    let mut to = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "From" => {
                                from = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "To" => {
                                to = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PortRange {
                        from: from,
                        to: to,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod network_interface {
    //! Property types for the `NetworkInterface` resource.

    /// The [`AWS::EC2::NetworkInterface.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-instanceipv6address.html) property type.
    #[derive(Debug)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        pub ipv6_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for InstanceIpv6Address {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Address", &self.ipv6_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceIpv6Address {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceIpv6Address, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceIpv6Address;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceIpv6Address")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ipv6_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ipv6Address" => {
                                ipv6_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceIpv6Address {
                        ipv6_address: ipv6_address.ok_or(::serde::de::Error::missing_field("Ipv6Address"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::NetworkInterface.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Debug)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        pub primary: ::Value<bool>,
        /// Property `PrivateIpAddress`.
        pub private_ip_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for PrivateIpAddressSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Primary", &self.primary)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateIpAddressSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateIpAddressSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateIpAddressSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateIpAddressSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary = None;
                    let mut private_ip_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Primary" => {
                                primary = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateIpAddressSpecification {
                        primary: primary.ok_or(::serde::de::Error::missing_field("Primary"))?,
                        private_ip_address: private_ip_address.ok_or(::serde::de::Error::missing_field("PrivateIpAddress"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod security_group {
    //! Property types for the `SecurityGroup` resource.

    /// The [`AWS::EC2::SecurityGroup.Egress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Debug)]
    pub struct Egress {
        /// Property `CidrIp`.
        pub cidr_ip: Option<::Value<String>>,
        /// Property `CidrIpv6`.
        pub cidr_ipv6: Option<::Value<String>>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `DestinationPrefixListId`.
        pub destination_prefix_list_id: Option<::Value<String>>,
        /// Property `DestinationSecurityGroupId`.
        pub destination_security_group_id: Option<::Value<String>>,
        /// Property `FromPort`.
        pub from_port: Option<::Value<u32>>,
        /// Property `IpProtocol`.
        pub ip_protocol: ::Value<String>,
        /// Property `ToPort`.
        pub to_port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Egress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIp", &self.cidr_ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIpv6", &self.cidr_ipv6)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPrefixListId", &self.destination_prefix_list_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationSecurityGroupId", &self.destination_security_group_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpProtocol", &self.ip_protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Egress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Egress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Egress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Egress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr_ip = None;
                    let mut cidr_ipv6 = None;
                    let mut description = None;
                    let mut destination_prefix_list_id = None;
                    let mut destination_security_group_id = None;
                    let mut from_port = None;
                    let mut ip_protocol = None;
                    let mut to_port = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CidrIp" => {
                                cidr_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CidrIpv6" => {
                                cidr_ipv6 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DestinationPrefixListId" => {
                                destination_prefix_list_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DestinationSecurityGroupId" => {
                                destination_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FromPort" => {
                                from_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IpProtocol" => {
                                ip_protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ToPort" => {
                                to_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Egress {
                        cidr_ip: cidr_ip,
                        cidr_ipv6: cidr_ipv6,
                        description: description,
                        destination_prefix_list_id: destination_prefix_list_id,
                        destination_security_group_id: destination_security_group_id,
                        from_port: from_port,
                        ip_protocol: ip_protocol.ok_or(::serde::de::Error::missing_field("IpProtocol"))?,
                        to_port: to_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Debug)]
    pub struct Ingress {
        /// Property `CidrIp`.
        pub cidr_ip: Option<::Value<String>>,
        /// Property `CidrIpv6`.
        pub cidr_ipv6: Option<::Value<String>>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `FromPort`.
        pub from_port: Option<::Value<u32>>,
        /// Property `IpProtocol`.
        pub ip_protocol: ::Value<String>,
        /// Property `SourceSecurityGroupId`.
        pub source_security_group_id: Option<::Value<String>>,
        /// Property `SourceSecurityGroupName`.
        pub source_security_group_name: Option<::Value<String>>,
        /// Property `SourceSecurityGroupOwnerId`.
        pub source_security_group_owner_id: Option<::Value<String>>,
        /// Property `ToPort`.
        pub to_port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Ingress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIp", &self.cidr_ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrIpv6", &self.cidr_ipv6)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpProtocol", &self.ip_protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupId", &self.source_security_group_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupName", &self.source_security_group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSecurityGroupOwnerId", &self.source_security_group_owner_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ingress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ingress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ingress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ingress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr_ip = None;
                    let mut cidr_ipv6 = None;
                    let mut description = None;
                    let mut from_port = None;
                    let mut ip_protocol = None;
                    let mut source_security_group_id = None;
                    let mut source_security_group_name = None;
                    let mut source_security_group_owner_id = None;
                    let mut to_port = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CidrIp" => {
                                cidr_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CidrIpv6" => {
                                cidr_ipv6 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FromPort" => {
                                from_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IpProtocol" => {
                                ip_protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceSecurityGroupId" => {
                                source_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceSecurityGroupName" => {
                                source_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceSecurityGroupOwnerId" => {
                                source_security_group_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ToPort" => {
                                to_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Ingress {
                        cidr_ip: cidr_ip,
                        cidr_ipv6: cidr_ipv6,
                        description: description,
                        from_port: from_port,
                        ip_protocol: ip_protocol.ok_or(::serde::de::Error::missing_field("IpProtocol"))?,
                        source_security_group_id: source_security_group_id,
                        source_security_group_name: source_security_group_name,
                        source_security_group_owner_id: source_security_group_owner_id,
                        to_port: to_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod spot_fleet {
    //! Property types for the `SpotFleet` resource.

    /// The [`AWS::EC2::SpotFleet.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings.html) property type.
    #[derive(Debug)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        pub ebs: Option<::Value<EbsBlockDevice>>,
        /// Property `NoDevice`.
        pub no_device: Option<::Value<String>>,
        /// Property `VirtualName`.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", &self.ebs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", &self.no_device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", &self.virtual_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name = None;
                    let mut ebs = None;
                    let mut no_device = None;
                    let mut virtual_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ebs" => {
                                ebs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NoDevice" => {
                                no_device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VirtualName" => {
                                virtual_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDeviceMapping {
                        device_name: device_name.ok_or(::serde::de::Error::missing_field("DeviceName"))?,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings-ebs.html) property type.
    #[derive(Debug)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbsBlockDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", &self.snapshot_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", &self.volume_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsBlockDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsBlockDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsBlockDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsBlockDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination = None;
                    let mut encrypted = None;
                    let mut iops = None;
                    let mut snapshot_id = None;
                    let mut volume_size = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Encrypted" => {
                                encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SnapshotId" => {
                                snapshot_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeSize" => {
                                volume_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsBlockDevice {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        snapshot_id: snapshot_id,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.GroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-securitygroups.html) property type.
    #[derive(Debug)]
    pub struct GroupIdentifier {
        /// Property `GroupId`.
        pub group_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for GroupIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GroupIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GroupIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GroupIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupId" => {
                                group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(GroupIdentifier {
                        group_id: group_id.ok_or(::serde::de::Error::missing_field("GroupId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.IamInstanceProfileSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-iaminstanceprofile.html) property type.
    #[derive(Debug)]
    pub struct IamInstanceProfileSpecification {
        /// Property `Arn`.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IamInstanceProfileSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IamInstanceProfileSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IamInstanceProfileSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IamInstanceProfileSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IamInstanceProfileSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(IamInstanceProfileSpecification {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instanceipv6address.html) property type.
    #[derive(Debug)]
    pub struct InstanceIpv6Address {
        /// Property `Ipv6Address`.
        pub ipv6_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for InstanceIpv6Address {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Address", &self.ipv6_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceIpv6Address {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceIpv6Address, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceIpv6Address;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceIpv6Address")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ipv6_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ipv6Address" => {
                                ipv6_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceIpv6Address {
                        ipv6_address: ipv6_address.ok_or(::serde::de::Error::missing_field("Ipv6Address"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.InstanceNetworkInterfaceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces.html) property type.
    #[derive(Debug)]
    pub struct InstanceNetworkInterfaceSpecification {
        /// Property `AssociatePublicIpAddress`.
        pub associate_public_ip_address: Option<::Value<bool>>,
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `DeviceIndex`.
        pub device_index: Option<::Value<u32>>,
        /// Property `Groups`.
        pub groups: Option<::ValueList<String>>,
        /// Property `Ipv6AddressCount`.
        pub ipv6_address_count: Option<::Value<u32>>,
        /// Property `Ipv6Addresses`.
        pub ipv6_addresses: Option<::ValueList<InstanceIpv6Address>>,
        /// Property `NetworkInterfaceId`.
        pub network_interface_id: Option<::Value<String>>,
        /// Property `PrivateIpAddresses`.
        pub private_ip_addresses: Option<::ValueList<PrivateIpAddressSpecification>>,
        /// Property `SecondaryPrivateIpAddressCount`.
        pub secondary_private_ip_address_count: Option<::Value<u32>>,
        /// Property `SubnetId`.
        pub subnet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceNetworkInterfaceSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatePublicIpAddress", &self.associate_public_ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceIndex", &self.device_index)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", &self.groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6AddressCount", &self.ipv6_address_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Addresses", &self.ipv6_addresses)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", &self.network_interface_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddresses", &self.private_ip_addresses)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryPrivateIpAddressCount", &self.secondary_private_ip_address_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceNetworkInterfaceSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceNetworkInterfaceSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceNetworkInterfaceSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceNetworkInterfaceSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut associate_public_ip_address = None;
                    let mut delete_on_termination = None;
                    let mut description = None;
                    let mut device_index = None;
                    let mut groups = None;
                    let mut ipv6_address_count = None;
                    let mut ipv6_addresses = None;
                    let mut network_interface_id = None;
                    let mut private_ip_addresses = None;
                    let mut secondary_private_ip_address_count = None;
                    let mut subnet_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssociatePublicIpAddress" => {
                                associate_public_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeviceIndex" => {
                                device_index = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Groups" => {
                                groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ipv6AddressCount" => {
                                ipv6_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ipv6Addresses" => {
                                ipv6_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NetworkInterfaceId" => {
                                network_interface_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddresses" => {
                                private_ip_addresses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SecondaryPrivateIpAddressCount" => {
                                secondary_private_ip_address_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubnetId" => {
                                subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceNetworkInterfaceSpecification {
                        associate_public_ip_address: associate_public_ip_address,
                        delete_on_termination: delete_on_termination,
                        description: description,
                        device_index: device_index,
                        groups: groups,
                        ipv6_address_count: ipv6_address_count,
                        ipv6_addresses: ipv6_addresses,
                        network_interface_id: network_interface_id,
                        private_ip_addresses: private_ip_addresses,
                        secondary_private_ip_address_count: secondary_private_ip_address_count,
                        subnet_id: subnet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces-privateipaddresses.html) property type.
    #[derive(Debug)]
    pub struct PrivateIpAddressSpecification {
        /// Property `Primary`.
        pub primary: Option<::Value<bool>>,
        /// Property `PrivateIpAddress`.
        pub private_ip_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for PrivateIpAddressSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Primary", &self.primary)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", &self.private_ip_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateIpAddressSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateIpAddressSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateIpAddressSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateIpAddressSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary = None;
                    let mut private_ip_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Primary" => {
                                primary = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateIpAddressSpecification {
                        primary: primary,
                        private_ip_address: private_ip_address.ok_or(::serde::de::Error::missing_field("PrivateIpAddress"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetLaunchSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications.html) property type.
    #[derive(Debug)]
    pub struct SpotFleetLaunchSpecification {
        /// Property `BlockDeviceMappings`.
        pub block_device_mappings: Option<::ValueList<BlockDeviceMapping>>,
        /// Property `EbsOptimized`.
        pub ebs_optimized: Option<::Value<bool>>,
        /// Property `IamInstanceProfile`.
        pub iam_instance_profile: Option<::Value<IamInstanceProfileSpecification>>,
        /// Property `ImageId`.
        pub image_id: ::Value<String>,
        /// Property `InstanceType`.
        pub instance_type: ::Value<String>,
        /// Property `KernelId`.
        pub kernel_id: Option<::Value<String>>,
        /// Property `KeyName`.
        pub key_name: Option<::Value<String>>,
        /// Property `Monitoring`.
        pub monitoring: Option<::Value<SpotFleetMonitoring>>,
        /// Property `NetworkInterfaces`.
        pub network_interfaces: Option<::ValueList<InstanceNetworkInterfaceSpecification>>,
        /// Property `Placement`.
        pub placement: Option<::Value<SpotPlacement>>,
        /// Property `RamdiskId`.
        pub ramdisk_id: Option<::Value<String>>,
        /// Property `SecurityGroups`.
        pub security_groups: Option<::ValueList<GroupIdentifier>>,
        /// Property `SpotPrice`.
        pub spot_price: Option<::Value<String>>,
        /// Property `SubnetId`.
        pub subnet_id: Option<::Value<String>>,
        /// Property `TagSpecifications`.
        pub tag_specifications: Option<::ValueList<SpotFleetTagSpecification>>,
        /// Property `UserData`.
        pub user_data: Option<::Value<String>>,
        /// Property `WeightedCapacity`.
        pub weighted_capacity: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for SpotFleetLaunchSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", &self.block_device_mappings)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamInstanceProfile", &self.iam_instance_profile)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", &self.image_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelId", &self.kernel_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyName", &self.key_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Monitoring", &self.monitoring)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaces", &self.network_interfaces)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Placement", &self.placement)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RamdiskId", &self.ramdisk_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotPrice", &self.spot_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagSpecifications", &self.tag_specifications)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserData", &self.user_data)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", &self.weighted_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotFleetLaunchSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotFleetLaunchSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotFleetLaunchSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotFleetLaunchSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_device_mappings = None;
                    let mut ebs_optimized = None;
                    let mut iam_instance_profile = None;
                    let mut image_id = None;
                    let mut instance_type = None;
                    let mut kernel_id = None;
                    let mut key_name = None;
                    let mut monitoring = None;
                    let mut network_interfaces = None;
                    let mut placement = None;
                    let mut ramdisk_id = None;
                    let mut security_groups = None;
                    let mut spot_price = None;
                    let mut subnet_id = None;
                    let mut tag_specifications = None;
                    let mut user_data = None;
                    let mut weighted_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDeviceMappings" => {
                                block_device_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsOptimized" => {
                                ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IamInstanceProfile" => {
                                iam_instance_profile = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ImageId" => {
                                image_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceType" => {
                                instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KernelId" => {
                                kernel_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KeyName" => {
                                key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Monitoring" => {
                                monitoring = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NetworkInterfaces" => {
                                network_interfaces = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Placement" => {
                                placement = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RamdiskId" => {
                                ramdisk_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SecurityGroups" => {
                                security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SpotPrice" => {
                                spot_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubnetId" => {
                                subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TagSpecifications" => {
                                tag_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "UserData" => {
                                user_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotFleetLaunchSpecification {
                        block_device_mappings: block_device_mappings,
                        ebs_optimized: ebs_optimized,
                        iam_instance_profile: iam_instance_profile,
                        image_id: image_id.ok_or(::serde::de::Error::missing_field("ImageId"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        kernel_id: kernel_id,
                        key_name: key_name,
                        monitoring: monitoring,
                        network_interfaces: network_interfaces,
                        placement: placement,
                        ramdisk_id: ramdisk_id,
                        security_groups: security_groups,
                        spot_price: spot_price,
                        subnet_id: subnet_id,
                        tag_specifications: tag_specifications,
                        user_data: user_data,
                        weighted_capacity: weighted_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-monitoring.html) property type.
    #[derive(Debug)]
    pub struct SpotFleetMonitoring {
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SpotFleetMonitoring {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotFleetMonitoring {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotFleetMonitoring, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotFleetMonitoring;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotFleetMonitoring")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotFleetMonitoring {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetRequestConfigData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata.html) property type.
    #[derive(Debug)]
    pub struct SpotFleetRequestConfigData {
        /// Property `AllocationStrategy`.
        pub allocation_strategy: Option<::Value<String>>,
        /// Property `ExcessCapacityTerminationPolicy`.
        pub excess_capacity_termination_policy: Option<::Value<String>>,
        /// Property `IamFleetRole`.
        pub iam_fleet_role: ::Value<String>,
        /// Property `LaunchSpecifications`.
        pub launch_specifications: ::ValueList<SpotFleetLaunchSpecification>,
        /// Property `ReplaceUnhealthyInstances`.
        pub replace_unhealthy_instances: Option<::Value<bool>>,
        /// Property `SpotPrice`.
        pub spot_price: Option<::Value<String>>,
        /// Property `TargetCapacity`.
        pub target_capacity: ::Value<u32>,
        /// Property `TerminateInstancesWithExpiration`.
        pub terminate_instances_with_expiration: Option<::Value<bool>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `ValidFrom`.
        pub valid_from: Option<::Value<String>>,
        /// Property `ValidUntil`.
        pub valid_until: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SpotFleetRequestConfigData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationStrategy", &self.allocation_strategy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcessCapacityTerminationPolicy", &self.excess_capacity_termination_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamFleetRole", &self.iam_fleet_role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchSpecifications", &self.launch_specifications)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceUnhealthyInstances", &self.replace_unhealthy_instances)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotPrice", &self.spot_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetCapacity", &self.target_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminateInstancesWithExpiration", &self.terminate_instances_with_expiration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidFrom", &self.valid_from)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidUntil", &self.valid_until)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotFleetRequestConfigData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotFleetRequestConfigData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotFleetRequestConfigData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotFleetRequestConfigData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allocation_strategy = None;
                    let mut excess_capacity_termination_policy = None;
                    let mut iam_fleet_role = None;
                    let mut launch_specifications = None;
                    let mut replace_unhealthy_instances = None;
                    let mut spot_price = None;
                    let mut target_capacity = None;
                    let mut terminate_instances_with_expiration = None;
                    let mut type_ = None;
                    let mut valid_from = None;
                    let mut valid_until = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllocationStrategy" => {
                                allocation_strategy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExcessCapacityTerminationPolicy" => {
                                excess_capacity_termination_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IamFleetRole" => {
                                iam_fleet_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LaunchSpecifications" => {
                                launch_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReplaceUnhealthyInstances" => {
                                replace_unhealthy_instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SpotPrice" => {
                                spot_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetCapacity" => {
                                target_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TerminateInstancesWithExpiration" => {
                                terminate_instances_with_expiration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ValidFrom" => {
                                valid_from = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ValidUntil" => {
                                valid_until = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotFleetRequestConfigData {
                        allocation_strategy: allocation_strategy,
                        excess_capacity_termination_policy: excess_capacity_termination_policy,
                        iam_fleet_role: iam_fleet_role.ok_or(::serde::de::Error::missing_field("IamFleetRole"))?,
                        launch_specifications: launch_specifications.ok_or(::serde::de::Error::missing_field("LaunchSpecifications"))?,
                        replace_unhealthy_instances: replace_unhealthy_instances,
                        spot_price: spot_price,
                        target_capacity: target_capacity.ok_or(::serde::de::Error::missing_field("TargetCapacity"))?,
                        terminate_instances_with_expiration: terminate_instances_with_expiration,
                        type_: type_,
                        valid_from: valid_from,
                        valid_until: valid_until,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetTagSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-tagspecifications.html) property type.
    #[derive(Debug)]
    pub struct SpotFleetTagSpecification {
        /// Property `ResourceType`.
        pub resource_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SpotFleetTagSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotFleetTagSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotFleetTagSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotFleetTagSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotFleetTagSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceType" => {
                                resource_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotFleetTagSpecification {
                        resource_type: resource_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EC2::SpotFleet.SpotPlacement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-placement.html) property type.
    #[derive(Debug)]
    pub struct SpotPlacement {
        /// Property `AvailabilityZone`.
        pub availability_zone: Option<::Value<String>>,
        /// Property `GroupName`.
        pub group_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SpotPlacement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotPlacement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotPlacement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotPlacement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotPlacement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone = None;
                    let mut group_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "GroupName" => {
                                group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotPlacement {
                        availability_zone: availability_zone,
                        group_name: group_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod vpn_connection {
    //! Property types for the `VPNConnection` resource.

    /// The [`AWS::EC2::VPNConnection.VpnTunnelOptionsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-vpntunneloptionsspecification.html) property type.
    #[derive(Debug)]
    pub struct VpnTunnelOptionsSpecification {
        /// Property `PreSharedKey`.
        pub pre_shared_key: Option<::Value<String>>,
        /// Property `TunnelInsideCidr`.
        pub tunnel_inside_cidr: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpnTunnelOptionsSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreSharedKey", &self.pre_shared_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TunnelInsideCidr", &self.tunnel_inside_cidr)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpnTunnelOptionsSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpnTunnelOptionsSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpnTunnelOptionsSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpnTunnelOptionsSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pre_shared_key = None;
                    let mut tunnel_inside_cidr = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PreSharedKey" => {
                                pre_shared_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TunnelInsideCidr" => {
                                tunnel_inside_cidr = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VpnTunnelOptionsSpecification {
                        pre_shared_key: pre_shared_key,
                        tunnel_inside_cidr: tunnel_inside_cidr,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
