//! Types for the `GreengrassV2` service.

/// The [`AWS::GreengrassV2::ComponentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-componentversion.html) resource type.
#[derive(Debug, Default)]
pub struct ComponentVersion {
    properties: ComponentVersionProperties
}

/// Properties for the `ComponentVersion` resource.
#[derive(Debug, Default)]
pub struct ComponentVersionProperties {
    /// Property [`InlineRecipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-componentversion.html#cfn-greengrassv2-componentversion-inlinerecipe).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inline_recipe: Option<::Value<String>>,
    /// Property [`LambdaFunction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-componentversion.html#cfn-greengrassv2-componentversion-lambdafunction).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub lambda_function: Option<::Value<self::component_version::LambdaFunctionRecipeSource>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-componentversion.html#cfn-greengrassv2-componentversion-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ComponentVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref inline_recipe) = self.inline_recipe {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InlineRecipe", inline_recipe)?;
        }
        if let Some(ref lambda_function) = self.lambda_function {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunction", lambda_function)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComponentVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComponentVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComponentVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut inline_recipe: Option<::Value<String>> = None;
                let mut lambda_function: Option<::Value<self::component_version::LambdaFunctionRecipeSource>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InlineRecipe" => {
                            inline_recipe = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaFunction" => {
                            lambda_function = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ComponentVersionProperties {
                    inline_recipe: inline_recipe,
                    lambda_function: lambda_function,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ComponentVersion {
    type Properties = ComponentVersionProperties;
    const TYPE: &'static str = "AWS::GreengrassV2::ComponentVersion";
    fn properties(&self) -> &ComponentVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComponentVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ComponentVersion {}

impl From<ComponentVersionProperties> for ComponentVersion {
    fn from(properties: ComponentVersionProperties) -> ComponentVersion {
        ComponentVersion { properties }
    }
}

pub mod component_version {
    //! Property types for the `ComponentVersion` resource.

    /// The [`AWS::GreengrassV2::ComponentVersion.ComponentDependencyRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentdependencyrequirement.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentDependencyRequirement {
        /// Property [`DependencyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentdependencyrequirement.html#cfn-greengrassv2-componentversion-componentdependencyrequirement-dependencytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dependency_type: Option<::Value<String>>,
        /// Property [`VersionRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentdependencyrequirement.html#cfn-greengrassv2-componentversion-componentdependencyrequirement-versionrequirement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version_requirement: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentDependencyRequirement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dependency_type) = self.dependency_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DependencyType", dependency_type)?;
            }
            if let Some(ref version_requirement) = self.version_requirement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionRequirement", version_requirement)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentDependencyRequirement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentDependencyRequirement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentDependencyRequirement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentDependencyRequirement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dependency_type: Option<::Value<String>> = None;
                    let mut version_requirement: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DependencyType" => {
                                dependency_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VersionRequirement" => {
                                version_requirement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentDependencyRequirement {
                        dependency_type: dependency_type,
                        version_requirement: version_requirement,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.ComponentPlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentplatform.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentPlatform {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentplatform.html#cfn-greengrassv2-componentversion-componentplatform-attributes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentplatform.html#cfn-greengrassv2-componentversion-componentplatform-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentPlatform {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentPlatform {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentPlatform, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentPlatform;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentPlatform")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentPlatform {
                        attributes: attributes,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaContainerParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaContainerParams {
        /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html#cfn-greengrassv2-componentversion-lambdacontainerparams-devices).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub devices: Option<::ValueList<LambdaDeviceMount>>,
        /// Property [`MemorySizeInKB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html#cfn-greengrassv2-componentversion-lambdacontainerparams-memorysizeinkb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory_size_in_kb: Option<::Value<u32>>,
        /// Property [`MountROSysfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html#cfn-greengrassv2-componentversion-lambdacontainerparams-mountrosysfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_ro_sysfs: Option<::Value<bool>>,
        /// Property [`Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html#cfn-greengrassv2-componentversion-lambdacontainerparams-volumes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volumes: Option<::ValueList<LambdaVolumeMount>>,
    }

    impl ::codec::SerializeValue for LambdaContainerParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref devices) = self.devices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", devices)?;
            }
            if let Some(ref memory_size_in_kb) = self.memory_size_in_kb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySizeInKB", memory_size_in_kb)?;
            }
            if let Some(ref mount_ro_sysfs) = self.mount_ro_sysfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountROSysfs", mount_ro_sysfs)?;
            }
            if let Some(ref volumes) = self.volumes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", volumes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaContainerParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaContainerParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaContainerParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaContainerParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut devices: Option<::ValueList<LambdaDeviceMount>> = None;
                    let mut memory_size_in_kb: Option<::Value<u32>> = None;
                    let mut mount_ro_sysfs: Option<::Value<bool>> = None;
                    let mut volumes: Option<::ValueList<LambdaVolumeMount>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Devices" => {
                                devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemorySizeInKB" => {
                                memory_size_in_kb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountROSysfs" => {
                                mount_ro_sysfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Volumes" => {
                                volumes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaContainerParams {
                        devices: devices,
                        memory_size_in_kb: memory_size_in_kb,
                        mount_ro_sysfs: mount_ro_sysfs,
                        volumes: volumes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaDeviceMount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdadevicemount.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaDeviceMount {
        /// Property [`AddGroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdadevicemount.html#cfn-greengrassv2-componentversion-lambdadevicemount-addgroupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub add_group_owner: Option<::Value<bool>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdadevicemount.html#cfn-greengrassv2-componentversion-lambdadevicemount-path).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdadevicemount.html#cfn-greengrassv2-componentversion-lambdadevicemount-permission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permission: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaDeviceMount {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_group_owner) = self.add_group_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddGroupOwner", add_group_owner)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref permission) = self.permission {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", permission)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaDeviceMount {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaDeviceMount, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaDeviceMount;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaDeviceMount")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_group_owner: Option<::Value<bool>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut permission: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddGroupOwner" => {
                                add_group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaDeviceMount {
                        add_group_owner: add_group_owner,
                        path: path,
                        permission: permission,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaeventsource.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaEventSource {
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaeventsource.html#cfn-greengrassv2-componentversion-lambdaeventsource-topic).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub topic: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaeventsource.html#cfn-greengrassv2-componentversion-lambdaeventsource-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaEventSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref topic) = self.topic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", topic)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaEventSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaEventSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaEventSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaEventSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut topic: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaEventSource {
                        topic: topic,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaExecutionParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaExecutionParameters {
        /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-environmentvariables).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment_variables: Option<::ValueMap<String>>,
        /// Property [`EventSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-eventsources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub event_sources: Option<::ValueList<LambdaEventSource>>,
        /// Property [`ExecArgs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-execargs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub exec_args: Option<::ValueList<String>>,
        /// Property [`InputPayloadEncodingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-inputpayloadencodingtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub input_payload_encoding_type: Option<::Value<String>>,
        /// Property [`LinuxProcessParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-linuxprocessparams).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub linux_process_params: Option<::Value<LambdaLinuxProcessParams>>,
        /// Property [`MaxIdleTimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-maxidletimeinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_idle_time_in_seconds: Option<::Value<u32>>,
        /// Property [`MaxInstancesCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-maxinstancescount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_instances_count: Option<::Value<u32>>,
        /// Property [`MaxQueueSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-maxqueuesize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_queue_size: Option<::Value<u32>>,
        /// Property [`Pinned`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-pinned).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pinned: Option<::Value<bool>>,
        /// Property [`StatusTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-statustimeoutinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub status_timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html#cfn-greengrassv2-componentversion-lambdaexecutionparameters-timeoutinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for LambdaExecutionParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref environment_variables) = self.environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
            }
            if let Some(ref event_sources) = self.event_sources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSources", event_sources)?;
            }
            if let Some(ref exec_args) = self.exec_args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecArgs", exec_args)?;
            }
            if let Some(ref input_payload_encoding_type) = self.input_payload_encoding_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPayloadEncodingType", input_payload_encoding_type)?;
            }
            if let Some(ref linux_process_params) = self.linux_process_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinuxProcessParams", linux_process_params)?;
            }
            if let Some(ref max_idle_time_in_seconds) = self.max_idle_time_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxIdleTimeInSeconds", max_idle_time_in_seconds)?;
            }
            if let Some(ref max_instances_count) = self.max_instances_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxInstancesCount", max_instances_count)?;
            }
            if let Some(ref max_queue_size) = self.max_queue_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxQueueSize", max_queue_size)?;
            }
            if let Some(ref pinned) = self.pinned {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pinned", pinned)?;
            }
            if let Some(ref status_timeout_in_seconds) = self.status_timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusTimeoutInSeconds", status_timeout_in_seconds)?;
            }
            if let Some(ref timeout_in_seconds) = self.timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", timeout_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaExecutionParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaExecutionParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaExecutionParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaExecutionParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut environment_variables: Option<::ValueMap<String>> = None;
                    let mut event_sources: Option<::ValueList<LambdaEventSource>> = None;
                    let mut exec_args: Option<::ValueList<String>> = None;
                    let mut input_payload_encoding_type: Option<::Value<String>> = None;
                    let mut linux_process_params: Option<::Value<LambdaLinuxProcessParams>> = None;
                    let mut max_idle_time_in_seconds: Option<::Value<u32>> = None;
                    let mut max_instances_count: Option<::Value<u32>> = None;
                    let mut max_queue_size: Option<::Value<u32>> = None;
                    let mut pinned: Option<::Value<bool>> = None;
                    let mut status_timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut timeout_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnvironmentVariables" => {
                                environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventSources" => {
                                event_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecArgs" => {
                                exec_args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputPayloadEncodingType" => {
                                input_payload_encoding_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinuxProcessParams" => {
                                linux_process_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxIdleTimeInSeconds" => {
                                max_idle_time_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxInstancesCount" => {
                                max_instances_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxQueueSize" => {
                                max_queue_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pinned" => {
                                pinned = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusTimeoutInSeconds" => {
                                status_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInSeconds" => {
                                timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaExecutionParameters {
                        environment_variables: environment_variables,
                        event_sources: event_sources,
                        exec_args: exec_args,
                        input_payload_encoding_type: input_payload_encoding_type,
                        linux_process_params: linux_process_params,
                        max_idle_time_in_seconds: max_idle_time_in_seconds,
                        max_instances_count: max_instances_count,
                        max_queue_size: max_queue_size,
                        pinned: pinned,
                        status_timeout_in_seconds: status_timeout_in_seconds,
                        timeout_in_seconds: timeout_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaFunctionRecipeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaFunctionRecipeSource {
        /// Property [`ComponentDependencies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-componentdependencies).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_dependencies: Option<::ValueMap<ComponentDependencyRequirement>>,
        /// Property [`ComponentLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-componentlambdaparameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_lambda_parameters: Option<::Value<LambdaExecutionParameters>>,
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-componentname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_name: Option<::Value<String>>,
        /// Property [`ComponentPlatforms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-componentplatforms).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_platforms: Option<::ValueList<ComponentPlatform>>,
        /// Property [`ComponentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-componentversion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub component_version: Option<::Value<String>>,
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html#cfn-greengrassv2-componentversion-lambdafunctionrecipesource-lambdaarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub lambda_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaFunctionRecipeSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_dependencies) = self.component_dependencies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentDependencies", component_dependencies)?;
            }
            if let Some(ref component_lambda_parameters) = self.component_lambda_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentLambdaParameters", component_lambda_parameters)?;
            }
            if let Some(ref component_name) = self.component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", component_name)?;
            }
            if let Some(ref component_platforms) = self.component_platforms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentPlatforms", component_platforms)?;
            }
            if let Some(ref component_version) = self.component_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentVersion", component_version)?;
            }
            if let Some(ref lambda_arn) = self.lambda_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", lambda_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaFunctionRecipeSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaFunctionRecipeSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaFunctionRecipeSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaFunctionRecipeSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_dependencies: Option<::ValueMap<ComponentDependencyRequirement>> = None;
                    let mut component_lambda_parameters: Option<::Value<LambdaExecutionParameters>> = None;
                    let mut component_name: Option<::Value<String>> = None;
                    let mut component_platforms: Option<::ValueList<ComponentPlatform>> = None;
                    let mut component_version: Option<::Value<String>> = None;
                    let mut lambda_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentDependencies" => {
                                component_dependencies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentLambdaParameters" => {
                                component_lambda_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentPlatforms" => {
                                component_platforms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentVersion" => {
                                component_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaFunctionRecipeSource {
                        component_dependencies: component_dependencies,
                        component_lambda_parameters: component_lambda_parameters,
                        component_name: component_name,
                        component_platforms: component_platforms,
                        component_version: component_version,
                        lambda_arn: lambda_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaLinuxProcessParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdalinuxprocessparams.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaLinuxProcessParams {
        /// Property [`ContainerParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdalinuxprocessparams.html#cfn-greengrassv2-componentversion-lambdalinuxprocessparams-containerparams).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_params: Option<::Value<LambdaContainerParams>>,
        /// Property [`IsolationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdalinuxprocessparams.html#cfn-greengrassv2-componentversion-lambdalinuxprocessparams-isolationmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub isolation_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaLinuxProcessParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_params) = self.container_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerParams", container_params)?;
            }
            if let Some(ref isolation_mode) = self.isolation_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsolationMode", isolation_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaLinuxProcessParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaLinuxProcessParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaLinuxProcessParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaLinuxProcessParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_params: Option<::Value<LambdaContainerParams>> = None;
                    let mut isolation_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerParams" => {
                                container_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsolationMode" => {
                                isolation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaLinuxProcessParams {
                        container_params: container_params,
                        isolation_mode: isolation_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GreengrassV2::ComponentVersion.LambdaVolumeMount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaVolumeMount {
        /// Property [`AddGroupOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html#cfn-greengrassv2-componentversion-lambdavolumemount-addgroupowner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub add_group_owner: Option<::Value<bool>>,
        /// Property [`DestinationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html#cfn-greengrassv2-componentversion-lambdavolumemount-destinationpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_path: Option<::Value<String>>,
        /// Property [`Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html#cfn-greengrassv2-componentversion-lambdavolumemount-permission).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permission: Option<::Value<String>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html#cfn-greengrassv2-componentversion-lambdavolumemount-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaVolumeMount {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_group_owner) = self.add_group_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddGroupOwner", add_group_owner)?;
            }
            if let Some(ref destination_path) = self.destination_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPath", destination_path)?;
            }
            if let Some(ref permission) = self.permission {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permission", permission)?;
            }
            if let Some(ref source_path) = self.source_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", source_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaVolumeMount {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaVolumeMount, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaVolumeMount;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaVolumeMount")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_group_owner: Option<::Value<bool>> = None;
                    let mut destination_path: Option<::Value<String>> = None;
                    let mut permission: Option<::Value<String>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddGroupOwner" => {
                                add_group_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationPath" => {
                                destination_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permission" => {
                                permission = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaVolumeMount {
                        add_group_owner: add_group_owner,
                        destination_path: destination_path,
                        permission: permission,
                        source_path: source_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
