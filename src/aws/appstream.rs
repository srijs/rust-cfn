//! Types for the `AppStream` service.

/// The [`AWS::AppStream::DirectoryConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html) resource type.
#[derive(Debug, Default)]
pub struct DirectoryConfig {
    properties: DirectoryConfigProperties
}

/// Properties for the `DirectoryConfig` resource.
#[derive(Debug, Default)]
pub struct DirectoryConfigProperties {
    /// Property [`DirectoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html#cfn-appstream-directoryconfig-directoryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_name: ::Value<String>,
    /// Property [`OrganizationalUnitDistinguishedNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html#cfn-appstream-directoryconfig-organizationalunitdistinguishednames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organizational_unit_distinguished_names: ::ValueList<String>,
    /// Property [`ServiceAccountCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html#cfn-appstream-directoryconfig-serviceaccountcredentials).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_account_credentials: ::Value<self::directory_config::ServiceAccountCredentials>,
}

impl ::serde::Serialize for DirectoryConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryName", &self.directory_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedNames", &self.organizational_unit_distinguished_names)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccountCredentials", &self.service_account_credentials)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DirectoryConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DirectoryConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DirectoryConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DirectoryConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut directory_name: Option<::Value<String>> = None;
                let mut organizational_unit_distinguished_names: Option<::ValueList<String>> = None;
                let mut service_account_credentials: Option<::Value<self::directory_config::ServiceAccountCredentials>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DirectoryName" => {
                            directory_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationalUnitDistinguishedNames" => {
                            organizational_unit_distinguished_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceAccountCredentials" => {
                            service_account_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DirectoryConfigProperties {
                    directory_name: directory_name.ok_or(::serde::de::Error::missing_field("DirectoryName"))?,
                    organizational_unit_distinguished_names: organizational_unit_distinguished_names.ok_or(::serde::de::Error::missing_field("OrganizationalUnitDistinguishedNames"))?,
                    service_account_credentials: service_account_credentials.ok_or(::serde::de::Error::missing_field("ServiceAccountCredentials"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DirectoryConfig {
    type Properties = DirectoryConfigProperties;
    const TYPE: &'static str = "AWS::AppStream::DirectoryConfig";
    fn properties(&self) -> &DirectoryConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DirectoryConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DirectoryConfig {}

impl From<DirectoryConfigProperties> for DirectoryConfig {
    fn from(properties: DirectoryConfigProperties) -> DirectoryConfig {
        DirectoryConfig { properties }
    }
}

/// The [`AWS::AppStream::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html) resource type.
#[derive(Debug, Default)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug, Default)]
pub struct FleetProperties {
    /// Property [`ComputeCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-computecapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compute_capacity: ::Value<self::fleet::ComputeCapacity>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisconnectTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-disconnecttimeoutinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disconnect_timeout_in_seconds: Option<::Value<u32>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`DomainJoinInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-domainjoininfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_join_info: Option<::Value<self::fleet::DomainJoinInfo>>,
    /// Property [`EnableDefaultInternetAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-enabledefaultinternetaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_default_internet_access: Option<::Value<bool>>,
    /// Property [`FleetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-fleettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fleet_type: Option<::Value<String>>,
    /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-iamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role_arn: Option<::Value<String>>,
    /// Property [`IdleDisconnectTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-idledisconnecttimeoutinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idle_disconnect_timeout_in_seconds: Option<::Value<u32>>,
    /// Property [`ImageArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-imagearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_arn: Option<::Value<String>>,
    /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-imagename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_name: Option<::Value<String>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-instancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`MaxUserDurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-maxuserdurationinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_user_duration_in_seconds: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`StreamView`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-streamview).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_view: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::fleet::VpcConfig>>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeCapacity", &self.compute_capacity)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref disconnect_timeout_in_seconds) = self.disconnect_timeout_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisconnectTimeoutInSeconds", disconnect_timeout_in_seconds)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref domain_join_info) = self.domain_join_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainJoinInfo", domain_join_info)?;
        }
        if let Some(ref enable_default_internet_access) = self.enable_default_internet_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDefaultInternetAccess", enable_default_internet_access)?;
        }
        if let Some(ref fleet_type) = self.fleet_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetType", fleet_type)?;
        }
        if let Some(ref iam_role_arn) = self.iam_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", iam_role_arn)?;
        }
        if let Some(ref idle_disconnect_timeout_in_seconds) = self.idle_disconnect_timeout_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleDisconnectTimeoutInSeconds", idle_disconnect_timeout_in_seconds)?;
        }
        if let Some(ref image_arn) = self.image_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageArn", image_arn)?;
        }
        if let Some(ref image_name) = self.image_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", image_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        if let Some(ref max_user_duration_in_seconds) = self.max_user_duration_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUserDurationInSeconds", max_user_duration_in_seconds)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref stream_view) = self.stream_view {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamView", stream_view)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
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
                let mut compute_capacity: Option<::Value<self::fleet::ComputeCapacity>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut disconnect_timeout_in_seconds: Option<::Value<u32>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut domain_join_info: Option<::Value<self::fleet::DomainJoinInfo>> = None;
                let mut enable_default_internet_access: Option<::Value<bool>> = None;
                let mut fleet_type: Option<::Value<String>> = None;
                let mut iam_role_arn: Option<::Value<String>> = None;
                let mut idle_disconnect_timeout_in_seconds: Option<::Value<u32>> = None;
                let mut image_arn: Option<::Value<String>> = None;
                let mut image_name: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut max_user_duration_in_seconds: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut stream_view: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_config: Option<::Value<self::fleet::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeCapacity" => {
                            compute_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisconnectTimeoutInSeconds" => {
                            disconnect_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainJoinInfo" => {
                            domain_join_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableDefaultInternetAccess" => {
                            enable_default_internet_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FleetType" => {
                            fleet_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoleArn" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdleDisconnectTimeoutInSeconds" => {
                            idle_disconnect_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageArn" => {
                            image_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageName" => {
                            image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxUserDurationInSeconds" => {
                            max_user_duration_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamView" => {
                            stream_view = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    compute_capacity: compute_capacity.ok_or(::serde::de::Error::missing_field("ComputeCapacity"))?,
                    description: description,
                    disconnect_timeout_in_seconds: disconnect_timeout_in_seconds,
                    display_name: display_name,
                    domain_join_info: domain_join_info,
                    enable_default_internet_access: enable_default_internet_access,
                    fleet_type: fleet_type,
                    iam_role_arn: iam_role_arn,
                    idle_disconnect_timeout_in_seconds: idle_disconnect_timeout_in_seconds,
                    image_arn: image_arn,
                    image_name: image_name,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    max_user_duration_in_seconds: max_user_duration_in_seconds,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    stream_view: stream_view,
                    tags: tags,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::AppStream::Fleet";
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

/// The [`AWS::AppStream::ImageBuilder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html) resource type.
#[derive(Debug, Default)]
pub struct ImageBuilder {
    properties: ImageBuilderProperties
}

/// Properties for the `ImageBuilder` resource.
#[derive(Debug, Default)]
pub struct ImageBuilderProperties {
    /// Property [`AccessEndpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-accessendpoints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_endpoints: Option<::ValueList<self::image_builder::AccessEndpoint>>,
    /// Property [`AppstreamAgentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-appstreamagentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub appstream_agent_version: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`DomainJoinInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-domainjoininfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_join_info: Option<::Value<self::image_builder::DomainJoinInfo>>,
    /// Property [`EnableDefaultInternetAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-enabledefaultinternetaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_default_internet_access: Option<::Value<bool>>,
    /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-iamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role_arn: Option<::Value<String>>,
    /// Property [`ImageArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-imagearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_arn: Option<::Value<String>>,
    /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-imagename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_name: Option<::Value<String>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-instancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html#cfn-appstream-imagebuilder-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::image_builder::VpcConfig>>,
}

impl ::serde::Serialize for ImageBuilderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_endpoints) = self.access_endpoints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessEndpoints", access_endpoints)?;
        }
        if let Some(ref appstream_agent_version) = self.appstream_agent_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppstreamAgentVersion", appstream_agent_version)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref domain_join_info) = self.domain_join_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainJoinInfo", domain_join_info)?;
        }
        if let Some(ref enable_default_internet_access) = self.enable_default_internet_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDefaultInternetAccess", enable_default_internet_access)?;
        }
        if let Some(ref iam_role_arn) = self.iam_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", iam_role_arn)?;
        }
        if let Some(ref image_arn) = self.image_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageArn", image_arn)?;
        }
        if let Some(ref image_name) = self.image_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", image_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageBuilderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageBuilderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImageBuilderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImageBuilderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_endpoints: Option<::ValueList<self::image_builder::AccessEndpoint>> = None;
                let mut appstream_agent_version: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut domain_join_info: Option<::Value<self::image_builder::DomainJoinInfo>> = None;
                let mut enable_default_internet_access: Option<::Value<bool>> = None;
                let mut iam_role_arn: Option<::Value<String>> = None;
                let mut image_arn: Option<::Value<String>> = None;
                let mut image_name: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_config: Option<::Value<self::image_builder::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessEndpoints" => {
                            access_endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppstreamAgentVersion" => {
                            appstream_agent_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainJoinInfo" => {
                            domain_join_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableDefaultInternetAccess" => {
                            enable_default_internet_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoleArn" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageArn" => {
                            image_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageName" => {
                            image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImageBuilderProperties {
                    access_endpoints: access_endpoints,
                    appstream_agent_version: appstream_agent_version,
                    description: description,
                    display_name: display_name,
                    domain_join_info: domain_join_info,
                    enable_default_internet_access: enable_default_internet_access,
                    iam_role_arn: iam_role_arn,
                    image_arn: image_arn,
                    image_name: image_name,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ImageBuilder {
    type Properties = ImageBuilderProperties;
    const TYPE: &'static str = "AWS::AppStream::ImageBuilder";
    fn properties(&self) -> &ImageBuilderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImageBuilderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ImageBuilder {}

impl From<ImageBuilderProperties> for ImageBuilder {
    fn from(properties: ImageBuilderProperties) -> ImageBuilder {
        ImageBuilder { properties }
    }
}

/// The [`AWS::AppStream::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html) resource type.
#[derive(Debug, Default)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Default)]
pub struct StackProperties {
    /// Property [`AccessEndpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-accessendpoints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_endpoints: Option<::ValueList<self::stack::AccessEndpoint>>,
    /// Property [`ApplicationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-applicationsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_settings: Option<::Value<self::stack::ApplicationSettings>>,
    /// Property [`AttributesToDelete`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-attributestodelete).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes_to_delete: Option<::ValueList<String>>,
    /// Property [`DeleteStorageConnectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-deletestorageconnectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_storage_connectors: Option<::Value<bool>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`EmbedHostDomains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-embedhostdomains).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub embed_host_domains: Option<::ValueList<String>>,
    /// Property [`FeedbackURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-feedbackurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub feedback_url: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RedirectURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-redirecturl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redirect_url: Option<::Value<String>>,
    /// Property [`StorageConnectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-storageconnectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_connectors: Option<::ValueList<self::stack::StorageConnector>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-usersettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_settings: Option<::ValueList<self::stack::UserSetting>>,
}

impl ::serde::Serialize for StackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_endpoints) = self.access_endpoints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessEndpoints", access_endpoints)?;
        }
        if let Some(ref application_settings) = self.application_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationSettings", application_settings)?;
        }
        if let Some(ref attributes_to_delete) = self.attributes_to_delete {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributesToDelete", attributes_to_delete)?;
        }
        if let Some(ref delete_storage_connectors) = self.delete_storage_connectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteStorageConnectors", delete_storage_connectors)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref embed_host_domains) = self.embed_host_domains {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmbedHostDomains", embed_host_domains)?;
        }
        if let Some(ref feedback_url) = self.feedback_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeedbackURL", feedback_url)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref redirect_url) = self.redirect_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectURL", redirect_url)?;
        }
        if let Some(ref storage_connectors) = self.storage_connectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageConnectors", storage_connectors)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_settings) = self.user_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSettings", user_settings)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_endpoints: Option<::ValueList<self::stack::AccessEndpoint>> = None;
                let mut application_settings: Option<::Value<self::stack::ApplicationSettings>> = None;
                let mut attributes_to_delete: Option<::ValueList<String>> = None;
                let mut delete_storage_connectors: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut embed_host_domains: Option<::ValueList<String>> = None;
                let mut feedback_url: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut redirect_url: Option<::Value<String>> = None;
                let mut storage_connectors: Option<::ValueList<self::stack::StorageConnector>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_settings: Option<::ValueList<self::stack::UserSetting>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessEndpoints" => {
                            access_endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationSettings" => {
                            application_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AttributesToDelete" => {
                            attributes_to_delete = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeleteStorageConnectors" => {
                            delete_storage_connectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EmbedHostDomains" => {
                            embed_host_domains = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FeedbackURL" => {
                            feedback_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedirectURL" => {
                            redirect_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageConnectors" => {
                            storage_connectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserSettings" => {
                            user_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackProperties {
                    access_endpoints: access_endpoints,
                    application_settings: application_settings,
                    attributes_to_delete: attributes_to_delete,
                    delete_storage_connectors: delete_storage_connectors,
                    description: description,
                    display_name: display_name,
                    embed_host_domains: embed_host_domains,
                    feedback_url: feedback_url,
                    name: name,
                    redirect_url: redirect_url,
                    storage_connectors: storage_connectors,
                    tags: tags,
                    user_settings: user_settings,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Stack {
    type Properties = StackProperties;
    const TYPE: &'static str = "AWS::AppStream::Stack";
    fn properties(&self) -> &StackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stack {}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::AppStream::StackFleetAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackfleetassociation.html) resource type.
#[derive(Debug, Default)]
pub struct StackFleetAssociation {
    properties: StackFleetAssociationProperties
}

/// Properties for the `StackFleetAssociation` resource.
#[derive(Debug, Default)]
pub struct StackFleetAssociationProperties {
    /// Property [`FleetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackfleetassociation.html#cfn-appstream-stackfleetassociation-fleetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fleet_name: ::Value<String>,
    /// Property [`StackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackfleetassociation.html#cfn-appstream-stackfleetassociation-stackname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stack_name: ::Value<String>,
}

impl ::serde::Serialize for StackFleetAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetName", &self.fleet_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackName", &self.stack_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackFleetAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackFleetAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackFleetAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackFleetAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fleet_name: Option<::Value<String>> = None;
                let mut stack_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FleetName" => {
                            fleet_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackName" => {
                            stack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackFleetAssociationProperties {
                    fleet_name: fleet_name.ok_or(::serde::de::Error::missing_field("FleetName"))?,
                    stack_name: stack_name.ok_or(::serde::de::Error::missing_field("StackName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StackFleetAssociation {
    type Properties = StackFleetAssociationProperties;
    const TYPE: &'static str = "AWS::AppStream::StackFleetAssociation";
    fn properties(&self) -> &StackFleetAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackFleetAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StackFleetAssociation {}

impl From<StackFleetAssociationProperties> for StackFleetAssociation {
    fn from(properties: StackFleetAssociationProperties) -> StackFleetAssociation {
        StackFleetAssociation { properties }
    }
}

/// The [`AWS::AppStream::StackUserAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html) resource type.
#[derive(Debug, Default)]
pub struct StackUserAssociation {
    properties: StackUserAssociationProperties
}

/// Properties for the `StackUserAssociation` resource.
#[derive(Debug, Default)]
pub struct StackUserAssociationProperties {
    /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html#cfn-appstream-stackuserassociation-authenticationtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authentication_type: ::Value<String>,
    /// Property [`SendEmailNotification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html#cfn-appstream-stackuserassociation-sendemailnotification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub send_email_notification: Option<::Value<bool>>,
    /// Property [`StackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html#cfn-appstream-stackuserassociation-stackname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_name: ::Value<String>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html#cfn-appstream-stackuserassociation-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for StackUserAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
        if let Some(ref send_email_notification) = self.send_email_notification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendEmailNotification", send_email_notification)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackName", &self.stack_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackUserAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackUserAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackUserAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackUserAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authentication_type: Option<::Value<String>> = None;
                let mut send_email_notification: Option<::Value<bool>> = None;
                let mut stack_name: Option<::Value<String>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthenticationType" => {
                            authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SendEmailNotification" => {
                            send_email_notification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackName" => {
                            stack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackUserAssociationProperties {
                    authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                    send_email_notification: send_email_notification,
                    stack_name: stack_name.ok_or(::serde::de::Error::missing_field("StackName"))?,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StackUserAssociation {
    type Properties = StackUserAssociationProperties;
    const TYPE: &'static str = "AWS::AppStream::StackUserAssociation";
    fn properties(&self) -> &StackUserAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackUserAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StackUserAssociation {}

impl From<StackUserAssociationProperties> for StackUserAssociation {
    fn from(properties: StackUserAssociationProperties) -> StackUserAssociation {
        StackUserAssociation { properties }
    }
}

/// The [`AWS::AppStream::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html#cfn-appstream-user-authenticationtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authentication_type: ::Value<String>,
    /// Property [`FirstName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html#cfn-appstream-user-firstname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub first_name: Option<::Value<String>>,
    /// Property [`LastName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html#cfn-appstream-user-lastname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub last_name: Option<::Value<String>>,
    /// Property [`MessageAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html#cfn-appstream-user-messageaction).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub message_action: Option<::Value<String>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html#cfn-appstream-user-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
        if let Some(ref first_name) = self.first_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstName", first_name)?;
        }
        if let Some(ref last_name) = self.last_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastName", last_name)?;
        }
        if let Some(ref message_action) = self.message_action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageAction", message_action)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authentication_type: Option<::Value<String>> = None;
                let mut first_name: Option<::Value<String>> = None;
                let mut last_name: Option<::Value<String>> = None;
                let mut message_action: Option<::Value<String>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthenticationType" => {
                            authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirstName" => {
                            first_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastName" => {
                            last_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MessageAction" => {
                            message_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                    first_name: first_name,
                    last_name: last_name,
                    message_action: message_action,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::AppStream::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

pub mod directory_config {
    //! Property types for the `DirectoryConfig` resource.

    /// The [`AWS::AppStream::DirectoryConfig.ServiceAccountCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-serviceaccountcredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceAccountCredentials {
        /// Property [`AccountName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-serviceaccountcredentials.html#cfn-appstream-directoryconfig-serviceaccountcredentials-accountname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_name: ::Value<String>,
        /// Property [`AccountPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-serviceaccountcredentials.html#cfn-appstream-directoryconfig-serviceaccountcredentials-accountpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_password: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceAccountCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountName", &self.account_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountPassword", &self.account_password)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceAccountCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceAccountCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceAccountCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceAccountCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_name: Option<::Value<String>> = None;
                    let mut account_password: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountName" => {
                                account_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccountPassword" => {
                                account_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceAccountCredentials {
                        account_name: account_name.ok_or(::serde::de::Error::missing_field("AccountName"))?,
                        account_password: account_password.ok_or(::serde::de::Error::missing_field("AccountPassword"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fleet {
    //! Property types for the `Fleet` resource.

    /// The [`AWS::AppStream::Fleet.ComputeCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-computecapacity.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputeCapacity {
        /// Property [`DesiredInstances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-computecapacity.html#cfn-appstream-fleet-computecapacity-desiredinstances).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_instances: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ComputeCapacity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredInstances", &self.desired_instances)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputeCapacity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeCapacity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeCapacity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeCapacity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut desired_instances: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredInstances" => {
                                desired_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeCapacity {
                        desired_instances: desired_instances.ok_or(::serde::de::Error::missing_field("DesiredInstances"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::Fleet.DomainJoinInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-domainjoininfo.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainJoinInfo {
        /// Property [`DirectoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-domainjoininfo.html#cfn-appstream-fleet-domainjoininfo-directoryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_name: Option<::Value<String>>,
        /// Property [`OrganizationalUnitDistinguishedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-domainjoininfo.html#cfn-appstream-fleet-domainjoininfo-organizationalunitdistinguishedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organizational_unit_distinguished_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DomainJoinInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_name) = self.directory_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryName", directory_name)?;
            }
            if let Some(ref organizational_unit_distinguished_name) = self.organizational_unit_distinguished_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedName", organizational_unit_distinguished_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainJoinInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainJoinInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainJoinInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainJoinInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_name: Option<::Value<String>> = None;
                    let mut organizational_unit_distinguished_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryName" => {
                                directory_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitDistinguishedName" => {
                                organizational_unit_distinguished_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainJoinInfo {
                        directory_name: directory_name,
                        organizational_unit_distinguished_name: organizational_unit_distinguished_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::Fleet.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-vpcconfig.html#cfn-appstream-fleet-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-vpcconfig.html#cfn-appstream-fleet-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod image_builder {
    //! Property types for the `ImageBuilder` resource.

    /// The [`AWS::AppStream::ImageBuilder.AccessEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-accessendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessEndpoint {
        /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-accessendpoint.html#cfn-appstream-imagebuilder-accessendpoint-endpointtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_type: ::Value<String>,
        /// Property [`VpceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-accessendpoint.html#cfn-appstream-imagebuilder-accessendpoint-vpceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpce_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AccessEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", &self.endpoint_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpceId", &self.vpce_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_type: Option<::Value<String>> = None;
                    let mut vpce_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointType" => {
                                endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpceId" => {
                                vpce_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessEndpoint {
                        endpoint_type: endpoint_type.ok_or(::serde::de::Error::missing_field("EndpointType"))?,
                        vpce_id: vpce_id.ok_or(::serde::de::Error::missing_field("VpceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::ImageBuilder.DomainJoinInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-domainjoininfo.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainJoinInfo {
        /// Property [`DirectoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-domainjoininfo.html#cfn-appstream-imagebuilder-domainjoininfo-directoryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_name: Option<::Value<String>>,
        /// Property [`OrganizationalUnitDistinguishedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-domainjoininfo.html#cfn-appstream-imagebuilder-domainjoininfo-organizationalunitdistinguishedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organizational_unit_distinguished_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DomainJoinInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_name) = self.directory_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryName", directory_name)?;
            }
            if let Some(ref organizational_unit_distinguished_name) = self.organizational_unit_distinguished_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedName", organizational_unit_distinguished_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainJoinInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainJoinInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainJoinInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainJoinInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_name: Option<::Value<String>> = None;
                    let mut organizational_unit_distinguished_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryName" => {
                                directory_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitDistinguishedName" => {
                                organizational_unit_distinguished_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainJoinInfo {
                        directory_name: directory_name,
                        organizational_unit_distinguished_name: organizational_unit_distinguished_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::ImageBuilder.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-vpcconfig.html#cfn-appstream-imagebuilder-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-vpcconfig.html#cfn-appstream-imagebuilder-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod stack {
    //! Property types for the `Stack` resource.

    /// The [`AWS::AppStream::Stack.AccessEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-accessendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessEndpoint {
        /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-accessendpoint.html#cfn-appstream-stack-accessendpoint-endpointtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_type: ::Value<String>,
        /// Property [`VpceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-accessendpoint.html#cfn-appstream-stack-accessendpoint-vpceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpce_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AccessEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", &self.endpoint_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpceId", &self.vpce_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_type: Option<::Value<String>> = None;
                    let mut vpce_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointType" => {
                                endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpceId" => {
                                vpce_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessEndpoint {
                        endpoint_type: endpoint_type.ok_or(::serde::de::Error::missing_field("EndpointType"))?,
                        vpce_id: vpce_id.ok_or(::serde::de::Error::missing_field("VpceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::Stack.ApplicationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-applicationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationSettings {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-applicationsettings.html#cfn-appstream-stack-applicationsettings-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`SettingsGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-applicationsettings.html#cfn-appstream-stack-applicationsettings-settingsgroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub settings_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApplicationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref settings_group) = self.settings_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SettingsGroup", settings_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut settings_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SettingsGroup" => {
                                settings_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationSettings {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        settings_group: settings_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::Stack.StorageConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-storageconnector.html) property type.
    #[derive(Debug, Default)]
    pub struct StorageConnector {
        /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-storageconnector.html#cfn-appstream-stack-storageconnector-connectortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_type: ::Value<String>,
        /// Property [`Domains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-storageconnector.html#cfn-appstream-stack-storageconnector-domains).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domains: Option<::ValueList<String>>,
        /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-storageconnector.html#cfn-appstream-stack-storageconnector-resourceidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StorageConnector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
            if let Some(ref domains) = self.domains {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domains", domains)?;
            }
            if let Some(ref resource_identifier) = self.resource_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StorageConnector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageConnector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StorageConnector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StorageConnector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_type: Option<::Value<String>> = None;
                    let mut domains: Option<::ValueList<String>> = None;
                    let mut resource_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorType" => {
                                connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Domains" => {
                                domains = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceIdentifier" => {
                                resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StorageConnector {
                        connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                        domains: domains,
                        resource_identifier: resource_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::Stack.UserSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-usersetting.html) property type.
    #[derive(Debug, Default)]
    pub struct UserSetting {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-usersetting.html#cfn-appstream-stack-usersetting-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
        /// Property [`Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-usersetting.html#cfn-appstream-stack-usersetting-permission).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub permission: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", &self.permission)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut permission: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserSetting {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        permission: permission.ok_or(::serde::de::Error::missing_field("Permission"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
