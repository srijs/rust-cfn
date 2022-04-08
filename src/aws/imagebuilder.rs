//! Types for the `ImageBuilder` service.

/// The [`AWS::ImageBuilder::Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html) resource type.
#[derive(Debug, Default)]
pub struct Component {
    properties: ComponentProperties
}

/// Properties for the `Component` resource.
#[derive(Debug, Default)]
pub struct ComponentProperties {
    /// Property [`ChangeDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-changedescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub change_description: Option<::Value<String>>,
    /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-data).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-platform).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platform: ::Value<String>,
    /// Property [`SupportedOsVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-supportedosversions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub supported_os_versions: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-uri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub uri: Option<::Value<String>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html#cfn-imagebuilder-component-version).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version: ::Value<String>,
}

impl ::serde::Serialize for ComponentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref change_description) = self.change_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChangeDescription", change_description)?;
        }
        if let Some(ref data) = self.data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", data)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", &self.platform)?;
        if let Some(ref supported_os_versions) = self.supported_os_versions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedOsVersions", supported_os_versions)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref uri) = self.uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", uri)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComponentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComponentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComponentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut change_description: Option<::Value<String>> = None;
                let mut data: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut platform: Option<::Value<String>> = None;
                let mut supported_os_versions: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut uri: Option<::Value<String>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChangeDescription" => {
                            change_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Data" => {
                            data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Platform" => {
                            platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportedOsVersions" => {
                            supported_os_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Uri" => {
                            uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ComponentProperties {
                    change_description: change_description,
                    data: data,
                    description: description,
                    kms_key_id: kms_key_id,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    platform: platform.ok_or(::serde::de::Error::missing_field("Platform"))?,
                    supported_os_versions: supported_os_versions,
                    tags: tags,
                    uri: uri,
                    version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Component {
    type Properties = ComponentProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::Component";
    fn properties(&self) -> &ComponentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComponentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Component {}

impl From<ComponentProperties> for Component {
    fn from(properties: ComponentProperties) -> Component {
        Component { properties }
    }
}

/// The [`AWS::ImageBuilder::ContainerRecipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html) resource type.
#[derive(Debug, Default)]
pub struct ContainerRecipe {
    properties: ContainerRecipeProperties
}

/// Properties for the `ContainerRecipe` resource.
#[derive(Debug, Default)]
pub struct ContainerRecipeProperties {
    /// Property [`Components`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-components).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub components: ::ValueList<self::container_recipe::ComponentConfiguration>,
    /// Property [`ContainerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-containertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub container_type: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DockerfileTemplateData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-dockerfiletemplatedata).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dockerfile_template_data: Option<::Value<String>>,
    /// Property [`DockerfileTemplateUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-dockerfiletemplateuri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dockerfile_template_uri: Option<::Value<String>>,
    /// Property [`ImageOsVersionOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-imageosversionoverride).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_os_version_override: Option<::Value<String>>,
    /// Property [`InstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-instanceconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_configuration: Option<::Value<self::container_recipe::InstanceConfiguration>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ParentImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-parentimage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_image: ::Value<String>,
    /// Property [`PlatformOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-platformoverride).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platform_override: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`TargetRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-targetrepository).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_repository: ::Value<self::container_recipe::TargetContainerRepository>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-version).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version: ::Value<String>,
    /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html#cfn-imagebuilder-containerrecipe-workingdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub working_directory: Option<::Value<String>>,
}

impl ::serde::Serialize for ContainerRecipeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Components", &self.components)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerType", &self.container_type)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref dockerfile_template_data) = self.dockerfile_template_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerfileTemplateData", dockerfile_template_data)?;
        }
        if let Some(ref dockerfile_template_uri) = self.dockerfile_template_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerfileTemplateUri", dockerfile_template_uri)?;
        }
        if let Some(ref image_os_version_override) = self.image_os_version_override {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageOsVersionOverride", image_os_version_override)?;
        }
        if let Some(ref instance_configuration) = self.instance_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceConfiguration", instance_configuration)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentImage", &self.parent_image)?;
        if let Some(ref platform_override) = self.platform_override {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformOverride", platform_override)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetRepository", &self.target_repository)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        if let Some(ref working_directory) = self.working_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContainerRecipeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerRecipeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerRecipeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContainerRecipeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut components: Option<::ValueList<self::container_recipe::ComponentConfiguration>> = None;
                let mut container_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut dockerfile_template_data: Option<::Value<String>> = None;
                let mut dockerfile_template_uri: Option<::Value<String>> = None;
                let mut image_os_version_override: Option<::Value<String>> = None;
                let mut instance_configuration: Option<::Value<self::container_recipe::InstanceConfiguration>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parent_image: Option<::Value<String>> = None;
                let mut platform_override: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut target_repository: Option<::Value<self::container_recipe::TargetContainerRepository>> = None;
                let mut version: Option<::Value<String>> = None;
                let mut working_directory: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Components" => {
                            components = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContainerType" => {
                            container_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DockerfileTemplateData" => {
                            dockerfile_template_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DockerfileTemplateUri" => {
                            dockerfile_template_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageOsVersionOverride" => {
                            image_os_version_override = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceConfiguration" => {
                            instance_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentImage" => {
                            parent_image = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlatformOverride" => {
                            platform_override = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetRepository" => {
                            target_repository = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkingDirectory" => {
                            working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContainerRecipeProperties {
                    components: components.ok_or(::serde::de::Error::missing_field("Components"))?,
                    container_type: container_type.ok_or(::serde::de::Error::missing_field("ContainerType"))?,
                    description: description,
                    dockerfile_template_data: dockerfile_template_data,
                    dockerfile_template_uri: dockerfile_template_uri,
                    image_os_version_override: image_os_version_override,
                    instance_configuration: instance_configuration,
                    kms_key_id: kms_key_id,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parent_image: parent_image.ok_or(::serde::de::Error::missing_field("ParentImage"))?,
                    platform_override: platform_override,
                    tags: tags,
                    target_repository: target_repository.ok_or(::serde::de::Error::missing_field("TargetRepository"))?,
                    version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    working_directory: working_directory,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContainerRecipe {
    type Properties = ContainerRecipeProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::ContainerRecipe";
    fn properties(&self) -> &ContainerRecipeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContainerRecipeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContainerRecipe {}

impl From<ContainerRecipeProperties> for ContainerRecipe {
    fn from(properties: ContainerRecipeProperties) -> ContainerRecipe {
        ContainerRecipe { properties }
    }
}

/// The [`AWS::ImageBuilder::DistributionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct DistributionConfiguration {
    properties: DistributionConfigurationProperties
}

/// Properties for the `DistributionConfiguration` resource.
#[derive(Debug, Default)]
pub struct DistributionConfigurationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html#cfn-imagebuilder-distributionconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Distributions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html#cfn-imagebuilder-distributionconfiguration-distributions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub distributions: ::ValueList<self::distribution_configuration::Distribution>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html#cfn-imagebuilder-distributionconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html#cfn-imagebuilder-distributionconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for DistributionConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Distributions", &self.distributions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DistributionConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DistributionConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DistributionConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut distributions: Option<::ValueList<self::distribution_configuration::Distribution>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Distributions" => {
                            distributions = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(DistributionConfigurationProperties {
                    description: description,
                    distributions: distributions.ok_or(::serde::de::Error::missing_field("Distributions"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DistributionConfiguration {
    type Properties = DistributionConfigurationProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::DistributionConfiguration";
    fn properties(&self) -> &DistributionConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DistributionConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DistributionConfiguration {}

impl From<DistributionConfigurationProperties> for DistributionConfiguration {
    fn from(properties: DistributionConfigurationProperties) -> DistributionConfiguration {
        DistributionConfiguration { properties }
    }
}

/// The [`AWS::ImageBuilder::Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html) resource type.
#[derive(Debug, Default)]
pub struct Image {
    properties: ImageProperties
}

/// Properties for the `Image` resource.
#[derive(Debug, Default)]
pub struct ImageProperties {
    /// Property [`ContainerRecipeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-containerrecipearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub container_recipe_arn: Option<::Value<String>>,
    /// Property [`DistributionConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-distributionconfigurationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub distribution_configuration_arn: Option<::Value<String>>,
    /// Property [`EnhancedImageMetadataEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-enhancedimagemetadataenabled).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub enhanced_image_metadata_enabled: Option<::Value<bool>>,
    /// Property [`ImageRecipeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-imagerecipearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_recipe_arn: Option<::Value<String>>,
    /// Property [`ImageTestsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-imagetestsconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_tests_configuration: Option<::Value<self::image::ImageTestsConfiguration>>,
    /// Property [`InfrastructureConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-infrastructureconfigurationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub infrastructure_configuration_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html#cfn-imagebuilder-image-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ImageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref container_recipe_arn) = self.container_recipe_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerRecipeArn", container_recipe_arn)?;
        }
        if let Some(ref distribution_configuration_arn) = self.distribution_configuration_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributionConfigurationArn", distribution_configuration_arn)?;
        }
        if let Some(ref enhanced_image_metadata_enabled) = self.enhanced_image_metadata_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedImageMetadataEnabled", enhanced_image_metadata_enabled)?;
        }
        if let Some(ref image_recipe_arn) = self.image_recipe_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageRecipeArn", image_recipe_arn)?;
        }
        if let Some(ref image_tests_configuration) = self.image_tests_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageTestsConfiguration", image_tests_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InfrastructureConfigurationArn", &self.infrastructure_configuration_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_recipe_arn: Option<::Value<String>> = None;
                let mut distribution_configuration_arn: Option<::Value<String>> = None;
                let mut enhanced_image_metadata_enabled: Option<::Value<bool>> = None;
                let mut image_recipe_arn: Option<::Value<String>> = None;
                let mut image_tests_configuration: Option<::Value<self::image::ImageTestsConfiguration>> = None;
                let mut infrastructure_configuration_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerRecipeArn" => {
                            container_recipe_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DistributionConfigurationArn" => {
                            distribution_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedImageMetadataEnabled" => {
                            enhanced_image_metadata_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageRecipeArn" => {
                            image_recipe_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageTestsConfiguration" => {
                            image_tests_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InfrastructureConfigurationArn" => {
                            infrastructure_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImageProperties {
                    container_recipe_arn: container_recipe_arn,
                    distribution_configuration_arn: distribution_configuration_arn,
                    enhanced_image_metadata_enabled: enhanced_image_metadata_enabled,
                    image_recipe_arn: image_recipe_arn,
                    image_tests_configuration: image_tests_configuration,
                    infrastructure_configuration_arn: infrastructure_configuration_arn.ok_or(::serde::de::Error::missing_field("InfrastructureConfigurationArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Image {
    type Properties = ImageProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::Image";
    fn properties(&self) -> &ImageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Image {}

impl From<ImageProperties> for Image {
    fn from(properties: ImageProperties) -> Image {
        Image { properties }
    }
}

/// The [`AWS::ImageBuilder::ImagePipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html) resource type.
#[derive(Debug, Default)]
pub struct ImagePipeline {
    properties: ImagePipelineProperties
}

/// Properties for the `ImagePipeline` resource.
#[derive(Debug, Default)]
pub struct ImagePipelineProperties {
    /// Property [`ContainerRecipeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-containerrecipearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub container_recipe_arn: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DistributionConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-distributionconfigurationarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub distribution_configuration_arn: Option<::Value<String>>,
    /// Property [`EnhancedImageMetadataEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-enhancedimagemetadataenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enhanced_image_metadata_enabled: Option<::Value<bool>>,
    /// Property [`ImageRecipeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-imagerecipearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_recipe_arn: Option<::Value<String>>,
    /// Property [`ImageTestsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-imagetestsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_tests_configuration: Option<::Value<self::image_pipeline::ImageTestsConfiguration>>,
    /// Property [`InfrastructureConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-infrastructureconfigurationarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub infrastructure_configuration_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<self::image_pipeline::Schedule>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html#cfn-imagebuilder-imagepipeline-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ImagePipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref container_recipe_arn) = self.container_recipe_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerRecipeArn", container_recipe_arn)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref distribution_configuration_arn) = self.distribution_configuration_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributionConfigurationArn", distribution_configuration_arn)?;
        }
        if let Some(ref enhanced_image_metadata_enabled) = self.enhanced_image_metadata_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedImageMetadataEnabled", enhanced_image_metadata_enabled)?;
        }
        if let Some(ref image_recipe_arn) = self.image_recipe_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageRecipeArn", image_recipe_arn)?;
        }
        if let Some(ref image_tests_configuration) = self.image_tests_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageTestsConfiguration", image_tests_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InfrastructureConfigurationArn", &self.infrastructure_configuration_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImagePipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImagePipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImagePipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImagePipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_recipe_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut distribution_configuration_arn: Option<::Value<String>> = None;
                let mut enhanced_image_metadata_enabled: Option<::Value<bool>> = None;
                let mut image_recipe_arn: Option<::Value<String>> = None;
                let mut image_tests_configuration: Option<::Value<self::image_pipeline::ImageTestsConfiguration>> = None;
                let mut infrastructure_configuration_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<self::image_pipeline::Schedule>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerRecipeArn" => {
                            container_recipe_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DistributionConfigurationArn" => {
                            distribution_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedImageMetadataEnabled" => {
                            enhanced_image_metadata_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageRecipeArn" => {
                            image_recipe_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageTestsConfiguration" => {
                            image_tests_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InfrastructureConfigurationArn" => {
                            infrastructure_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImagePipelineProperties {
                    container_recipe_arn: container_recipe_arn,
                    description: description,
                    distribution_configuration_arn: distribution_configuration_arn,
                    enhanced_image_metadata_enabled: enhanced_image_metadata_enabled,
                    image_recipe_arn: image_recipe_arn,
                    image_tests_configuration: image_tests_configuration,
                    infrastructure_configuration_arn: infrastructure_configuration_arn.ok_or(::serde::de::Error::missing_field("InfrastructureConfigurationArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schedule: schedule,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ImagePipeline {
    type Properties = ImagePipelineProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::ImagePipeline";
    fn properties(&self) -> &ImagePipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImagePipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ImagePipeline {}

impl From<ImagePipelineProperties> for ImagePipeline {
    fn from(properties: ImagePipelineProperties) -> ImagePipeline {
        ImagePipeline { properties }
    }
}

/// The [`AWS::ImageBuilder::ImageRecipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html) resource type.
#[derive(Debug, Default)]
pub struct ImageRecipe {
    properties: ImageRecipeProperties
}

/// Properties for the `ImageRecipe` resource.
#[derive(Debug, Default)]
pub struct ImageRecipeProperties {
    /// Property [`AdditionalInstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-additionalinstanceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_instance_configuration: Option<::Value<self::image_recipe::AdditionalInstanceConfiguration>>,
    /// Property [`BlockDeviceMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-blockdevicemappings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub block_device_mappings: Option<::ValueList<self::image_recipe::InstanceBlockDeviceMapping>>,
    /// Property [`Components`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-components).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub components: ::ValueList<self::image_recipe::ComponentConfiguration>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ParentImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-parentimage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_image: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-version).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version: ::Value<String>,
    /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html#cfn-imagebuilder-imagerecipe-workingdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub working_directory: Option<::Value<String>>,
}

impl ::serde::Serialize for ImageRecipeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_instance_configuration) = self.additional_instance_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInstanceConfiguration", additional_instance_configuration)?;
        }
        if let Some(ref block_device_mappings) = self.block_device_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", block_device_mappings)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Components", &self.components)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentImage", &self.parent_image)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        if let Some(ref working_directory) = self.working_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageRecipeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageRecipeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImageRecipeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImageRecipeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_instance_configuration: Option<::Value<self::image_recipe::AdditionalInstanceConfiguration>> = None;
                let mut block_device_mappings: Option<::ValueList<self::image_recipe::InstanceBlockDeviceMapping>> = None;
                let mut components: Option<::ValueList<self::image_recipe::ComponentConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parent_image: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut version: Option<::Value<String>> = None;
                let mut working_directory: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalInstanceConfiguration" => {
                            additional_instance_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Components" => {
                            components = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentImage" => {
                            parent_image = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkingDirectory" => {
                            working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImageRecipeProperties {
                    additional_instance_configuration: additional_instance_configuration,
                    block_device_mappings: block_device_mappings,
                    components: components.ok_or(::serde::de::Error::missing_field("Components"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parent_image: parent_image.ok_or(::serde::de::Error::missing_field("ParentImage"))?,
                    tags: tags,
                    version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    working_directory: working_directory,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ImageRecipe {
    type Properties = ImageRecipeProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::ImageRecipe";
    fn properties(&self) -> &ImageRecipeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImageRecipeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ImageRecipe {}

impl From<ImageRecipeProperties> for ImageRecipe {
    fn from(properties: ImageRecipeProperties) -> ImageRecipe {
        ImageRecipe { properties }
    }
}

/// The [`AWS::ImageBuilder::InfrastructureConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct InfrastructureConfiguration {
    properties: InfrastructureConfigurationProperties
}

/// Properties for the `InfrastructureConfiguration` resource.
#[derive(Debug, Default)]
pub struct InfrastructureConfigurationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceMetadataOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-instancemetadataoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_metadata_options: Option<::Value<self::infrastructure_configuration::InstanceMetadataOptions>>,
    /// Property [`InstanceProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-instanceprofilename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_name: ::Value<String>,
    /// Property [`InstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-instancetypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_types: Option<::ValueList<String>>,
    /// Property [`KeyPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-keypair).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_pair: Option<::Value<String>>,
    /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-logging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging: Option<::Value<self::infrastructure_configuration::Logging>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-resourcetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_tags: Option<::ValueMap<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-snstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arn: Option<::Value<String>>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-subnetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`TerminateInstanceOnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html#cfn-imagebuilder-infrastructureconfiguration-terminateinstanceonfailure).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub terminate_instance_on_failure: Option<::Value<bool>>,
}

impl ::serde::Serialize for InfrastructureConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref instance_metadata_options) = self.instance_metadata_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceMetadataOptions", instance_metadata_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileName", &self.instance_profile_name)?;
        if let Some(ref instance_types) = self.instance_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypes", instance_types)?;
        }
        if let Some(ref key_pair) = self.key_pair {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPair", key_pair)?;
        }
        if let Some(ref logging) = self.logging {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref resource_tags) = self.resource_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref sns_topic_arn) = self.sns_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", sns_topic_arn)?;
        }
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref terminate_instance_on_failure) = self.terminate_instance_on_failure {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminateInstanceOnFailure", terminate_instance_on_failure)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InfrastructureConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InfrastructureConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InfrastructureConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InfrastructureConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_metadata_options: Option<::Value<self::infrastructure_configuration::InstanceMetadataOptions>> = None;
                let mut instance_profile_name: Option<::Value<String>> = None;
                let mut instance_types: Option<::ValueList<String>> = None;
                let mut key_pair: Option<::Value<String>> = None;
                let mut logging: Option<::Value<self::infrastructure_configuration::Logging>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut resource_tags: Option<::ValueMap<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut sns_topic_arn: Option<::Value<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut terminate_instance_on_failure: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceMetadataOptions" => {
                            instance_metadata_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileName" => {
                            instance_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceTypes" => {
                            instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyPair" => {
                            key_pair = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Logging" => {
                            logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTags" => {
                            resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TerminateInstanceOnFailure" => {
                            terminate_instance_on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InfrastructureConfigurationProperties {
                    description: description,
                    instance_metadata_options: instance_metadata_options,
                    instance_profile_name: instance_profile_name.ok_or(::serde::de::Error::missing_field("InstanceProfileName"))?,
                    instance_types: instance_types,
                    key_pair: key_pair,
                    logging: logging,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    resource_tags: resource_tags,
                    security_group_ids: security_group_ids,
                    sns_topic_arn: sns_topic_arn,
                    subnet_id: subnet_id,
                    tags: tags,
                    terminate_instance_on_failure: terminate_instance_on_failure,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InfrastructureConfiguration {
    type Properties = InfrastructureConfigurationProperties;
    const TYPE: &'static str = "AWS::ImageBuilder::InfrastructureConfiguration";
    fn properties(&self) -> &InfrastructureConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InfrastructureConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InfrastructureConfiguration {}

impl From<InfrastructureConfigurationProperties> for InfrastructureConfiguration {
    fn from(properties: InfrastructureConfigurationProperties) -> InfrastructureConfiguration {
        InfrastructureConfiguration { properties }
    }
}

pub mod container_recipe {
    //! Property types for the `ContainerRecipe` resource.

    /// The [`AWS::ImageBuilder::ContainerRecipe.ComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-componentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentConfiguration {
        /// Property [`ComponentArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-componentconfiguration.html#cfn-imagebuilder-containerrecipe-componentconfiguration-componentarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_arn) = self.component_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentArn", component_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentArn" => {
                                component_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentConfiguration {
                        component_arn: component_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ContainerRecipe.EbsInstanceBlockDeviceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct EbsInstanceBlockDeviceSpecification {
        /// Property [`DeleteOnTermination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-deleteontermination).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-encrypted).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encrypted: Option<::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-iops).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`SnapshotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-snapshotid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub snapshot_id: Option<::Value<String>>,
        /// Property [`Throughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-throughput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub throughput: Option<::Value<u32>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-volumesize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification-volumetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbsInstanceBlockDeviceSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_on_termination) = self.delete_on_termination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", delete_on_termination)?;
            }
            if let Some(ref encrypted) = self.encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref snapshot_id) = self.snapshot_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", snapshot_id)?;
            }
            if let Some(ref throughput) = self.throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throughput", throughput)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsInstanceBlockDeviceSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsInstanceBlockDeviceSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsInstanceBlockDeviceSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsInstanceBlockDeviceSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination: Option<::Value<bool>> = None;
                    let mut encrypted: Option<::Value<bool>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut snapshot_id: Option<::Value<String>> = None;
                    let mut throughput: Option<::Value<u32>> = None;
                    let mut volume_size: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotId" => {
                                snapshot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Throughput" => {
                                throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsInstanceBlockDeviceSpecification {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        kms_key_id: kms_key_id,
                        snapshot_id: snapshot_id,
                        throughput: throughput,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ContainerRecipe.InstanceBlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceBlockDeviceMapping {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html#cfn-imagebuilder-containerrecipe-instanceblockdevicemapping-devicename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_name: Option<::Value<String>>,
        /// Property [`Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html#cfn-imagebuilder-containerrecipe-instanceblockdevicemapping-ebs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs: Option<::Value<EbsInstanceBlockDeviceSpecification>>,
        /// Property [`NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html#cfn-imagebuilder-containerrecipe-instanceblockdevicemapping-nodevice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub no_device: Option<::Value<String>>,
        /// Property [`VirtualName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html#cfn-imagebuilder-containerrecipe-instanceblockdevicemapping-virtualname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceBlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref device_name) = self.device_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", device_name)?;
            }
            if let Some(ref ebs) = self.ebs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", ebs)?;
            }
            if let Some(ref no_device) = self.no_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", no_device)?;
            }
            if let Some(ref virtual_name) = self.virtual_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", virtual_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceBlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceBlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceBlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceBlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name: Option<::Value<String>> = None;
                    let mut ebs: Option<::Value<EbsInstanceBlockDeviceSpecification>> = None;
                    let mut no_device: Option<::Value<String>> = None;
                    let mut virtual_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ebs" => {
                                ebs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoDevice" => {
                                no_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualName" => {
                                virtual_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceBlockDeviceMapping {
                        device_name: device_name,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ContainerRecipe.InstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceConfiguration {
        /// Property [`BlockDeviceMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceconfiguration.html#cfn-imagebuilder-containerrecipe-instanceconfiguration-blockdevicemappings).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub block_device_mappings: Option<::ValueList<InstanceBlockDeviceMapping>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceconfiguration.html#cfn-imagebuilder-containerrecipe-instanceconfiguration-image).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_device_mappings) = self.block_device_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", block_device_mappings)?;
            }
            if let Some(ref image) = self.image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", image)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_device_mappings: Option<::ValueList<InstanceBlockDeviceMapping>> = None;
                    let mut image: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDeviceMappings" => {
                                block_device_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceConfiguration {
                        block_device_mappings: block_device_mappings,
                        image: image,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ContainerRecipe.TargetContainerRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-targetcontainerrepository.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetContainerRepository {
        /// Property [`RepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-targetcontainerrepository.html#cfn-imagebuilder-containerrecipe-targetcontainerrepository-repositoryname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_name: Option<::Value<String>>,
        /// Property [`Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-targetcontainerrepository.html#cfn-imagebuilder-containerrecipe-targetcontainerrepository-service).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub service: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TargetContainerRepository {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref repository_name) = self.repository_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", repository_name)?;
            }
            if let Some(ref service) = self.service {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Service", service)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetContainerRepository {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetContainerRepository, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetContainerRepository;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetContainerRepository")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut repository_name: Option<::Value<String>> = None;
                    let mut service: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RepositoryName" => {
                                repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Service" => {
                                service = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetContainerRepository {
                        repository_name: repository_name,
                        service: service,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod distribution_configuration {
    //! Property types for the `DistributionConfiguration` resource.

    /// The [`AWS::ImageBuilder::DistributionConfiguration.Distribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html) property type.
    #[derive(Debug, Default)]
    pub struct Distribution {
        /// Property [`AmiDistributionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html#cfn-imagebuilder-distributionconfiguration-distribution-amidistributionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ami_distribution_configuration: Option<::Value<::json::Value>>,
        /// Property [`ContainerDistributionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html#cfn-imagebuilder-distributionconfiguration-distribution-containerdistributionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_distribution_configuration: Option<::Value<::json::Value>>,
        /// Property [`LaunchTemplateConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html#cfn-imagebuilder-distributionconfiguration-distribution-launchtemplateconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_configurations: Option<::ValueList<LaunchTemplateConfiguration>>,
        /// Property [`LicenseConfigurationArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html#cfn-imagebuilder-distributionconfiguration-distribution-licenseconfigurationarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub license_configuration_arns: Option<::ValueList<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html#cfn-imagebuilder-distributionconfiguration-distribution-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
    }

    impl ::codec::SerializeValue for Distribution {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ami_distribution_configuration) = self.ami_distribution_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmiDistributionConfiguration", ami_distribution_configuration)?;
            }
            if let Some(ref container_distribution_configuration) = self.container_distribution_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerDistributionConfiguration", container_distribution_configuration)?;
            }
            if let Some(ref launch_template_configurations) = self.launch_template_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateConfigurations", launch_template_configurations)?;
            }
            if let Some(ref license_configuration_arns) = self.license_configuration_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseConfigurationArns", license_configuration_arns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Distribution {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Distribution, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Distribution;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Distribution")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ami_distribution_configuration: Option<::Value<::json::Value>> = None;
                    let mut container_distribution_configuration: Option<::Value<::json::Value>> = None;
                    let mut launch_template_configurations: Option<::ValueList<LaunchTemplateConfiguration>> = None;
                    let mut license_configuration_arns: Option<::ValueList<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AmiDistributionConfiguration" => {
                                ami_distribution_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerDistributionConfiguration" => {
                                container_distribution_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplateConfigurations" => {
                                launch_template_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LicenseConfigurationArns" => {
                                license_configuration_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Distribution {
                        ami_distribution_configuration: ami_distribution_configuration,
                        container_distribution_configuration: container_distribution_configuration,
                        launch_template_configurations: launch_template_configurations,
                        license_configuration_arns: license_configuration_arns,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::DistributionConfiguration.LaunchTemplateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchtemplateconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplateConfiguration {
        /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchtemplateconfiguration.html#cfn-imagebuilder-distributionconfiguration-launchtemplateconfiguration-accountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_id: Option<::Value<String>>,
        /// Property [`LaunchTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchtemplateconfiguration.html#cfn-imagebuilder-distributionconfiguration-launchtemplateconfiguration-launchtemplateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_id: Option<::Value<String>>,
        /// Property [`SetDefaultVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchtemplateconfiguration.html#cfn-imagebuilder-distributionconfiguration-launchtemplateconfiguration-setdefaultversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_default_version: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LaunchTemplateConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref account_id) = self.account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", account_id)?;
            }
            if let Some(ref launch_template_id) = self.launch_template_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateId", launch_template_id)?;
            }
            if let Some(ref set_default_version) = self.set_default_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetDefaultVersion", set_default_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LaunchTemplateConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplateConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplateConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplateConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_id: Option<::Value<String>> = None;
                    let mut launch_template_id: Option<::Value<String>> = None;
                    let mut set_default_version: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountId" => {
                                account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplateId" => {
                                launch_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetDefaultVersion" => {
                                set_default_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplateConfiguration {
                        account_id: account_id,
                        launch_template_id: launch_template_id,
                        set_default_version: set_default_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod image {
    //! Property types for the `Image` resource.

    /// The [`AWS::ImageBuilder::Image.ImageTestsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagetestsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageTestsConfiguration {
        /// Property [`ImageTestsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagetestsconfiguration.html#cfn-imagebuilder-image-imagetestsconfiguration-imagetestsenabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_tests_enabled: Option<::Value<bool>>,
        /// Property [`TimeoutMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagetestsconfiguration.html#cfn-imagebuilder-image-imagetestsconfiguration-timeoutminutes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout_minutes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ImageTestsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_tests_enabled) = self.image_tests_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageTestsEnabled", image_tests_enabled)?;
            }
            if let Some(ref timeout_minutes) = self.timeout_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutMinutes", timeout_minutes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageTestsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageTestsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageTestsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageTestsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_tests_enabled: Option<::Value<bool>> = None;
                    let mut timeout_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageTestsEnabled" => {
                                image_tests_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutMinutes" => {
                                timeout_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageTestsConfiguration {
                        image_tests_enabled: image_tests_enabled,
                        timeout_minutes: timeout_minutes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod image_pipeline {
    //! Property types for the `ImagePipeline` resource.

    /// The [`AWS::ImageBuilder::ImagePipeline.ImageTestsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-imagetestsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageTestsConfiguration {
        /// Property [`ImageTestsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-imagetestsconfiguration.html#cfn-imagebuilder-imagepipeline-imagetestsconfiguration-imagetestsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_tests_enabled: Option<::Value<bool>>,
        /// Property [`TimeoutMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-imagetestsconfiguration.html#cfn-imagebuilder-imagepipeline-imagetestsconfiguration-timeoutminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_minutes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ImageTestsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_tests_enabled) = self.image_tests_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageTestsEnabled", image_tests_enabled)?;
            }
            if let Some(ref timeout_minutes) = self.timeout_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutMinutes", timeout_minutes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageTestsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageTestsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageTestsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageTestsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_tests_enabled: Option<::Value<bool>> = None;
                    let mut timeout_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageTestsEnabled" => {
                                image_tests_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutMinutes" => {
                                timeout_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageTestsConfiguration {
                        image_tests_enabled: image_tests_enabled,
                        timeout_minutes: timeout_minutes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImagePipeline.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`PipelineExecutionStartCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-schedule.html#cfn-imagebuilder-imagepipeline-schedule-pipelineexecutionstartcondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pipeline_execution_start_condition: Option<::Value<String>>,
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-schedule.html#cfn-imagebuilder-imagepipeline-schedule-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pipeline_execution_start_condition) = self.pipeline_execution_start_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineExecutionStartCondition", pipeline_execution_start_condition)?;
            }
            if let Some(ref schedule_expression) = self.schedule_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pipeline_execution_start_condition: Option<::Value<String>> = None;
                    let mut schedule_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PipelineExecutionStartCondition" => {
                                pipeline_execution_start_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        pipeline_execution_start_condition: pipeline_execution_start_condition,
                        schedule_expression: schedule_expression,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod image_recipe {
    //! Property types for the `ImageRecipe` resource.

    /// The [`AWS::ImageBuilder::ImageRecipe.AdditionalInstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-additionalinstanceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AdditionalInstanceConfiguration {
        /// Property [`SystemsManagerAgent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-additionalinstanceconfiguration.html#cfn-imagebuilder-imagerecipe-additionalinstanceconfiguration-systemsmanageragent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub systems_manager_agent: Option<::Value<SystemsManagerAgent>>,
        /// Property [`UserDataOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-additionalinstanceconfiguration.html#cfn-imagebuilder-imagerecipe-additionalinstanceconfiguration-userdataoverride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_data_override: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AdditionalInstanceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref systems_manager_agent) = self.systems_manager_agent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemsManagerAgent", systems_manager_agent)?;
            }
            if let Some(ref user_data_override) = self.user_data_override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserDataOverride", user_data_override)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdditionalInstanceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdditionalInstanceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdditionalInstanceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdditionalInstanceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut systems_manager_agent: Option<::Value<SystemsManagerAgent>> = None;
                    let mut user_data_override: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SystemsManagerAgent" => {
                                systems_manager_agent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserDataOverride" => {
                                user_data_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdditionalInstanceConfiguration {
                        systems_manager_agent: systems_manager_agent,
                        user_data_override: user_data_override,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImageRecipe.ComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentConfiguration {
        /// Property [`ComponentArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentconfiguration.html#cfn-imagebuilder-imagerecipe-componentconfiguration-componentarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_arn: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentconfiguration.html#cfn-imagebuilder-imagerecipe-componentconfiguration-parameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub parameters: Option<::ValueList<ComponentParameter>>,
    }

    impl ::codec::SerializeValue for ComponentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_arn) = self.component_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentArn", component_arn)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_arn: Option<::Value<String>> = None;
                    let mut parameters: Option<::ValueList<ComponentParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentArn" => {
                                component_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentConfiguration {
                        component_arn: component_arn,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImageRecipe.ComponentParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentparameter.html#cfn-imagebuilder-imagerecipe-componentparameter-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentparameter.html#cfn-imagebuilder-imagerecipe-componentparameter-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ComponentParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::ValueList<String>> = None;

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

                    Ok(ComponentParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImageRecipe.EbsInstanceBlockDeviceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct EbsInstanceBlockDeviceSpecification {
        /// Property [`DeleteOnTermination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-deleteontermination).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-encrypted).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encrypted: Option<::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-iops).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`SnapshotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-snapshotid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub snapshot_id: Option<::Value<String>>,
        /// Property [`Throughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-throughput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub throughput: Option<::Value<u32>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-volumesize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html#cfn-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification-volumetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbsInstanceBlockDeviceSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_on_termination) = self.delete_on_termination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", delete_on_termination)?;
            }
            if let Some(ref encrypted) = self.encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref snapshot_id) = self.snapshot_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", snapshot_id)?;
            }
            if let Some(ref throughput) = self.throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throughput", throughput)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsInstanceBlockDeviceSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsInstanceBlockDeviceSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsInstanceBlockDeviceSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsInstanceBlockDeviceSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination: Option<::Value<bool>> = None;
                    let mut encrypted: Option<::Value<bool>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut snapshot_id: Option<::Value<String>> = None;
                    let mut throughput: Option<::Value<u32>> = None;
                    let mut volume_size: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotId" => {
                                snapshot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Throughput" => {
                                throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsInstanceBlockDeviceSpecification {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        kms_key_id: kms_key_id,
                        snapshot_id: snapshot_id,
                        throughput: throughput,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImageRecipe.InstanceBlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceBlockDeviceMapping {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html#cfn-imagebuilder-imagerecipe-instanceblockdevicemapping-devicename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_name: Option<::Value<String>>,
        /// Property [`Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html#cfn-imagebuilder-imagerecipe-instanceblockdevicemapping-ebs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs: Option<::Value<EbsInstanceBlockDeviceSpecification>>,
        /// Property [`NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html#cfn-imagebuilder-imagerecipe-instanceblockdevicemapping-nodevice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub no_device: Option<::Value<String>>,
        /// Property [`VirtualName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html#cfn-imagebuilder-imagerecipe-instanceblockdevicemapping-virtualname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceBlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref device_name) = self.device_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", device_name)?;
            }
            if let Some(ref ebs) = self.ebs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", ebs)?;
            }
            if let Some(ref no_device) = self.no_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", no_device)?;
            }
            if let Some(ref virtual_name) = self.virtual_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", virtual_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceBlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceBlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceBlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceBlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name: Option<::Value<String>> = None;
                    let mut ebs: Option<::Value<EbsInstanceBlockDeviceSpecification>> = None;
                    let mut no_device: Option<::Value<String>> = None;
                    let mut virtual_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ebs" => {
                                ebs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoDevice" => {
                                no_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualName" => {
                                virtual_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceBlockDeviceMapping {
                        device_name: device_name,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::ImageRecipe.SystemsManagerAgent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-systemsmanageragent.html) property type.
    #[derive(Debug, Default)]
    pub struct SystemsManagerAgent {
        /// Property [`UninstallAfterBuild`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-systemsmanageragent.html#cfn-imagebuilder-imagerecipe-systemsmanageragent-uninstallafterbuild).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uninstall_after_build: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SystemsManagerAgent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref uninstall_after_build) = self.uninstall_after_build {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UninstallAfterBuild", uninstall_after_build)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SystemsManagerAgent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SystemsManagerAgent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SystemsManagerAgent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SystemsManagerAgent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut uninstall_after_build: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UninstallAfterBuild" => {
                                uninstall_after_build = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SystemsManagerAgent {
                        uninstall_after_build: uninstall_after_build,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod infrastructure_configuration {
    //! Property types for the `InfrastructureConfiguration` resource.

    /// The [`AWS::ImageBuilder::InfrastructureConfiguration.InstanceMetadataOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-instancemetadataoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceMetadataOptions {
        /// Property [`HttpPutResponseHopLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-instancemetadataoptions.html#cfn-imagebuilder-infrastructureconfiguration-instancemetadataoptions-httpputresponsehoplimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_put_response_hop_limit: Option<::Value<u32>>,
        /// Property [`HttpTokens`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-instancemetadataoptions.html#cfn-imagebuilder-infrastructureconfiguration-instancemetadataoptions-httptokens).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_tokens: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceMetadataOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_put_response_hop_limit) = self.http_put_response_hop_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpPutResponseHopLimit", http_put_response_hop_limit)?;
            }
            if let Some(ref http_tokens) = self.http_tokens {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpTokens", http_tokens)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceMetadataOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceMetadataOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceMetadataOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceMetadataOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_put_response_hop_limit: Option<::Value<u32>> = None;
                    let mut http_tokens: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpPutResponseHopLimit" => {
                                http_put_response_hop_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpTokens" => {
                                http_tokens = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceMetadataOptions {
                        http_put_response_hop_limit: http_put_response_hop_limit,
                        http_tokens: http_tokens,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::InfrastructureConfiguration.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`S3Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-logging.html#cfn-imagebuilder-infrastructureconfiguration-logging-s3logs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_logs: Option<::Value<S3Logs>>,
    }

    impl ::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_logs) = self.s3_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Logs", s3_logs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_logs: Option<::Value<S3Logs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Logs" => {
                                s3_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        s3_logs: s3_logs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ImageBuilder::InfrastructureConfiguration.S3Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-s3logs.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Logs {
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-s3logs.html#cfn-imagebuilder-infrastructureconfiguration-s3logs-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: Option<::Value<String>>,
        /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-s3logs.html#cfn-imagebuilder-infrastructureconfiguration-s3logs-s3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Logs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_bucket_name) = self.s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", s3_bucket_name)?;
            }
            if let Some(ref s3_key_prefix) = self.s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Logs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Logs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Logs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Logs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Logs {
                        s3_bucket_name: s3_bucket_name,
                        s3_key_prefix: s3_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
