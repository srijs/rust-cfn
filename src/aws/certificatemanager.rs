/// The [`AWS::CertificateManager::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="DomainValidationOptions")]
    pub domain_validation_options: Vec<()>,
    #[serde(rename="SubjectAlternativeNames")]
    pub subject_alternative_names: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
}

impl<'a> ::Resource<'a> for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::CertificateManager::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}
