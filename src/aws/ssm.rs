//! Types for the `SSM` service.

/// The [`AWS::SSM::Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html) resource type.
#[derive(Debug)]
pub struct Association {
    properties: AssociationProperties
}

/// Properties for the `Association` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationProperties {
    /// Property `AssociationName`.
    #[serde(rename = "AssociationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub association_name: Option<::Value<String>>,
    /// Property `DocumentVersion`.
    #[serde(rename = "DocumentVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_version: Option<::Value<String>>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::ValueMap<self::association::ParameterValues>>,
    /// Property `ScheduleExpression`.
    #[serde(rename = "ScheduleExpression")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<::Value<String>>,
    /// Property `Targets`.
    #[serde(rename = "Targets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<::ValueList<self::association::Target>>,
}

impl<'a> ::Resource<'a> for Association {
    type Properties = AssociationProperties;
    const TYPE: &'static str = "AWS::SSM::Association";
    fn properties(&self) -> &AssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Association {}

impl From<AssociationProperties> for Association {
    fn from(properties: AssociationProperties) -> Association {
        Association { properties }
    }
}

/// The [`AWS::SSM::Document`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html) resource type.
#[derive(Debug)]
pub struct Document {
    properties: DocumentProperties
}

/// Properties for the `Document` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentProperties {
    /// Property `Content`.
    #[serde(rename = "Content")]
    pub content: ::Value<::json::Value>,
    /// Property `DocumentType`.
    #[serde(rename = "DocumentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for Document {
    type Properties = DocumentProperties;
    const TYPE: &'static str = "AWS::SSM::Document";
    fn properties(&self) -> &DocumentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Document {}

impl From<DocumentProperties> for Document {
    fn from(properties: DocumentProperties) -> Document {
        Document { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindowTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html) resource type.
#[derive(Debug)]
pub struct MaintenanceWindowTask {
    properties: MaintenanceWindowTaskProperties
}

/// Properties for the `MaintenanceWindowTask` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `LoggingInfo`.
    #[serde(rename = "LoggingInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<::Value<self::maintenance_window_task::LoggingInfo>>,
    /// Property `MaxConcurrency`.
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: ::Value<String>,
    /// Property `MaxErrors`.
    #[serde(rename = "MaxErrors")]
    pub max_errors: ::Value<String>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Priority`.
    #[serde(rename = "Priority")]
    pub priority: ::Value<u32>,
    /// Property `ServiceRoleArn`.
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: ::Value<String>,
    /// Property `Targets`.
    #[serde(rename = "Targets")]
    pub targets: ::ValueList<self::maintenance_window_task::Target>,
    /// Property `TaskArn`.
    #[serde(rename = "TaskArn")]
    pub task_arn: ::Value<String>,
    /// Property `TaskInvocationParameters`.
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<::Value<self::maintenance_window_task::TaskInvocationParameters>>,
    /// Property `TaskParameters`.
    #[serde(rename = "TaskParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_parameters: Option<::Value<::json::Value>>,
    /// Property `TaskType`.
    #[serde(rename = "TaskType")]
    pub task_type: ::Value<String>,
    /// Property `WindowId`.
    #[serde(rename = "WindowId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for MaintenanceWindowTask {
    type Properties = MaintenanceWindowTaskProperties;
    const TYPE: &'static str = "AWS::SSM::MaintenanceWindowTask";
    fn properties(&self) -> &MaintenanceWindowTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MaintenanceWindowTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MaintenanceWindowTask {}

impl From<MaintenanceWindowTaskProperties> for MaintenanceWindowTask {
    fn from(properties: MaintenanceWindowTaskProperties) -> MaintenanceWindowTask {
        MaintenanceWindowTask { properties }
    }
}

/// The [`AWS::SSM::Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html) resource type.
#[derive(Debug)]
pub struct Parameter {
    properties: ParameterProperties
}

/// Properties for the `Parameter` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterProperties {
    /// Property `AllowedPattern`.
    #[serde(rename = "AllowedPattern")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
    /// Property `Value`.
    #[serde(rename = "Value")]
    pub value: ::Value<String>,
}

impl<'a> ::Resource<'a> for Parameter {
    type Properties = ParameterProperties;
    const TYPE: &'static str = "AWS::SSM::Parameter";
    fn properties(&self) -> &ParameterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Parameter {}

impl From<ParameterProperties> for Parameter {
    fn from(properties: ParameterProperties) -> Parameter {
        Parameter { properties }
    }
}

/// The [`AWS::SSM::PatchBaseline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html) resource type.
#[derive(Debug)]
pub struct PatchBaseline {
    properties: PatchBaselineProperties
}

/// Properties for the `PatchBaseline` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchBaselineProperties {
    /// Property `ApprovalRules`.
    #[serde(rename = "ApprovalRules")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<::Value<self::patch_baseline::RuleGroup>>,
    /// Property `ApprovedPatches`.
    #[serde(rename = "ApprovedPatches")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<::ValueList<String>>,
    /// Property `ApprovedPatchesComplianceLevel`.
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<::Value<String>>,
    /// Property `ApprovedPatchesEnableNonSecurity`.
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<::Value<bool>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `GlobalFilters`.
    #[serde(rename = "GlobalFilters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<::Value<self::patch_baseline::PatchFilterGroup>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `OperatingSystem`.
    #[serde(rename = "OperatingSystem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<::Value<String>>,
    /// Property `PatchGroups`.
    #[serde(rename = "PatchGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<::ValueList<String>>,
    /// Property `RejectedPatches`.
    #[serde(rename = "RejectedPatches")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<::ValueList<String>>,
    /// Property `Sources`.
    #[serde(rename = "Sources")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<::ValueList<self::patch_baseline::PatchSource>>,
}

impl<'a> ::Resource<'a> for PatchBaseline {
    type Properties = PatchBaselineProperties;
    const TYPE: &'static str = "AWS::SSM::PatchBaseline";
    fn properties(&self) -> &PatchBaselineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PatchBaselineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PatchBaseline {}

impl From<PatchBaselineProperties> for PatchBaseline {
    fn from(properties: PatchBaselineProperties) -> PatchBaseline {
        PatchBaseline { properties }
    }
}

pub mod association {
    //! Property types for the `Association` resource.

