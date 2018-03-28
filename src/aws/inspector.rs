/// The [`AWS::Inspector::AssessmentTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttarget.html) resource.
#[derive(Serialize, Deserialize)]
pub struct AssessmentTarget {
    properties: AssessmentTargetProperties
}

/// Properties for the `AssessmentTarget` resource.
#[derive(Serialize, Deserialize)]
pub struct AssessmentTargetProperties {
    #[serde(rename="AssessmentTargetName")]
    pub assessment_target_name: (),
    #[serde(rename="ResourceGroupArn")]
    pub resource_group_arn: (),
}

impl<'a> ::Resource<'a> for AssessmentTarget {
    type Properties = AssessmentTargetProperties;
    const TYPE: &'static str = "AWS::Inspector::AssessmentTarget";
    fn properties(&self) -> &AssessmentTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssessmentTargetProperties {
        &mut self.properties
    }
}

impl From<AssessmentTargetProperties> for AssessmentTarget {
    fn from(properties: AssessmentTargetProperties) -> AssessmentTarget {
        AssessmentTarget { properties }
    }
}

/// The [`AWS::Inspector::AssessmentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct AssessmentTemplate {
    properties: AssessmentTemplateProperties
}

/// Properties for the `AssessmentTemplate` resource.
#[derive(Serialize, Deserialize)]
pub struct AssessmentTemplateProperties {
    #[serde(rename="AssessmentTargetArn")]
    pub assessment_target_arn: (),
    #[serde(rename="AssessmentTemplateName")]
    pub assessment_template_name: (),
    #[serde(rename="DurationInSeconds")]
    pub duration_in_seconds: (),
    #[serde(rename="RulesPackageArns")]
    pub rules_package_arns: (),
    #[serde(rename="UserAttributesForFindings")]
    pub user_attributes_for_findings: (),
}

impl<'a> ::Resource<'a> for AssessmentTemplate {
    type Properties = AssessmentTemplateProperties;
    const TYPE: &'static str = "AWS::Inspector::AssessmentTemplate";
    fn properties(&self) -> &AssessmentTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssessmentTemplateProperties {
        &mut self.properties
    }
}

impl From<AssessmentTemplateProperties> for AssessmentTemplate {
    fn from(properties: AssessmentTemplateProperties) -> AssessmentTemplate {
        AssessmentTemplate { properties }
    }
}

/// The [`AWS::Inspector::ResourceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-resourcegroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ResourceGroup {
    properties: ResourceGroupProperties
}

/// Properties for the `ResourceGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    #[serde(rename="ResourceGroupTags")]
    pub resource_group_tags: (),
}

impl<'a> ::Resource<'a> for ResourceGroup {
    type Properties = ResourceGroupProperties;
    const TYPE: &'static str = "AWS::Inspector::ResourceGroup";
    fn properties(&self) -> &ResourceGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceGroupProperties {
        &mut self.properties
    }
}

impl From<ResourceGroupProperties> for ResourceGroup {
    fn from(properties: ResourceGroupProperties) -> ResourceGroup {
        ResourceGroup { properties }
    }
}

