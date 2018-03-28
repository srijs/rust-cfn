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
    pub parameters: ::std::collections::HashMap<String, ()>,
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename="Targets")]
    pub targets: Vec<()>,
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
    pub content: String,
    #[serde(rename="DocumentType")]
    pub document_type: String,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
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
    pub logging_info: (),
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
    pub targets: Vec<()>,
    #[serde(rename="TaskArn")]
    pub task_arn: String,
    #[serde(rename="TaskInvocationParameters")]
    pub task_invocation_parameters: (),
    #[serde(rename="TaskParameters")]
    pub task_parameters: String,
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
    pub approval_rules: (),
    #[serde(rename="ApprovedPatches")]
    pub approved_patches: Vec<String>,
    #[serde(rename="ApprovedPatchesComplianceLevel")]
    pub approved_patches_compliance_level: String,
    #[serde(rename="ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: bool,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="GlobalFilters")]
    pub global_filters: (),
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="OperatingSystem")]
    pub operating_system: String,
    #[serde(rename="PatchGroups")]
    pub patch_groups: Vec<String>,
    #[serde(rename="RejectedPatches")]
    pub rejected_patches: Vec<String>,
    #[serde(rename="Sources")]
    pub sources: Vec<()>,
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

