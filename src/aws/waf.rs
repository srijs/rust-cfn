//! Types for the `WAF` service.

/// The [`AWS::WAF::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-bytematchset.html) resource type.
#[derive(Debug)]
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ByteMatchSetProperties {
    /// Property `ByteMatchTuples`.
    #[serde(rename = "ByteMatchTuples")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_match_tuples: Option<::ValueList<self::byte_match_set::ByteMatchTuple>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
}

impl<'a> ::Resource<'a> for ByteMatchSet {
    type Properties = ByteMatchSetProperties;
    const TYPE: &'static str = "AWS::WAF::ByteMatchSet";
    fn properties(&self) -> &ByteMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ByteMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ByteMatchSet {}

impl From<ByteMatchSetProperties> for ByteMatchSet {
    fn from(properties: ByteMatchSetProperties) -> ByteMatchSet {
        ByteMatchSet { properties }
    }
}

/// The [`AWS::WAF::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-ipset.html) resource type.
#[derive(Debug)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct IPSetProperties {
    /// Property `IPSetDescriptors`.
    #[serde(rename = "IPSetDescriptors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_set_descriptors: Option<::ValueList<self::ip_set::IPSetDescriptor>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
}

impl<'a> ::Resource<'a> for IPSet {
    type Properties = IPSetProperties;
    const TYPE: &'static str = "AWS::WAF::IPSet";
    fn properties(&self) -> &IPSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IPSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IPSet {}

impl From<IPSetProperties> for IPSet {
    fn from(properties: IPSetProperties) -> IPSet {
        IPSet { properties }
    }
}

/// The [`AWS::WAF::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-rule.html) resource type.
#[derive(Debug)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleProperties {
    /// Property `MetricName`.
    #[serde(rename = "MetricName")]
    pub metric_name: ::Value<String>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Predicates`.
    #[serde(rename = "Predicates")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicates: Option<::ValueList<self::rule::Predicate>>,
}

impl<'a> ::Resource<'a> for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::WAF::Rule";
    fn properties(&self) -> &RuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Rule {}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

/// The [`AWS::WAF::SizeConstraintSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-sizeconstraintset.html) resource type.
#[derive(Debug)]
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeConstraintSetProperties {
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `SizeConstraints`.
    #[serde(rename = "SizeConstraints")]
    pub size_constraints: ::ValueList<self::size_constraint_set::SizeConstraint>,
}

impl<'a> ::Resource<'a> for SizeConstraintSet {
    type Properties = SizeConstraintSetProperties;
    const TYPE: &'static str = "AWS::WAF::SizeConstraintSet";
    fn properties(&self) -> &SizeConstraintSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SizeConstraintSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SizeConstraintSet {}

impl From<SizeConstraintSetProperties> for SizeConstraintSet {
    fn from(properties: SizeConstraintSetProperties) -> SizeConstraintSet {
        SizeConstraintSet { properties }
    }
}

/// The [`AWS::WAF::SqlInjectionMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-sqlinjectionmatchset.html) resource type.
#[derive(Debug)]
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SqlInjectionMatchSetProperties {
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `SqlInjectionMatchTuples`.
    #[serde(rename = "SqlInjectionMatchTuples")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_tuples: Option<::ValueList<self::sql_injection_match_set::SqlInjectionMatchTuple>>,
}

impl<'a> ::Resource<'a> for SqlInjectionMatchSet {
    type Properties = SqlInjectionMatchSetProperties;
    const TYPE: &'static str = "AWS::WAF::SqlInjectionMatchSet";
    fn properties(&self) -> &SqlInjectionMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SqlInjectionMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SqlInjectionMatchSet {}

impl From<SqlInjectionMatchSetProperties> for SqlInjectionMatchSet {
    fn from(properties: SqlInjectionMatchSetProperties) -> SqlInjectionMatchSet {
        SqlInjectionMatchSet { properties }
    }
}

/// The [`AWS::WAF::WebACL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-webacl.html) resource type.
#[derive(Debug)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebACLProperties {
    /// Property `DefaultAction`.
    #[serde(rename = "DefaultAction")]
    pub default_action: ::Value<self::web_acl::WafAction>,
    /// Property `MetricName`.
    #[serde(rename = "MetricName")]
    pub metric_name: ::Value<String>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Rules`.
    #[serde(rename = "Rules")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<::ValueList<self::web_acl::ActivatedRule>>,
}

impl<'a> ::Resource<'a> for WebACL {
    type Properties = WebACLProperties;
    const TYPE: &'static str = "AWS::WAF::WebACL";
    fn properties(&self) -> &WebACLProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACL {}

impl From<WebACLProperties> for WebACL {
    fn from(properties: WebACLProperties) -> WebACL {
        WebACL { properties }
    }
}

/// The [`AWS::WAF::XssMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-xssmatchset.html) resource type.
#[derive(Debug)]
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct XssMatchSetProperties {
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `XssMatchTuples`.
    #[serde(rename = "XssMatchTuples")]
    pub xss_match_tuples: ::ValueList<self::xss_match_set::XssMatchTuple>,
}

impl<'a> ::Resource<'a> for XssMatchSet {
    type Properties = XssMatchSetProperties;
    const TYPE: &'static str = "AWS::WAF::XssMatchSet";
    fn properties(&self) -> &XssMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut XssMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for XssMatchSet {}

impl From<XssMatchSetProperties> for XssMatchSet {
    fn from(properties: XssMatchSetProperties) -> XssMatchSet {
        XssMatchSet { properties }
    }
}

pub mod byte_match_set {
    //! Property types for the `ByteMatchSet` resource.

