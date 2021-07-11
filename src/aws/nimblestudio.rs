//! Types for the `NimbleStudio` service.

/// The [`AWS::NimbleStudio::LaunchProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html) resource type.
#[derive(Debug, Default)]
pub struct LaunchProfile {
    properties: LaunchProfileProperties
}

/// Properties for the `LaunchProfile` resource.
#[derive(Debug, Default)]
pub struct LaunchProfileProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Ec2SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-ec2subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_subnet_ids: ::ValueList<String>,
    /// Property [`LaunchProfileProtocolVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-launchprofileprotocolversions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_profile_protocol_versions: ::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`StreamConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-streamconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_configuration: ::Value<self::launch_profile::StreamConfiguration>,
    /// Property [`StudioComponentIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-studiocomponentids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub studio_component_ids: ::ValueList<String>,
    /// Property [`StudioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-studioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub studio_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-launchprofile.html#cfn-nimblestudio-launchprofile-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for LaunchProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2SubnetIds", &self.ec2_subnet_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchProfileProtocolVersions", &self.launch_profile_protocol_versions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamConfiguration", &self.stream_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioComponentIds", &self.studio_component_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioId", &self.studio_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut ec2_subnet_ids: Option<::ValueList<String>> = None;
                let mut launch_profile_protocol_versions: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut stream_configuration: Option<::Value<self::launch_profile::StreamConfiguration>> = None;
                let mut studio_component_ids: Option<::ValueList<String>> = None;
                let mut studio_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ec2SubnetIds" => {
                            ec2_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchProfileProtocolVersions" => {
                            launch_profile_protocol_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamConfiguration" => {
                            stream_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioComponentIds" => {
                            studio_component_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioId" => {
                            studio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchProfileProperties {
                    description: description,
                    ec2_subnet_ids: ec2_subnet_ids.ok_or(::serde::de::Error::missing_field("Ec2SubnetIds"))?,
                    launch_profile_protocol_versions: launch_profile_protocol_versions.ok_or(::serde::de::Error::missing_field("LaunchProfileProtocolVersions"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    stream_configuration: stream_configuration.ok_or(::serde::de::Error::missing_field("StreamConfiguration"))?,
                    studio_component_ids: studio_component_ids.ok_or(::serde::de::Error::missing_field("StudioComponentIds"))?,
                    studio_id: studio_id.ok_or(::serde::de::Error::missing_field("StudioId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LaunchProfile {
    type Properties = LaunchProfileProperties;
    const TYPE: &'static str = "AWS::NimbleStudio::LaunchProfile";
    fn properties(&self) -> &LaunchProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LaunchProfile {}

impl From<LaunchProfileProperties> for LaunchProfile {
    fn from(properties: LaunchProfileProperties) -> LaunchProfile {
        LaunchProfile { properties }
    }
}

/// The [`AWS::NimbleStudio::StreamingImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html) resource type.
#[derive(Debug, Default)]
pub struct StreamingImage {
    properties: StreamingImageProperties
}

/// Properties for the `StreamingImage` resource.
#[derive(Debug, Default)]
pub struct StreamingImageProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html#cfn-nimblestudio-streamingimage-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Ec2ImageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html#cfn-nimblestudio-streamingimage-ec2imageid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_image_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html#cfn-nimblestudio-streamingimage-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`StudioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html#cfn-nimblestudio-streamingimage-studioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub studio_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-streamingimage.html#cfn-nimblestudio-streamingimage-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for StreamingImageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2ImageId", &self.ec2_image_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioId", &self.studio_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamingImageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingImageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamingImageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamingImageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut ec2_image_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut studio_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ec2ImageId" => {
                            ec2_image_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioId" => {
                            studio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamingImageProperties {
                    description: description,
                    ec2_image_id: ec2_image_id.ok_or(::serde::de::Error::missing_field("Ec2ImageId"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    studio_id: studio_id.ok_or(::serde::de::Error::missing_field("StudioId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StreamingImage {
    type Properties = StreamingImageProperties;
    const TYPE: &'static str = "AWS::NimbleStudio::StreamingImage";
    fn properties(&self) -> &StreamingImageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamingImageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StreamingImage {}

impl From<StreamingImageProperties> for StreamingImage {
    fn from(properties: StreamingImageProperties) -> StreamingImage {
        StreamingImage { properties }
    }
}

/// The [`AWS::NimbleStudio::Studio`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html) resource type.
#[derive(Debug, Default)]
pub struct Studio {
    properties: StudioProperties
}

/// Properties for the `Studio` resource.
#[derive(Debug, Default)]
pub struct StudioProperties {
    /// Property [`AdminRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-adminrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_role_arn: ::Value<String>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: ::Value<String>,
    /// Property [`StudioEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-studioencryptionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub studio_encryption_configuration: Option<::Value<self::studio::StudioEncryptionConfiguration>>,
    /// Property [`StudioName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-studioname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub studio_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`UserRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studio.html#cfn-nimblestudio-studio-userrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_role_arn: ::Value<String>,
}

impl ::serde::Serialize for StudioProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminRoleArn", &self.admin_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
        if let Some(ref studio_encryption_configuration) = self.studio_encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioEncryptionConfiguration", studio_encryption_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioName", &self.studio_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserRoleArn", &self.user_role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StudioProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StudioProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StudioProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StudioProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut admin_role_arn: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut studio_encryption_configuration: Option<::Value<self::studio::StudioEncryptionConfiguration>> = None;
                let mut studio_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut user_role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdminRoleArn" => {
                            admin_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioEncryptionConfiguration" => {
                            studio_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioName" => {
                            studio_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserRoleArn" => {
                            user_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StudioProperties {
                    admin_role_arn: admin_role_arn.ok_or(::serde::de::Error::missing_field("AdminRoleArn"))?,
                    display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                    studio_encryption_configuration: studio_encryption_configuration,
                    studio_name: studio_name.ok_or(::serde::de::Error::missing_field("StudioName"))?,
                    tags: tags,
                    user_role_arn: user_role_arn.ok_or(::serde::de::Error::missing_field("UserRoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Studio {
    type Properties = StudioProperties;
    const TYPE: &'static str = "AWS::NimbleStudio::Studio";
    fn properties(&self) -> &StudioProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StudioProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Studio {}

impl From<StudioProperties> for Studio {
    fn from(properties: StudioProperties) -> Studio {
        Studio { properties }
    }
}

/// The [`AWS::NimbleStudio::StudioComponent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html) resource type.
#[derive(Debug, Default)]
pub struct StudioComponent {
    properties: StudioComponentProperties
}

/// Properties for the `StudioComponent` resource.
#[derive(Debug, Default)]
pub struct StudioComponentProperties {
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<self::studio_component::StudioComponentConfiguration>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Ec2SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-ec2securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_ids: Option<::ValueList<String>>,
    /// Property [`InitializationScripts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-initializationscripts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub initialization_scripts: Option<::ValueList<self::studio_component::StudioComponentInitializationScript>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ScriptParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-scriptparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub script_parameters: Option<::ValueList<self::studio_component::ScriptParameterKeyValue>>,
    /// Property [`StudioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-studioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub studio_id: ::Value<String>,
    /// Property [`Subtype`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-subtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subtype: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-nimblestudio-studiocomponent.html#cfn-nimblestudio-studiocomponent-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for StudioComponentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref ec2_security_group_ids) = self.ec2_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2SecurityGroupIds", ec2_security_group_ids)?;
        }
        if let Some(ref initialization_scripts) = self.initialization_scripts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitializationScripts", initialization_scripts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref script_parameters) = self.script_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScriptParameters", script_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StudioId", &self.studio_id)?;
        if let Some(ref subtype) = self.subtype {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subtype", subtype)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StudioComponentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StudioComponentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StudioComponentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StudioComponentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration: Option<::Value<self::studio_component::StudioComponentConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut ec2_security_group_ids: Option<::ValueList<String>> = None;
                let mut initialization_scripts: Option<::ValueList<self::studio_component::StudioComponentInitializationScript>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut script_parameters: Option<::ValueList<self::studio_component::ScriptParameterKeyValue>> = None;
                let mut studio_id: Option<::Value<String>> = None;
                let mut subtype: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ec2SecurityGroupIds" => {
                            ec2_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InitializationScripts" => {
                            initialization_scripts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScriptParameters" => {
                            script_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StudioId" => {
                            studio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subtype" => {
                            subtype = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StudioComponentProperties {
                    configuration: configuration,
                    description: description,
                    ec2_security_group_ids: ec2_security_group_ids,
                    initialization_scripts: initialization_scripts,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    script_parameters: script_parameters,
                    studio_id: studio_id.ok_or(::serde::de::Error::missing_field("StudioId"))?,
                    subtype: subtype,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StudioComponent {
    type Properties = StudioComponentProperties;
    const TYPE: &'static str = "AWS::NimbleStudio::StudioComponent";
    fn properties(&self) -> &StudioComponentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StudioComponentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StudioComponent {}

impl From<StudioComponentProperties> for StudioComponent {
    fn from(properties: StudioComponentProperties) -> StudioComponent {
        StudioComponent { properties }
    }
}

pub mod launch_profile {
    //! Property types for the `LaunchProfile` resource.

    /// The [`AWS::NimbleStudio::LaunchProfile.StreamConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-launchprofile-streamconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamConfiguration {
        /// Property [`ClipboardMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-launchprofile-streamconfiguration.html#cfn-nimblestudio-launchprofile-streamconfiguration-clipboardmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clipboard_mode: ::Value<String>,
        /// Property [`Ec2InstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-launchprofile-streamconfiguration.html#cfn-nimblestudio-launchprofile-streamconfiguration-ec2instancetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ec2_instance_types: ::ValueList<String>,
        /// Property [`MaxSessionLengthInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-launchprofile-streamconfiguration.html#cfn-nimblestudio-launchprofile-streamconfiguration-maxsessionlengthinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_session_length_in_minutes: Option<::Value<f64>>,
        /// Property [`StreamingImageIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-launchprofile-streamconfiguration.html#cfn-nimblestudio-launchprofile-streamconfiguration-streamingimageids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub streaming_image_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for StreamConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClipboardMode", &self.clipboard_mode)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2InstanceTypes", &self.ec2_instance_types)?;
            if let Some(ref max_session_length_in_minutes) = self.max_session_length_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSessionLengthInMinutes", max_session_length_in_minutes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingImageIds", &self.streaming_image_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut clipboard_mode: Option<::Value<String>> = None;
                    let mut ec2_instance_types: Option<::ValueList<String>> = None;
                    let mut max_session_length_in_minutes: Option<::Value<f64>> = None;
                    let mut streaming_image_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClipboardMode" => {
                                clipboard_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2InstanceTypes" => {
                                ec2_instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSessionLengthInMinutes" => {
                                max_session_length_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamingImageIds" => {
                                streaming_image_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamConfiguration {
                        clipboard_mode: clipboard_mode.ok_or(::serde::de::Error::missing_field("ClipboardMode"))?,
                        ec2_instance_types: ec2_instance_types.ok_or(::serde::de::Error::missing_field("Ec2InstanceTypes"))?,
                        max_session_length_in_minutes: max_session_length_in_minutes,
                        streaming_image_ids: streaming_image_ids.ok_or(::serde::de::Error::missing_field("StreamingImageIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod studio {
    //! Property types for the `Studio` resource.

    /// The [`AWS::NimbleStudio::Studio.StudioEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studio-studioencryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct StudioEncryptionConfiguration {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studio-studioencryptionconfiguration.html#cfn-nimblestudio-studio-studioencryptionconfiguration-keyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studio-studioencryptionconfiguration.html#cfn-nimblestudio-studio-studioencryptionconfiguration-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for StudioEncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", &self.key_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StudioEncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StudioEncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StudioEncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StudioEncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StudioEncryptionConfiguration {
                        key_arn: key_arn,
                        key_type: key_type.ok_or(::serde::de::Error::missing_field("KeyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod studio_component {
    //! Property types for the `StudioComponent` resource.

    /// The [`AWS::NimbleStudio::StudioComponent.ActiveDirectoryComputerAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectorycomputerattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct ActiveDirectoryComputerAttribute {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectorycomputerattribute.html#cfn-nimblestudio-studiocomponent-activedirectorycomputerattribute-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectorycomputerattribute.html#cfn-nimblestudio-studiocomponent-activedirectorycomputerattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ActiveDirectoryComputerAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActiveDirectoryComputerAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActiveDirectoryComputerAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActiveDirectoryComputerAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActiveDirectoryComputerAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActiveDirectoryComputerAttribute {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.ActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectoryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ActiveDirectoryConfiguration {
        /// Property [`ComputerAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectoryconfiguration.html#cfn-nimblestudio-studiocomponent-activedirectoryconfiguration-computerattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub computer_attributes: Option<::ValueList<ActiveDirectoryComputerAttribute>>,
        /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectoryconfiguration.html#cfn-nimblestudio-studiocomponent-activedirectoryconfiguration-directoryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_id: Option<::Value<String>>,
        /// Property [`OrganizationalUnitDistinguishedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-activedirectoryconfiguration.html#cfn-nimblestudio-studiocomponent-activedirectoryconfiguration-organizationalunitdistinguishedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organizational_unit_distinguished_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ActiveDirectoryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref computer_attributes) = self.computer_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputerAttributes", computer_attributes)?;
            }
            if let Some(ref directory_id) = self.directory_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", directory_id)?;
            }
            if let Some(ref organizational_unit_distinguished_name) = self.organizational_unit_distinguished_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedName", organizational_unit_distinguished_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActiveDirectoryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActiveDirectoryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActiveDirectoryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActiveDirectoryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut computer_attributes: Option<::ValueList<ActiveDirectoryComputerAttribute>> = None;
                    let mut directory_id: Option<::Value<String>> = None;
                    let mut organizational_unit_distinguished_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputerAttributes" => {
                                computer_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DirectoryId" => {
                                directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitDistinguishedName" => {
                                organizational_unit_distinguished_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActiveDirectoryConfiguration {
                        computer_attributes: computer_attributes,
                        directory_id: directory_id,
                        organizational_unit_distinguished_name: organizational_unit_distinguished_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.ComputeFarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-computefarmconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputeFarmConfiguration {
        /// Property [`ActiveDirectoryUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-computefarmconfiguration.html#cfn-nimblestudio-studiocomponent-computefarmconfiguration-activedirectoryuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub active_directory_user: Option<::Value<String>>,
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-computefarmconfiguration.html#cfn-nimblestudio-studiocomponent-computefarmconfiguration-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComputeFarmConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref active_directory_user) = self.active_directory_user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveDirectoryUser", active_directory_user)?;
            }
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputeFarmConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeFarmConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeFarmConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeFarmConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active_directory_user: Option<::Value<String>> = None;
                    let mut endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActiveDirectoryUser" => {
                                active_directory_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeFarmConfiguration {
                        active_directory_user: active_directory_user,
                        endpoint: endpoint,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.LicenseServiceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-licenseserviceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LicenseServiceConfiguration {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-licenseserviceconfiguration.html#cfn-nimblestudio-studiocomponent-licenseserviceconfiguration-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LicenseServiceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LicenseServiceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LicenseServiceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LicenseServiceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LicenseServiceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LicenseServiceConfiguration {
                        endpoint: endpoint,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.ScriptParameterKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-scriptparameterkeyvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct ScriptParameterKeyValue {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-scriptparameterkeyvalue.html#cfn-nimblestudio-studiocomponent-scriptparameterkeyvalue-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-scriptparameterkeyvalue.html#cfn-nimblestudio-studiocomponent-scriptparameterkeyvalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScriptParameterKeyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScriptParameterKeyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScriptParameterKeyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScriptParameterKeyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScriptParameterKeyValue")
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

                    Ok(ScriptParameterKeyValue {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.SharedFileSystemConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SharedFileSystemConfiguration {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html#cfn-nimblestudio-studiocomponent-sharedfilesystemconfiguration-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<String>>,
        /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html#cfn-nimblestudio-studiocomponent-sharedfilesystemconfiguration-filesystemid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_system_id: Option<::Value<String>>,
        /// Property [`LinuxMountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html#cfn-nimblestudio-studiocomponent-sharedfilesystemconfiguration-linuxmountpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub linux_mount_point: Option<::Value<String>>,
        /// Property [`ShareName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html#cfn-nimblestudio-studiocomponent-sharedfilesystemconfiguration-sharename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub share_name: Option<::Value<String>>,
        /// Property [`WindowsMountDrive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-sharedfilesystemconfiguration.html#cfn-nimblestudio-studiocomponent-sharedfilesystemconfiguration-windowsmountdrive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub windows_mount_drive: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SharedFileSystemConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            if let Some(ref file_system_id) = self.file_system_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", file_system_id)?;
            }
            if let Some(ref linux_mount_point) = self.linux_mount_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinuxMountPoint", linux_mount_point)?;
            }
            if let Some(ref share_name) = self.share_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShareName", share_name)?;
            }
            if let Some(ref windows_mount_drive) = self.windows_mount_drive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowsMountDrive", windows_mount_drive)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SharedFileSystemConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SharedFileSystemConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SharedFileSystemConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SharedFileSystemConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut file_system_id: Option<::Value<String>> = None;
                    let mut linux_mount_point: Option<::Value<String>> = None;
                    let mut share_name: Option<::Value<String>> = None;
                    let mut windows_mount_drive: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileSystemId" => {
                                file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinuxMountPoint" => {
                                linux_mount_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShareName" => {
                                share_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WindowsMountDrive" => {
                                windows_mount_drive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SharedFileSystemConfiguration {
                        endpoint: endpoint,
                        file_system_id: file_system_id,
                        linux_mount_point: linux_mount_point,
                        share_name: share_name,
                        windows_mount_drive: windows_mount_drive,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.StudioComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct StudioComponentConfiguration {
        /// Property [`ActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentconfiguration.html#cfn-nimblestudio-studiocomponent-studiocomponentconfiguration-activedirectoryconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub active_directory_configuration: Option<::Value<ActiveDirectoryConfiguration>>,
        /// Property [`ComputeFarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentconfiguration.html#cfn-nimblestudio-studiocomponent-studiocomponentconfiguration-computefarmconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_farm_configuration: Option<::Value<ComputeFarmConfiguration>>,
        /// Property [`LicenseServiceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentconfiguration.html#cfn-nimblestudio-studiocomponent-studiocomponentconfiguration-licenseserviceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub license_service_configuration: Option<::Value<LicenseServiceConfiguration>>,
        /// Property [`SharedFileSystemConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentconfiguration.html#cfn-nimblestudio-studiocomponent-studiocomponentconfiguration-sharedfilesystemconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shared_file_system_configuration: Option<::Value<SharedFileSystemConfiguration>>,
    }

    impl ::codec::SerializeValue for StudioComponentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref active_directory_configuration) = self.active_directory_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveDirectoryConfiguration", active_directory_configuration)?;
            }
            if let Some(ref compute_farm_configuration) = self.compute_farm_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeFarmConfiguration", compute_farm_configuration)?;
            }
            if let Some(ref license_service_configuration) = self.license_service_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseServiceConfiguration", license_service_configuration)?;
            }
            if let Some(ref shared_file_system_configuration) = self.shared_file_system_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharedFileSystemConfiguration", shared_file_system_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StudioComponentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StudioComponentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StudioComponentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StudioComponentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active_directory_configuration: Option<::Value<ActiveDirectoryConfiguration>> = None;
                    let mut compute_farm_configuration: Option<::Value<ComputeFarmConfiguration>> = None;
                    let mut license_service_configuration: Option<::Value<LicenseServiceConfiguration>> = None;
                    let mut shared_file_system_configuration: Option<::Value<SharedFileSystemConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActiveDirectoryConfiguration" => {
                                active_directory_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComputeFarmConfiguration" => {
                                compute_farm_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LicenseServiceConfiguration" => {
                                license_service_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharedFileSystemConfiguration" => {
                                shared_file_system_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StudioComponentConfiguration {
                        active_directory_configuration: active_directory_configuration,
                        compute_farm_configuration: compute_farm_configuration,
                        license_service_configuration: license_service_configuration,
                        shared_file_system_configuration: shared_file_system_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NimbleStudio::StudioComponent.StudioComponentInitializationScript`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentinitializationscript.html) property type.
    #[derive(Debug, Default)]
    pub struct StudioComponentInitializationScript {
        /// Property [`LaunchProfileProtocolVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentinitializationscript.html#cfn-nimblestudio-studiocomponent-studiocomponentinitializationscript-launchprofileprotocolversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_profile_protocol_version: Option<::Value<String>>,
        /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentinitializationscript.html#cfn-nimblestudio-studiocomponent-studiocomponentinitializationscript-platform).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform: Option<::Value<String>>,
        /// Property [`RunContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentinitializationscript.html#cfn-nimblestudio-studiocomponent-studiocomponentinitializationscript-runcontext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub run_context: Option<::Value<String>>,
        /// Property [`Script`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-nimblestudio-studiocomponent-studiocomponentinitializationscript.html#cfn-nimblestudio-studiocomponent-studiocomponentinitializationscript-script).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub script: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StudioComponentInitializationScript {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref launch_profile_protocol_version) = self.launch_profile_protocol_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchProfileProtocolVersion", launch_profile_protocol_version)?;
            }
            if let Some(ref platform) = self.platform {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", platform)?;
            }
            if let Some(ref run_context) = self.run_context {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunContext", run_context)?;
            }
            if let Some(ref script) = self.script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Script", script)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StudioComponentInitializationScript {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StudioComponentInitializationScript, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StudioComponentInitializationScript;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StudioComponentInitializationScript")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut launch_profile_protocol_version: Option<::Value<String>> = None;
                    let mut platform: Option<::Value<String>> = None;
                    let mut run_context: Option<::Value<String>> = None;
                    let mut script: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LaunchProfileProtocolVersion" => {
                                launch_profile_protocol_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Platform" => {
                                platform = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunContext" => {
                                run_context = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Script" => {
                                script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StudioComponentInitializationScript {
                        launch_profile_protocol_version: launch_profile_protocol_version,
                        platform: platform,
                        run_context: run_context,
                        script: script,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
