//! Types for the `IoT1Click` service.

/// The [`AWS::IoT1Click::Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-device.html) resource type.
#[derive(Debug, Default)]
pub struct Device {
    properties: DeviceProperties
}

/// Properties for the `Device` resource.
#[derive(Debug, Default)]
pub struct DeviceProperties {
    /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-device.html#cfn-iot1click-device-deviceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_id: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-device.html#cfn-iot1click-device-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: ::Value<bool>,
}

impl ::serde::Serialize for DeviceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", &self.device_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut device_id: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeviceId" => {
                            device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceProperties {
                    device_id: device_id.ok_or(::serde::de::Error::missing_field("DeviceId"))?,
                    enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Device {
    type Properties = DeviceProperties;
    const TYPE: &'static str = "AWS::IoT1Click::Device";
    fn properties(&self) -> &DeviceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Device {}

impl From<DeviceProperties> for Device {
    fn from(properties: DeviceProperties) -> Device {
        Device { properties }
    }
}

/// The [`AWS::IoT1Click::Placement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-placement.html) resource type.
#[derive(Debug, Default)]
pub struct Placement {
    properties: PlacementProperties
}

/// Properties for the `Placement` resource.
#[derive(Debug, Default)]
pub struct PlacementProperties {
    /// Property [`AssociatedDevices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-placement.html#cfn-iot1click-placement-associateddevices).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub associated_devices: Option<::Value<::json::Value>>,
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-placement.html#cfn-iot1click-placement-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::Value<::json::Value>>,
    /// Property [`PlacementName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-placement.html#cfn-iot1click-placement-placementname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub placement_name: Option<::Value<String>>,
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-placement.html#cfn-iot1click-placement-projectname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_name: ::Value<String>,
}

impl ::serde::Serialize for PlacementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associated_devices) = self.associated_devices {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedDevices", associated_devices)?;
        }
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        if let Some(ref placement_name) = self.placement_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementName", placement_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", &self.project_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PlacementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PlacementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PlacementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associated_devices: Option<::Value<::json::Value>> = None;
                let mut attributes: Option<::Value<::json::Value>> = None;
                let mut placement_name: Option<::Value<String>> = None;
                let mut project_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatedDevices" => {
                            associated_devices = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementName" => {
                            placement_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PlacementProperties {
                    associated_devices: associated_devices,
                    attributes: attributes,
                    placement_name: placement_name,
                    project_name: project_name.ok_or(::serde::de::Error::missing_field("ProjectName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Placement {
    type Properties = PlacementProperties;
    const TYPE: &'static str = "AWS::IoT1Click::Placement";
    fn properties(&self) -> &PlacementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlacementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Placement {}

impl From<PlacementProperties> for Placement {
    fn from(properties: PlacementProperties) -> Placement {
        Placement { properties }
    }
}

/// The [`AWS::IoT1Click::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-project.html#cfn-iot1click-project-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PlacementTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-project.html#cfn-iot1click-project-placementtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub placement_template: ::Value<self::project::PlacementTemplate>,
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot1click-project.html#cfn-iot1click-project-projectname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementTemplate", &self.placement_template)?;
        if let Some(ref project_name) = self.project_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", project_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut placement_template: Option<::Value<self::project::PlacementTemplate>> = None;
                let mut project_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementTemplate" => {
                            placement_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    description: description,
                    placement_template: placement_template.ok_or(::serde::de::Error::missing_field("PlacementTemplate"))?,
                    project_name: project_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::IoT1Click::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

pub mod project {
    //! Property types for the `Project` resource.

    /// The [`AWS::IoT1Click::Project.DeviceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-devicetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceTemplate {
        /// Property [`CallbackOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-devicetemplate.html#cfn-iot1click-project-devicetemplate-callbackoverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub callback_overrides: Option<::Value<::json::Value>>,
        /// Property [`DeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-devicetemplate.html#cfn-iot1click-project-devicetemplate-devicetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeviceTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref callback_overrides) = self.callback_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CallbackOverrides", callback_overrides)?;
            }
            if let Some(ref device_type) = self.device_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceType", device_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut callback_overrides: Option<::Value<::json::Value>> = None;
                    let mut device_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CallbackOverrides" => {
                                callback_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceType" => {
                                device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceTemplate {
                        callback_overrides: callback_overrides,
                        device_type: device_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT1Click::Project.PlacementTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-placementtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementTemplate {
        /// Property [`DefaultAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-placementtemplate.html#cfn-iot1click-project-placementtemplate-defaultattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_attributes: Option<::Value<::json::Value>>,
        /// Property [`DeviceTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot1click-project-placementtemplate.html#cfn-iot1click-project-placementtemplate-devicetemplates).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_templates: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for PlacementTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_attributes) = self.default_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAttributes", default_attributes)?;
            }
            if let Some(ref device_templates) = self.device_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceTemplates", device_templates)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_attributes: Option<::Value<::json::Value>> = None;
                    let mut device_templates: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultAttributes" => {
                                default_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceTemplates" => {
                                device_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementTemplate {
                        default_attributes: default_attributes,
                        device_templates: device_templates,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
