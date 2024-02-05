//! Types for the `Amplify` service.

/// The [`AWS::Amplify::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html) resource type.
#[derive(Debug, Default)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Default)]
pub struct AppProperties {
    /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-accesstoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_token: Option<::Value<String>>,
    /// Property [`AutoBranchCreationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-autobranchcreationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_branch_creation_config: Option<::Value<self::app::AutoBranchCreationConfig>>,
    /// Property [`BasicAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-basicauthconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub basic_auth_config: Option<::Value<self::app::BasicAuthConfig>>,
    /// Property [`BuildSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-buildspec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub build_spec: Option<::Value<String>>,
    /// Property [`CustomHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-customheaders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_headers: Option<::Value<String>>,
    /// Property [`CustomRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-customrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_rules: Option<::ValueList<self::app::CustomRule>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EnableBranchAutoDeletion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-enablebranchautodeletion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_branch_auto_deletion: Option<::Value<bool>>,
    /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-environmentvariables).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_variables: Option<::ValueList<self::app::EnvironmentVariable>>,
    /// Property [`IAMServiceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-iamservicerole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_service_role: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OauthToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-oauthtoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub oauth_token: Option<::Value<String>>,
    /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-platform).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub platform: Option<::Value<String>>,
    /// Property [`Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-repository).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html#cfn-amplify-app-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_token) = self.access_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
        }
        if let Some(ref auto_branch_creation_config) = self.auto_branch_creation_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoBranchCreationConfig", auto_branch_creation_config)?;
        }
        if let Some(ref basic_auth_config) = self.basic_auth_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthConfig", basic_auth_config)?;
        }
        if let Some(ref build_spec) = self.build_spec {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildSpec", build_spec)?;
        }
        if let Some(ref custom_headers) = self.custom_headers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomHeaders", custom_headers)?;
        }
        if let Some(ref custom_rules) = self.custom_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRules", custom_rules)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enable_branch_auto_deletion) = self.enable_branch_auto_deletion {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableBranchAutoDeletion", enable_branch_auto_deletion)?;
        }
        if let Some(ref environment_variables) = self.environment_variables {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
        }
        if let Some(ref iam_service_role) = self.iam_service_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMServiceRole", iam_service_role)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref oauth_token) = self.oauth_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OauthToken", oauth_token)?;
        }
        if let Some(ref platform) = self.platform {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", platform)?;
        }
        if let Some(ref repository) = self.repository {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Repository", repository)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_token: Option<::Value<String>> = None;
                let mut auto_branch_creation_config: Option<::Value<self::app::AutoBranchCreationConfig>> = None;
                let mut basic_auth_config: Option<::Value<self::app::BasicAuthConfig>> = None;
                let mut build_spec: Option<::Value<String>> = None;
                let mut custom_headers: Option<::Value<String>> = None;
                let mut custom_rules: Option<::ValueList<self::app::CustomRule>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut enable_branch_auto_deletion: Option<::Value<bool>> = None;
                let mut environment_variables: Option<::ValueList<self::app::EnvironmentVariable>> = None;
                let mut iam_service_role: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut oauth_token: Option<::Value<String>> = None;
                let mut platform: Option<::Value<String>> = None;
                let mut repository: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessToken" => {
                            access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoBranchCreationConfig" => {
                            auto_branch_creation_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BasicAuthConfig" => {
                            basic_auth_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BuildSpec" => {
                            build_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomHeaders" => {
                            custom_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomRules" => {
                            custom_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableBranchAutoDeletion" => {
                            enable_branch_auto_deletion = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentVariables" => {
                            environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IAMServiceRole" => {
                            iam_service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OauthToken" => {
                            oauth_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Platform" => {
                            platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Repository" => {
                            repository = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppProperties {
                    access_token: access_token,
                    auto_branch_creation_config: auto_branch_creation_config,
                    basic_auth_config: basic_auth_config,
                    build_spec: build_spec,
                    custom_headers: custom_headers,
                    custom_rules: custom_rules,
                    description: description,
                    enable_branch_auto_deletion: enable_branch_auto_deletion,
                    environment_variables: environment_variables,
                    iam_service_role: iam_service_role,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    oauth_token: oauth_token,
                    platform: platform,
                    repository: repository,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::Amplify::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::Amplify::Branch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html) resource type.
#[derive(Debug, Default)]
pub struct Branch {
    properties: BranchProperties
}

/// Properties for the `Branch` resource.
#[derive(Debug, Default)]
pub struct BranchProperties {
    /// Property [`AppId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-appid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_id: ::Value<String>,
    /// Property [`Backend`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-backend).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backend: Option<::Value<self::branch::Backend>>,
    /// Property [`BasicAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-basicauthconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub basic_auth_config: Option<::Value<self::branch::BasicAuthConfig>>,
    /// Property [`BranchName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-branchname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub branch_name: ::Value<String>,
    /// Property [`BuildSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-buildspec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub build_spec: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EnableAutoBuild`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-enableautobuild).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_auto_build: Option<::Value<bool>>,
    /// Property [`EnablePerformanceMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-enableperformancemode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_performance_mode: Option<::Value<bool>>,
    /// Property [`EnablePullRequestPreview`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-enablepullrequestpreview).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_pull_request_preview: Option<::Value<bool>>,
    /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-environmentvariables).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_variables: Option<::ValueList<self::branch::EnvironmentVariable>>,
    /// Property [`Framework`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-framework).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub framework: Option<::Value<String>>,
    /// Property [`PullRequestEnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-pullrequestenvironmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pull_request_environment_name: Option<::Value<String>>,
    /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-stage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html#cfn-amplify-branch-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BranchProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppId", &self.app_id)?;
        if let Some(ref backend) = self.backend {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Backend", backend)?;
        }
        if let Some(ref basic_auth_config) = self.basic_auth_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthConfig", basic_auth_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BranchName", &self.branch_name)?;
        if let Some(ref build_spec) = self.build_spec {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildSpec", build_spec)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enable_auto_build) = self.enable_auto_build {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoBuild", enable_auto_build)?;
        }
        if let Some(ref enable_performance_mode) = self.enable_performance_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePerformanceMode", enable_performance_mode)?;
        }
        if let Some(ref enable_pull_request_preview) = self.enable_pull_request_preview {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePullRequestPreview", enable_pull_request_preview)?;
        }
        if let Some(ref environment_variables) = self.environment_variables {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
        }
        if let Some(ref framework) = self.framework {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Framework", framework)?;
        }
        if let Some(ref pull_request_environment_name) = self.pull_request_environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PullRequestEnvironmentName", pull_request_environment_name)?;
        }
        if let Some(ref stage) = self.stage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", stage)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BranchProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BranchProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BranchProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BranchProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_id: Option<::Value<String>> = None;
                let mut backend: Option<::Value<self::branch::Backend>> = None;
                let mut basic_auth_config: Option<::Value<self::branch::BasicAuthConfig>> = None;
                let mut branch_name: Option<::Value<String>> = None;
                let mut build_spec: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut enable_auto_build: Option<::Value<bool>> = None;
                let mut enable_performance_mode: Option<::Value<bool>> = None;
                let mut enable_pull_request_preview: Option<::Value<bool>> = None;
                let mut environment_variables: Option<::ValueList<self::branch::EnvironmentVariable>> = None;
                let mut framework: Option<::Value<String>> = None;
                let mut pull_request_environment_name: Option<::Value<String>> = None;
                let mut stage: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppId" => {
                            app_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Backend" => {
                            backend = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BasicAuthConfig" => {
                            basic_auth_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BranchName" => {
                            branch_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BuildSpec" => {
                            build_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableAutoBuild" => {
                            enable_auto_build = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnablePerformanceMode" => {
                            enable_performance_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnablePullRequestPreview" => {
                            enable_pull_request_preview = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentVariables" => {
                            environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Framework" => {
                            framework = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PullRequestEnvironmentName" => {
                            pull_request_environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stage" => {
                            stage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BranchProperties {
                    app_id: app_id.ok_or(::serde::de::Error::missing_field("AppId"))?,
                    backend: backend,
                    basic_auth_config: basic_auth_config,
                    branch_name: branch_name.ok_or(::serde::de::Error::missing_field("BranchName"))?,
                    build_spec: build_spec,
                    description: description,
                    enable_auto_build: enable_auto_build,
                    enable_performance_mode: enable_performance_mode,
                    enable_pull_request_preview: enable_pull_request_preview,
                    environment_variables: environment_variables,
                    framework: framework,
                    pull_request_environment_name: pull_request_environment_name,
                    stage: stage,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Branch {
    type Properties = BranchProperties;
    const TYPE: &'static str = "AWS::Amplify::Branch";
    fn properties(&self) -> &BranchProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BranchProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Branch {}

impl From<BranchProperties> for Branch {
    fn from(properties: BranchProperties) -> Branch {
        Branch { properties }
    }
}

/// The [`AWS::Amplify::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`AppId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-appid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_id: ::Value<String>,
    /// Property [`AutoSubDomainCreationPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-autosubdomaincreationpatterns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_sub_domain_creation_patterns: Option<::ValueList<String>>,
    /// Property [`AutoSubDomainIAMRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-autosubdomainiamrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_sub_domain_iam_role: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`EnableAutoSubDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-enableautosubdomain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_auto_sub_domain: Option<::Value<bool>>,
    /// Property [`SubDomainSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html#cfn-amplify-domain-subdomainsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sub_domain_settings: ::ValueList<self::domain::SubDomainSetting>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppId", &self.app_id)?;
        if let Some(ref auto_sub_domain_creation_patterns) = self.auto_sub_domain_creation_patterns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoSubDomainCreationPatterns", auto_sub_domain_creation_patterns)?;
        }
        if let Some(ref auto_sub_domain_iam_role) = self.auto_sub_domain_iam_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoSubDomainIAMRole", auto_sub_domain_iam_role)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref enable_auto_sub_domain) = self.enable_auto_sub_domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoSubDomain", enable_auto_sub_domain)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubDomainSettings", &self.sub_domain_settings)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_id: Option<::Value<String>> = None;
                let mut auto_sub_domain_creation_patterns: Option<::ValueList<String>> = None;
                let mut auto_sub_domain_iam_role: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut enable_auto_sub_domain: Option<::Value<bool>> = None;
                let mut sub_domain_settings: Option<::ValueList<self::domain::SubDomainSetting>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppId" => {
                            app_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoSubDomainCreationPatterns" => {
                            auto_sub_domain_creation_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoSubDomainIAMRole" => {
                            auto_sub_domain_iam_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableAutoSubDomain" => {
                            enable_auto_sub_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubDomainSettings" => {
                            sub_domain_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    app_id: app_id.ok_or(::serde::de::Error::missing_field("AppId"))?,
                    auto_sub_domain_creation_patterns: auto_sub_domain_creation_patterns,
                    auto_sub_domain_iam_role: auto_sub_domain_iam_role,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    enable_auto_sub_domain: enable_auto_sub_domain,
                    sub_domain_settings: sub_domain_settings.ok_or(::serde::de::Error::missing_field("SubDomainSettings"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::Amplify::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

pub mod app {
    //! Property types for the `App` resource.

    /// The [`AWS::Amplify::App.AutoBranchCreationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoBranchCreationConfig {
        /// Property [`AutoBranchCreationPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-autobranchcreationpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_branch_creation_patterns: Option<::ValueList<String>>,
        /// Property [`BasicAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-basicauthconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_auth_config: Option<::Value<BasicAuthConfig>>,
        /// Property [`BuildSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-buildspec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub build_spec: Option<::Value<String>>,
        /// Property [`EnableAutoBranchCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-enableautobranchcreation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_auto_branch_creation: Option<::Value<bool>>,
        /// Property [`EnableAutoBuild`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-enableautobuild).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_auto_build: Option<::Value<bool>>,
        /// Property [`EnablePerformanceMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-enableperformancemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_performance_mode: Option<::Value<bool>>,
        /// Property [`EnablePullRequestPreview`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-enablepullrequestpreview).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_pull_request_preview: Option<::Value<bool>>,
        /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-environmentvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_variables: Option<::ValueList<EnvironmentVariable>>,
        /// Property [`Framework`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-framework).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework: Option<::Value<String>>,
        /// Property [`PullRequestEnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-pullrequestenvironmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pull_request_environment_name: Option<::Value<String>>,
        /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html#cfn-amplify-app-autobranchcreationconfig-stage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stage: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AutoBranchCreationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_branch_creation_patterns) = self.auto_branch_creation_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoBranchCreationPatterns", auto_branch_creation_patterns)?;
            }
            if let Some(ref basic_auth_config) = self.basic_auth_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthConfig", basic_auth_config)?;
            }
            if let Some(ref build_spec) = self.build_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildSpec", build_spec)?;
            }
            if let Some(ref enable_auto_branch_creation) = self.enable_auto_branch_creation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoBranchCreation", enable_auto_branch_creation)?;
            }
            if let Some(ref enable_auto_build) = self.enable_auto_build {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoBuild", enable_auto_build)?;
            }
            if let Some(ref enable_performance_mode) = self.enable_performance_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePerformanceMode", enable_performance_mode)?;
            }
            if let Some(ref enable_pull_request_preview) = self.enable_pull_request_preview {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePullRequestPreview", enable_pull_request_preview)?;
            }
            if let Some(ref environment_variables) = self.environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
            }
            if let Some(ref framework) = self.framework {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Framework", framework)?;
            }
            if let Some(ref pull_request_environment_name) = self.pull_request_environment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PullRequestEnvironmentName", pull_request_environment_name)?;
            }
            if let Some(ref stage) = self.stage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", stage)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoBranchCreationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoBranchCreationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoBranchCreationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoBranchCreationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_branch_creation_patterns: Option<::ValueList<String>> = None;
                    let mut basic_auth_config: Option<::Value<BasicAuthConfig>> = None;
                    let mut build_spec: Option<::Value<String>> = None;
                    let mut enable_auto_branch_creation: Option<::Value<bool>> = None;
                    let mut enable_auto_build: Option<::Value<bool>> = None;
                    let mut enable_performance_mode: Option<::Value<bool>> = None;
                    let mut enable_pull_request_preview: Option<::Value<bool>> = None;
                    let mut environment_variables: Option<::ValueList<EnvironmentVariable>> = None;
                    let mut framework: Option<::Value<String>> = None;
                    let mut pull_request_environment_name: Option<::Value<String>> = None;
                    let mut stage: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoBranchCreationPatterns" => {
                                auto_branch_creation_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BasicAuthConfig" => {
                                basic_auth_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BuildSpec" => {
                                build_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAutoBranchCreation" => {
                                enable_auto_branch_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAutoBuild" => {
                                enable_auto_build = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnablePerformanceMode" => {
                                enable_performance_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnablePullRequestPreview" => {
                                enable_pull_request_preview = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentVariables" => {
                                environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Framework" => {
                                framework = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PullRequestEnvironmentName" => {
                                pull_request_environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Stage" => {
                                stage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoBranchCreationConfig {
                        auto_branch_creation_patterns: auto_branch_creation_patterns,
                        basic_auth_config: basic_auth_config,
                        build_spec: build_spec,
                        enable_auto_branch_creation: enable_auto_branch_creation,
                        enable_auto_build: enable_auto_build,
                        enable_performance_mode: enable_performance_mode,
                        enable_pull_request_preview: enable_pull_request_preview,
                        environment_variables: environment_variables,
                        framework: framework,
                        pull_request_environment_name: pull_request_environment_name,
                        stage: stage,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Amplify::App.BasicAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-basicauthconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct BasicAuthConfig {
        /// Property [`EnableBasicAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-basicauthconfig.html#cfn-amplify-app-basicauthconfig-enablebasicauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_basic_auth: Option<::Value<bool>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-basicauthconfig.html#cfn-amplify-app-basicauthconfig-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-basicauthconfig.html#cfn-amplify-app-basicauthconfig-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BasicAuthConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_basic_auth) = self.enable_basic_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableBasicAuth", enable_basic_auth)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BasicAuthConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BasicAuthConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BasicAuthConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BasicAuthConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_basic_auth: Option<::Value<bool>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableBasicAuth" => {
                                enable_basic_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BasicAuthConfig {
                        enable_basic_auth: enable_basic_auth,
                        password: password,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Amplify::App.CustomRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomRule {
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html#cfn-amplify-app-customrule-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: Option<::Value<String>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html#cfn-amplify-app-customrule-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<String>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html#cfn-amplify-app-customrule-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html#cfn-amplify-app-customrule-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition) = self.condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomRule {
                        condition: condition,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                        status: status,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Amplify::App.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-environmentvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-environmentvariable.html#cfn-amplify-app-environmentvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-environmentvariable.html#cfn-amplify-app-environmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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

                    Ok(EnvironmentVariable {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod branch {
    //! Property types for the `Branch` resource.

    /// The [`AWS::Amplify::Branch.Backend`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-backend.html) property type.
    #[derive(Debug, Default)]
    pub struct Backend {
        /// Property [`StackArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-backend.html#cfn-amplify-branch-backend-stackarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Backend {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref stack_arn) = self.stack_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackArn", stack_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Backend {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Backend, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Backend;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Backend")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stack_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StackArn" => {
                                stack_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Backend {
                        stack_arn: stack_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Amplify::Branch.BasicAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-basicauthconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct BasicAuthConfig {
        /// Property [`EnableBasicAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-basicauthconfig.html#cfn-amplify-branch-basicauthconfig-enablebasicauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_basic_auth: Option<::Value<bool>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-basicauthconfig.html#cfn-amplify-branch-basicauthconfig-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-basicauthconfig.html#cfn-amplify-branch-basicauthconfig-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for BasicAuthConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_basic_auth) = self.enable_basic_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableBasicAuth", enable_basic_auth)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BasicAuthConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BasicAuthConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BasicAuthConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BasicAuthConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_basic_auth: Option<::Value<bool>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableBasicAuth" => {
                                enable_basic_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BasicAuthConfig {
                        enable_basic_auth: enable_basic_auth,
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Amplify::Branch.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-environmentvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-environmentvariable.html#cfn-amplify-branch-environmentvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-environmentvariable.html#cfn-amplify-branch-environmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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

                    Ok(EnvironmentVariable {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::Amplify::Domain.SubDomainSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-subdomainsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct SubDomainSetting {
        /// Property [`BranchName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-subdomainsetting.html#cfn-amplify-domain-subdomainsetting-branchname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub branch_name: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-subdomainsetting.html#cfn-amplify-domain-subdomainsetting-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubDomainSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BranchName", &self.branch_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubDomainSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubDomainSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubDomainSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubDomainSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut branch_name: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BranchName" => {
                                branch_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubDomainSetting {
                        branch_name: branch_name.ok_or(::serde::de::Error::missing_field("BranchName"))?,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
