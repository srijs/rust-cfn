/// The [`AWS::KinesisAnalytics::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-application.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename="ApplicationCode")]
    pub application_code: String,
    #[serde(rename="ApplicationDescription")]
    pub application_description: String,
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Inputs")]
    pub inputs: Vec<()>,
}

impl<'a> ::Resource<'a> for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::KinesisAnalytics::ApplicationOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationoutput.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationOutput {
    properties: ApplicationOutputProperties
}

/// Properties for the `ApplicationOutput` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationOutputProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Output")]
    pub output: (),
}

impl<'a> ::Resource<'a> for ApplicationOutput {
    type Properties = ApplicationOutputProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::ApplicationOutput";
    fn properties(&self) -> &ApplicationOutputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationOutputProperties {
        &mut self.properties
    }
}

impl From<ApplicationOutputProperties> for ApplicationOutput {
    fn from(properties: ApplicationOutputProperties) -> ApplicationOutput {
        ApplicationOutput { properties }
    }
}

/// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationreferencedatasource.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationReferenceDataSource {
    properties: ApplicationReferenceDataSourceProperties
}

/// Properties for the `ApplicationReferenceDataSource` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationReferenceDataSourceProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="ReferenceDataSource")]
    pub reference_data_source: (),
}

impl<'a> ::Resource<'a> for ApplicationReferenceDataSource {
    type Properties = ApplicationReferenceDataSourceProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::ApplicationReferenceDataSource";
    fn properties(&self) -> &ApplicationReferenceDataSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationReferenceDataSourceProperties {
        &mut self.properties
    }
}

impl From<ApplicationReferenceDataSourceProperties> for ApplicationReferenceDataSource {
    fn from(properties: ApplicationReferenceDataSourceProperties) -> ApplicationReferenceDataSource {
        ApplicationReferenceDataSource { properties }
    }
}