    /// The [`AWS::SSM::Association.ParameterValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-parametervalues.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParameterValues {
        /// Property `ParameterValues`.
        #[serde(rename = "ParameterValues")]
        pub parameter_values: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(ParameterValues);

    /// The [`AWS::SSM::Association.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Values`.
        #[serde(rename = "Values")]
        pub values: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(Target);
}

pub mod maintenance_window_task {
    //! Property types for the `MaintenanceWindowTask` resource.

    /// The [`AWS::SSM::MaintenanceWindowTask.LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoggingInfo {
        /// Property `Region`.
        #[serde(rename = "Region")]
        pub region: ::Value<String>,
        /// Property `S3Bucket`.
        #[serde(rename = "S3Bucket")]
        pub s3_bucket: ::Value<String>,
        /// Property `S3Prefix`.
        #[serde(rename = "S3Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LoggingInfo);

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowAutomationParameters {
        /// Property `DocumentVersion`.
        #[serde(rename = "DocumentVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub document_version: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
    }

    cfn_internal__inherit_codec_impls!(MaintenanceWindowAutomationParameters);

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowLambdaParameters {
        /// Property `ClientContext`.
        #[serde(rename = "ClientContext")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client_context: Option<::Value<String>>,
        /// Property `Payload`.
        #[serde(rename = "Payload")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<::Value<String>>,
        /// Property `Qualifier`.
        #[serde(rename = "Qualifier")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub qualifier: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(MaintenanceWindowLambdaParameters);

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowRunCommandParameters {
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<::Value<String>>,
        /// Property `DocumentHash`.
        #[serde(rename = "DocumentHash")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub document_hash: Option<::Value<String>>,
        /// Property `DocumentHashType`.
        #[serde(rename = "DocumentHashType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub document_hash_type: Option<::Value<String>>,
        /// Property `NotificationConfig`.
        #[serde(rename = "NotificationConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_config: Option<::Value<NotificationConfig>>,
        /// Property `OutputS3BucketName`.
        #[serde(rename = "OutputS3BucketName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output_s3_bucket_name: Option<::Value<String>>,
        /// Property `OutputS3KeyPrefix`.
        #[serde(rename = "OutputS3KeyPrefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output_s3_key_prefix: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `ServiceRoleArn`.
        #[serde(rename = "ServiceRoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service_role_arn: Option<::Value<String>>,
        /// Property `TimeoutSeconds`.
        #[serde(rename = "TimeoutSeconds")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout_seconds: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(MaintenanceWindowRunCommandParameters);

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        /// Property `Input`.
        #[serde(rename = "Input")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(MaintenanceWindowStepFunctionsParameters);

    /// The [`AWS::SSM::MaintenanceWindowTask.NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfig {
        /// Property `NotificationArn`.
        #[serde(rename = "NotificationArn")]
        pub notification_arn: ::Value<String>,
        /// Property `NotificationEvents`.
        #[serde(rename = "NotificationEvents")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_events: Option<::ValueList<String>>,
        /// Property `NotificationType`.
        #[serde(rename = "NotificationType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(NotificationConfig);

    /// The [`AWS::SSM::MaintenanceWindowTask.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Values`.
        #[serde(rename = "Values")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub values: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(Target);

    /// The [`AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskInvocationParameters {
        /// Property `MaintenanceWindowAutomationParameters`.
        #[serde(rename = "MaintenanceWindowAutomationParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maintenance_window_automation_parameters: Option<::Value<MaintenanceWindowAutomationParameters>>,
        /// Property `MaintenanceWindowLambdaParameters`.
        #[serde(rename = "MaintenanceWindowLambdaParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maintenance_window_lambda_parameters: Option<::Value<MaintenanceWindowLambdaParameters>>,
        /// Property `MaintenanceWindowRunCommandParameters`.
        #[serde(rename = "MaintenanceWindowRunCommandParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maintenance_window_run_command_parameters: Option<::Value<MaintenanceWindowRunCommandParameters>>,
        /// Property `MaintenanceWindowStepFunctionsParameters`.
        #[serde(rename = "MaintenanceWindowStepFunctionsParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maintenance_window_step_functions_parameters: Option<::Value<MaintenanceWindowStepFunctionsParameters>>,
    }

    cfn_internal__inherit_codec_impls!(TaskInvocationParameters);
}

pub mod patch_baseline {
    //! Property types for the `PatchBaseline` resource.

