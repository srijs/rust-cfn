//! Types for the `SES` service.

/// The [`AWS::SES::ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html) resource type.
#[derive(Debug)]
pub struct ConfigurationSet {
    properties: ConfigurationSetProperties
}

/// Properties for the `ConfigurationSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSetProperties {
    /// Property `Name`.
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

impl ::private::Sealed for ConfigurationSet {}

impl From<ConfigurationSetProperties> for ConfigurationSet {
    fn from(properties: ConfigurationSetProperties) -> ConfigurationSet {
        ConfigurationSet { properties }
    }
}

/// The [`AWS::SES::ConfigurationSetEventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html) resource type.
#[derive(Debug)]
pub struct ConfigurationSetEventDestination {
    properties: ConfigurationSetEventDestinationProperties
}

/// Properties for the `ConfigurationSetEventDestination` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSetEventDestinationProperties {
    /// Property `ConfigurationSetName`.
    #[serde(rename="ConfigurationSetName")]
    pub configuration_set_name: String,
    /// Property `EventDestination`.
    #[serde(rename="EventDestination")]
    pub event_destination: self::configuration_set_event_destination::EventDestination,
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

impl ::private::Sealed for ConfigurationSetEventDestination {}

impl From<ConfigurationSetEventDestinationProperties> for ConfigurationSetEventDestination {
    fn from(properties: ConfigurationSetEventDestinationProperties) -> ConfigurationSetEventDestination {
        ConfigurationSetEventDestination { properties }
    }
}

/// The [`AWS::SES::ReceiptFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html) resource type.
#[derive(Debug)]
pub struct ReceiptFilter {
    properties: ReceiptFilterProperties
}

/// Properties for the `ReceiptFilter` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptFilterProperties {
    /// Property `Filter`.
    #[serde(rename="Filter")]
    pub filter: self::receipt_filter::Filter,
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

impl ::private::Sealed for ReceiptFilter {}

impl From<ReceiptFilterProperties> for ReceiptFilter {
    fn from(properties: ReceiptFilterProperties) -> ReceiptFilter {
        ReceiptFilter { properties }
    }
}

/// The [`AWS::SES::ReceiptRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html) resource type.
#[derive(Debug)]
pub struct ReceiptRule {
    properties: ReceiptRuleProperties
}

/// Properties for the `ReceiptRule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptRuleProperties {
    /// Property `After`.
    #[serde(rename="After")]
    pub after: String,
    /// Property `Rule`.
    #[serde(rename="Rule")]
    pub rule: self::receipt_rule::Rule,
    /// Property `RuleSetName`.
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

impl ::private::Sealed for ReceiptRule {}

impl From<ReceiptRuleProperties> for ReceiptRule {
    fn from(properties: ReceiptRuleProperties) -> ReceiptRule {
        ReceiptRule { properties }
    }
}

/// The [`AWS::SES::ReceiptRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html) resource type.
#[derive(Debug)]
pub struct ReceiptRuleSet {
    properties: ReceiptRuleSetProperties
}

/// Properties for the `ReceiptRuleSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptRuleSetProperties {
    /// Property `RuleSetName`.
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

impl ::private::Sealed for ReceiptRuleSet {}

impl From<ReceiptRuleSetProperties> for ReceiptRuleSet {
    fn from(properties: ReceiptRuleSetProperties) -> ReceiptRuleSet {
        ReceiptRuleSet { properties }
    }
}

/// The [`AWS::SES::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html) resource type.
#[derive(Debug)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateProperties {
    /// Property `Template`.
    #[serde(rename="Template")]
    pub template: self::template::Template,
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

impl ::private::Sealed for Template {}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

pub mod configuration_set_event_destination {
    //! Property types for the `ConfigurationSetEventDestination` resource.

