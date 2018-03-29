/// The [`AWS::WAF::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-bytematchset.html) resource type.
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct ByteMatchSetProperties {
    #[serde(rename="ByteMatchTuples")]
    pub byte_match_tuples: Vec<self::byte_match_set::ByteMatchTuple>,
    #[serde(rename="Name")]
    pub name: String,
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
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Serialize, Deserialize)]
pub struct IPSetProperties {
    #[serde(rename="IPSetDescriptors")]
    pub ip_set_descriptors: Vec<self::ip_set::IPSetDescriptor>,
    #[serde(rename="Name")]
    pub name: String,
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
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Serialize, Deserialize)]
pub struct RuleProperties {
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Predicates")]
    pub predicates: Vec<self::rule::Predicate>,
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
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Serialize, Deserialize)]
pub struct SizeConstraintSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="SizeConstraints")]
    pub size_constraints: Vec<self::size_constraint_set::SizeConstraint>,
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
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct SqlInjectionMatchSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Vec<self::sql_injection_match_set::SqlInjectionMatchTuple>,
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
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Serialize, Deserialize)]
pub struct WebACLProperties {
    #[serde(rename="DefaultAction")]
    pub default_action: self::web_acl::WafAction,
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Rules")]
    pub rules: Vec<self::web_acl::ActivatedRule>,
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
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct XssMatchSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="XssMatchTuples")]
    pub xss_match_tuples: Vec<self::xss_match_set::XssMatchTuple>,
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
    /// The [`AWS::WAF::ByteMatchSet.ByteMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ByteMatchTuple {
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        #[serde(rename="PositionalConstraint")]
        pub positional_constraint: String,
        #[serde(rename="TargetString")]
        pub target_string: String,
        #[serde(rename="TargetStringBase64")]
        pub target_string_base64: String,
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }

    /// The [`AWS::WAF::ByteMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct FieldToMatch {
        #[serde(rename="Data")]
        pub data: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod ip_set {
    /// The [`AWS::WAF::IPSet.IPSetDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-ipset-ipsetdescriptors.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct IPSetDescriptor {
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

pub mod rule {
    /// The [`AWS::WAF::Rule.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-rule-predicates.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Predicate {
        #[serde(rename="DataId")]
        pub data_id: String,
        #[serde(rename="Negated")]
        pub negated: bool,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod size_constraint_set {
    /// The [`AWS::WAF::SizeConstraintSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint-fieldtomatch.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct FieldToMatch {
        #[serde(rename="Data")]
        pub data: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAF::SizeConstraintSet.SizeConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SizeConstraint {
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        #[serde(rename="Size")]
        pub size: u32,
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }

}

pub mod sql_injection_match_set {
    /// The [`AWS::WAF::SqlInjectionMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct FieldToMatch {
        #[serde(rename="Data")]
        pub data: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAF::SqlInjectionMatchSet.SqlInjectionMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sqlinjectionmatchset-sqlinjectionmatchtuples.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SqlInjectionMatchTuple {
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }

}

pub mod web_acl {
    /// The [`AWS::WAF::WebACL.ActivatedRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-rules.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ActivatedRule {
        #[serde(rename="Action")]
        pub action: WafAction,
        #[serde(rename="Priority")]
        pub priority: u32,
        #[serde(rename="RuleId")]
        pub rule_id: String,
    }

    /// The [`AWS::WAF::WebACL.WafAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-action.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct WafAction {
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod xss_match_set {
    /// The [`AWS::WAF::XssMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple-fieldtomatch.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct FieldToMatch {
        #[serde(rename="Data")]
        pub data: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::WAF::XssMatchSet.XssMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct XssMatchTuple {
        #[serde(rename="FieldToMatch")]
        pub field_to_match: FieldToMatch,
        #[serde(rename="TextTransformation")]
        pub text_transformation: String,
    }

}

