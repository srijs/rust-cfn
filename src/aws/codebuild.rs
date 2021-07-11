//! Types for the `CodeBuild` service.

/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
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
    /// Property [`BuildBatchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-buildbatchconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub build_batch_config: Option<::Value<self::project::ProjectBuildBatchConfig>>,
    /// Property [`Cache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-cache).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache: Option<::Value<self::project::ProjectCache>>,
    /// Property [`ConcurrentBuildLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-concurrentbuildlimit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub concurrent_build_limit: Option<::Value<u32>>,
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
    /// Property [`FileSystemLocations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-filesystemlocations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_system_locations: Option<::ValueList<self::project::ProjectFileSystemLocation>>,
    /// Property [`LogsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-logsconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logs_config: Option<::Value<self::project::LogsConfig>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`QueuedTimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-queuedtimeoutinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub queued_timeout_in_minutes: Option<::Value<u32>>,
    /// Property [`SecondaryArtifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-secondaryartifacts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secondary_artifacts: Option<::ValueList<self::project::Artifacts>>,
    /// Property [`SecondarySourceVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-secondarysourceversions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secondary_source_versions: Option<::ValueList<self::project::ProjectSourceVersion>>,
    /// Property [`SecondarySources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-secondarysources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secondary_sources: Option<::ValueList<self::project::Source>>,
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
    /// Property [`SourceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html#cfn-codebuild-project-sourceversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_version: Option<::Value<String>>,
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
        if let Some(ref build_batch_config) = self.build_batch_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildBatchConfig", build_batch_config)?;
        }
        if let Some(ref cache) = self.cache {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cache", cache)?;
        }
        if let Some(ref concurrent_build_limit) = self.concurrent_build_limit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConcurrentBuildLimit", concurrent_build_limit)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
        if let Some(ref file_system_locations) = self.file_system_locations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemLocations", file_system_locations)?;
        }
        if let Some(ref logs_config) = self.logs_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogsConfig", logs_config)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref queued_timeout_in_minutes) = self.queued_timeout_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueuedTimeoutInMinutes", queued_timeout_in_minutes)?;
        }
        if let Some(ref secondary_artifacts) = self.secondary_artifacts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryArtifacts", secondary_artifacts)?;
        }
        if let Some(ref secondary_source_versions) = self.secondary_source_versions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondarySourceVersions", secondary_source_versions)?;
        }
        if let Some(ref secondary_sources) = self.secondary_sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondarySources", secondary_sources)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref source_version) = self.source_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVersion", source_version)?;
        }
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
                let mut build_batch_config: Option<::Value<self::project::ProjectBuildBatchConfig>> = None;
                let mut cache: Option<::Value<self::project::ProjectCache>> = None;
                let mut concurrent_build_limit: Option<::Value<u32>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut environment: Option<::Value<self::project::Environment>> = None;
                let mut file_system_locations: Option<::ValueList<self::project::ProjectFileSystemLocation>> = None;
                let mut logs_config: Option<::Value<self::project::LogsConfig>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut queued_timeout_in_minutes: Option<::Value<u32>> = None;
                let mut secondary_artifacts: Option<::ValueList<self::project::Artifacts>> = None;
                let mut secondary_source_versions: Option<::ValueList<self::project::ProjectSourceVersion>> = None;
                let mut secondary_sources: Option<::ValueList<self::project::Source>> = None;
                let mut service_role: Option<::Value<String>> = None;
                let mut source: Option<::Value<self::project::Source>> = None;
                let mut source_version: Option<::Value<String>> = None;
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
                        "BuildBatchConfig" => {
                            build_batch_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cache" => {
                            cache = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConcurrentBuildLimit" => {
                            concurrent_build_limit = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "FileSystemLocations" => {
                            file_system_locations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogsConfig" => {
                            logs_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueuedTimeoutInMinutes" => {
                            queued_timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecondaryArtifacts" => {
                            secondary_artifacts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecondarySourceVersions" => {
                            secondary_source_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecondarySources" => {
                            secondary_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRole" => {
                            service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceVersion" => {
                            source_version = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    build_batch_config: build_batch_config,
                    cache: cache,
                    concurrent_build_limit: concurrent_build_limit,
                    description: description,
                    encryption_key: encryption_key,
                    environment: environment.ok_or(::serde::de::Error::missing_field("Environment"))?,
                    file_system_locations: file_system_locations,
                    logs_config: logs_config,
                    name: name,
                    queued_timeout_in_minutes: queued_timeout_in_minutes,
                    secondary_artifacts: secondary_artifacts,
                    secondary_source_versions: secondary_source_versions,
                    secondary_sources: secondary_sources,
                    service_role: service_role.ok_or(::serde::de::Error::missing_field("ServiceRole"))?,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    source_version: source_version,
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

/// The [`AWS::CodeBuild::ReportGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html) resource type.
#[derive(Debug, Default)]
pub struct ReportGroup {
    properties: ReportGroupProperties
}

/// Properties for the `ReportGroup` resource.
#[derive(Debug, Default)]
pub struct ReportGroupProperties {
    /// Property [`DeleteReports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html#cfn-codebuild-reportgroup-deletereports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_reports: Option<::Value<bool>>,
    /// Property [`ExportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html#cfn-codebuild-reportgroup-exportconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub export_config: ::Value<self::report_group::ReportExportConfig>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html#cfn-codebuild-reportgroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html#cfn-codebuild-reportgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html#cfn-codebuild-reportgroup-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ReportGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delete_reports) = self.delete_reports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteReports", delete_reports)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportConfig", &self.export_config)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReportGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReportGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReportGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut delete_reports: Option<::Value<bool>> = None;
                let mut export_config: Option<::Value<self::report_group::ReportExportConfig>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeleteReports" => {
                            delete_reports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExportConfig" => {
                            export_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ReportGroupProperties {
                    delete_reports: delete_reports,
                    export_config: export_config.ok_or(::serde::de::Error::missing_field("ExportConfig"))?,
                    name: name,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReportGroup {
    type Properties = ReportGroupProperties;
    const TYPE: &'static str = "AWS::CodeBuild::ReportGroup";
    fn properties(&self) -> &ReportGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReportGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReportGroup {}

impl From<ReportGroupProperties> for ReportGroup {
    fn from(properties: ReportGroupProperties) -> ReportGroup {
        ReportGroup { properties }
    }
}

/// The [`AWS::CodeBuild::SourceCredential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html) resource type.
#[derive(Debug, Default)]
pub struct SourceCredential {
    properties: SourceCredentialProperties
}

/// Properties for the `SourceCredential` resource.
#[derive(Debug, Default)]
pub struct SourceCredentialProperties {
    /// Property [`AuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html#cfn-codebuild-sourcecredential-authtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth_type: ::Value<String>,
    /// Property [`ServerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html#cfn-codebuild-sourcecredential-servertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_type: ::Value<String>,
    /// Property [`Token`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html#cfn-codebuild-sourcecredential-token).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token: ::Value<String>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html#cfn-codebuild-sourcecredential-username).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username: Option<::Value<String>>,
}

impl ::serde::Serialize for SourceCredentialProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", &self.auth_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerType", &self.server_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Token", &self.token)?;
        if let Some(ref username) = self.username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SourceCredentialProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceCredentialProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SourceCredentialProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SourceCredentialProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth_type: Option<::Value<String>> = None;
                let mut server_type: Option<::Value<String>> = None;
                let mut token: Option<::Value<String>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthType" => {
                            auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerType" => {
                            server_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Token" => {
                            token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SourceCredentialProperties {
                    auth_type: auth_type.ok_or(::serde::de::Error::missing_field("AuthType"))?,
                    server_type: server_type.ok_or(::serde::de::Error::missing_field("ServerType"))?,
                    token: token.ok_or(::serde::de::Error::missing_field("Token"))?,
                    username: username,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SourceCredential {
    type Properties = SourceCredentialProperties;
    const TYPE: &'static str = "AWS::CodeBuild::SourceCredential";
    fn properties(&self) -> &SourceCredentialProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SourceCredentialProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SourceCredential {}

impl From<SourceCredentialProperties> for SourceCredential {
    fn from(properties: SourceCredentialProperties) -> SourceCredential {
        SourceCredential { properties }
    }
}

pub mod project {
    //! Property types for the `Project` resource.

    /// The [`AWS::CodeBuild::Project.Artifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html) property type.
    #[derive(Debug, Default)]
    pub struct Artifacts {
        /// Property [`ArtifactIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-artifactidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub artifact_identifier: Option<::Value<String>>,
        /// Property [`EncryptionDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-encryptiondisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_disabled: Option<::Value<bool>>,
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
        /// Property [`OverrideArtifactName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html#cfn-codebuild-project-artifacts-overrideartifactname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub override_artifact_name: Option<::Value<bool>>,
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
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Artifacts {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref artifact_identifier) = self.artifact_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArtifactIdentifier", artifact_identifier)?;
            }
            if let Some(ref encryption_disabled) = self.encryption_disabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionDisabled", encryption_disabled)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref namespace_type) = self.namespace_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceType", namespace_type)?;
            }
            if let Some(ref override_artifact_name) = self.override_artifact_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverrideArtifactName", override_artifact_name)?;
            }
            if let Some(ref packaging) = self.packaging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packaging", packaging)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut artifact_identifier: Option<::Value<String>> = None;
                    let mut encryption_disabled: Option<::Value<bool>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut namespace_type: Option<::Value<String>> = None;
                    let mut override_artifact_name: Option<::Value<bool>> = None;
                    let mut packaging: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArtifactIdentifier" => {
                                artifact_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionDisabled" => {
                                encryption_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceType" => {
                                namespace_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OverrideArtifactName" => {
                                override_artifact_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Packaging" => {
                                packaging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Artifacts {
                        artifact_identifier: artifact_identifier,
                        encryption_disabled: encryption_disabled,
                        location: location,
                        name: name,
                        namespace_type: namespace_type,
                        override_artifact_name: override_artifact_name,
                        packaging: packaging,
                        path: path,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.BatchRestrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-batchrestrictions.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchRestrictions {
        /// Property [`ComputeTypesAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-batchrestrictions.html#cfn-codebuild-project-batchrestrictions-computetypesallowed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_types_allowed: Option<::ValueList<String>>,
        /// Property [`MaximumBuildsAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-batchrestrictions.html#cfn-codebuild-project-batchrestrictions-maximumbuildsallowed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_builds_allowed: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BatchRestrictions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compute_types_allowed) = self.compute_types_allowed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeTypesAllowed", compute_types_allowed)?;
            }
            if let Some(ref maximum_builds_allowed) = self.maximum_builds_allowed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBuildsAllowed", maximum_builds_allowed)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchRestrictions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchRestrictions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchRestrictions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchRestrictions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_types_allowed: Option<::ValueList<String>> = None;
                    let mut maximum_builds_allowed: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeTypesAllowed" => {
                                compute_types_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBuildsAllowed" => {
                                maximum_builds_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchRestrictions {
                        compute_types_allowed: compute_types_allowed,
                        maximum_builds_allowed: maximum_builds_allowed,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.BuildStatusConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-buildstatusconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct BuildStatusConfig {
        /// Property [`Context`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-buildstatusconfig.html#cfn-codebuild-project-buildstatusconfig-context).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub context: Option<::Value<String>>,
        /// Property [`TargetUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-buildstatusconfig.html#cfn-codebuild-project-buildstatusconfig-targeturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BuildStatusConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref context) = self.context {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Context", context)?;
            }
            if let Some(ref target_url) = self.target_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetUrl", target_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BuildStatusConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BuildStatusConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BuildStatusConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BuildStatusConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut context: Option<::Value<String>> = None;
                    let mut target_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Context" => {
                                context = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetUrl" => {
                                target_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BuildStatusConfig {
                        context: context,
                        target_url: target_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.CloudWatchLogsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-cloudwatchlogsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogsConfig {
        /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-cloudwatchlogsconfig.html#cfn-codebuild-project-cloudwatchlogsconfig-groupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_name: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-cloudwatchlogsconfig.html#cfn-codebuild-project-cloudwatchlogsconfig-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-cloudwatchlogsconfig.html#cfn-codebuild-project-cloudwatchlogsconfig-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchLogsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group_name) = self.group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            if let Some(ref stream_name) = self.stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", stream_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_name: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupName" => {
                                group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogsConfig {
                        group_name: group_name,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                        stream_name: stream_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: Option<::Value<String>>,
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
        /// Property [`ImagePullCredentialsType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-imagepullcredentialstype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_pull_credentials_type: Option<::Value<String>>,
        /// Property [`PrivilegedMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-privilegedmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub privileged_mode: Option<::Value<bool>>,
        /// Property [`RegistryCredential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-registrycredential).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_credential: Option<::Value<RegistryCredential>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html#cfn-codebuild-project-environment-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate) = self.certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeType", &self.compute_type)?;
            if let Some(ref environment_variables) = self.environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref image_pull_credentials_type) = self.image_pull_credentials_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImagePullCredentialsType", image_pull_credentials_type)?;
            }
            if let Some(ref privileged_mode) = self.privileged_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivilegedMode", privileged_mode)?;
            }
            if let Some(ref registry_credential) = self.registry_credential {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryCredential", registry_credential)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut certificate: Option<::Value<String>> = None;
                    let mut compute_type: Option<::Value<String>> = None;
                    let mut environment_variables: Option<::ValueList<EnvironmentVariable>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut image_pull_credentials_type: Option<::Value<String>> = None;
                    let mut privileged_mode: Option<::Value<bool>> = None;
                    let mut registry_credential: Option<::Value<RegistryCredential>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComputeType" => {
                                compute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentVariables" => {
                                environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImagePullCredentialsType" => {
                                image_pull_credentials_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivilegedMode" => {
                                privileged_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistryCredential" => {
                                registry_credential = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        certificate: certificate,
                        compute_type: compute_type.ok_or(::serde::de::Error::missing_field("ComputeType"))?,
                        environment_variables: environment_variables,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        image_pull_credentials_type: image_pull_credentials_type,
                        privileged_mode: privileged_mode,
                        registry_credential: registry_credential,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html) property type.
    #[derive(Debug, Default)]
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
        pub r#type: Option<::Value<String>>,
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
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
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
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentVariable {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.FilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-filtergroup.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterGroup {
    }

    impl ::codec::SerializeValue for FilterGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(FilterGroup {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.GitSubmodulesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-gitsubmodulesconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct GitSubmodulesConfig {
        /// Property [`FetchSubmodules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-gitsubmodulesconfig.html#cfn-codebuild-project-gitsubmodulesconfig-fetchsubmodules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fetch_submodules: ::Value<bool>,
    }

    impl ::codec::SerializeValue for GitSubmodulesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FetchSubmodules", &self.fetch_submodules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GitSubmodulesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GitSubmodulesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GitSubmodulesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GitSubmodulesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fetch_submodules: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FetchSubmodules" => {
                                fetch_submodules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GitSubmodulesConfig {
                        fetch_submodules: fetch_submodules.ok_or(::serde::de::Error::missing_field("FetchSubmodules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.LogsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-logsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LogsConfig {
        /// Property [`CloudWatchLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-logsconfig.html#cfn-codebuild-project-logsconfig-cloudwatchlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs: Option<::Value<CloudWatchLogsConfig>>,
        /// Property [`S3Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-logsconfig.html#cfn-codebuild-project-logsconfig-s3logs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_logs: Option<::Value<S3LogsConfig>>,
    }

    impl ::codec::SerializeValue for LogsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs) = self.cloud_watch_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogs", cloud_watch_logs)?;
            }
            if let Some(ref s3_logs) = self.s3_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Logs", s3_logs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs: Option<::Value<CloudWatchLogsConfig>> = None;
                    let mut s3_logs: Option<::Value<S3LogsConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogs" => {
                                cloud_watch_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Logs" => {
                                s3_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogsConfig {
                        cloud_watch_logs: cloud_watch_logs,
                        s3_logs: s3_logs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectBuildBatchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectBuildBatchConfig {
        /// Property [`CombineArtifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html#cfn-codebuild-project-projectbuildbatchconfig-combineartifacts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub combine_artifacts: Option<::Value<bool>>,
        /// Property [`Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html#cfn-codebuild-project-projectbuildbatchconfig-restrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restrictions: Option<::Value<BatchRestrictions>>,
        /// Property [`ServiceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html#cfn-codebuild-project-projectbuildbatchconfig-servicerole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_role: Option<::Value<String>>,
        /// Property [`TimeoutInMins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html#cfn-codebuild-project-projectbuildbatchconfig-timeoutinmins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_mins: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ProjectBuildBatchConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref combine_artifacts) = self.combine_artifacts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CombineArtifacts", combine_artifacts)?;
            }
            if let Some(ref restrictions) = self.restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Restrictions", restrictions)?;
            }
            if let Some(ref service_role) = self.service_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", service_role)?;
            }
            if let Some(ref timeout_in_mins) = self.timeout_in_mins {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMins", timeout_in_mins)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectBuildBatchConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectBuildBatchConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectBuildBatchConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectBuildBatchConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut combine_artifacts: Option<::Value<bool>> = None;
                    let mut restrictions: Option<::Value<BatchRestrictions>> = None;
                    let mut service_role: Option<::Value<String>> = None;
                    let mut timeout_in_mins: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CombineArtifacts" => {
                                combine_artifacts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Restrictions" => {
                                restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceRole" => {
                                service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInMins" => {
                                timeout_in_mins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectBuildBatchConfig {
                        combine_artifacts: combine_artifacts,
                        restrictions: restrictions,
                        service_role: service_role,
                        timeout_in_mins: timeout_in_mins,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectCache {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html#cfn-codebuild-project-projectcache-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Modes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html#cfn-codebuild-project-projectcache-modes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub modes: Option<::ValueList<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html#cfn-codebuild-project-projectcache-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProjectCache {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref modes) = self.modes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Modes", modes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut modes: Option<::ValueList<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Modes" => {
                                modes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectCache {
                        location: location,
                        modes: modes,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectFileSystemLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectFileSystemLocation {
        /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html#cfn-codebuild-project-projectfilesystemlocation-identifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifier: ::Value<String>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html#cfn-codebuild-project-projectfilesystemlocation-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: ::Value<String>,
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html#cfn-codebuild-project-projectfilesystemlocation-mountoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_options: Option<::Value<String>>,
        /// Property [`MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html#cfn-codebuild-project-projectfilesystemlocation-mountpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_point: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html#cfn-codebuild-project-projectfilesystemlocation-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProjectFileSystemLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", &self.identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            if let Some(ref mount_options) = self.mount_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", mount_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoint", &self.mount_point)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectFileSystemLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectFileSystemLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectFileSystemLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectFileSystemLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut identifier: Option<::Value<String>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut mount_options: Option<::Value<String>> = None;
                    let mut mount_point: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Identifier" => {
                                identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPoint" => {
                                mount_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectFileSystemLocation {
                        identifier: identifier.ok_or(::serde::de::Error::missing_field("Identifier"))?,
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                        mount_options: mount_options,
                        mount_point: mount_point.ok_or(::serde::de::Error::missing_field("MountPoint"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectSourceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectsourceversion.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectSourceVersion {
        /// Property [`SourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectsourceversion.html#cfn-codebuild-project-projectsourceversion-sourceidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_identifier: ::Value<String>,
        /// Property [`SourceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectsourceversion.html#cfn-codebuild-project-projectsourceversion-sourceversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProjectSourceVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIdentifier", &self.source_identifier)?;
            if let Some(ref source_version) = self.source_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVersion", source_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectSourceVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectSourceVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectSourceVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectSourceVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_identifier: Option<::Value<String>> = None;
                    let mut source_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceIdentifier" => {
                                source_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceVersion" => {
                                source_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectSourceVersion {
                        source_identifier: source_identifier.ok_or(::serde::de::Error::missing_field("SourceIdentifier"))?,
                        source_version: source_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.ProjectTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectTriggers {
        /// Property [`BuildType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html#cfn-codebuild-project-projecttriggers-buildtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub build_type: Option<::Value<String>>,
        /// Property [`FilterGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html#cfn-codebuild-project-projecttriggers-filtergroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_groups: Option<::ValueList<FilterGroup>>,
        /// Property [`Webhook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html#cfn-codebuild-project-projecttriggers-webhook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub webhook: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ProjectTriggers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref build_type) = self.build_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildType", build_type)?;
            }
            if let Some(ref filter_groups) = self.filter_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterGroups", filter_groups)?;
            }
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
                    let mut build_type: Option<::Value<String>> = None;
                    let mut filter_groups: Option<::ValueList<FilterGroup>> = None;
                    let mut webhook: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BuildType" => {
                                build_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterGroups" => {
                                filter_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Webhook" => {
                                webhook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectTriggers {
                        build_type: build_type,
                        filter_groups: filter_groups,
                        webhook: webhook,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.RegistryCredential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-registrycredential.html) property type.
    #[derive(Debug, Default)]
    pub struct RegistryCredential {
        /// Property [`Credential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-registrycredential.html#cfn-codebuild-project-registrycredential-credential).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credential: ::Value<String>,
        /// Property [`CredentialProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-registrycredential.html#cfn-codebuild-project-registrycredential-credentialprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credential_provider: ::Value<String>,
    }

    impl ::codec::SerializeValue for RegistryCredential {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credential", &self.credential)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialProvider", &self.credential_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegistryCredential {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegistryCredential, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegistryCredential;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegistryCredential")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credential: Option<::Value<String>> = None;
                    let mut credential_provider: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Credential" => {
                                credential = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CredentialProvider" => {
                                credential_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegistryCredential {
                        credential: credential.ok_or(::serde::de::Error::missing_field("Credential"))?,
                        credential_provider: credential_provider.ok_or(::serde::de::Error::missing_field("CredentialProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.S3LogsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-s3logsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3LogsConfig {
        /// Property [`EncryptionDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-s3logsconfig.html#cfn-codebuild-project-s3logsconfig-encryptiondisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_disabled: Option<::Value<bool>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-s3logsconfig.html#cfn-codebuild-project-s3logsconfig-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-s3logsconfig.html#cfn-codebuild-project-s3logsconfig-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3LogsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption_disabled) = self.encryption_disabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionDisabled", encryption_disabled)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3LogsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3LogsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3LogsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3LogsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_disabled: Option<::Value<bool>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionDisabled" => {
                                encryption_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3LogsConfig {
                        encryption_disabled: encryption_disabled,
                        location: location,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html) property type.
    #[derive(Debug, Default)]
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
        /// Property [`BuildStatusConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-buildstatusconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub build_status_config: Option<::Value<BuildStatusConfig>>,
        /// Property [`GitCloneDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-gitclonedepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub git_clone_depth: Option<::Value<u32>>,
        /// Property [`GitSubmodulesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-gitsubmodulesconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub git_submodules_config: Option<::Value<GitSubmodulesConfig>>,
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
        /// Property [`ReportBuildStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-reportbuildstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_build_status: Option<::Value<bool>>,
        /// Property [`SourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-sourceidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_identifier: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html#cfn-codebuild-project-source-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
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
            if let Some(ref build_status_config) = self.build_status_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildStatusConfig", build_status_config)?;
            }
            if let Some(ref git_clone_depth) = self.git_clone_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitCloneDepth", git_clone_depth)?;
            }
            if let Some(ref git_submodules_config) = self.git_submodules_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitSubmodulesConfig", git_submodules_config)?;
            }
            if let Some(ref insecure_ssl) = self.insecure_ssl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsecureSsl", insecure_ssl)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref report_build_status) = self.report_build_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportBuildStatus", report_build_status)?;
            }
            if let Some(ref source_identifier) = self.source_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIdentifier", source_identifier)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut build_status_config: Option<::Value<BuildStatusConfig>> = None;
                    let mut git_clone_depth: Option<::Value<u32>> = None;
                    let mut git_submodules_config: Option<::Value<GitSubmodulesConfig>> = None;
                    let mut insecure_ssl: Option<::Value<bool>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut report_build_status: Option<::Value<bool>> = None;
                    let mut source_identifier: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Auth" => {
                                auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BuildSpec" => {
                                build_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BuildStatusConfig" => {
                                build_status_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GitCloneDepth" => {
                                git_clone_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GitSubmodulesConfig" => {
                                git_submodules_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InsecureSsl" => {
                                insecure_ssl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportBuildStatus" => {
                                report_build_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceIdentifier" => {
                                source_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        auth: auth,
                        build_spec: build_spec,
                        build_status_config: build_status_config,
                        git_clone_depth: git_clone_depth,
                        git_submodules_config: git_submodules_config,
                        insecure_ssl: insecure_ssl,
                        location: location,
                        report_build_status: report_build_status,
                        source_identifier: source_identifier,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.SourceAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html) property type.
    #[derive(Debug, Default)]
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
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceAuth {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref resource) = self.resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", resource)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Resource" => {
                                resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAuth {
                        resource: resource,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: Option<::ValueList<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html#cfn-codebuild-project-vpcconfig-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnets) = self.subnets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
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
                        security_group_ids: security_group_ids,
                        subnets: subnets,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::Project.WebhookFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-webhookfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct WebhookFilter {
        /// Property [`ExcludeMatchedPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-webhookfilter.html#cfn-codebuild-project-webhookfilter-excludematchedpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_matched_pattern: Option<::Value<bool>>,
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-webhookfilter.html#cfn-codebuild-project-webhookfilter-pattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-webhookfilter.html#cfn-codebuild-project-webhookfilter-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for WebhookFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclude_matched_pattern) = self.exclude_matched_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeMatchedPattern", exclude_matched_pattern)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", &self.pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebhookFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebhookFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebhookFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebhookFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exclude_matched_pattern: Option<::Value<bool>> = None;
                    let mut pattern: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludeMatchedPattern" => {
                                exclude_matched_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pattern" => {
                                pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebhookFilter {
                        exclude_matched_pattern: exclude_matched_pattern,
                        pattern: pattern.ok_or(::serde::de::Error::missing_field("Pattern"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod report_group {
    //! Property types for the `ReportGroup` resource.

    /// The [`AWS::CodeBuild::ReportGroup.ReportExportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-reportexportconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ReportExportConfig {
        /// Property [`ExportConfigType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-reportexportconfig.html#cfn-codebuild-reportgroup-reportexportconfig-exportconfigtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub export_config_type: ::Value<String>,
        /// Property [`S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-reportexportconfig.html#cfn-codebuild-reportgroup-reportexportconfig-s3destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_destination: Option<::Value<S3ReportExportConfig>>,
    }

    impl ::codec::SerializeValue for ReportExportConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportConfigType", &self.export_config_type)?;
            if let Some(ref s3_destination) = self.s3_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Destination", s3_destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReportExportConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportExportConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReportExportConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReportExportConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut export_config_type: Option<::Value<String>> = None;
                    let mut s3_destination: Option<::Value<S3ReportExportConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExportConfigType" => {
                                export_config_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Destination" => {
                                s3_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReportExportConfig {
                        export_config_type: export_config_type.ok_or(::serde::de::Error::missing_field("ExportConfigType"))?,
                        s3_destination: s3_destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeBuild::ReportGroup.S3ReportExportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3ReportExportConfig {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`BucketOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-bucketowner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_owner: Option<::Value<String>>,
        /// Property [`EncryptionDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-encryptiondisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_disabled: Option<::Value<bool>>,
        /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-encryptionkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_key: Option<::Value<String>>,
        /// Property [`Packaging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-packaging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub packaging: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html#cfn-codebuild-reportgroup-s3reportexportconfig-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3ReportExportConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref bucket_owner) = self.bucket_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketOwner", bucket_owner)?;
            }
            if let Some(ref encryption_disabled) = self.encryption_disabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionDisabled", encryption_disabled)?;
            }
            if let Some(ref encryption_key) = self.encryption_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
            }
            if let Some(ref packaging) = self.packaging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packaging", packaging)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3ReportExportConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3ReportExportConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3ReportExportConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3ReportExportConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut bucket_owner: Option<::Value<String>> = None;
                    let mut encryption_disabled: Option<::Value<bool>> = None;
                    let mut encryption_key: Option<::Value<String>> = None;
                    let mut packaging: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketOwner" => {
                                bucket_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionDisabled" => {
                                encryption_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionKey" => {
                                encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Packaging" => {
                                packaging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ReportExportConfig {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        bucket_owner: bucket_owner,
                        encryption_disabled: encryption_disabled,
                        encryption_key: encryption_key,
                        packaging: packaging,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
