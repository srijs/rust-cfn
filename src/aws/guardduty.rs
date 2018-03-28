/// The [`AWS::GuardDuty::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Serialize, Deserialize)]
pub struct DetectorProperties {
    #[serde(rename="Enable")]
    pub enable: (),
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

impl From<DetectorProperties> for Detector {
    fn from(properties: DetectorProperties) -> Detector {
        Detector { properties }
    }
}

/// The [`AWS::GuardDuty::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Serialize, Deserialize)]
pub struct IPSetProperties {
    #[serde(rename="Activate")]
    pub activate: (),
    #[serde(rename="DetectorId")]
    pub detector_id: (),
    #[serde(rename="Format")]
    pub format: (),
    #[serde(rename="Location")]
    pub location: (),
    #[serde(rename="Name")]
    pub name: (),
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

impl From<IPSetProperties> for IPSet {
    fn from(properties: IPSetProperties) -> IPSet {
        IPSet { properties }
    }
}

/// The [`AWS::GuardDuty::Master`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Master {
    properties: MasterProperties
}

/// Properties for the `Master` resource.
#[derive(Serialize, Deserialize)]
pub struct MasterProperties {
    #[serde(rename="DetectorId")]
    pub detector_id: (),
    #[serde(rename="InvitationId")]
    pub invitation_id: (),
    #[serde(rename="MasterId")]
    pub master_id: (),
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

impl From<MasterProperties> for Master {
    fn from(properties: MasterProperties) -> Master {
        Master { properties }
    }
}

/// The [`AWS::GuardDuty::Member`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Member {
    properties: MemberProperties
}

/// Properties for the `Member` resource.
#[derive(Serialize, Deserialize)]
pub struct MemberProperties {
    #[serde(rename="DetectorId")]
    pub detector_id: (),
    #[serde(rename="Email")]
    pub email: (),
    #[serde(rename="MemberId")]
    pub member_id: (),
    #[serde(rename="Message")]
    pub message: (),
    #[serde(rename="Status")]
    pub status: (),
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

impl From<MemberProperties> for Member {
    fn from(properties: MemberProperties) -> Member {
        Member { properties }
    }
}

/// The [`AWS::GuardDuty::ThreatIntelSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ThreatIntelSet {
    properties: ThreatIntelSetProperties
}

/// Properties for the `ThreatIntelSet` resource.
#[derive(Serialize, Deserialize)]
pub struct ThreatIntelSetProperties {
    #[serde(rename="Activate")]
    pub activate: (),
    #[serde(rename="DetectorId")]
    pub detector_id: (),
    #[serde(rename="Format")]
    pub format: (),
    #[serde(rename="Location")]
    pub location: (),
    #[serde(rename="Name")]
    pub name: (),
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

impl From<ThreatIntelSetProperties> for ThreatIntelSet {
    fn from(properties: ThreatIntelSetProperties) -> ThreatIntelSet {
        ThreatIntelSet { properties }
    }
}

