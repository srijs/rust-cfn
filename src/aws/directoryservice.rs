//! Types for the `DirectoryService` service.

/// The [`AWS::DirectoryService::MicrosoftAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-microsoftad.html) resource type.
#[derive(Debug)]
pub struct MicrosoftAD {
    properties: MicrosoftADProperties
}

/// Properties for the `MicrosoftAD` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MicrosoftADProperties {
    /// Property `CreateAlias`.
    #[serde(rename="CreateAlias")]
    pub create_alias: bool,
    /// Property `EnableSso`.
    #[serde(rename="EnableSso")]
    pub enable_sso: bool,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Password`.
    #[serde(rename="Password")]
    pub password: String,
    /// Property `ShortName`.
    #[serde(rename="ShortName")]
    pub short_name: String,
    /// Property `VpcSettings`.
    #[serde(rename="VpcSettings")]
    pub vpc_settings: self::microsoft_ad::VpcSettings,
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

impl ::private::Sealed for MicrosoftAD {}

impl From<MicrosoftADProperties> for MicrosoftAD {
    fn from(properties: MicrosoftADProperties) -> MicrosoftAD {
        MicrosoftAD { properties }
    }
}

/// The [`AWS::DirectoryService::SimpleAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-simplead.html) resource type.
#[derive(Debug)]
pub struct SimpleAD {
    properties: SimpleADProperties
}

/// Properties for the `SimpleAD` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleADProperties {
    /// Property `CreateAlias`.
    #[serde(rename="CreateAlias")]
    pub create_alias: bool,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `EnableSso`.
    #[serde(rename="EnableSso")]
    pub enable_sso: bool,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Password`.
    #[serde(rename="Password")]
    pub password: String,
    /// Property `ShortName`.
    #[serde(rename="ShortName")]
    pub short_name: String,
    /// Property `Size`.
    #[serde(rename="Size")]
    pub size: String,
    /// Property `VpcSettings`.
    #[serde(rename="VpcSettings")]
    pub vpc_settings: self::simple_ad::VpcSettings,
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

impl ::private::Sealed for SimpleAD {}

impl From<SimpleADProperties> for SimpleAD {
    fn from(properties: SimpleADProperties) -> SimpleAD {
        SimpleAD { properties }
    }
}

pub mod microsoft_ad {
    //! Property types for the `MicrosoftAD` resource.

    /// The [`AWS::DirectoryService::MicrosoftAD.VpcSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-microsoftad-vpcsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcSettings {
        /// Property `SubnetIds`.
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
        /// Property `VpcId`.
        #[serde(rename="VpcId")]
        pub vpc_id: String,
    }
}

pub mod simple_ad {
    //! Property types for the `SimpleAD` resource.

    /// The [`AWS::DirectoryService::SimpleAD.VpcSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-simplead-vpcsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcSettings {
        /// Property `SubnetIds`.
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
        /// Property `VpcId`.
        #[serde(rename="VpcId")]
        pub vpc_id: String,
    }
}
