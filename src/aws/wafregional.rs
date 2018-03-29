//! Types for the `WAFRegional` service.

/// The [`AWS::WAFRegional::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html) resource type.
#[derive(Debug)]
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ByteMatchSetProperties {
    /// Property `ByteMatchTuples`.
    #[serde(rename="ByteMatchTuples")]
    pub byte_match_tuples: Vec<self::byte_match_set::ByteMatchTuple>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
}

impl<'a> ::Resource<'a> for ByteMatchSet {
    type Properties = ByteMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::ByteMatchSet";
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

/// The [`AWS::WAFRegional::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html) resource type.
#[derive(Debug)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct IPSetProperties {
    /// Property `IPSetDescriptors`.
    #[serde(rename="IPSetDescriptors")]
    pub ip_set_descriptors: Vec<self::ip_set::IPSetDescriptor>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
}

impl<'a> ::Resource<'a> for IPSet {
    type Properties = IPSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::IPSet";
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

/// The [`AWS::WAFRegional::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html) resource type.
#[derive(Debug)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleProperties {
    /// Property `MetricName`.
    #[serde(rename="MetricName")]
    pub metric_name: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Predicates`.
    #[serde(rename="Predicates")]
    pub predicates: Vec<self::rule::Predicate>,
}

impl<'a> ::Resource<'a> for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::WAFRegional::Rule";
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

/// The [`AWS::WAFRegional::SizeConstraintSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html) resource type.
#[derive(Debug)]
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeConstraintSetProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `SizeConstraints`.
    #[serde(rename="SizeConstraints")]
    pub size_constraints: Vec<self::size_constraint_set::SizeConstraint>,
}

impl<'a> ::Resource<'a> for SizeConstraintSet {
    type Properties = SizeConstraintSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::SizeConstraintSet";
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

/// The [`AWS::WAFRegional::SqlInjectionMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html) resource type.
#[derive(Debug)]
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SqlInjectionMatchSetProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `SqlInjectionMatchTuples`.
    #[serde(rename="SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Vec<self::sql_injection_match_set::SqlInjectionMatchTuple>,
}

impl<'a> ::Resource<'a> for SqlInjectionMatchSet {
    type Properties = SqlInjectionMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::SqlInjectionMatchSet";
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

/// The [`AWS::WAFRegional::WebACL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html) resource type.
#[derive(Debug)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebACLProperties {
    /// Property `DefaultAction`.
    #[serde(rename="DefaultAction")]
    pub default_action: self::web_acl::Action,
    /// Property `MetricName`.
    #[serde(rename="MetricName")]
    pub metric_name: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Rules`.
    #[serde(rename="Rules")]
    pub rules: Vec<self::web_acl::Rule>,
}

impl<'a> ::Resource<'a> for WebACL {
    type Properties = WebACLProperties;
    const TYPE: &'static str = "AWS::WAFRegional::WebACL";
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

/// The [`AWS::WAFRegional::WebACLAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html) resource type.
#[derive(Debug)]
pub struct WebACLAssociation {
    properties: WebACLAssociationProperties
}

/// Properties for the `WebACLAssociation` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebACLAssociationProperties {
    /// Property `ResourceArn`.
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    /// Property `WebACLId`.
    #[serde(rename="WebACLId")]
    pub web_acl_id: String,
}

impl<'a> ::Resource<'a> for WebACLAssociation {
    type Properties = WebACLAssociationProperties;
    const TYPE: &'static str = "AWS::WAFRegional::WebACLAssociation";
    fn properties(&self) -> &WebACLAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACLAssociation {}

impl From<WebACLAssociationProperties> for WebACLAssociation {
    fn from(properties: WebACLAssociationProperties) -> WebACLAssociation {
        WebACLAssociation { properties }
    }
}

/// The [`AWS::WAFRegional::XssMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html) resource type.
#[derive(Debug)]
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct XssMatchSetProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `XssMatchTuples`.
    #[serde(rename="XssMatchTuples")]
    pub xss_match_tuples: Vec<self::xss_match_set::XssMatchTuple>,
}

impl<'a> ::Resource<'a> for XssMatchSet {
    type Properties = XssMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::XssMatchSet";
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

    /// The [`AWS::WAFRegional::ByteMatchSet.ByteMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ByteMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        /// Property `PositionalConstraint`.
        #[serde(rename="PositionalConstraint")]
        pub positional_constraint: String,
        /// Property `TargetString`.
        #[serde(rename="TargetString")]
        pub target_string: String,
        /// Property `TargetStringBase64`.
        #[serde(rename="TargetStringBase64")]
        pub target_string_base64: String,
        /// Property `TextTransformation`.
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }

    /// The [`AWS::WAFRegional::ByteMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename="Data")]
        pub data: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod ip_set {
    //! Property types for the `IPSet` resource.

    /// The [`AWS::WAFRegional::IPSet.IPSetDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ipset-ipsetdescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IPSetDescriptor {
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::WAFRegional::Rule.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Predicate {
        /// Property `DataId`.
        #[serde(rename="DataId")]
        pub data_id: String,
        /// Property `Negated`.
        #[serde(rename="Negated")]
        pub negated: bool,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod size_constraint_set {
    //! Property types for the `SizeConstraintSet` resource.

    /// The [`AWS::WAFRegional::SizeConstraintSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename="Data")]
        pub data: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAFRegional::SizeConstraintSet.SizeConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SizeConstraint {
        /// Property `ComparisonOperator`.
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        /// Property `FieldToMatch`.
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        /// Property `Size`.
        #[serde(rename="Size")]
        pub size: u32,
        /// Property `TextTransformation`.
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }
}

pub mod sql_injection_match_set {
    //! Property types for the `SqlInjectionMatchSet` resource.

    /// The [`AWS::WAFRegional::SqlInjectionMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename="Data")]
        pub data: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAFRegional::SqlInjectionMatchSet.SqlInjectionMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SqlInjectionMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        /// Property `TextTransformation`.
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }
}

pub mod web_acl {
    //! Property types for the `WebACL` resource.

    /// The [`AWS::WAFRegional::WebACL.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAFRegional::WebACL.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `Action`.
        #[serde(rename="Action")]
        pub action: Action,
        /// Property `Priority`.
        #[serde(rename="Priority")]
        pub priority: u32,
        /// Property `RuleId`.
        #[serde(rename="RuleId")]
        pub rule_id: String,
    }
}

pub mod xss_match_set {
    //! Property types for the `XssMatchSet` resource.

    /// The [`AWS::WAFRegional::XssMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-fieldtomatch.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FieldToMatch {
        /// Property `Data`.
        #[serde(rename="Data")]
        pub data: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAFRegional::XssMatchSet.XssMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-xssmatchtuple.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct XssMatchTuple {
        /// Property `FieldToMatch`.
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        /// Property `TextTransformation`.
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }
}
