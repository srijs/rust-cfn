/// The [`AWS::SSM::Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Association {
    properties: AssociationProperties
}

/// Properties for the `Association` resource.
#[derive(Serialize, Deserialize)]
pub struct AssociationProperties {
    #[serde(rename="AssociationName")]
    pub association_name: (),
    #[serde(rename="DocumentVersion")]
    pub document_version: (),
    #[serde(rename="InstanceId")]
    pub instance_id: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Parameters")]
    pub parameters: (),
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: (),
    #[serde(rename="Targets")]
    pub targets: (),
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
    pub content: (),
    #[serde(rename="DocumentType")]
    pub document_type: (),
    #[serde(rename="Tags")]
    pub tags: (),
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
    pub description: (),
    #[serde(rename="LoggingInfo")]
    pub logging_info: (),
    #[serde(rename="MaxConcurrency")]
    pub max_concurrency: (),
    #[serde(rename="MaxErrors")]
    pub max_errors: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Priority")]
    pub priority: (),
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: (),
    #[serde(rename="Targets")]
    pub targets: (),
    #[serde(rename="TaskArn")]
    pub task_arn: (),
    #[serde(rename="TaskInvocationParameters")]
    pub task_invocation_parameters: (),
    #[serde(rename="TaskParameters")]
    pub task_parameters: (),
    #[serde(rename="TaskType")]
    pub task_type: (),
    #[serde(rename="WindowId")]
    pub window_id: (),
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
    pub allowed_pattern: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Type")]
    pub type_: (),
    #[serde(rename="Value")]
    pub value: (),
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
    pub approved_patches: (),
    #[serde(rename="ApprovedPatchesComplianceLevel")]
    pub approved_patches_compliance_level: (),
    #[serde(rename="ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="GlobalFilters")]
    pub global_filters: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="OperatingSystem")]
    pub operating_system: (),
    #[serde(rename="PatchGroups")]
    pub patch_groups: (),
    #[serde(rename="RejectedPatches")]
    pub rejected_patches: (),
    #[serde(rename="Sources")]
    pub sources: (),
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

