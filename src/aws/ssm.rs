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
    pub association_name: String,
    /// Property `DocumentVersion`.
    #[serde(rename="DocumentVersion")]
    pub document_version: String,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, self::association::ParameterValues>,
    /// Property `ScheduleExpression`.
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    pub targets: Vec<self::association::Target>,
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
    pub document_type: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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
    pub description: String,
    /// Property `LoggingInfo`.
    #[serde(rename="LoggingInfo")]
    pub logging_info: self::maintenance_window_task::LoggingInfo,
    /// Property `MaxConcurrency`.
    #[serde(rename="MaxConcurrency")]
    pub max_concurrency: String,
    /// Property `MaxErrors`.
    #[serde(rename="MaxErrors")]
    pub max_errors: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
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
    pub task_invocation_parameters: self::maintenance_window_task::TaskInvocationParameters,
    /// Property `TaskParameters`.
    #[serde(rename="TaskParameters")]
    pub task_parameters: ::json::Value,
    /// Property `TaskType`.
    #[serde(rename="TaskType")]
    pub task_type: String,
    /// Property `WindowId`.
    #[serde(rename="WindowId")]
    pub window_id: String,
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
    pub allowed_pattern: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
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
    pub approval_rules: self::patch_baseline::RuleGroup,
    /// Property `ApprovedPatches`.
    #[serde(rename="ApprovedPatches")]
    pub approved_patches: Vec<String>,
    /// Property `ApprovedPatchesComplianceLevel`.
    #[serde(rename="ApprovedPatchesComplianceLevel")]
    pub approved_patches_compliance_level: String,
    /// Property `ApprovedPatchesEnableNonSecurity`.
    #[serde(rename="ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: bool,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `GlobalFilters`.
    #[serde(rename="GlobalFilters")]
    pub global_filters: self::patch_baseline::PatchFilterGroup,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `OperatingSystem`.
    #[serde(rename="OperatingSystem")]
    pub operating_system: String,
    /// Property `PatchGroups`.
    #[serde(rename="PatchGroups")]
    pub patch_groups: Vec<String>,
    /// Property `RejectedPatches`.
    #[serde(rename="RejectedPatches")]
    pub rejected_patches: Vec<String>,
    /// Property `Sources`.
    #[serde(rename="Sources")]
    pub sources: Vec<self::patch_baseline::PatchSource>,
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
        pub s3_prefix: String,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowAutomationParameters {
        /// Property `DocumentVersion`.
        #[serde(rename="DocumentVersion")]
        pub document_version: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowLambdaParameters {
        /// Property `ClientContext`.
        #[serde(rename="ClientContext")]
        pub client_context: String,
        /// Property `Payload`.
        #[serde(rename="Payload")]
        pub payload: String,
        /// Property `Qualifier`.
        #[serde(rename="Qualifier")]
        pub qualifier: String,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowRunCommandParameters {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        pub comment: String,
        /// Property `DocumentHash`.
        #[serde(rename="DocumentHash")]
        pub document_hash: String,
        /// Property `DocumentHashType`.
        #[serde(rename="DocumentHashType")]
        pub document_hash_type: String,
        /// Property `NotificationConfig`.
        #[serde(rename="NotificationConfig")]
        pub notification_config: NotificationConfig,
        /// Property `OutputS3BucketName`.
        #[serde(rename="OutputS3BucketName")]
        pub output_s3_bucket_name: String,
        /// Property `OutputS3KeyPrefix`.
        #[serde(rename="OutputS3KeyPrefix")]
        pub output_s3_key_prefix: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `ServiceRoleArn`.
        #[serde(rename="ServiceRoleArn")]
        pub service_role_arn: String,
        /// Property `TimeoutSeconds`.
        #[serde(rename="TimeoutSeconds")]
        pub timeout_seconds: u32,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        /// Property `Input`.
        #[serde(rename="Input")]
        pub input: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfig {
        /// Property `NotificationArn`.
        #[serde(rename="NotificationArn")]
        pub notification_arn: String,
        /// Property `NotificationEvents`.
        #[serde(rename="NotificationEvents")]
        pub notification_events: Vec<String>,
        /// Property `NotificationType`.
        #[serde(rename="NotificationType")]
        pub notification_type: String,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskInvocationParameters {
        /// Property `MaintenanceWindowAutomationParameters`.
        #[serde(rename="MaintenanceWindowAutomationParameters")]
        pub maintenance_window_automation_parameters: MaintenanceWindowAutomationParameters,
        /// Property `MaintenanceWindowLambdaParameters`.
        #[serde(rename="MaintenanceWindowLambdaParameters")]
        pub maintenance_window_lambda_parameters: MaintenanceWindowLambdaParameters,
        /// Property `MaintenanceWindowRunCommandParameters`.
        #[serde(rename="MaintenanceWindowRunCommandParameters")]
        pub maintenance_window_run_command_parameters: MaintenanceWindowRunCommandParameters,
        /// Property `MaintenanceWindowStepFunctionsParameters`.
        #[serde(rename="MaintenanceWindowStepFunctionsParameters")]
        pub maintenance_window_step_functions_parameters: MaintenanceWindowStepFunctionsParameters,
    }
}

pub mod patch_baseline {
    //! Property types for the `PatchBaseline` resource.

    /// The [`AWS::SSM::PatchBaseline.PatchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilter {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::SSM::PatchBaseline.PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchFilterGroup {
        /// Property `PatchFilters`.
        #[serde(rename="PatchFilters")]
        pub patch_filters: Vec<PatchFilter>,
    }

    /// The [`AWS::SSM::PatchBaseline.PatchSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PatchSource {
        /// Property `Configuration`.
        #[serde(rename="Configuration")]
        pub configuration: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Products`.
        #[serde(rename="Products")]
        pub products: Vec<String>,
    }

    /// The [`AWS::SSM::PatchBaseline.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `ApproveAfterDays`.
        #[serde(rename="ApproveAfterDays")]
        pub approve_after_days: u32,
        /// Property `ComplianceLevel`.
        #[serde(rename="ComplianceLevel")]
        pub compliance_level: String,
        /// Property `EnableNonSecurity`.
        #[serde(rename="EnableNonSecurity")]
        pub enable_non_security: bool,
        /// Property `PatchFilterGroup`.
        #[serde(rename="PatchFilterGroup")]
        pub patch_filter_group: PatchFilterGroup,
    }

    /// The [`AWS::SSM::PatchBaseline.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RuleGroup {
        /// Property `PatchRules`.
        #[serde(rename="PatchRules")]
        pub patch_rules: Vec<Rule>,
    }
}
