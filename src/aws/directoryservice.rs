/// The [`AWS::DirectoryService::MicrosoftAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-microsoftad.html) resource.
#[derive(Serialize, Deserialize)]
pub struct MicrosoftAD {
    properties: MicrosoftADProperties
}

/// Properties for the `MicrosoftAD` resource.
#[derive(Serialize, Deserialize)]
pub struct MicrosoftADProperties {
    #[serde(rename="CreateAlias")]
    pub create_alias: (),
    #[serde(rename="EnableSso")]
    pub enable_sso: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Password")]
    pub password: (),
    #[serde(rename="ShortName")]
    pub short_name: (),
    #[serde(rename="VpcSettings")]
    pub vpc_settings: (),
}

impl<'a> ::Resource<'a> for MicrosoftAD {
    type Properties = MicrosoftADProperties;
    const TYPE: &'static str = "AWS::DirectoryService::MicrosoftAD";
    fn properties(&self) -> &MicrosoftADProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MicrosoftADProperties {
        &mut self.properties
    }
}

impl From<MicrosoftADProperties> for MicrosoftAD {
    fn from(properties: MicrosoftADProperties) -> MicrosoftAD {
        MicrosoftAD { properties }
    }
}

/// The [`AWS::DirectoryService::SimpleAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-simplead.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SimpleAD {
    properties: SimpleADProperties
}

/// Properties for the `SimpleAD` resource.
#[derive(Serialize, Deserialize)]
pub struct SimpleADProperties {
    #[serde(rename="CreateAlias")]
    pub create_alias: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="EnableSso")]
    pub enable_sso: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Password")]
    pub password: (),
    #[serde(rename="ShortName")]
    pub short_name: (),
    #[serde(rename="Size")]
    pub size: (),
    #[serde(rename="VpcSettings")]
    pub vpc_settings: (),
}

impl<'a> ::Resource<'a> for SimpleAD {
    type Properties = SimpleADProperties;
    const TYPE: &'static str = "AWS::DirectoryService::SimpleAD";
    fn properties(&self) -> &SimpleADProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SimpleADProperties {
        &mut self.properties
    }
}

impl From<SimpleADProperties> for SimpleAD {
    fn from(properties: SimpleADProperties) -> SimpleAD {
        SimpleAD { properties }
    }
}

