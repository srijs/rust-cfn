/// The [`AWS::WAFRegional::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct ByteMatchSetProperties {
    #[serde(rename="ByteMatchTuples")]
    pub byte_match_tuples: Vec<()>,
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

impl From<ByteMatchSetProperties> for ByteMatchSet {
    fn from(properties: ByteMatchSetProperties) -> ByteMatchSet {
        ByteMatchSet { properties }
    }
}

/// The [`AWS::WAFRegional::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Serialize, Deserialize)]
pub struct IPSetProperties {
    #[serde(rename="IPSetDescriptors")]
    pub ip_set_descriptors: Vec<()>,
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

impl From<IPSetProperties> for IPSet {
    fn from(properties: IPSetProperties) -> IPSet {
        IPSet { properties }
    }
}

/// The [`AWS::WAFRegional::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html) resource.
#[derive(Serialize, Deserialize)]
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
    pub predicates: Vec<()>,
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

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

/// The [`AWS::WAFRegional::SizeConstraintSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Serialize, Deserialize)]
pub struct SizeConstraintSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="SizeConstraints")]
    pub size_constraints: Vec<()>,
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

impl From<SizeConstraintSetProperties> for SizeConstraintSet {
    fn from(properties: SizeConstraintSetProperties) -> SizeConstraintSet {
        SizeConstraintSet { properties }
    }
}

/// The [`AWS::WAFRegional::SqlInjectionMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct SqlInjectionMatchSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Vec<()>,
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

impl From<SqlInjectionMatchSetProperties> for SqlInjectionMatchSet {
    fn from(properties: SqlInjectionMatchSetProperties) -> SqlInjectionMatchSet {
        SqlInjectionMatchSet { properties }
    }
}

/// The [`AWS::WAFRegional::WebACL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html) resource.
#[derive(Serialize, Deserialize)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Serialize, Deserialize)]
pub struct WebACLProperties {
    #[serde(rename="DefaultAction")]
    pub default_action: (),
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Rules")]
    pub rules: Vec<()>,
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

impl From<WebACLProperties> for WebACL {
    fn from(properties: WebACLProperties) -> WebACL {
        WebACL { properties }
    }
}

/// The [`AWS::WAFRegional::WebACLAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html) resource.
#[derive(Serialize, Deserialize)]
pub struct WebACLAssociation {
    properties: WebACLAssociationProperties
}

/// Properties for the `WebACLAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct WebACLAssociationProperties {
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
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

impl From<WebACLAssociationProperties> for WebACLAssociation {
    fn from(properties: WebACLAssociationProperties) -> WebACLAssociation {
        WebACLAssociation { properties }
    }
}

/// The [`AWS::WAFRegional::XssMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Serialize, Deserialize)]
pub struct XssMatchSetProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="XssMatchTuples")]
    pub xss_match_tuples: Vec<()>,
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

impl From<XssMatchSetProperties> for XssMatchSet {
    fn from(properties: XssMatchSetProperties) -> XssMatchSet {
        XssMatchSet { properties }
    }
}