    /// The [`AWS::SES::ConfigurationSetEventDestination.CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-cloudwatchdestination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchDestination {
        /// Property `DimensionConfigurations`.
        #[serde(rename="DimensionConfigurations")]
        pub dimension_configurations: Vec<DimensionConfiguration>,
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.DimensionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DimensionConfiguration {
        /// Property `DefaultDimensionValue`.
        #[serde(rename="DefaultDimensionValue")]
        pub default_dimension_value: String,
        /// Property `DimensionName`.
        #[serde(rename="DimensionName")]
        pub dimension_name: String,
        /// Property `DimensionValueSource`.
        #[serde(rename="DimensionValueSource")]
        pub dimension_value_source: String,
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EventDestination {
        /// Property `CloudWatchDestination`.
        #[serde(rename="CloudWatchDestination")]
        pub cloud_watch_destination: CloudWatchDestination,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `KinesisFirehoseDestination`.
        #[serde(rename="KinesisFirehoseDestination")]
        pub kinesis_firehose_destination: KinesisFirehoseDestination,
        /// Property `MatchingEventTypes`.
        #[serde(rename="MatchingEventTypes")]
        pub matching_event_types: Vec<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisFirehoseDestination {
        /// Property `DeliveryStreamARN`.
        #[serde(rename="DeliveryStreamARN")]
        pub delivery_stream_arn: String,
        /// Property `IAMRoleARN`.
        #[serde(rename="IAMRoleARN")]
        pub iam_role_arn: String,
    }
}

pub mod receipt_filter {
    //! Property types for the `ReceiptFilter` resource.

    /// The [`AWS::SES::ReceiptFilter.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Filter {
        /// Property `IpFilter`.
        #[serde(rename="IpFilter")]
        pub ip_filter: IpFilter,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::SES::ReceiptFilter.IpFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IpFilter {
        /// Property `Cidr`.
        #[serde(rename="Cidr")]
        pub cidr: String,
        /// Property `Policy`.
        #[serde(rename="Policy")]
        pub policy: String,
    }
}

pub mod receipt_rule {
    //! Property types for the `ReceiptRule` resource.

    /// The [`AWS::SES::ReceiptRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `AddHeaderAction`.
        #[serde(rename="AddHeaderAction")]
        pub add_header_action: AddHeaderAction,
        /// Property `BounceAction`.
        #[serde(rename="BounceAction")]
        pub bounce_action: BounceAction,
        /// Property `LambdaAction`.
        #[serde(rename="LambdaAction")]
        pub lambda_action: LambdaAction,
        /// Property `S3Action`.
        #[serde(rename="S3Action")]
        pub s3_action: S3Action,
        /// Property `SNSAction`.
        #[serde(rename="SNSAction")]
        pub sns_action: SNSAction,
        /// Property `StopAction`.
        #[serde(rename="StopAction")]
        pub stop_action: StopAction,
        /// Property `WorkmailAction`.
        #[serde(rename="WorkmailAction")]
        pub workmail_action: WorkmailAction,
    }

    /// The [`AWS::SES::ReceiptRule.AddHeaderAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddHeaderAction {
        /// Property `HeaderName`.
        #[serde(rename="HeaderName")]
        pub header_name: String,
        /// Property `HeaderValue`.
        #[serde(rename="HeaderValue")]
        pub header_value: String,
    }

    /// The [`AWS::SES::ReceiptRule.BounceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BounceAction {
        /// Property `Message`.
        #[serde(rename="Message")]
        pub message: String,
        /// Property `Sender`.
        #[serde(rename="Sender")]
        pub sender: String,
        /// Property `SmtpReplyCode`.
        #[serde(rename="SmtpReplyCode")]
        pub smtp_reply_code: String,
        /// Property `StatusCode`.
        #[serde(rename="StatusCode")]
        pub status_code: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }

    /// The [`AWS::SES::ReceiptRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaAction {
        /// Property `FunctionArn`.
        #[serde(rename="FunctionArn")]
        pub function_arn: String,
        /// Property `InvocationType`.
        #[serde(rename="InvocationType")]
        pub invocation_type: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }

    /// The [`AWS::SES::ReceiptRule.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `Actions`.
        #[serde(rename="Actions")]
        pub actions: Vec<Action>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Recipients`.
        #[serde(rename="Recipients")]
        pub recipients: Vec<String>,
        /// Property `ScanEnabled`.
        #[serde(rename="ScanEnabled")]
        pub scan_enabled: bool,
        /// Property `TlsPolicy`.
        #[serde(rename="TlsPolicy")]
        pub tls_policy: String,
    }

    /// The [`AWS::SES::ReceiptRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Action {
        /// Property `BucketName`.
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        /// Property `KmsKeyArn`.
        #[serde(rename="KmsKeyArn")]
        pub kms_key_arn: String,
        /// Property `ObjectKeyPrefix`.
        #[serde(rename="ObjectKeyPrefix")]
        pub object_key_prefix: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }

    /// The [`AWS::SES::ReceiptRule.SNSAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SNSAction {
        /// Property `Encoding`.
        #[serde(rename="Encoding")]
        pub encoding: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }

    /// The [`AWS::SES::ReceiptRule.StopAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StopAction {
        /// Property `Scope`.
        #[serde(rename="Scope")]
        pub scope: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }

    /// The [`AWS::SES::ReceiptRule.WorkmailAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WorkmailAction {
        /// Property `OrganizationArn`.
        #[serde(rename="OrganizationArn")]
        pub organization_arn: String,
        /// Property `TopicArn`.
        #[serde(rename="TopicArn")]
        pub topic_arn: String,
    }
}

pub mod template {
    //! Property types for the `Template` resource.

    /// The [`AWS::SES::Template.Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Template {
        /// Property `HtmlPart`.
        #[serde(rename="HtmlPart")]
        pub html_part: String,
        /// Property `SubjectPart`.
        #[serde(rename="SubjectPart")]
        pub subject_part: String,
        /// Property `TemplateName`.
        #[serde(rename="TemplateName")]
        pub template_name: String,
        /// Property `TextPart`.
        #[serde(rename="TextPart")]
        pub text_part: String,
    }
}
