//! Types for the `GuardDuty` service.

/// The [`AWS::GuardDuty::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html) resource type.
#[derive(Debug)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DetectorProperties {
    /// Property `Enable`.
    #[serde(rename="Enable")]
    pub enable: bool,
}

impl<'a> ::Resource<'a> for Detector {
    type Properties = DetectorProperties;
    const TYPE: &'static str = "AWS::GuardDuty::Detector";
    fn properties(&self) -> &DetectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DetectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Detector {}

impl From<DetectorProperties> for Detector {
    fn from(properties: DetectorProperties) -> Detector {
        Detector { properties }
    }
}

/// The [`AWS::GuardDuty::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html) resource type.
#[derive(Debug)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct IPSetProperties {
    /// Property `Activate`.
    #[serde(rename="Activate")]
    pub activate: bool,
    /// Property `DetectorId`.
    #[serde(rename="DetectorId")]
    pub detector_id: String,
    /// Property `Format`.
    #[serde(rename="Format")]
    pub format: String,
    /// Property `Location`.
    #[serde(rename="Location")]
    pub location: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl<'a> ::Resource<'a> for IPSet {
    type Properties = IPSetProperties;
    const TYPE: &'static str = "AWS::GuardDuty::IPSet";
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

/// The [`AWS::GuardDuty::Master`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html) resource type.
#[derive(Debug)]
pub struct Master {
    properties: MasterProperties
}

/// Properties for the `Master` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MasterProperties {
    /// Property `DetectorId`.
    #[serde(rename="DetectorId")]
    pub detector_id: String,
    /// Property `InvitationId`.
    #[serde(rename="InvitationId")]
    pub invitation_id: String,
    /// Property `MasterId`.
    #[serde(rename="MasterId")]
    pub master_id: String,
}

impl<'a> ::Resource<'a> for Master {
    type Properties = MasterProperties;
    const TYPE: &'static str = "AWS::GuardDuty::Master";
    fn properties(&self) -> &MasterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MasterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Master {}

impl From<MasterProperties> for Master {
    fn from(properties: MasterProperties) -> Master {
        Master { properties }
    }
}

/// The [`AWS::GuardDuty::Member`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html) resource type.
#[derive(Debug)]
pub struct Member {
    properties: MemberProperties
}

/// Properties for the `Member` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberProperties {
    /// Property `DetectorId`.
    #[serde(rename="DetectorId")]
    pub detector_id: String,
    /// Property `Email`.
    #[serde(rename="Email")]
    pub email: String,
    /// Property `MemberId`.
    #[serde(rename="MemberId")]
    pub member_id: String,
    /// Property `Message`.
    #[serde(rename="Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Property `Status`.
    #[serde(rename="Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl<'a> ::Resource<'a> for Member {
    type Properties = MemberProperties;
    const TYPE: &'static str = "AWS::GuardDuty::Member";
    fn properties(&self) -> &MemberProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MemberProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Member {}

impl From<MemberProperties> for Member {
    fn from(properties: MemberProperties) -> Member {
        Member { properties }
    }
}

/// The [`AWS::GuardDuty::ThreatIntelSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html) resource type.
#[derive(Debug)]
pub struct ThreatIntelSet {
    properties: ThreatIntelSetProperties
}

/// Properties for the `ThreatIntelSet` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatIntelSetProperties {
    /// Property `Activate`.
    #[serde(rename="Activate")]
    pub activate: bool,
    /// Property `DetectorId`.
    #[serde(rename="DetectorId")]
    pub detector_id: String,
    /// Property `Format`.
    #[serde(rename="Format")]
    pub format: String,
    /// Property `Location`.
    #[serde(rename="Location")]
    pub location: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl<'a> ::Resource<'a> for ThreatIntelSet {
    type Properties = ThreatIntelSetProperties;
    const TYPE: &'static str = "AWS::GuardDuty::ThreatIntelSet";
    fn properties(&self) -> &ThreatIntelSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThreatIntelSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThreatIntelSet {}

impl From<ThreatIntelSetProperties> for ThreatIntelSet {
    fn from(properties: ThreatIntelSetProperties) -> ThreatIntelSet {
        ThreatIntelSet { properties }
    }
}
