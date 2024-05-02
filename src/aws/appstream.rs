//! Types for the `AppStream` service.

/// The [`AWS::AppStream::AppBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html) resource type.
#[derive(Debug, Default)]
pub struct AppBlock {
    properties: AppBlockProperties
}

/// Properties for the `AppBlock` resource.
#[derive(Debug, Default)]
pub struct AppBlockProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-displayname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PackagingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-packagingtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub packaging_type: Option<::Value<String>>,
    /// Property [`PostSetupScriptDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-postsetupscriptdetails).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub post_setup_script_details: Option<::Value<self::app_block::ScriptDetails>>,
    /// Property [`SetupScriptDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-setupscriptdetails).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub setup_script_details: Option<::Value<self::app_block::ScriptDetails>>,
    /// Property [`SourceS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-sources3location).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_s3_location: ::Value<self::app_block::S3Location>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html#cfn-appstream-appblock-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AppBlockProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref packaging_type) = self.packaging_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackagingType", packaging_type)?;
        }
        if let Some(ref post_setup_script_details) = self.post_setup_script_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostSetupScriptDetails", post_setup_script_details)?;
        }
        if let Some(ref setup_script_details) = self.setup_script_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetupScriptDetails", setup_script_details)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceS3Location", &self.source_s3_location)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppBlockProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppBlockProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppBlockProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppBlockProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut packaging_type: Option<::Value<String>> = None;
                let mut post_setup_script_details: Option<::Value<self::app_block::ScriptDetails>> = None;
                let mut setup_script_details: Option<::Value<self::app_block::ScriptDetails>> = None;
                let mut source_s3_location: Option<::Value<self::app_block::S3Location>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackagingType" => {
                            packaging_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PostSetupScriptDetails" => {
                            post_setup_script_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SetupScriptDetails" => {
                            setup_script_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceS3Location" => {
                            source_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppBlockProperties {
                    description: description,
                    display_name: display_name,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    packaging_type: packaging_type,
                    post_setup_script_details: post_setup_script_details,
                    setup_script_details: setup_script_details,
                    source_s3_location: source_s3_location.ok_or(::serde::de::Error::missing_field("SourceS3Location"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AppBlock {
    type Properties = AppBlockProperties;
    const TYPE: &'static str = "AWS::AppStream::AppBlock";
    fn properties(&self) -> &AppBlockProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppBlockProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AppBlock {}

impl From<AppBlockProperties> for AppBlock {
    fn from(properties: AppBlockProperties) -> AppBlock {
        AppBlock { properties }
    }
}

/// The [`AWS::AppStream::AppBlockBuilder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html) resource type.
#[derive(Debug, Default)]
pub struct AppBlockBuilder {
    properties: AppBlockBuilderProperties
}

/// Properties for the `AppBlockBuilder` resource.
#[derive(Debug, Default)]
pub struct AppBlockBuilderProperties {
    /// Property [`AccessEndpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-accessendpoints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_endpoints: Option<::ValueList<self::app_block_builder::AccessEndpoint>>,
    /// Property [`AppBlockArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-appblockarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_block_arns: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`EnableDefaultInternetAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-enabledefaultinternetaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_default_internet_access: Option<::Value<bool>>,
    /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-iamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role_arn: Option<::Value<String>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-instancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-platform).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub platform: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html#cfn-appstream-appblockbuilder-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: ::Value<self::app_block_builder::VpcConfig>,
}

impl ::serde::Serialize for AppBlockBuilderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_endpoints) = self.access_endpoints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessEndpoints", access_endpoints)?;
        }
        if let Some(ref app_block_arns) = self.app_block_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppBlockArns", app_block_arns)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref enable_default_internet_access) = self.enable_default_internet_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDefaultInternetAccess", enable_default_internet_access)?;
        }
        if let Some(ref iam_role_arn) = self.iam_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", iam_role_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", &self.platform)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", &self.vpc_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppBlockBuilderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppBlockBuilderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppBlockBuilderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppBlockBuilderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_endpoints: Option<::ValueList<self::app_block_builder::AccessEndpoint>> = None;
                let mut app_block_arns: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut enable_default_internet_access: Option<::Value<bool>> = None;
                let mut iam_role_arn: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut platform: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_config: Option<::Value<self::app_block_builder::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessEndpoints" => {
                            access_endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppBlockArns" => {
                            app_block_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableDefaultInternetAccess" => {
                            enable_default_internet_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoleArn" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Platform" => {
                            platform = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(AppBlockBuilderProperties {
                    access_endpoints: access_endpoints,
                    app_block_arns: app_block_arns,
                    description: description,
                    display_name: display_name,
                    enable_default_internet_access: enable_default_internet_access,
                    iam_role_arn: iam_role_arn,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    platform: platform.ok_or(::serde::de::Error::missing_field("Platform"))?,
                    tags: tags,
                    vpc_config: vpc_config.ok_or(::serde::de::Error::missing_field("VpcConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AppBlockBuilder {
    type Properties = AppBlockBuilderProperties;
    const TYPE: &'static str = "AWS::AppStream::AppBlockBuilder";
    fn properties(&self) -> &AppBlockBuilderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppBlockBuilderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AppBlockBuilder {}

impl From<AppBlockBuilderProperties> for AppBlockBuilder {
    fn from(properties: AppBlockBuilderProperties) -> AppBlockBuilder {
        AppBlockBuilder { properties }
    }
}

/// The [`AWS::AppStream::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`AppBlockArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-appblockarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_block_arn: ::Value<String>,
    /// Property [`AttributesToDelete`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-attributestodelete).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes_to_delete: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`IconS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-icons3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub icon_s3_location: ::Value<self::application::S3Location>,
    /// Property [`InstanceFamilies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-instancefamilies).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_families: ::ValueList<String>,
    /// Property [`LaunchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-launchparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_parameters: Option<::Value<String>>,
    /// Property [`LaunchPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-launchpath).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_path: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Platforms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-platforms).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platforms: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html#cfn-appstream-application-workingdirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub working_directory: Option<::Value<String>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppBlockArn", &self.app_block_arn)?;
        if let Some(ref attributes_to_delete) = self.attributes_to_delete {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributesToDelete", attributes_to_delete)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IconS3Location", &self.icon_s3_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceFamilies", &self.instance_families)?;
        if let Some(ref launch_parameters) = self.launch_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchParameters", launch_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchPath", &self.launch_path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platforms", &self.platforms)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref working_directory) = self.working_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_block_arn: Option<::Value<String>> = None;
                let mut attributes_to_delete: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut icon_s3_location: Option<::Value<self::application::S3Location>> = None;
                let mut instance_families: Option<::ValueList<String>> = None;
                let mut launch_parameters: Option<::Value<String>> = None;
                let mut launch_path: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut platforms: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut working_directory: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppBlockArn" => {
                            app_block_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AttributesToDelete" => {
                            attributes_to_delete = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IconS3Location" => {
                            icon_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceFamilies" => {
                            instance_families = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchParameters" => {
                            launch_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchPath" => {
                            launch_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Platforms" => {
                            platforms = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkingDirectory" => {
                            working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    app_block_arn: app_block_arn.ok_or(::serde::de::Error::missing_field("AppBlockArn"))?,
                    attributes_to_delete: attributes_to_delete,
                    description: description,
                    display_name: display_name,
                    icon_s3_location: icon_s3_location.ok_or(::serde::de::Error::missing_field("IconS3Location"))?,
                    instance_families: instance_families.ok_or(::serde::de::Error::missing_field("InstanceFamilies"))?,
                    launch_parameters: launch_parameters,
                    launch_path: launch_path.ok_or(::serde::de::Error::missing_field("LaunchPath"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    platforms: platforms.ok_or(::serde::de::Error::missing_field("Platforms"))?,
                    tags: tags,
                    working_directory: working_directory,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::AppStream::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::AppStream::ApplicationEntitlementAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationentitlementassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationEntitlementAssociation {
    properties: ApplicationEntitlementAssociationProperties
}

/// Properties for the `ApplicationEntitlementAssociation` resource.
#[derive(Debug, Default)]
pub struct ApplicationEntitlementAssociationProperties {
    /// Property [`ApplicationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationentitlementassociation.html#cfn-appstream-applicationentitlementassociation-applicationidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_identifier: ::Value<String>,
    /// Property [`EntitlementName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationentitlementassociation.html#cfn-appstream-applicationentitlementassociation-entitlementname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub entitlement_name: ::Value<String>,
    /// Property [`StackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationentitlementassociation.html#cfn-appstream-applicationentitlementassociation-stackname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_name: ::Value<String>,
}

impl ::serde::Serialize for ApplicationEntitlementAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationIdentifier", &self.application_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntitlementName", &self.entitlement_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackName", &self.stack_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationEntitlementAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationEntitlementAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationEntitlementAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationEntitlementAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_identifier: Option<::Value<String>> = None;
                let mut entitlement_name: Option<::Value<String>> = None;
                let mut stack_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationIdentifier" => {
                            application_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntitlementName" => {
                            entitlement_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackName" => {
                            stack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationEntitlementAssociationProperties {
                    application_identifier: application_identifier.ok_or(::serde::de::Error::missing_field("ApplicationIdentifier"))?,
                    entitlement_name: entitlement_name.ok_or(::serde::de::Error::missing_field("EntitlementName"))?,
                    stack_name: stack_name.ok_or(::serde::de::Error::missing_field("StackName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationEntitlementAssociation {
    type Properties = ApplicationEntitlementAssociationProperties;
    const TYPE: &'static str = "AWS::AppStream::ApplicationEntitlementAssociation";
    fn properties(&self) -> &ApplicationEntitlementAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationEntitlementAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationEntitlementAssociation {}

impl From<ApplicationEntitlementAssociationProperties> for ApplicationEntitlementAssociation {
    fn from(properties: ApplicationEntitlementAssociationProperties) -> ApplicationEntitlementAssociation {
        ApplicationEntitlementAssociation { properties }
    }
}

/// The [`AWS::AppStream::ApplicationFleetAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationfleetassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationFleetAssociation {
    properties: ApplicationFleetAssociationProperties
}

/// Properties for the `ApplicationFleetAssociation` resource.
#[derive(Debug, Default)]
pub struct ApplicationFleetAssociationProperties {
    /// Property [`ApplicationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationfleetassociation.html#cfn-appstream-applicationfleetassociation-applicationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_arn: ::Value<String>,
    /// Property [`FleetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationfleetassociation.html#cfn-appstream-applicationfleetassociation-fleetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fleet_name: ::Value<String>,
}

impl ::serde::Serialize for ApplicationFleetAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationArn", &self.application_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetName", &self.fleet_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationFleetAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationFleetAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationFleetAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationFleetAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_arn: Option<::Value<String>> = None;
                let mut fleet_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationArn" => {
                            application_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FleetName" => {
                            fleet_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationFleetAssociationProperties {
                    application_arn: application_arn.ok_or(::serde::de::Error::missing_field("ApplicationArn"))?,
                    fleet_name: fleet_name.ok_or(::serde::de::Error::missing_field("FleetName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationFleetAssociation {
    type Properties = ApplicationFleetAssociationProperties;
    const TYPE: &'static str = "AWS::AppStream::ApplicationFleetAssociation";
    fn properties(&self) -> &ApplicationFleetAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationFleetAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationFleetAssociation {}

impl From<ApplicationFleetAssociationProperties> for ApplicationFleetAssociation {
    fn from(properties: ApplicationFleetAssociationProperties) -> ApplicationFleetAssociation {
        ApplicationFleetAssociation { properties }
    }
}

/// The [`AWS::AppStream::DirectoryConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html) resource type.
#[derive(Debug, Default)]
pub struct DirectoryConfig {
    properties: DirectoryConfigProperties
}

/// Properties for the `DirectoryConfig` resource.
#[derive(Debug, Default)]
pub struct DirectoryConfigProperties {
    /// Property [`CertificateBasedAuthProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html#cfn-appstream-directoryconfig-certificatebasedauthproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_based_auth_properties: Option<::Value<self::directory_config::CertificateBasedAuthProperties>>,
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
        if let Some(ref certificate_based_auth_properties) = self.certificate_based_auth_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateBasedAuthProperties", certificate_based_auth_properties)?;
        }
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
                let mut certificate_based_auth_properties: Option<::Value<self::directory_config::CertificateBasedAuthProperties>> = None;
                let mut directory_name: Option<::Value<String>> = None;
                let mut organizational_unit_distinguished_names: Option<::ValueList<String>> = None;
                let mut service_account_credentials: Option<::Value<self::directory_config::ServiceAccountCredentials>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateBasedAuthProperties" => {
                            certificate_based_auth_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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
                    certificate_based_auth_properties: certificate_based_auth_properties,
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

/// The [`AWS::AppStream::Entitlement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html) resource type.
#[derive(Debug, Default)]
pub struct Entitlement {
    properties: EntitlementProperties
}

/// Properties for the `Entitlement` resource.
#[derive(Debug, Default)]
pub struct EntitlementProperties {
    /// Property [`AppVisibility`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html#cfn-appstream-entitlement-appvisibility).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_visibility: ::Value<String>,
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html#cfn-appstream-entitlement-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: ::ValueList<self::entitlement::Attribute>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html#cfn-appstream-entitlement-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html#cfn-appstream-entitlement-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`StackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html#cfn-appstream-entitlement-stackname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_name: ::Value<String>,
}

impl ::serde::Serialize for EntitlementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppVisibility", &self.app_visibility)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackName", &self.stack_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EntitlementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EntitlementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EntitlementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EntitlementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_visibility: Option<::Value<String>> = None;
                let mut attributes: Option<::ValueList<self::entitlement::Attribute>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut stack_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppVisibility" => {
                            app_visibility = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackName" => {
                            stack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EntitlementProperties {
                    app_visibility: app_visibility.ok_or(::serde::de::Error::missing_field("AppVisibility"))?,
                    attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    stack_name: stack_name.ok_or(::serde::de::Error::missing_field("StackName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Entitlement {
    type Properties = EntitlementProperties;
    const TYPE: &'static str = "AWS::AppStream::Entitlement";
    fn properties(&self) -> &EntitlementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EntitlementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Entitlement {}

impl From<EntitlementProperties> for Entitlement {
    fn from(properties: EntitlementProperties) -> Entitlement {
        Entitlement { properties }
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
    pub compute_capacity: Option<::Value<self::fleet::ComputeCapacity>>,
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
    /// Property [`MaxConcurrentSessions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-maxconcurrentsessions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_concurrent_sessions: Option<::Value<u32>>,
    /// Property [`MaxSessionsPerInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-maxsessionsperinstance).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_sessions_per_instance: Option<::Value<u32>>,
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
    /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-platform).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub platform: Option<::Value<String>>,
    /// Property [`SessionScriptS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-sessionscripts3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_script_s3_location: Option<::Value<self::fleet::S3Location>>,
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
    /// Property [`UsbDeviceFilterStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-usbdevicefilterstrings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub usb_device_filter_strings: Option<::ValueList<String>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html#cfn-appstream-fleet-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::fleet::VpcConfig>>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compute_capacity) = self.compute_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeCapacity", compute_capacity)?;
        }
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
        if let Some(ref max_concurrent_sessions) = self.max_concurrent_sessions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentSessions", max_concurrent_sessions)?;
        }
        if let Some(ref max_sessions_per_instance) = self.max_sessions_per_instance {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSessionsPerInstance", max_sessions_per_instance)?;
        }
        if let Some(ref max_user_duration_in_seconds) = self.max_user_duration_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUserDurationInSeconds", max_user_duration_in_seconds)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref platform) = self.platform {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", platform)?;
        }
        if let Some(ref session_script_s3_location) = self.session_script_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionScriptS3Location", session_script_s3_location)?;
        }
        if let Some(ref stream_view) = self.stream_view {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamView", stream_view)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref usb_device_filter_strings) = self.usb_device_filter_strings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsbDeviceFilterStrings", usb_device_filter_strings)?;
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
                let mut max_concurrent_sessions: Option<::Value<u32>> = None;
                let mut max_sessions_per_instance: Option<::Value<u32>> = None;
                let mut max_user_duration_in_seconds: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut platform: Option<::Value<String>> = None;
                let mut session_script_s3_location: Option<::Value<self::fleet::S3Location>> = None;
                let mut stream_view: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut usb_device_filter_strings: Option<::ValueList<String>> = None;
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
                        "MaxConcurrentSessions" => {
                            max_concurrent_sessions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSessionsPerInstance" => {
                            max_sessions_per_instance = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxUserDurationInSeconds" => {
                            max_user_duration_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Platform" => {
                            platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SessionScriptS3Location" => {
                            session_script_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamView" => {
                            stream_view = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsbDeviceFilterStrings" => {
                            usb_device_filter_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    compute_capacity: compute_capacity,
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
                    max_concurrent_sessions: max_concurrent_sessions,
                    max_sessions_per_instance: max_sessions_per_instance,
                    max_user_duration_in_seconds: max_user_duration_in_seconds,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    platform: platform,
                    session_script_s3_location: session_script_s3_location,
                    stream_view: stream_view,
                    tags: tags,
                    usb_device_filter_strings: usb_device_filter_strings,
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
    /// Property [`StreamingExperienceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html#cfn-appstream-stack-streamingexperiencesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub streaming_experience_settings: Option<::Value<self::stack::StreamingExperienceSettings>>,
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
        if let Some(ref streaming_experience_settings) = self.streaming_experience_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingExperienceSettings", streaming_experience_settings)?;
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
                let mut streaming_experience_settings: Option<::Value<self::stack::StreamingExperienceSettings>> = None;
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
                        "StreamingExperienceSettings" => {
                            streaming_experience_settings = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    streaming_experience_settings: streaming_experience_settings,
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

pub mod app_block {
    //! Property types for the `AppBlock` resource.

    /// The [`AWS::AppStream::AppBlock.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-s3location.html#cfn-appstream-appblock-s3location-s3bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-s3location.html#cfn-appstream-appblock-s3location-s3key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            if let Some(ref s3_key) = self.s3_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", s3_key)?;
            }
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
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppStream::AppBlock.ScriptDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct ScriptDetails {
        /// Property [`ExecutableParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html#cfn-appstream-appblock-scriptdetails-executableparameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub executable_parameters: Option<::Value<String>>,
        /// Property [`ExecutablePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html#cfn-appstream-appblock-scriptdetails-executablepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub executable_path: ::Value<String>,
        /// Property [`ScriptS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html#cfn-appstream-appblock-scriptdetails-scripts3location).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub script_s3_location: ::Value<S3Location>,
        /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html#cfn-appstream-appblock-scriptdetails-timeoutinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ScriptDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref executable_parameters) = self.executable_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutableParameters", executable_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutablePath", &self.executable_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScriptS3Location", &self.script_s3_location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", &self.timeout_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScriptDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScriptDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScriptDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScriptDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut executable_parameters: Option<::Value<String>> = None;
                    let mut executable_path: Option<::Value<String>> = None;
                    let mut script_s3_location: Option<::Value<S3Location>> = None;
                    let mut timeout_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutableParameters" => {
                                executable_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecutablePath" => {
                                executable_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScriptS3Location" => {
                                script_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInSeconds" => {
                                timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScriptDetails {
                        executable_parameters: executable_parameters,
                        executable_path: executable_path.ok_or(::serde::de::Error::missing_field("ExecutablePath"))?,
                        script_s3_location: script_s3_location.ok_or(::serde::de::Error::missing_field("ScriptS3Location"))?,
                        timeout_in_seconds: timeout_in_seconds.ok_or(::serde::de::Error::missing_field("TimeoutInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod app_block_builder {
    //! Property types for the `AppBlockBuilder` resource.

    /// The [`AWS::AppStream::AppBlockBuilder.AccessEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-accessendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessEndpoint {
        /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-accessendpoint.html#cfn-appstream-appblockbuilder-accessendpoint-endpointtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_type: ::Value<String>,
        /// Property [`VpceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-accessendpoint.html#cfn-appstream-appblockbuilder-accessendpoint-vpceid).
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

    /// The [`AWS::AppStream::AppBlockBuilder.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-vpcconfig.html#cfn-appstream-appblockbuilder-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-vpcconfig.html#cfn-appstream-appblockbuilder-vpcconfig-subnetids).
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

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::AppStream::Application.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-application-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-application-s3location.html#cfn-appstream-application-s3location-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-application-s3location.html#cfn-appstream-application-s3location-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
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
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod directory_config {
    //! Property types for the `DirectoryConfig` resource.

    /// The [`AWS::AppStream::DirectoryConfig.CertificateBasedAuthProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-certificatebasedauthproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct CertificateBasedAuthProperties {
        /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-certificatebasedauthproperties.html#cfn-appstream-directoryconfig-certificatebasedauthproperties-certificateauthorityarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_authority_arn: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-certificatebasedauthproperties.html#cfn-appstream-directoryconfig-certificatebasedauthproperties-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CertificateBasedAuthProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_authority_arn) = self.certificate_authority_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", certificate_authority_arn)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CertificateBasedAuthProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateBasedAuthProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CertificateBasedAuthProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CertificateBasedAuthProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_authority_arn: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateAuthorityArn" => {
                                certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CertificateBasedAuthProperties {
                        certificate_authority_arn: certificate_authority_arn,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

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

pub mod entitlement {
    //! Property types for the `Entitlement` resource.

    /// The [`AWS::AppStream::Entitlement.Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-entitlement-attribute.html) property type.
    #[derive(Debug, Default)]
    pub struct Attribute {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-entitlement-attribute.html#cfn-appstream-entitlement-attribute-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-entitlement-attribute.html#cfn-appstream-entitlement-attribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Attribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Attribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attribute")
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

                    Ok(Attribute {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
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
        pub desired_instances: Option<::Value<u32>>,
        /// Property [`DesiredSessions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-computecapacity.html#cfn-appstream-fleet-computecapacity-desiredsessions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_sessions: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ComputeCapacity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref desired_instances) = self.desired_instances {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredInstances", desired_instances)?;
            }
            if let Some(ref desired_sessions) = self.desired_sessions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredSessions", desired_sessions)?;
            }
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
                    let mut desired_sessions: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredInstances" => {
                                desired_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DesiredSessions" => {
                                desired_sessions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeCapacity {
                        desired_instances: desired_instances,
                        desired_sessions: desired_sessions,
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

    /// The [`AWS::AppStream::Fleet.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-s3location.html#cfn-appstream-fleet-s3location-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-s3location.html#cfn-appstream-fleet-s3location-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
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
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
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

    /// The [`AWS::AppStream::Stack.StreamingExperienceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-streamingexperiencesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamingExperienceSettings {
        /// Property [`PreferredProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-streamingexperiencesettings.html#cfn-appstream-stack-streamingexperiencesettings-preferredprotocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preferred_protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StreamingExperienceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref preferred_protocol) = self.preferred_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredProtocol", preferred_protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamingExperienceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingExperienceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamingExperienceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamingExperienceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut preferred_protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PreferredProtocol" => {
                                preferred_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamingExperienceSettings {
                        preferred_protocol: preferred_protocol,
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
        /// Property [`MaximumLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-usersetting.html#cfn-appstream-stack-usersetting-maximumlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_length: Option<::Value<u32>>,
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
            if let Some(ref maximum_length) = self.maximum_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumLength", maximum_length)?;
            }
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
                    let mut maximum_length: Option<::Value<u32>> = None;
                    let mut permission: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumLength" => {
                                maximum_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserSetting {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        maximum_length: maximum_length,
                        permission: permission.ok_or(::serde::de::Error::missing_field("Permission"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
