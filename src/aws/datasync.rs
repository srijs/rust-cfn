//! Types for the `DataSync` service.

/// The [`AWS::DataSync::Agent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html) resource type.
#[derive(Debug, Default)]
pub struct Agent {
    properties: AgentProperties
}

/// Properties for the `Agent` resource.
#[derive(Debug, Default)]
pub struct AgentProperties {
    /// Property [`ActivationKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-activationkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub activation_key: Option<::Value<String>>,
    /// Property [`AgentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-agentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_name: Option<::Value<String>>,
    /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-securitygrouparns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_arns: Option<::ValueList<String>>,
    /// Property [`SubnetArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-subnetarns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_arns: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html#cfn-datasync-agent-vpcendpointid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_endpoint_id: Option<::Value<String>>,
}

impl ::serde::Serialize for AgentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref activation_key) = self.activation_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActivationKey", activation_key)?;
        }
        if let Some(ref agent_name) = self.agent_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentName", agent_name)?;
        }
        if let Some(ref security_group_arns) = self.security_group_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", security_group_arns)?;
        }
        if let Some(ref subnet_arns) = self.subnet_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetArns", subnet_arns)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AgentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AgentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AgentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AgentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut activation_key: Option<::Value<String>> = None;
                let mut agent_name: Option<::Value<String>> = None;
                let mut security_group_arns: Option<::ValueList<String>> = None;
                let mut subnet_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_endpoint_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActivationKey" => {
                            activation_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AgentName" => {
                            agent_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupArns" => {
                            security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetArns" => {
                            subnet_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcEndpointId" => {
                            vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AgentProperties {
                    activation_key: activation_key,
                    agent_name: agent_name,
                    security_group_arns: security_group_arns,
                    subnet_arns: subnet_arns,
                    tags: tags,
                    vpc_endpoint_id: vpc_endpoint_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Agent {
    type Properties = AgentProperties;
    const TYPE: &'static str = "AWS::DataSync::Agent";
    fn properties(&self) -> &AgentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AgentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Agent {}

impl From<AgentProperties> for Agent {
    fn from(properties: AgentProperties) -> Agent {
        Agent { properties }
    }
}

/// The [`AWS::DataSync::LocationAzureBlob`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html) resource type.
#[derive(Debug, Default)]
pub struct LocationAzureBlob {
    properties: LocationAzureBlobProperties
}

/// Properties for the `LocationAzureBlob` resource.
#[derive(Debug, Default)]
pub struct LocationAzureBlobProperties {
    /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-agentarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_arns: ::ValueList<String>,
    /// Property [`AzureAccessTier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-azureaccesstier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub azure_access_tier: Option<::Value<String>>,
    /// Property [`AzureBlobAuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-azureblobauthenticationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub azure_blob_authentication_type: ::Value<String>,
    /// Property [`AzureBlobContainerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-azureblobcontainerurl).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub azure_blob_container_url: Option<::Value<String>>,
    /// Property [`AzureBlobSasConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-azureblobsasconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub azure_blob_sas_configuration: Option<::Value<self::location_azure_blob::AzureBlobSasConfiguration>>,
    /// Property [`AzureBlobType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-azureblobtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub azure_blob_type: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-subdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html#cfn-datasync-locationazureblob-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationAzureBlobProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
        if let Some(ref azure_access_tier) = self.azure_access_tier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureAccessTier", azure_access_tier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureBlobAuthenticationType", &self.azure_blob_authentication_type)?;
        if let Some(ref azure_blob_container_url) = self.azure_blob_container_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureBlobContainerUrl", azure_blob_container_url)?;
        }
        if let Some(ref azure_blob_sas_configuration) = self.azure_blob_sas_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureBlobSasConfiguration", azure_blob_sas_configuration)?;
        }
        if let Some(ref azure_blob_type) = self.azure_blob_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureBlobType", azure_blob_type)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationAzureBlobProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationAzureBlobProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationAzureBlobProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationAzureBlobProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_arns: Option<::ValueList<String>> = None;
                let mut azure_access_tier: Option<::Value<String>> = None;
                let mut azure_blob_authentication_type: Option<::Value<String>> = None;
                let mut azure_blob_container_url: Option<::Value<String>> = None;
                let mut azure_blob_sas_configuration: Option<::Value<self::location_azure_blob::AzureBlobSasConfiguration>> = None;
                let mut azure_blob_type: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentArns" => {
                            agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AzureAccessTier" => {
                            azure_access_tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AzureBlobAuthenticationType" => {
                            azure_blob_authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AzureBlobContainerUrl" => {
                            azure_blob_container_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AzureBlobSasConfiguration" => {
                            azure_blob_sas_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AzureBlobType" => {
                            azure_blob_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationAzureBlobProperties {
                    agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    azure_access_tier: azure_access_tier,
                    azure_blob_authentication_type: azure_blob_authentication_type.ok_or(::serde::de::Error::missing_field("AzureBlobAuthenticationType"))?,
                    azure_blob_container_url: azure_blob_container_url,
                    azure_blob_sas_configuration: azure_blob_sas_configuration,
                    azure_blob_type: azure_blob_type,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationAzureBlob {
    type Properties = LocationAzureBlobProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationAzureBlob";
    fn properties(&self) -> &LocationAzureBlobProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationAzureBlobProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationAzureBlob {}

impl From<LocationAzureBlobProperties> for LocationAzureBlob {
    fn from(properties: LocationAzureBlobProperties) -> LocationAzureBlob {
        LocationAzureBlob { properties }
    }
}

/// The [`AWS::DataSync::LocationEFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html) resource type.
#[derive(Debug, Default)]
pub struct LocationEFS {
    properties: LocationEFSProperties
}

/// Properties for the `LocationEFS` resource.
#[derive(Debug, Default)]
pub struct LocationEFSProperties {
    /// Property [`AccessPointArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-accesspointarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub access_point_arn: Option<::Value<String>>,
    /// Property [`Ec2Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-ec2config).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_config: ::Value<self::location_efs::Ec2Config>,
    /// Property [`EfsFilesystemArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-efsfilesystemarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub efs_filesystem_arn: Option<::Value<String>>,
    /// Property [`FileSystemAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-filesystemaccessrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_access_role_arn: Option<::Value<String>>,
    /// Property [`InTransitEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-intransitencryption).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub in_transit_encryption: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html#cfn-datasync-locationefs-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationEFSProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_point_arn) = self.access_point_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPointArn", access_point_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2Config", &self.ec2_config)?;
        if let Some(ref efs_filesystem_arn) = self.efs_filesystem_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EfsFilesystemArn", efs_filesystem_arn)?;
        }
        if let Some(ref file_system_access_role_arn) = self.file_system_access_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemAccessRoleArn", file_system_access_role_arn)?;
        }
        if let Some(ref in_transit_encryption) = self.in_transit_encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InTransitEncryption", in_transit_encryption)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationEFSProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationEFSProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationEFSProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationEFSProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_point_arn: Option<::Value<String>> = None;
                let mut ec2_config: Option<::Value<self::location_efs::Ec2Config>> = None;
                let mut efs_filesystem_arn: Option<::Value<String>> = None;
                let mut file_system_access_role_arn: Option<::Value<String>> = None;
                let mut in_transit_encryption: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessPointArn" => {
                            access_point_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ec2Config" => {
                            ec2_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EfsFilesystemArn" => {
                            efs_filesystem_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemAccessRoleArn" => {
                            file_system_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InTransitEncryption" => {
                            in_transit_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationEFSProperties {
                    access_point_arn: access_point_arn,
                    ec2_config: ec2_config.ok_or(::serde::de::Error::missing_field("Ec2Config"))?,
                    efs_filesystem_arn: efs_filesystem_arn,
                    file_system_access_role_arn: file_system_access_role_arn,
                    in_transit_encryption: in_transit_encryption,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationEFS {
    type Properties = LocationEFSProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationEFS";
    fn properties(&self) -> &LocationEFSProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationEFSProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationEFS {}

impl From<LocationEFSProperties> for LocationEFS {
    fn from(properties: LocationEFSProperties) -> LocationEFS {
        LocationEFS { properties }
    }
}

/// The [`AWS::DataSync::LocationFSxLustre`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html) resource type.
#[derive(Debug, Default)]
pub struct LocationFSxLustre {
    properties: LocationFSxLustreProperties
}

/// Properties for the `LocationFSxLustre` resource.
#[derive(Debug, Default)]
pub struct LocationFSxLustreProperties {
    /// Property [`FsxFilesystemArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html#cfn-datasync-locationfsxlustre-fsxfilesystemarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fsx_filesystem_arn: Option<::Value<String>>,
    /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html#cfn-datasync-locationfsxlustre-securitygrouparns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_arns: ::ValueList<String>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html#cfn-datasync-locationfsxlustre-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html#cfn-datasync-locationfsxlustre-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationFSxLustreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref fsx_filesystem_arn) = self.fsx_filesystem_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FsxFilesystemArn", fsx_filesystem_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", &self.security_group_arns)?;
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationFSxLustreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationFSxLustreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationFSxLustreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationFSxLustreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fsx_filesystem_arn: Option<::Value<String>> = None;
                let mut security_group_arns: Option<::ValueList<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FsxFilesystemArn" => {
                            fsx_filesystem_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupArns" => {
                            security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationFSxLustreProperties {
                    fsx_filesystem_arn: fsx_filesystem_arn,
                    security_group_arns: security_group_arns.ok_or(::serde::de::Error::missing_field("SecurityGroupArns"))?,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationFSxLustre {
    type Properties = LocationFSxLustreProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationFSxLustre";
    fn properties(&self) -> &LocationFSxLustreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationFSxLustreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationFSxLustre {}

impl From<LocationFSxLustreProperties> for LocationFSxLustre {
    fn from(properties: LocationFSxLustreProperties) -> LocationFSxLustre {
        LocationFSxLustre { properties }
    }
}

/// The [`AWS::DataSync::LocationFSxONTAP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html) resource type.
#[derive(Debug, Default)]
pub struct LocationFSxONTAP {
    properties: LocationFSxONTAPProperties
}

/// Properties for the `LocationFSxONTAP` resource.
#[derive(Debug, Default)]
pub struct LocationFSxONTAPProperties {
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html#cfn-datasync-locationfsxontap-protocol).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol: Option<::Value<self::location_f_sx_ontap::Protocol>>,
    /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html#cfn-datasync-locationfsxontap-securitygrouparns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_arns: ::ValueList<String>,
    /// Property [`StorageVirtualMachineArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html#cfn-datasync-locationfsxontap-storagevirtualmachinearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_virtual_machine_arn: ::Value<String>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html#cfn-datasync-locationfsxontap-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html#cfn-datasync-locationfsxontap-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationFSxONTAPProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", &self.security_group_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageVirtualMachineArn", &self.storage_virtual_machine_arn)?;
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationFSxONTAPProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationFSxONTAPProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationFSxONTAPProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationFSxONTAPProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut protocol: Option<::Value<self::location_f_sx_ontap::Protocol>> = None;
                let mut security_group_arns: Option<::ValueList<String>> = None;
                let mut storage_virtual_machine_arn: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupArns" => {
                            security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageVirtualMachineArn" => {
                            storage_virtual_machine_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationFSxONTAPProperties {
                    protocol: protocol,
                    security_group_arns: security_group_arns.ok_or(::serde::de::Error::missing_field("SecurityGroupArns"))?,
                    storage_virtual_machine_arn: storage_virtual_machine_arn.ok_or(::serde::de::Error::missing_field("StorageVirtualMachineArn"))?,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationFSxONTAP {
    type Properties = LocationFSxONTAPProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationFSxONTAP";
    fn properties(&self) -> &LocationFSxONTAPProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationFSxONTAPProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationFSxONTAP {}

impl From<LocationFSxONTAPProperties> for LocationFSxONTAP {
    fn from(properties: LocationFSxONTAPProperties) -> LocationFSxONTAP {
        LocationFSxONTAP { properties }
    }
}

/// The [`AWS::DataSync::LocationFSxOpenZFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html) resource type.
#[derive(Debug, Default)]
pub struct LocationFSxOpenZFS {
    properties: LocationFSxOpenZFSProperties
}

/// Properties for the `LocationFSxOpenZFS` resource.
#[derive(Debug, Default)]
pub struct LocationFSxOpenZFSProperties {
    /// Property [`FsxFilesystemArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html#cfn-datasync-locationfsxopenzfs-fsxfilesystemarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fsx_filesystem_arn: Option<::Value<String>>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html#cfn-datasync-locationfsxopenzfs-protocol).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol: ::Value<self::location_f_sx_open_zfs::Protocol>,
    /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html#cfn-datasync-locationfsxopenzfs-securitygrouparns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_arns: ::ValueList<String>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html#cfn-datasync-locationfsxopenzfs-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html#cfn-datasync-locationfsxopenzfs-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationFSxOpenZFSProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref fsx_filesystem_arn) = self.fsx_filesystem_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FsxFilesystemArn", fsx_filesystem_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", &self.security_group_arns)?;
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationFSxOpenZFSProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationFSxOpenZFSProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationFSxOpenZFSProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationFSxOpenZFSProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fsx_filesystem_arn: Option<::Value<String>> = None;
                let mut protocol: Option<::Value<self::location_f_sx_open_zfs::Protocol>> = None;
                let mut security_group_arns: Option<::ValueList<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FsxFilesystemArn" => {
                            fsx_filesystem_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupArns" => {
                            security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationFSxOpenZFSProperties {
                    fsx_filesystem_arn: fsx_filesystem_arn,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    security_group_arns: security_group_arns.ok_or(::serde::de::Error::missing_field("SecurityGroupArns"))?,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationFSxOpenZFS {
    type Properties = LocationFSxOpenZFSProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationFSxOpenZFS";
    fn properties(&self) -> &LocationFSxOpenZFSProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationFSxOpenZFSProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationFSxOpenZFS {}

impl From<LocationFSxOpenZFSProperties> for LocationFSxOpenZFS {
    fn from(properties: LocationFSxOpenZFSProperties) -> LocationFSxOpenZFS {
        LocationFSxOpenZFS { properties }
    }
}

/// The [`AWS::DataSync::LocationFSxWindows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html) resource type.
#[derive(Debug, Default)]
pub struct LocationFSxWindows {
    properties: LocationFSxWindowsProperties
}

/// Properties for the `LocationFSxWindows` resource.
#[derive(Debug, Default)]
pub struct LocationFSxWindowsProperties {
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`FsxFilesystemArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-fsxfilesystemarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fsx_filesystem_arn: Option<::Value<String>>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-password).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-securitygrouparns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_arns: ::ValueList<String>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html#cfn-datasync-locationfsxwindows-user).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user: ::Value<String>,
}

impl ::serde::Serialize for LocationFSxWindowsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref fsx_filesystem_arn) = self.fsx_filesystem_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FsxFilesystemArn", fsx_filesystem_arn)?;
        }
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", &self.security_group_arns)?;
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", &self.user)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationFSxWindowsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationFSxWindowsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationFSxWindowsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationFSxWindowsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain: Option<::Value<String>> = None;
                let mut fsx_filesystem_arn: Option<::Value<String>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut security_group_arns: Option<::ValueList<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FsxFilesystemArn" => {
                            fsx_filesystem_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupArns" => {
                            security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "User" => {
                            user = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationFSxWindowsProperties {
                    domain: domain,
                    fsx_filesystem_arn: fsx_filesystem_arn,
                    password: password,
                    security_group_arns: security_group_arns.ok_or(::serde::de::Error::missing_field("SecurityGroupArns"))?,
                    subdirectory: subdirectory,
                    tags: tags,
                    user: user.ok_or(::serde::de::Error::missing_field("User"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationFSxWindows {
    type Properties = LocationFSxWindowsProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationFSxWindows";
    fn properties(&self) -> &LocationFSxWindowsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationFSxWindowsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationFSxWindows {}

impl From<LocationFSxWindowsProperties> for LocationFSxWindows {
    fn from(properties: LocationFSxWindowsProperties) -> LocationFSxWindows {
        LocationFSxWindows { properties }
    }
}

/// The [`AWS::DataSync::LocationHDFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html) resource type.
#[derive(Debug, Default)]
pub struct LocationHDFS {
    properties: LocationHDFSProperties
}

/// Properties for the `LocationHDFS` resource.
#[derive(Debug, Default)]
pub struct LocationHDFSProperties {
    /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-agentarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_arns: ::ValueList<String>,
    /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-authenticationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authentication_type: ::Value<String>,
    /// Property [`BlockSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-blocksize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub block_size: Option<::Value<u32>>,
    /// Property [`KerberosKeytab`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-kerberoskeytab).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kerberos_keytab: Option<::Value<String>>,
    /// Property [`KerberosKrb5Conf`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-kerberoskrb5conf).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kerberos_krb5_conf: Option<::Value<String>>,
    /// Property [`KerberosPrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-kerberosprincipal).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kerberos_principal: Option<::Value<String>>,
    /// Property [`KmsKeyProviderUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-kmskeyprovideruri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_provider_uri: Option<::Value<String>>,
    /// Property [`NameNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-namenodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name_nodes: ::ValueList<self::location_hdfs::NameNode>,
    /// Property [`QopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-qopconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub qop_configuration: Option<::Value<self::location_hdfs::QopConfiguration>>,
    /// Property [`ReplicationFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-replicationfactor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_factor: Option<::Value<u32>>,
    /// Property [`SimpleUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-simpleuser).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub simple_user: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-subdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html#cfn-datasync-locationhdfs-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationHDFSProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
        if let Some(ref block_size) = self.block_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockSize", block_size)?;
        }
        if let Some(ref kerberos_keytab) = self.kerberos_keytab {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KerberosKeytab", kerberos_keytab)?;
        }
        if let Some(ref kerberos_krb5_conf) = self.kerberos_krb5_conf {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KerberosKrb5Conf", kerberos_krb5_conf)?;
        }
        if let Some(ref kerberos_principal) = self.kerberos_principal {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KerberosPrincipal", kerberos_principal)?;
        }
        if let Some(ref kms_key_provider_uri) = self.kms_key_provider_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyProviderUri", kms_key_provider_uri)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameNodes", &self.name_nodes)?;
        if let Some(ref qop_configuration) = self.qop_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QopConfiguration", qop_configuration)?;
        }
        if let Some(ref replication_factor) = self.replication_factor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationFactor", replication_factor)?;
        }
        if let Some(ref simple_user) = self.simple_user {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SimpleUser", simple_user)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationHDFSProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationHDFSProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationHDFSProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationHDFSProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_arns: Option<::ValueList<String>> = None;
                let mut authentication_type: Option<::Value<String>> = None;
                let mut block_size: Option<::Value<u32>> = None;
                let mut kerberos_keytab: Option<::Value<String>> = None;
                let mut kerberos_krb5_conf: Option<::Value<String>> = None;
                let mut kerberos_principal: Option<::Value<String>> = None;
                let mut kms_key_provider_uri: Option<::Value<String>> = None;
                let mut name_nodes: Option<::ValueList<self::location_hdfs::NameNode>> = None;
                let mut qop_configuration: Option<::Value<self::location_hdfs::QopConfiguration>> = None;
                let mut replication_factor: Option<::Value<u32>> = None;
                let mut simple_user: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentArns" => {
                            agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthenticationType" => {
                            authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BlockSize" => {
                            block_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KerberosKeytab" => {
                            kerberos_keytab = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KerberosKrb5Conf" => {
                            kerberos_krb5_conf = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KerberosPrincipal" => {
                            kerberos_principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyProviderUri" => {
                            kms_key_provider_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NameNodes" => {
                            name_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QopConfiguration" => {
                            qop_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationFactor" => {
                            replication_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SimpleUser" => {
                            simple_user = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationHDFSProperties {
                    agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                    block_size: block_size,
                    kerberos_keytab: kerberos_keytab,
                    kerberos_krb5_conf: kerberos_krb5_conf,
                    kerberos_principal: kerberos_principal,
                    kms_key_provider_uri: kms_key_provider_uri,
                    name_nodes: name_nodes.ok_or(::serde::de::Error::missing_field("NameNodes"))?,
                    qop_configuration: qop_configuration,
                    replication_factor: replication_factor,
                    simple_user: simple_user,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationHDFS {
    type Properties = LocationHDFSProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationHDFS";
    fn properties(&self) -> &LocationHDFSProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationHDFSProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationHDFS {}

impl From<LocationHDFSProperties> for LocationHDFS {
    fn from(properties: LocationHDFSProperties) -> LocationHDFS {
        LocationHDFS { properties }
    }
}

/// The [`AWS::DataSync::LocationNFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html) resource type.
#[derive(Debug, Default)]
pub struct LocationNFS {
    properties: LocationNFSProperties
}

/// Properties for the `LocationNFS` resource.
#[derive(Debug, Default)]
pub struct LocationNFSProperties {
    /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html#cfn-datasync-locationnfs-mountoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mount_options: Option<::Value<self::location_nfs::MountOptions>>,
    /// Property [`OnPremConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html#cfn-datasync-locationnfs-onpremconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub on_prem_config: ::Value<self::location_nfs::OnPremConfig>,
    /// Property [`ServerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html#cfn-datasync-locationnfs-serverhostname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_hostname: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html#cfn-datasync-locationnfs-subdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html#cfn-datasync-locationnfs-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationNFSProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref mount_options) = self.mount_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", mount_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnPremConfig", &self.on_prem_config)?;
        if let Some(ref server_hostname) = self.server_hostname {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerHostname", server_hostname)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationNFSProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationNFSProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationNFSProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationNFSProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mount_options: Option<::Value<self::location_nfs::MountOptions>> = None;
                let mut on_prem_config: Option<::Value<self::location_nfs::OnPremConfig>> = None;
                let mut server_hostname: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MountOptions" => {
                            mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnPremConfig" => {
                            on_prem_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerHostname" => {
                            server_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationNFSProperties {
                    mount_options: mount_options,
                    on_prem_config: on_prem_config.ok_or(::serde::de::Error::missing_field("OnPremConfig"))?,
                    server_hostname: server_hostname,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationNFS {
    type Properties = LocationNFSProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationNFS";
    fn properties(&self) -> &LocationNFSProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationNFSProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationNFS {}

impl From<LocationNFSProperties> for LocationNFS {
    fn from(properties: LocationNFSProperties) -> LocationNFS {
        LocationNFS { properties }
    }
}

/// The [`AWS::DataSync::LocationObjectStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html) resource type.
#[derive(Debug, Default)]
pub struct LocationObjectStorage {
    properties: LocationObjectStorageProperties
}

/// Properties for the `LocationObjectStorage` resource.
#[derive(Debug, Default)]
pub struct LocationObjectStorageProperties {
    /// Property [`AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-accesskey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_key: Option<::Value<String>>,
    /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-agentarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_arns: ::ValueList<String>,
    /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-bucketname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_name: Option<::Value<String>>,
    /// Property [`SecretKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-secretkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secret_key: Option<::Value<String>>,
    /// Property [`ServerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-servercertificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_certificate: Option<::Value<String>>,
    /// Property [`ServerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-serverhostname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_hostname: Option<::Value<String>>,
    /// Property [`ServerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-serverport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_port: Option<::Value<u32>>,
    /// Property [`ServerProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-serverprotocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_protocol: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-subdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html#cfn-datasync-locationobjectstorage-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationObjectStorageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_key) = self.access_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessKey", access_key)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
        if let Some(ref bucket_name) = self.bucket_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
        }
        if let Some(ref secret_key) = self.secret_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretKey", secret_key)?;
        }
        if let Some(ref server_certificate) = self.server_certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificate", server_certificate)?;
        }
        if let Some(ref server_hostname) = self.server_hostname {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerHostname", server_hostname)?;
        }
        if let Some(ref server_port) = self.server_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerPort", server_port)?;
        }
        if let Some(ref server_protocol) = self.server_protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerProtocol", server_protocol)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationObjectStorageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationObjectStorageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationObjectStorageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationObjectStorageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_key: Option<::Value<String>> = None;
                let mut agent_arns: Option<::ValueList<String>> = None;
                let mut bucket_name: Option<::Value<String>> = None;
                let mut secret_key: Option<::Value<String>> = None;
                let mut server_certificate: Option<::Value<String>> = None;
                let mut server_hostname: Option<::Value<String>> = None;
                let mut server_port: Option<::Value<u32>> = None;
                let mut server_protocol: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessKey" => {
                            access_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AgentArns" => {
                            agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketName" => {
                            bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretKey" => {
                            secret_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerCertificate" => {
                            server_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerHostname" => {
                            server_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerPort" => {
                            server_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerProtocol" => {
                            server_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationObjectStorageProperties {
                    access_key: access_key,
                    agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    bucket_name: bucket_name,
                    secret_key: secret_key,
                    server_certificate: server_certificate,
                    server_hostname: server_hostname,
                    server_port: server_port,
                    server_protocol: server_protocol,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationObjectStorage {
    type Properties = LocationObjectStorageProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationObjectStorage";
    fn properties(&self) -> &LocationObjectStorageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationObjectStorageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationObjectStorage {}

impl From<LocationObjectStorageProperties> for LocationObjectStorage {
    fn from(properties: LocationObjectStorageProperties) -> LocationObjectStorage {
        LocationObjectStorage { properties }
    }
}

/// The [`AWS::DataSync::LocationS3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html) resource type.
#[derive(Debug, Default)]
pub struct LocationS3 {
    properties: LocationS3Properties
}

/// Properties for the `LocationS3` resource.
#[derive(Debug, Default)]
pub struct LocationS3Properties {
    /// Property [`S3BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html#cfn-datasync-locations3-s3bucketarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_bucket_arn: Option<::Value<String>>,
    /// Property [`S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html#cfn-datasync-locations3-s3config).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_config: ::Value<self::location_s3::S3Config>,
    /// Property [`S3StorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html#cfn-datasync-locations3-s3storageclass).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_storage_class: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html#cfn-datasync-locations3-subdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html#cfn-datasync-locations3-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LocationS3Properties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref s3_bucket_arn) = self.s3_bucket_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketArn", s3_bucket_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Config", &self.s3_config)?;
        if let Some(ref s3_storage_class) = self.s3_storage_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3StorageClass", s3_storage_class)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationS3Properties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationS3Properties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationS3Properties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationS3Properties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut s3_bucket_arn: Option<::Value<String>> = None;
                let mut s3_config: Option<::Value<self::location_s3::S3Config>> = None;
                let mut s3_storage_class: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "S3BucketArn" => {
                            s3_bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Config" => {
                            s3_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3StorageClass" => {
                            s3_storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationS3Properties {
                    s3_bucket_arn: s3_bucket_arn,
                    s3_config: s3_config.ok_or(::serde::de::Error::missing_field("S3Config"))?,
                    s3_storage_class: s3_storage_class,
                    subdirectory: subdirectory,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationS3 {
    type Properties = LocationS3Properties;
    const TYPE: &'static str = "AWS::DataSync::LocationS3";
    fn properties(&self) -> &LocationS3Properties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationS3Properties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationS3 {}

impl From<LocationS3Properties> for LocationS3 {
    fn from(properties: LocationS3Properties) -> LocationS3 {
        LocationS3 { properties }
    }
}

/// The [`AWS::DataSync::LocationSMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html) resource type.
#[derive(Debug, Default)]
pub struct LocationSMB {
    properties: LocationSMBProperties
}

/// Properties for the `LocationSMB` resource.
#[derive(Debug, Default)]
pub struct LocationSMBProperties {
    /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-agentarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_arns: ::ValueList<String>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-mountoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mount_options: Option<::Value<self::location_smb::MountOptions>>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-password).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`ServerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-serverhostname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_hostname: Option<::Value<String>>,
    /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-subdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subdirectory: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html#cfn-datasync-locationsmb-user).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user: ::Value<String>,
}

impl ::serde::Serialize for LocationSMBProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref mount_options) = self.mount_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", mount_options)?;
        }
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        if let Some(ref server_hostname) = self.server_hostname {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerHostname", server_hostname)?;
        }
        if let Some(ref subdirectory) = self.subdirectory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", &self.user)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LocationSMBProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationSMBProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LocationSMBProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LocationSMBProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_arns: Option<::ValueList<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut mount_options: Option<::Value<self::location_smb::MountOptions>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut server_hostname: Option<::Value<String>> = None;
                let mut subdirectory: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentArns" => {
                            agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MountOptions" => {
                            mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerHostname" => {
                            server_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subdirectory" => {
                            subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "User" => {
                            user = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LocationSMBProperties {
                    agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    domain: domain,
                    mount_options: mount_options,
                    password: password,
                    server_hostname: server_hostname,
                    subdirectory: subdirectory,
                    tags: tags,
                    user: user.ok_or(::serde::de::Error::missing_field("User"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LocationSMB {
    type Properties = LocationSMBProperties;
    const TYPE: &'static str = "AWS::DataSync::LocationSMB";
    fn properties(&self) -> &LocationSMBProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LocationSMBProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LocationSMB {}

impl From<LocationSMBProperties> for LocationSMB {
    fn from(properties: LocationSMBProperties) -> LocationSMB {
        LocationSMB { properties }
    }
}

/// The [`AWS::DataSync::StorageSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html) resource type.
#[derive(Debug, Default)]
pub struct StorageSystem {
    properties: StorageSystemProperties
}

/// Properties for the `StorageSystem` resource.
#[derive(Debug, Default)]
pub struct StorageSystemProperties {
    /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-agentarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_arns: ::ValueList<String>,
    /// Property [`CloudWatchLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-cloudwatchloggrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_log_group_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ServerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-serverconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_configuration: ::Value<self::storage_system::ServerConfiguration>,
    /// Property [`ServerCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-servercredentials).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_credentials: Option<::Value<self::storage_system::ServerCredentials>>,
    /// Property [`SystemType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-systemtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub system_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-storagesystem.html#cfn-datasync-storagesystem-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StorageSystemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
        if let Some(ref cloud_watch_log_group_arn) = self.cloud_watch_log_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupArn", cloud_watch_log_group_arn)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerConfiguration", &self.server_configuration)?;
        if let Some(ref server_credentials) = self.server_credentials {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCredentials", server_credentials)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemType", &self.system_type)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StorageSystemProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageSystemProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageSystemProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StorageSystemProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_arns: Option<::ValueList<String>> = None;
                let mut cloud_watch_log_group_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut server_configuration: Option<::Value<self::storage_system::ServerConfiguration>> = None;
                let mut server_credentials: Option<::Value<self::storage_system::ServerCredentials>> = None;
                let mut system_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentArns" => {
                            agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchLogGroupArn" => {
                            cloud_watch_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerConfiguration" => {
                            server_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerCredentials" => {
                            server_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SystemType" => {
                            system_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StorageSystemProperties {
                    agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    cloud_watch_log_group_arn: cloud_watch_log_group_arn,
                    name: name,
                    server_configuration: server_configuration.ok_or(::serde::de::Error::missing_field("ServerConfiguration"))?,
                    server_credentials: server_credentials,
                    system_type: system_type.ok_or(::serde::de::Error::missing_field("SystemType"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StorageSystem {
    type Properties = StorageSystemProperties;
    const TYPE: &'static str = "AWS::DataSync::StorageSystem";
    fn properties(&self) -> &StorageSystemProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StorageSystemProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StorageSystem {}

impl From<StorageSystemProperties> for StorageSystem {
    fn from(properties: StorageSystemProperties) -> StorageSystem {
        StorageSystem { properties }
    }
}

/// The [`AWS::DataSync::Task`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html) resource type.
#[derive(Debug, Default)]
pub struct Task {
    properties: TaskProperties
}

/// Properties for the `Task` resource.
#[derive(Debug, Default)]
pub struct TaskProperties {
    /// Property [`CloudWatchLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-cloudwatchloggrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_log_group_arn: Option<::Value<String>>,
    /// Property [`DestinationLocationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-destinationlocationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub destination_location_arn: ::Value<String>,
    /// Property [`Excludes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-excludes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub excludes: Option<::ValueList<self::task::FilterRule>>,
    /// Property [`Includes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-includes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub includes: Option<::ValueList<self::task::FilterRule>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-options).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub options: Option<::Value<self::task::Options>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<self::task::TaskSchedule>>,
    /// Property [`SourceLocationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-sourcelocationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_location_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TaskReportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html#cfn-datasync-task-taskreportconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_report_config: Option<::Value<self::task::TaskReportConfig>>,
}

impl ::serde::Serialize for TaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cloud_watch_log_group_arn) = self.cloud_watch_log_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupArn", cloud_watch_log_group_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationLocationArn", &self.destination_location_arn)?;
        if let Some(ref excludes) = self.excludes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Excludes", excludes)?;
        }
        if let Some(ref includes) = self.includes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Includes", includes)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref options) = self.options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
        }
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLocationArn", &self.source_location_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task_report_config) = self.task_report_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskReportConfig", task_report_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_watch_log_group_arn: Option<::Value<String>> = None;
                let mut destination_location_arn: Option<::Value<String>> = None;
                let mut excludes: Option<::ValueList<self::task::FilterRule>> = None;
                let mut includes: Option<::ValueList<self::task::FilterRule>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut options: Option<::Value<self::task::Options>> = None;
                let mut schedule: Option<::Value<self::task::TaskSchedule>> = None;
                let mut source_location_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task_report_config: Option<::Value<self::task::TaskReportConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudWatchLogGroupArn" => {
                            cloud_watch_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationLocationArn" => {
                            destination_location_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Excludes" => {
                            excludes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Includes" => {
                            includes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Options" => {
                            options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceLocationArn" => {
                            source_location_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskReportConfig" => {
                            task_report_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskProperties {
                    cloud_watch_log_group_arn: cloud_watch_log_group_arn,
                    destination_location_arn: destination_location_arn.ok_or(::serde::de::Error::missing_field("DestinationLocationArn"))?,
                    excludes: excludes,
                    includes: includes,
                    name: name,
                    options: options,
                    schedule: schedule,
                    source_location_arn: source_location_arn.ok_or(::serde::de::Error::missing_field("SourceLocationArn"))?,
                    tags: tags,
                    task_report_config: task_report_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Task {
    type Properties = TaskProperties;
    const TYPE: &'static str = "AWS::DataSync::Task";
    fn properties(&self) -> &TaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Task {}

impl From<TaskProperties> for Task {
    fn from(properties: TaskProperties) -> Task {
        Task { properties }
    }
}

pub mod location_azure_blob {
    //! Property types for the `LocationAzureBlob` resource.

    /// The [`AWS::DataSync::LocationAzureBlob.AzureBlobSasConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-azureblobsasconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AzureBlobSasConfiguration {
        /// Property [`AzureBlobSasToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-azureblobsasconfiguration.html#cfn-datasync-locationazureblob-azureblobsasconfiguration-azureblobsastoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub azure_blob_sas_token: ::Value<String>,
    }

    impl ::codec::SerializeValue for AzureBlobSasConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AzureBlobSasToken", &self.azure_blob_sas_token)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AzureBlobSasConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AzureBlobSasConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AzureBlobSasConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AzureBlobSasConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut azure_blob_sas_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AzureBlobSasToken" => {
                                azure_blob_sas_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AzureBlobSasConfiguration {
                        azure_blob_sas_token: azure_blob_sas_token.ok_or(::serde::de::Error::missing_field("AzureBlobSasToken"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_efs {
    //! Property types for the `LocationEFS` resource.

    /// The [`AWS::DataSync::LocationEFS.Ec2Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationefs-ec2config.html) property type.
    #[derive(Debug, Default)]
    pub struct Ec2Config {
        /// Property [`SecurityGroupArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationefs-ec2config.html#cfn-datasync-locationefs-ec2config-securitygrouparns).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_arns: ::ValueList<String>,
        /// Property [`SubnetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationefs-ec2config.html#cfn-datasync-locationefs-ec2config-subnetarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Ec2Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupArns", &self.security_group_arns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetArn", &self.subnet_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ec2Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ec2Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ec2Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ec2Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_arns: Option<::ValueList<String>> = None;
                    let mut subnet_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupArns" => {
                                security_group_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetArn" => {
                                subnet_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ec2Config {
                        security_group_arns: security_group_arns.ok_or(::serde::de::Error::missing_field("SecurityGroupArns"))?,
                        subnet_arn: subnet_arn.ok_or(::serde::de::Error::missing_field("SubnetArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_f_sx_ontap {
    //! Property types for the `LocationFSxONTAP` resource.

    /// The [`AWS::DataSync::LocationFSxONTAP.NFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfs.html) property type.
    #[derive(Debug, Default)]
    pub struct NFS {
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfs.html#cfn-datasync-locationfsxontap-nfs-mountoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_options: ::Value<NfsMountOptions>,
    }

    impl ::codec::SerializeValue for NFS {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", &self.mount_options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NFS {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NFS, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NFS;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NFS")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mount_options: Option<::Value<NfsMountOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NFS {
                        mount_options: mount_options.ok_or(::serde::de::Error::missing_field("MountOptions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxONTAP.NfsMountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfsmountoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct NfsMountOptions {
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfsmountoptions.html#cfn-datasync-locationfsxontap-nfsmountoptions-version).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NfsMountOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NfsMountOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NfsMountOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NfsMountOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NfsMountOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NfsMountOptions {
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxONTAP.Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-protocol.html) property type.
    #[derive(Debug, Default)]
    pub struct Protocol {
        /// Property [`NFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-protocol.html#cfn-datasync-locationfsxontap-protocol-nfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub nfs: Option<::Value<NFS>>,
        /// Property [`SMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-protocol.html#cfn-datasync-locationfsxontap-protocol-smb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub smb: Option<::Value<SMB>>,
    }

    impl ::codec::SerializeValue for Protocol {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref nfs) = self.nfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NFS", nfs)?;
            }
            if let Some(ref smb) = self.smb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SMB", smb)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Protocol {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Protocol, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Protocol;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Protocol")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut nfs: Option<::Value<NFS>> = None;
                    let mut smb: Option<::Value<SMB>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NFS" => {
                                nfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SMB" => {
                                smb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Protocol {
                        nfs: nfs,
                        smb: smb,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxONTAP.SMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html) property type.
    #[derive(Debug, Default)]
    pub struct SMB {
        /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html#cfn-datasync-locationfsxontap-smb-domain).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub domain: Option<::Value<String>>,
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html#cfn-datasync-locationfsxontap-smb-mountoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_options: ::Value<SmbMountOptions>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html#cfn-datasync-locationfsxontap-smb-password).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html#cfn-datasync-locationfsxontap-smb-user).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub user: ::Value<String>,
    }

    impl ::codec::SerializeValue for SMB {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref domain) = self.domain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", &self.mount_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", &self.user)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SMB {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SMB, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SMB;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SMB")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain: Option<::Value<String>> = None;
                    let mut mount_options: Option<::Value<SmbMountOptions>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut user: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Domain" => {
                                domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SMB {
                        domain: domain,
                        mount_options: mount_options.ok_or(::serde::de::Error::missing_field("MountOptions"))?,
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        user: user.ok_or(::serde::de::Error::missing_field("User"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxONTAP.SmbMountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smbmountoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SmbMountOptions {
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smbmountoptions.html#cfn-datasync-locationfsxontap-smbmountoptions-version).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SmbMountOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SmbMountOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SmbMountOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SmbMountOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SmbMountOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SmbMountOptions {
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_f_sx_open_zfs {
    //! Property types for the `LocationFSxOpenZFS` resource.

    /// The [`AWS::DataSync::LocationFSxOpenZFS.MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-mountoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MountOptions {
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-mountoptions.html#cfn-datasync-locationfsxopenzfs-mountoptions-version).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MountOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MountOptions {
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxOpenZFS.NFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-nfs.html) property type.
    #[derive(Debug, Default)]
    pub struct NFS {
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-nfs.html#cfn-datasync-locationfsxopenzfs-nfs-mountoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_options: ::Value<MountOptions>,
    }

    impl ::codec::SerializeValue for NFS {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", &self.mount_options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NFS {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NFS, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NFS;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NFS")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mount_options: Option<::Value<MountOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NFS {
                        mount_options: mount_options.ok_or(::serde::de::Error::missing_field("MountOptions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationFSxOpenZFS.Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-protocol.html) property type.
    #[derive(Debug, Default)]
    pub struct Protocol {
        /// Property [`NFS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-protocol.html#cfn-datasync-locationfsxopenzfs-protocol-nfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub nfs: Option<::Value<NFS>>,
    }

    impl ::codec::SerializeValue for Protocol {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref nfs) = self.nfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NFS", nfs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Protocol {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Protocol, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Protocol;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Protocol")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut nfs: Option<::Value<NFS>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NFS" => {
                                nfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Protocol {
                        nfs: nfs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_hdfs {
    //! Property types for the `LocationHDFS` resource.

    /// The [`AWS::DataSync::LocationHDFS.NameNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-namenode.html) property type.
    #[derive(Debug, Default)]
    pub struct NameNode {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-namenode.html#cfn-datasync-locationhdfs-namenode-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-namenode.html#cfn-datasync-locationhdfs-namenode-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for NameNode {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", &self.hostname)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NameNode {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NameNode, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NameNode;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NameNode")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NameNode {
                        hostname: hostname.ok_or(::serde::de::Error::missing_field("Hostname"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationHDFS.QopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-qopconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct QopConfiguration {
        /// Property [`DataTransferProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-qopconfiguration.html#cfn-datasync-locationhdfs-qopconfiguration-datatransferprotection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_transfer_protection: Option<::Value<String>>,
        /// Property [`RpcProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-qopconfiguration.html#cfn-datasync-locationhdfs-qopconfiguration-rpcprotection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rpc_protection: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for QopConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_transfer_protection) = self.data_transfer_protection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransferProtection", data_transfer_protection)?;
            }
            if let Some(ref rpc_protection) = self.rpc_protection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RpcProtection", rpc_protection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QopConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QopConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QopConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QopConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_transfer_protection: Option<::Value<String>> = None;
                    let mut rpc_protection: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataTransferProtection" => {
                                data_transfer_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RpcProtection" => {
                                rpc_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QopConfiguration {
                        data_transfer_protection: data_transfer_protection,
                        rpc_protection: rpc_protection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_nfs {
    //! Property types for the `LocationNFS` resource.

    /// The [`AWS::DataSync::LocationNFS.MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-mountoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MountOptions {
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-mountoptions.html#cfn-datasync-locationnfs-mountoptions-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MountOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MountOptions {
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::LocationNFS.OnPremConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-onpremconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OnPremConfig {
        /// Property [`AgentArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-onpremconfig.html#cfn-datasync-locationnfs-onpremconfig-agentarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub agent_arns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for OnPremConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentArns", &self.agent_arns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OnPremConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnPremConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnPremConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnPremConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut agent_arns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AgentArns" => {
                                agent_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnPremConfig {
                        agent_arns: agent_arns.ok_or(::serde::de::Error::missing_field("AgentArns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_s3 {
    //! Property types for the `LocationS3` resource.

    /// The [`AWS::DataSync::LocationS3.S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locations3-s3config.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Config {
        /// Property [`BucketAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locations3-s3config.html#cfn-datasync-locations3-s3config-bucketaccessrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_access_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketAccessRoleArn", &self.bucket_access_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_access_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketAccessRoleArn" => {
                                bucket_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Config {
                        bucket_access_role_arn: bucket_access_role_arn.ok_or(::serde::de::Error::missing_field("BucketAccessRoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod location_smb {
    //! Property types for the `LocationSMB` resource.

    /// The [`AWS::DataSync::LocationSMB.MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-mountoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MountOptions {
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-mountoptions.html#cfn-datasync-locationsmb-mountoptions-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MountOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MountOptions {
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod storage_system {
    //! Property types for the `StorageSystem` resource.

    /// The [`AWS::DataSync::StorageSystem.ServerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-serverconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerConfiguration {
        /// Property [`ServerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-serverconfiguration.html#cfn-datasync-storagesystem-serverconfiguration-serverhostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_hostname: ::Value<String>,
        /// Property [`ServerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-serverconfiguration.html#cfn-datasync-storagesystem-serverconfiguration-serverport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ServerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerHostname", &self.server_hostname)?;
            if let Some(ref server_port) = self.server_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerPort", server_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_hostname: Option<::Value<String>> = None;
                    let mut server_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerHostname" => {
                                server_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerPort" => {
                                server_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerConfiguration {
                        server_hostname: server_hostname.ok_or(::serde::de::Error::missing_field("ServerHostname"))?,
                        server_port: server_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::StorageSystem.ServerCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-servercredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerCredentials {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-servercredentials.html#cfn-datasync-storagesystem-servercredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-storagesystem-servercredentials.html#cfn-datasync-storagesystem-servercredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServerCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerCredentials {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task {
    //! Property types for the `Task` resource.

    /// The [`AWS::DataSync::Task.Deleted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-deleted.html) property type.
    #[derive(Debug, Default)]
    pub struct Deleted {
        /// Property [`ReportLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-deleted.html#cfn-datasync-task-deleted-reportlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Deleted {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref report_level) = self.report_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportLevel", report_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Deleted {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Deleted, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Deleted;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Deleted")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut report_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReportLevel" => {
                                report_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Deleted {
                        report_level: report_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-destination.html) property type.
    #[derive(Debug, Default)]
    pub struct Destination {
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-destination.html#cfn-datasync-task-destination-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3>>,
    }

    impl ::codec::SerializeValue for Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
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
                    let mut s3: Option<::Value<S3>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Destination {
                        s3: s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.FilterRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-filterrule.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterRule {
        /// Property [`FilterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-filterrule.html#cfn-datasync-task-filterrule-filtertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_type: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-filterrule.html#cfn-datasync-task-filterrule-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FilterRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filter_type) = self.filter_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterType", filter_type)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filter_type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FilterType" => {
                                filter_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterRule {
                        filter_type: filter_type,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html) property type.
    #[derive(Debug, Default)]
    pub struct Options {
        /// Property [`Atime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-atime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub atime: Option<::Value<String>>,
        /// Property [`BytesPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-bytespersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bytes_per_second: Option<::Value<u32>>,
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-gid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gid: Option<::Value<String>>,
        /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-loglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_level: Option<::Value<String>>,
        /// Property [`Mtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-mtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mtime: Option<::Value<String>>,
        /// Property [`ObjectTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-objecttags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_tags: Option<::Value<String>>,
        /// Property [`OverwriteMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-overwritemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overwrite_mode: Option<::Value<String>>,
        /// Property [`PosixPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-posixpermissions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub posix_permissions: Option<::Value<String>>,
        /// Property [`PreserveDeletedFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-preservedeletedfiles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preserve_deleted_files: Option<::Value<String>>,
        /// Property [`PreserveDevices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-preservedevices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preserve_devices: Option<::Value<String>>,
        /// Property [`SecurityDescriptorCopyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-securitydescriptorcopyflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_descriptor_copy_flags: Option<::Value<String>>,
        /// Property [`TaskQueueing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-taskqueueing).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_queueing: Option<::Value<String>>,
        /// Property [`TransferMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-transfermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transfer_mode: Option<::Value<String>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-uid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uid: Option<::Value<String>>,
        /// Property [`VerifyMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html#cfn-datasync-task-options-verifymode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub verify_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Options {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref atime) = self.atime {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Atime", atime)?;
            }
            if let Some(ref bytes_per_second) = self.bytes_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BytesPerSecond", bytes_per_second)?;
            }
            if let Some(ref gid) = self.gid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", gid)?;
            }
            if let Some(ref log_level) = self.log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", log_level)?;
            }
            if let Some(ref mtime) = self.mtime {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mtime", mtime)?;
            }
            if let Some(ref object_tags) = self.object_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTags", object_tags)?;
            }
            if let Some(ref overwrite_mode) = self.overwrite_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverwriteMode", overwrite_mode)?;
            }
            if let Some(ref posix_permissions) = self.posix_permissions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PosixPermissions", posix_permissions)?;
            }
            if let Some(ref preserve_deleted_files) = self.preserve_deleted_files {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreserveDeletedFiles", preserve_deleted_files)?;
            }
            if let Some(ref preserve_devices) = self.preserve_devices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreserveDevices", preserve_devices)?;
            }
            if let Some(ref security_descriptor_copy_flags) = self.security_descriptor_copy_flags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityDescriptorCopyFlags", security_descriptor_copy_flags)?;
            }
            if let Some(ref task_queueing) = self.task_queueing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskQueueing", task_queueing)?;
            }
            if let Some(ref transfer_mode) = self.transfer_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransferMode", transfer_mode)?;
            }
            if let Some(ref uid) = self.uid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", uid)?;
            }
            if let Some(ref verify_mode) = self.verify_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerifyMode", verify_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Options {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Options, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Options;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Options")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut atime: Option<::Value<String>> = None;
                    let mut bytes_per_second: Option<::Value<u32>> = None;
                    let mut gid: Option<::Value<String>> = None;
                    let mut log_level: Option<::Value<String>> = None;
                    let mut mtime: Option<::Value<String>> = None;
                    let mut object_tags: Option<::Value<String>> = None;
                    let mut overwrite_mode: Option<::Value<String>> = None;
                    let mut posix_permissions: Option<::Value<String>> = None;
                    let mut preserve_deleted_files: Option<::Value<String>> = None;
                    let mut preserve_devices: Option<::Value<String>> = None;
                    let mut security_descriptor_copy_flags: Option<::Value<String>> = None;
                    let mut task_queueing: Option<::Value<String>> = None;
                    let mut transfer_mode: Option<::Value<String>> = None;
                    let mut uid: Option<::Value<String>> = None;
                    let mut verify_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Atime" => {
                                atime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BytesPerSecond" => {
                                bytes_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogLevel" => {
                                log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mtime" => {
                                mtime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectTags" => {
                                object_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OverwriteMode" => {
                                overwrite_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PosixPermissions" => {
                                posix_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreserveDeletedFiles" => {
                                preserve_deleted_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreserveDevices" => {
                                preserve_devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityDescriptorCopyFlags" => {
                                security_descriptor_copy_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskQueueing" => {
                                task_queueing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransferMode" => {
                                transfer_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VerifyMode" => {
                                verify_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Options {
                        atime: atime,
                        bytes_per_second: bytes_per_second,
                        gid: gid,
                        log_level: log_level,
                        mtime: mtime,
                        object_tags: object_tags,
                        overwrite_mode: overwrite_mode,
                        posix_permissions: posix_permissions,
                        preserve_deleted_files: preserve_deleted_files,
                        preserve_devices: preserve_devices,
                        security_descriptor_copy_flags: security_descriptor_copy_flags,
                        task_queueing: task_queueing,
                        transfer_mode: transfer_mode,
                        uid: uid,
                        verify_mode: verify_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html) property type.
    #[derive(Debug, Default)]
    pub struct Overrides {
        /// Property [`Deleted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html#cfn-datasync-task-overrides-deleted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deleted: Option<::Value<Deleted>>,
        /// Property [`Skipped`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html#cfn-datasync-task-overrides-skipped).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skipped: Option<::Value<Skipped>>,
        /// Property [`Transferred`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html#cfn-datasync-task-overrides-transferred).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transferred: Option<::Value<Transferred>>,
        /// Property [`Verified`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html#cfn-datasync-task-overrides-verified).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub verified: Option<::Value<Verified>>,
    }

    impl ::codec::SerializeValue for Overrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref deleted) = self.deleted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Deleted", deleted)?;
            }
            if let Some(ref skipped) = self.skipped {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Skipped", skipped)?;
            }
            if let Some(ref transferred) = self.transferred {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Transferred", transferred)?;
            }
            if let Some(ref verified) = self.verified {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Verified", verified)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Overrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Overrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Overrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Overrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut deleted: Option<::Value<Deleted>> = None;
                    let mut skipped: Option<::Value<Skipped>> = None;
                    let mut transferred: Option<::Value<Transferred>> = None;
                    let mut verified: Option<::Value<Verified>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Deleted" => {
                                deleted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Skipped" => {
                                skipped = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Transferred" => {
                                transferred = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Verified" => {
                                verified = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Overrides {
                        deleted: deleted,
                        skipped: skipped,
                        transferred: transferred,
                        verified: verified,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-s3.html) property type.
    #[derive(Debug, Default)]
    pub struct S3 {
        /// Property [`BucketAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-s3.html#cfn-datasync-task-s3-bucketaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_access_role_arn: Option<::Value<String>>,
        /// Property [`S3BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-s3.html#cfn-datasync-task-s3-s3bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_arn: Option<::Value<String>>,
        /// Property [`Subdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-s3.html#cfn-datasync-task-s3-subdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subdirectory: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_access_role_arn) = self.bucket_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketAccessRoleArn", bucket_access_role_arn)?;
            }
            if let Some(ref s3_bucket_arn) = self.s3_bucket_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketArn", s3_bucket_arn)?;
            }
            if let Some(ref subdirectory) = self.subdirectory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subdirectory", subdirectory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_access_role_arn: Option<::Value<String>> = None;
                    let mut s3_bucket_arn: Option<::Value<String>> = None;
                    let mut subdirectory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketAccessRoleArn" => {
                                bucket_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketArn" => {
                                s3_bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subdirectory" => {
                                subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3 {
                        bucket_access_role_arn: bucket_access_role_arn,
                        s3_bucket_arn: s3_bucket_arn,
                        subdirectory: subdirectory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Skipped`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-skipped.html) property type.
    #[derive(Debug, Default)]
    pub struct Skipped {
        /// Property [`ReportLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-skipped.html#cfn-datasync-task-skipped-reportlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Skipped {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref report_level) = self.report_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportLevel", report_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Skipped {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Skipped, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Skipped;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Skipped")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut report_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReportLevel" => {
                                report_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Skipped {
                        report_level: report_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.TaskReportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskReportConfig {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html#cfn-datasync-task-taskreportconfig-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<Destination>,
        /// Property [`ObjectVersionIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html#cfn-datasync-task-taskreportconfig-objectversionids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_version_ids: Option<::Value<String>>,
        /// Property [`OutputType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html#cfn-datasync-task-taskreportconfig-outputtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_type: ::Value<String>,
        /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html#cfn-datasync-task-taskreportconfig-overrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overrides: Option<::Value<Overrides>>,
        /// Property [`ReportLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html#cfn-datasync-task-taskreportconfig-reportlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TaskReportConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            if let Some(ref object_version_ids) = self.object_version_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersionIds", object_version_ids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputType", &self.output_type)?;
            if let Some(ref overrides) = self.overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", overrides)?;
            }
            if let Some(ref report_level) = self.report_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportLevel", report_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskReportConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskReportConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskReportConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskReportConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<Destination>> = None;
                    let mut object_version_ids: Option<::Value<String>> = None;
                    let mut output_type: Option<::Value<String>> = None;
                    let mut overrides: Option<::Value<Overrides>> = None;
                    let mut report_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectVersionIds" => {
                                object_version_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputType" => {
                                output_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overrides" => {
                                overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportLevel" => {
                                report_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskReportConfig {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        object_version_ids: object_version_ids,
                        output_type: output_type.ok_or(::serde::de::Error::missing_field("OutputType"))?,
                        overrides: overrides,
                        report_level: report_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.TaskSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskschedule.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskSchedule {
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskschedule.html#cfn-datasync-task-taskschedule-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskSchedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskSchedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskSchedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskSchedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskSchedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskSchedule {
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Transferred`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-transferred.html) property type.
    #[derive(Debug, Default)]
    pub struct Transferred {
        /// Property [`ReportLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-transferred.html#cfn-datasync-task-transferred-reportlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Transferred {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref report_level) = self.report_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportLevel", report_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Transferred {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Transferred, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Transferred;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Transferred")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut report_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReportLevel" => {
                                report_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Transferred {
                        report_level: report_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataSync::Task.Verified`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-verified.html) property type.
    #[derive(Debug, Default)]
    pub struct Verified {
        /// Property [`ReportLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-verified.html#cfn-datasync-task-verified-reportlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Verified {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref report_level) = self.report_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportLevel", report_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Verified {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Verified, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Verified;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Verified")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut report_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReportLevel" => {
                                report_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Verified {
                        report_level: report_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
