/// The [`AWS::SSM::Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Association {
    properties: AssociationProperties
}

/// Properties for the `Association` resource.
#[derive(Serialize, Deserialize)]
pub struct AssociationProperties {
    #[serde(rename="AssociationName")]
    pub association_name: String,
    #[serde(rename="DocumentVersion")]
    pub document_version: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, self::association::ParameterValues>,
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
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

impl From<AssociationProperties> for Association {
    fn from(properties: AssociationProperties) -> Association {
        Association { properties }
    }
}

/// The [`AWS::SSM::Document`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Document {
    properties: DocumentProperties
}

/// Properties for the `Document` resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentProperties {
    #[serde(rename="Content")]
    pub content: ::json::Value,
    #[serde(rename="DocumentType")]
    pub document_type: String,
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

impl From<DocumentProperties> for Document {
    fn from(properties: DocumentProperties) -> Document {
        Document { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindowTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html) resource.
#[derive(Serialize, Deserialize)]
pub struct MaintenanceWindowTask {
    properties: MaintenanceWindowTaskProperties
}

/// Properties for the `MaintenanceWindowTask` resource.
#[derive(Serialize, Deserialize)]
pub struct MaintenanceWindowTaskProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="LoggingInfo")]
    pub logging_info: self::maintenance_window_task::LoggingInfo,
    #[serde(rename="MaxConcurrency")]
    pub max_concurrency: String,
    #[serde(rename="MaxErrors")]
    pub max_errors: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Priority")]
    pub priority: u32,
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[serde(rename="Targets")]
    pub targets: Vec<self::maintenance_window_task::Target>,
    #[serde(rename="TaskArn")]
    pub task_arn: String,
    #[serde(rename="TaskInvocationParameters")]
    pub task_invocation_parameters: self::maintenance_window_task::TaskInvocationParameters,
    #[serde(rename="TaskParameters")]
    pub task_parameters: ::json::Value,
    #[serde(rename="TaskType")]
    pub task_type: String,
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

impl From<MaintenanceWindowTaskProperties> for MaintenanceWindowTask {
    fn from(properties: MaintenanceWindowTaskProperties) -> MaintenanceWindowTask {
        MaintenanceWindowTask { properties }
    }
}

/// The [`AWS::SSM::Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Parameter {
    properties: ParameterProperties
}

/// Properties for the `Parameter` resource.
#[derive(Serialize, Deserialize)]
pub struct ParameterProperties {
    #[serde(rename="AllowedPattern")]
    pub allowed_pattern: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Type")]
    pub type_: String,
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

impl From<ParameterProperties> for Parameter {
    fn from(properties: ParameterProperties) -> Parameter {
        Parameter { properties }
    }
}

/// The [`AWS::SSM::PatchBaseline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html) resource.
#[derive(Serialize, Deserialize)]
pub struct PatchBaseline {
    properties: PatchBaselineProperties
}

/// Properties for the `PatchBaseline` resource.
#[derive(Serialize, Deserialize)]
pub struct PatchBaselineProperties {
    #[serde(rename="ApprovalRules")]
    pub approval_rules: self::patch_baseline::RuleGroup,
    #[serde(rename="ApprovedPatches")]
    pub approved_patches: Vec<String>,
    #[serde(rename="ApprovedPatchesComplianceLevel")]
    pub approved_patches_compliance_level: String,
    #[serde(rename="ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: bool,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="GlobalFilters")]
    pub global_filters: self::patch_baseline::PatchFilterGroup,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="OperatingSystem")]
    pub operating_system: String,
    #[serde(rename="PatchGroups")]
    pub patch_groups: Vec<String>,
    #[serde(rename="RejectedPatches")]
    pub rejected_patches: Vec<String>,
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

impl From<PatchBaselineProperties> for PatchBaseline {
    fn from(properties: PatchBaselineProperties) -> PatchBaseline {
        PatchBaseline { properties }
    }
}

pub mod association {
    #[derive(Serialize, Deserialize)]
    pub struct ParameterValues {
        #[serde(rename="ParameterValues")]
        pub parameter_values: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Target {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

}

pub mod maintenance_window_task {
    #[derive(Serialize, Deserialize)]
    pub struct LoggingInfo {
        #[serde(rename="Region")]
        pub region: String,
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        #[serde(rename="S3Prefix")]
        pub s3_prefix: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaintenanceWindowAutomationParameters {
        #[serde(rename="DocumentVersion")]
        pub document_version: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaintenanceWindowLambdaParameters {
        #[serde(rename="ClientContext")]
        pub client_context: String,
        #[serde(rename="Payload")]
        pub payload: String,
        #[serde(rename="Qualifier")]
        pub qualifier: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaintenanceWindowRunCommandParameters {
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="DocumentHash")]
        pub document_hash: String,
        #[serde(rename="DocumentHashType")]
        pub document_hash_type: String,
        #[serde(rename="NotificationConfig")]
        pub notification_config: NotificationConfig,
        #[serde(rename="OutputS3BucketName")]
        pub output_s3_bucket_name: String,
        #[serde(rename="OutputS3KeyPrefix")]
        pub output_s3_key_prefix: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="ServiceRoleArn")]
        pub service_role_arn: String,
        #[serde(rename="TimeoutSeconds")]
        pub timeout_seconds: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        #[serde(rename="Input")]
        pub input: String,
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NotificationConfig {
        #[serde(rename="NotificationArn")]
        pub notification_arn: String,
        #[serde(rename="NotificationEvents")]
        pub notification_events: Vec<String>,
        #[serde(rename="NotificationType")]
        pub notification_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Target {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TaskInvocationParameters {
        #[serde(rename="MaintenanceWindowAutomationParameters")]
        pub maintenance_window_automation_parameters: MaintenanceWindowAutomationParameters,
        #[serde(rename="MaintenanceWindowLambdaParameters")]
        pub maintenance_window_lambda_parameters: MaintenanceWindowLambdaParameters,
        #[serde(rename="MaintenanceWindowRunCommandParameters")]
        pub maintenance_window_run_command_parameters: MaintenanceWindowRunCommandParameters,
        #[serde(rename="MaintenanceWindowStepFunctionsParameters")]
        pub maintenance_window_step_functions_parameters: MaintenanceWindowStepFunctionsParameters,
    }

}

pub mod patch_baseline {
    #[derive(Serialize, Deserialize)]
    pub struct PatchFilter {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PatchFilterGroup {
        #[serde(rename="PatchFilters")]
        pub patch_filters: Vec<PatchFilter>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PatchSource {
        #[serde(rename="Configuration")]
        pub configuration: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Products")]
        pub products: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Rule {
        #[serde(rename="ApproveAfterDays")]
        pub approve_after_days: u32,
        #[serde(rename="ComplianceLevel")]
        pub compliance_level: String,
        #[serde(rename="EnableNonSecurity")]
        pub enable_non_security: bool,
        #[serde(rename="PatchFilterGroup")]
        pub patch_filter_group: PatchFilterGroup,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RuleGroup {
        #[serde(rename="PatchRules")]
        pub patch_rules: Vec<Rule>,
    }

}

