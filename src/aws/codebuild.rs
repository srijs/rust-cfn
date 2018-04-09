//! Types for the `CodeBuild` service.

/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource type.
#[derive(Debug)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug)]
pub struct ProjectProperties {
    /// Property `Artifacts`.
    pub artifacts: ::Value<self::project::Artifacts>,
    /// Property `BadgeEnabled`.
    pub badge_enabled: Option<::Value<bool>>,
    /// Property `Cache`.
    pub cache: Option<::Value<self::project::ProjectCache>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EncryptionKey`.
    pub encryption_key: Option<::Value<String>>,
    /// Property `Environment`.
    pub environment: ::Value<self::project::Environment>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `ServiceRole`.
    pub service_role: ::Value<String>,
    /// Property `Source`.
    pub source: ::Value<self::project::Source>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TimeoutInMinutes`.
    pub timeout_in_minutes: Option<::Value<u32>>,
    /// Property `Triggers`.
    pub triggers: Option<::Value<self::project::ProjectTriggers>>,
    /// Property `VpcConfig`.
    pub vpc_config: Option<::Value<self::project::VpcConfig>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Artifacts", &self.artifacts)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BadgeEnabled", &self.badge_enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cache", &self.cache)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", &self.encryption_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMinutes", &self.timeout_in_minutes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Triggers", &self.triggers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", &self.vpc_config)?;
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
                let mut artifacts = None;
                let mut badge_enabled = None;
                let mut cache = None;
                let mut description = None;
                let mut encryption_key = None;
                let mut environment = None;
                let mut name = None;
                let mut service_role = None;
                let mut source = None;
                let mut tags = None;
                let mut timeout_in_minutes = None;
                let mut triggers = None;
                let mut vpc_config = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Artifacts" => {
                            artifacts = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BadgeEnabled" => {
                            badge_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Cache" => {
                            cache = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EncryptionKey" => {
                            encryption_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Environment" => {
                            environment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceRole" => {
                            service_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Source" => {
                            source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TimeoutInMinutes" => {
                            timeout_in_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Triggers" => {
                            triggers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcConfig" => {
                            vpc_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Location`.
        pub location: Option<::Value<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `NamespaceType`.
        pub namespace_type: Option<::Value<String>>,
        /// Property `Packaging`.
        pub packaging: Option<::Value<String>>,
        /// Property `Path`.
        pub path: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Artifacts {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceType", &self.namespace_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packaging", &self.packaging)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
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
                    let mut location = None;
                    let mut name = None;
                    let mut namespace_type = None;
                    let mut packaging = None;
                    let mut path = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NamespaceType" => {
                                namespace_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Packaging" => {
                                packaging = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Path" => {
                                path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `ComputeType`.
        pub compute_type: ::Value<String>,
        /// Property `EnvironmentVariables`.
        pub environment_variables: Option<::ValueList<EnvironmentVariable>>,
        /// Property `Image`.
        pub image: ::Value<String>,
        /// Property `PrivilegedMode`.
        pub privileged_mode: Option<::Value<bool>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeType", &self.compute_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", &self.environment_variables)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivilegedMode", &self.privileged_mode)?;
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
                    let mut compute_type = None;
                    let mut environment_variables = None;
                    let mut image = None;
                    let mut privileged_mode = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeType" => {
                                compute_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EnvironmentVariables" => {
                                environment_variables = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Image" => {
                                image = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivilegedMode" => {
                                privileged_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
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
                    let mut name = None;
                    let mut type_ = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Location`.
        pub location: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProjectCache {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
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
                    let mut location = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Webhook`.
        pub webhook: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ProjectTriggers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Webhook", &self.webhook)?;
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
                    let mut webhook = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Webhook" => {
                                webhook = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Auth`.
        pub auth: Option<::Value<SourceAuth>>,
        /// Property `BuildSpec`.
        pub build_spec: Option<::Value<String>>,
        /// Property `GitCloneDepth`.
        pub git_clone_depth: Option<::Value<u32>>,
        /// Property `InsecureSsl`.
        pub insecure_ssl: Option<::Value<bool>>,
        /// Property `Location`.
        pub location: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Auth", &self.auth)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildSpec", &self.build_spec)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitCloneDepth", &self.git_clone_depth)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsecureSsl", &self.insecure_ssl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
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
                    let mut auth = None;
                    let mut build_spec = None;
                    let mut git_clone_depth = None;
                    let mut insecure_ssl = None;
                    let mut location = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Auth" => {
                                auth = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BuildSpec" => {
                                build_spec = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "GitCloneDepth" => {
                                git_clone_depth = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InsecureSsl" => {
                                insecure_ssl = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Location" => {
                                location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `Resource`.
        pub resource: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceAuth {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
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
                    let mut resource = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Resource" => {
                                resource = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
        /// Property `SecurityGroupIds`.
        pub security_group_ids: ::ValueList<String>,
        /// Property `Subnets`.
        pub subnets: ::ValueList<String>,
        /// Property `VpcId`.
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
                    let mut security_group_ids = None;
                    let mut subnets = None;
                    let mut vpc_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Subnets" => {
                                subnets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VpcId" => {
                                vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
