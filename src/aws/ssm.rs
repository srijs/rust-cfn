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
    #[serde(rename="AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// Property `DocumentVersion`.
    #[serde(rename="DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, self::association::ParameterValues>>,
    /// Property `ScheduleExpression`.
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<self::association::Target>>,
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
    #[serde(rename="Content")]
    pub content: ::json::Value,
    /// Property `DocumentType`.
    #[serde(rename="DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `LoggingInfo`.
    #[serde(rename="LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<self::maintenance_window_task::LoggingInfo>,
    /// Property `MaxConcurrency`.
    #[serde(rename="MaxConcurrency")]
    pub max_concurrency: String,
    /// Property `MaxErrors`.
    #[serde(rename="MaxErrors")]
    pub max_errors: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `Priority`.
    #[serde(rename="Priority")]
    pub priority: u32,
    /// Property `ServiceRoleArn`.
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    pub targets: Vec<self::maintenance_window_task::Target>,
    /// Property `TaskArn`.
    #[serde(rename="TaskArn")]
    pub task_arn: String,
    /// Property `TaskInvocationParameters`.
    #[serde(rename="TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<self::maintenance_window_task::TaskInvocationParameters>,
    /// Property `TaskParameters`.
    #[serde(rename="TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters: Option<::json::Value>,
    /// Property `TaskType`.
    #[serde(rename="TaskType")]
    pub task_type: String,
    /// Property `WindowId`.
    #[serde(rename="WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
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
    #[serde(rename="AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
    /// Property `Value`.
    #[serde(rename="Value")]
    pub value: String,
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
    #[serde(rename="ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<self::patch_baseline::RuleGroup>,
    /// Property `ApprovedPatches`.
    #[serde(rename="ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// Property `ApprovedPatchesComplianceLevel`.
    #[serde(rename="ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// Property `ApprovedPatchesEnableNonSecurity`.
    #[serde(rename="ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `GlobalFilters`.
    #[serde(rename="GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<self::patch_baseline::PatchFilterGroup>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `OperatingSystem`.
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Property `PatchGroups`.
    #[serde(rename="PatchGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<Vec<String>>,
    /// Property `RejectedPatches`.
    #[serde(rename="RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// Property `Sources`.
    #[serde(rename="Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<self::patch_baseline::PatchSource>>,
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
        #[serde(rename="ParameterValues")]
        pub parameter_values: Vec<String>,
    }

    /// The [`AWS::SSM::Association.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }
}

pub mod maintenance_window_task {
    //! Property types for the `MaintenanceWindowTask` resource.

    /// The [`AWS::SSM::MaintenanceWindowTask.LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoggingInfo {
        /// Property `Region`.
        #[serde(rename="Region")]
        pub region: String,
        /// Property `S3Bucket`.
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        /// Property `S3Prefix`.
        #[serde(rename="S3Prefix")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_prefix: Option<String>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowAutomationParameters {
        /// Property `DocumentVersion`.
        #[serde(rename="DocumentVersion")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document_version: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowLambdaParameters {
        /// Property `ClientContext`.
        #[serde(rename="ClientContext")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_context: Option<String>,
        /// Property `Payload`.
        #[serde(rename="Payload")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub payload: Option<String>,
        /// Property `Qualifier`.
        #[serde(rename="Qualifier")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qualifier: Option<String>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowRunCommandParameters {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        /// Property `DocumentHash`.
        #[serde(rename="DocumentHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document_hash: Option<String>,
        /// Property `DocumentHashType`.
        #[serde(rename="DocumentHashType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document_hash_type: Option<String>,
        /// Property `NotificationConfig`.
        #[serde(rename="NotificationConfig")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_config: Option<NotificationConfig>,
        /// Property `OutputS3BucketName`.
        #[serde(rename="OutputS3BucketName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_s3_bucket_name: Option<String>,
        /// Property `OutputS3KeyPrefix`.
        #[serde(rename="OutputS3KeyPrefix")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_s3_key_prefix: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `ServiceRoleArn`.
        #[serde(rename="ServiceRoleArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_role_arn: Option<String>,
        /// Property `TimeoutSeconds`.
        #[serde(rename="TimeoutSeconds")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout_seconds: Option<u32>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        /// Property `Input`.
        #[serde(rename="Input")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfig {
        /// Property `NotificationArn`.
        #[serde(rename="NotificationArn")]
        pub notification_arn: String,
        /// Property `NotificationEvents`.
        #[serde(rename="NotificationEvents")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_events: Option<Vec<String>>,
        /// Property `NotificationType`.
        #[serde(rename="NotificationType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_type: Option<String>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Vec<String>>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskInvocationParameters {
        /// Property `MaintenanceWindowAutomationParameters`.
        #[serde(rename="MaintenanceWindowAutomationParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maintenance_window_automation_parameters: Option<MaintenanceWindowAutomationParameters>,
        /// Property `MaintenanceWindowLambdaParameters`.
        #[serde(rename="MaintenanceWindowLambdaParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maintenance_window_lambda_parameters: Option<MaintenanceWindowLambdaParameters>,
        /// Property `MaintenanceWindowRunCommandParameters`.
        #[serde(rename="MaintenanceWindowRunCommandParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maintenance_window_run_command_parameters: Option<MaintenanceWindowRunCommandParameters>,
        /// Property `MaintenanceWindowStepFunctionsParameters`.
        #[serde(rename="MaintenanceWindowStepFunctionsParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maintenance_window_step_functions_parameters: Option<MaintenanceWindowStepFunctionsParameters>,
    }
}

pub mod patch_baseline {
    //! Property types for the `PatchBaseline` resource.

    /// The [`AWS::SSM::PatchBaseline.PatchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilter {
        /// Property `Key`.
        #[serde(rename="Key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        /// Property `Values`.
        #[serde(rename="Values")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Vec<String>>,
    }

    /// The [`AWS::SSM::PatchBaseline.PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilterGroup {
        /// Property `PatchFilters`.
        #[serde(rename="PatchFilters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub patch_filters: Option<Vec<PatchFilter>>,
    }

    /// The [`AWS::SSM::PatchBaseline.PatchSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchSource {
        /// Property `Configuration`.
        #[serde(rename="Configuration")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Products`.
        #[serde(rename="Products")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub products: Option<Vec<String>>,
    }

    /// The [`AWS::SSM::PatchBaseline.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `ApproveAfterDays`.
        #[serde(rename="ApproveAfterDays")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub approve_after_days: Option<u32>,
        /// Property `ComplianceLevel`.
        #[serde(rename="ComplianceLevel")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compliance_level: Option<String>,
        /// Property `EnableNonSecurity`.
        #[serde(rename="EnableNonSecurity")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_non_security: Option<bool>,
        /// Property `PatchFilterGroup`.
        #[serde(rename="PatchFilterGroup")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub patch_filter_group: Option<PatchFilterGroup>,
    }

    /// The [`AWS::SSM::PatchBaseline.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RuleGroup {
        /// Property `PatchRules`.
        #[serde(rename="PatchRules")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub patch_rules: Option<Vec<Rule>>,
    }
}
