/// The [`AWS::SES::ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationSet {
    properties: ConfigurationSetProperties
}

/// Properties for the `ConfigurationSet` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationSetProperties {
    #[serde(rename="Name")]
    pub name: String,
}

impl<'a> ::Resource<'a> for ConfigurationSet {
    type Properties = ConfigurationSetProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSet";
    fn properties(&self) -> &ConfigurationSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetProperties {
        &mut self.properties
    }
}

impl From<ConfigurationSetProperties> for ConfigurationSet {
    fn from(properties: ConfigurationSetProperties) -> ConfigurationSet {
        ConfigurationSet { properties }
    }
}

/// The [`AWS::SES::ConfigurationSetEventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationSetEventDestination {
    properties: ConfigurationSetEventDestinationProperties
}

/// Properties for the `ConfigurationSetEventDestination` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationSetEventDestinationProperties {
    #[serde(rename="ConfigurationSetName")]
    pub configuration_set_name: String,
    #[serde(rename="EventDestination")]
    pub event_destination: (),
}

impl<'a> ::Resource<'a> for ConfigurationSetEventDestination {
    type Properties = ConfigurationSetEventDestinationProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSetEventDestination";
    fn properties(&self) -> &ConfigurationSetEventDestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetEventDestinationProperties {
        &mut self.properties
    }
}

impl From<ConfigurationSetEventDestinationProperties> for ConfigurationSetEventDestination {
    fn from(properties: ConfigurationSetEventDestinationProperties) -> ConfigurationSetEventDestination {
        ConfigurationSetEventDestination { properties }
    }
}

/// The [`AWS::SES::ReceiptFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptFilter {
    properties: ReceiptFilterProperties
}

/// Properties for the `ReceiptFilter` resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptFilterProperties {
    #[serde(rename="Filter")]
    pub filter: (),
}

impl<'a> ::Resource<'a> for ReceiptFilter {
    type Properties = ReceiptFilterProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptFilter";
    fn properties(&self) -> &ReceiptFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptFilterProperties {
        &mut self.properties
    }
}

impl From<ReceiptFilterProperties> for ReceiptFilter {
    fn from(properties: ReceiptFilterProperties) -> ReceiptFilter {
        ReceiptFilter { properties }
    }
}

/// The [`AWS::SES::ReceiptRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptRule {
    properties: ReceiptRuleProperties
}

/// Properties for the `ReceiptRule` resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptRuleProperties {
    #[serde(rename="After")]
    pub after: String,
    #[serde(rename="Rule")]
    pub rule: (),
    #[serde(rename="RuleSetName")]
    pub rule_set_name: String,
}

impl<'a> ::Resource<'a> for ReceiptRule {
    type Properties = ReceiptRuleProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRule";
    fn properties(&self) -> &ReceiptRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleProperties {
        &mut self.properties
    }
}

impl From<ReceiptRuleProperties> for ReceiptRule {
    fn from(properties: ReceiptRuleProperties) -> ReceiptRule {
        ReceiptRule { properties }
    }
}

/// The [`AWS::SES::ReceiptRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptRuleSet {
    properties: ReceiptRuleSetProperties
}

/// Properties for the `ReceiptRuleSet` resource.
#[derive(Serialize, Deserialize)]
pub struct ReceiptRuleSetProperties {
    #[serde(rename="RuleSetName")]
    pub rule_set_name: String,
}

impl<'a> ::Resource<'a> for ReceiptRuleSet {
    type Properties = ReceiptRuleSetProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRuleSet";
    fn properties(&self) -> &ReceiptRuleSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleSetProperties {
        &mut self.properties
    }
}

impl From<ReceiptRuleSetProperties> for ReceiptRuleSet {
    fn from(properties: ReceiptRuleSetProperties) -> ReceiptRuleSet {
        ReceiptRuleSet { properties }
    }
}

/// The [`AWS::SES::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Serialize, Deserialize)]
pub struct TemplateProperties {
    #[serde(rename="Template")]
    pub template: (),
}

impl<'a> ::Resource<'a> for Template {
    type Properties = TemplateProperties;
    const TYPE: &'static str = "AWS::SES::Template";
    fn properties(&self) -> &TemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateProperties {
        &mut self.properties
    }
}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