    /// The [`AWS::WAF::ByteMatchSet.ByteMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ByteMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename = "FieldToMatch")]
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `PositionalConstraint`.
        #[serde(rename = "PositionalConstraint")]
        pub positional_constraint: ::Value<String>,
        /// Property `TargetString`.
        #[serde(rename = "TargetString")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_string: Option<::Value<String>>,
        /// Property `TargetStringBase64`.
        #[serde(rename = "TargetStringBase64")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_string_base64: Option<::Value<String>>,
        /// Property `TextTransformation`.
        #[serde(rename = "TextTransformation")]
        pub text_transformation: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ByteMatchTuple);

    /// The [`AWS::WAF::ByteMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename = "Data")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(FieldToMatch);
}

pub mod ip_set {
    //! Property types for the `IPSet` resource.

    /// The [`AWS::WAF::IPSet.IPSetDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-ipset-ipsetdescriptors.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IPSetDescriptor {
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(IPSetDescriptor);
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::WAF::Rule.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-rule-predicates.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Predicate {
        /// Property `DataId`.
        #[serde(rename = "DataId")]
        pub data_id: ::Value<String>,
        /// Property `Negated`.
        #[serde(rename = "Negated")]
        pub negated: ::Value<bool>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Predicate);
}

pub mod size_constraint_set {
    //! Property types for the `SizeConstraintSet` resource.

    /// The [`AWS::WAF::SizeConstraintSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename = "Data")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(FieldToMatch);

    /// The [`AWS::WAF::SizeConstraintSet.SizeConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SizeConstraint {
        /// Property `ComparisonOperator`.
        #[serde(rename = "ComparisonOperator")]
        pub comparison_operator: ::Value<String>,
        /// Property `FieldToMatch`.
        #[serde(rename = "FieldToMatch")]
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `Size`.
        #[serde(rename = "Size")]
        pub size: ::Value<u32>,
        /// Property `TextTransformation`.
        #[serde(rename = "TextTransformation")]
        pub text_transformation: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SizeConstraint);
}

pub mod sql_injection_match_set {
    //! Property types for the `SqlInjectionMatchSet` resource.

    /// The [`AWS::WAF::SqlInjectionMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename = "Data")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(FieldToMatch);

    /// The [`AWS::WAF::SqlInjectionMatchSet.SqlInjectionMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sqlinjectionmatchset-sqlinjectionmatchtuples.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SqlInjectionMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename = "FieldToMatch")]
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `TextTransformation`.
        #[serde(rename = "TextTransformation")]
        pub text_transformation: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SqlInjectionMatchTuple);
}

pub mod web_acl {
    //! Property types for the `WebACL` resource.

    /// The [`AWS::WAF::WebACL.ActivatedRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-rules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ActivatedRule {
        /// Property `Action`.
        #[serde(rename = "Action")]
        pub action: ::Value<WafAction>,
        /// Property `Priority`.
        #[serde(rename = "Priority")]
        pub priority: ::Value<u32>,
        /// Property `RuleId`.
        #[serde(rename = "RuleId")]
        pub rule_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ActivatedRule);

    /// The [`AWS::WAF::WebACL.WafAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WafAction {
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(WafAction);
}

pub mod xss_match_set {
    //! Property types for the `XssMatchSet` resource.

    /// The [`AWS::WAF::XssMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename = "Data")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(FieldToMatch);

    /// The [`AWS::WAF::XssMatchSet.XssMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct XssMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename = "FieldToMatch")]
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `TextTransformation`.
        #[serde(rename = "TextTransformation")]
        pub text_transformation: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(XssMatchTuple);
}
