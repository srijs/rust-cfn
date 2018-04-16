//! Types for the `CodeBuild` service.

/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource type.
#[derive(Debug)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug)]
pub struct ProjectProperties {
    /// Property [`Artifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-artifacts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub artifacts: ::Value<self::project::Artifacts>,
    /// Property [`BadgeEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-badgeenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub badge_enabled: Option<::Value<bool>>,
    /// Property [`Cache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-cache).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache: Option<::Value<self::project::ProjectCache>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-encryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_key: Option<::Value<String>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: ::Value<self::project::Environment>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ServiceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-servicerole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_role: ::Value<String>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-source).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source: ::Value<self::project::Source>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-timeoutinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout_in_minutes: Option<::Value<u32>>,
    /// Property [`Triggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-triggers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub triggers: Option<::Value<self::project::ProjectTriggers>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::project::VpcConfig>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Artifacts", &self.artifacts)?;
        if let Some(ref badge_enabled) = self.badge_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BadgeEnabled", badge_enabled)?;
        }
        if let Some(ref cache) = self.cache {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cache", cache)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout_in_minutes) = self.timeout_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMinutes", timeout_in_minutes)?;
        }
        if let Some(ref triggers) = self.triggers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Triggers", triggers)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
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
                let mut artifacts: Option<::Value<self::project::Artifacts>> = None;
                let mut badge_enabled: Option<::Value<bool>> = None;
                let mut cache: Option<::Value<self::project::ProjectCache>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut environment: Option<::Value<self::project::Environment>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut service_role: Option<::Value<String>> = None;
                let mut source: Option<::Value<self::project::Source>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout_in_minutes: Option<::Value<u32>> = None;
                let mut triggers: Option<::Value<self::project::ProjectTriggers>> = None;
                let mut vpc_config: Option<::Value<self::project::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Artifacts" => {
                            artifacts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BadgeEnabled" => {
                            badge_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cache" => {
                            cache = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKey" => {
                            encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRole" => {
                            service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeoutInMinutes" => {
                            timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Triggers" => {
                            triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    artifacts: artifacts.ok_or(::serde::de::Error::missing_field("Artifacts"))?,
                    badge_enabled: badge_enabled,
                    cache: cache,
                    description: description,
                    encryption_key: encryption_key,
                    environment: environment.ok_or(::serde::de::Error::missing_field("Environment"))?,
                    name: name,
                    service_role: service_role.ok_or(::serde::de::Error::missing_field("ServiceRole"))?,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    tags: tags,
                    timeout_in_minutes: timeout_in_minutes,
                    triggers: triggers,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::CodeBuild::Project";
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

    /// The [`AWS::CodeBuild::Project.Artifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html) property type.
    #[derive(Debug)]
    pub struct Artifacts {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`NamespaceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-namespacetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_type: Option<::Value<String>>,
        /// Property [`Packaging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-packaging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub packaging: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Artifacts {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref namespace_type) = self.namespace_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceType", namespace_type)?;
            }
            if let Some(ref packaging) = self.packaging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packaging", packaging)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Artifacts {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Artifacts, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Artifacts;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Artifacts")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut location: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut namespace_type: Option<::Value<String>> = None;
                    let mut packaging: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceType" => {
                                namespace_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Packaging" => {
                                packaging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Artifacts {
                        location: location,
                        name: name,
                        namespace_type: namespace_type,
                        packaging: packaging,
                        path: path,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html) property type.
    #[derive(Debug)]
    pub struct Environment {
        /// Property [`ComputeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-computetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_type: ::Value<String>,
        /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-environmentvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_variables: Option<::ValueList<EnvironmentVariable>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-image).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image: ::Value<String>,
        /// Property [`PrivilegedMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-privilegedmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub privileged_mode: Option<::Value<bool>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeType", &self.compute_type)?;
            if let Some(ref environment_variables) = self.environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref privileged_mode) = self.privileged_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivilegedMode", privileged_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_type: Option<::Value<String>> = None;
                    let mut environment_variables: Option<::ValueList<EnvironmentVariable>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut privileged_mode: Option<::Value<bool>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeType" => {
                                compute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentVariables" => {
                                environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivilegedMode" => {
                                privileged_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        compute_type: compute_type.ok_or(::serde::de::Error::missing_field("ComputeType"))?,
                        environment_variables: environment_variables,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        privileged_mode: privileged_mode,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html) property type.
    #[derive(Debug)]
    pub struct EnvironmentVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html#cfn-codebuild-project-environmentvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html#cfn-codebuild-project-environmentvariable-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html#cfn-codebuild-project-environmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentVariable {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        type_: type_,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html) property type.
    #[derive(Debug)]
    pub struct ProjectCache {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html#cfn-codebuild-project-projectcache-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html#cfn-codebuild-project-projectcache-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProjectCache {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectCache {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectCache, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectCache;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectCache")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut location: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectCache {
                        location: location,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html) property type.
    #[derive(Debug)]
    pub struct ProjectTriggers {
        /// Property [`Webhook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html#cfn-codebuild-project-projecttriggers-webhook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub webhook: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ProjectTriggers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref webhook) = self.webhook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Webhook", webhook)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectTriggers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectTriggers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectTriggers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectTriggers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut webhook: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Webhook" => {
                                webhook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectTriggers {
                        webhook: webhook,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html) property type.
    #[derive(Debug)]
    pub struct Source {
        /// Property [`Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-auth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth: Option<::Value<SourceAuth>>,
        /// Property [`BuildSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-buildspec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub build_spec: Option<::Value<String>>,
        /// Property [`GitCloneDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-gitclonedepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub git_clone_depth: Option<::Value<u32>>,
        /// Property [`InsecureSsl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-insecuressl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insecure_ssl: Option<::Value<bool>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth) = self.auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Auth", auth)?;
            }
            if let Some(ref build_spec) = self.build_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildSpec", build_spec)?;
            }
            if let Some(ref git_clone_depth) = self.git_clone_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitCloneDepth", git_clone_depth)?;
            }
            if let Some(ref insecure_ssl) = self.insecure_ssl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsecureSsl", insecure_ssl)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth: Option<::Value<SourceAuth>> = None;
                    let mut build_spec: Option<::Value<String>> = None;
                    let mut git_clone_depth: Option<::Value<u32>> = None;
                    let mut insecure_ssl: Option<::Value<bool>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Auth" => {
                                auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BuildSpec" => {
                                build_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GitCloneDepth" => {
                                git_clone_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InsecureSsl" => {
                                insecure_ssl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        auth: auth,
                        build_spec: build_spec,
                        git_clone_depth: git_clone_depth,
                        insecure_ssl: insecure_ssl,
                        location: location,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.SourceAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html) property type.
    #[derive(Debug)]
    pub struct SourceAuth {
        /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html#cfn-codebuild-project-sourceauth-resource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html#cfn-codebuild-project-sourceauth-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceAuth {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref resource) = self.resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", resource)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceAuth {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceAuth, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceAuth;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceAuth")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Resource" => {
                                resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAuth {
                        resource: resource,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html) property type.
    #[derive(Debug)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
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
                    let mut subnets: Option<::ValueList<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                        vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