    /// The [`AWS::SSM::PatchBaseline.PatchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilter {
        /// Property `Key`.
        #[serde(rename = "Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<::Value<String>>,
        /// Property `Values`.
        #[serde(rename = "Values")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub values: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(PatchFilter);

    /// The [`AWS::SSM::PatchBaseline.PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilterGroup {
        /// Property `PatchFilters`.
        #[serde(rename = "PatchFilters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub patch_filters: Option<::ValueList<PatchFilter>>,
    }

    cfn_internal__inherit_codec_impls!(PatchFilterGroup);

    /// The [`AWS::SSM::PatchBaseline.PatchSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchSource {
        /// Property `Configuration`.
        #[serde(rename = "Configuration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Products`.
        #[serde(rename = "Products")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub products: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(PatchSource);

    /// The [`AWS::SSM::PatchBaseline.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `ApproveAfterDays`.
        #[serde(rename = "ApproveAfterDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub approve_after_days: Option<::Value<u32>>,
        /// Property `ComplianceLevel`.
        #[serde(rename = "ComplianceLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compliance_level: Option<::Value<String>>,
        /// Property `EnableNonSecurity`.
        #[serde(rename = "EnableNonSecurity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable_non_security: Option<::Value<bool>>,
        /// Property `PatchFilterGroup`.
        #[serde(rename = "PatchFilterGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub patch_filter_group: Option<::Value<PatchFilterGroup>>,
    }

    cfn_internal__inherit_codec_impls!(Rule);

    /// The [`AWS::SSM::PatchBaseline.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RuleGroup {
        /// Property `PatchRules`.
        #[serde(rename = "PatchRules")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub patch_rules: Option<::ValueList<Rule>>,
    }

    cfn_internal__inherit_codec_impls!(RuleGroup);
}
