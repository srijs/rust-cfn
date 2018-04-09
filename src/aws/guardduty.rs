//! Types for the `GuardDuty` service.

/// The [`AWS::GuardDuty::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html) resource type.
#[derive(Debug)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Debug)]
pub struct DetectorProperties {
    /// Property `Enable`.
    pub enable: ::Value<bool>,
}

impl ::serde::Serialize for DetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", &self.enable)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DetectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DetectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DetectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DetectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enable = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enable" => {
                            enable = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DetectorProperties {
                    enable: enable.ok_or(::serde::de::Error::missing_field("Enable"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct IPSetProperties {
    /// Property `Activate`.
    pub activate: ::Value<bool>,
    /// Property `DetectorId`.
    pub detector_id: ::Value<String>,
    /// Property `Format`.
    pub format: ::Value<String>,
    /// Property `Location`.
    pub location: ::Value<String>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for IPSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activate", &self.activate)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IPSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IPSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IPSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut activate = None;
                let mut detector_id = None;
                let mut format = None;
                let mut location = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Activate" => {
                            activate = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DetectorId" => {
                            detector_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Format" => {
                            format = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Location" => {
                            location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(IPSetProperties {
                    activate: activate.ok_or(::serde::de::Error::missing_field("Activate"))?,
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                    location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct MasterProperties {
    /// Property `DetectorId`.
    pub detector_id: ::Value<String>,
    /// Property `InvitationId`.
    pub invitation_id: ::Value<String>,
    /// Property `MasterId`.
    pub master_id: ::Value<String>,
}

impl ::serde::Serialize for MasterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvitationId", &self.invitation_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterId", &self.master_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MasterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MasterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MasterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MasterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut detector_id = None;
                let mut invitation_id = None;
                let mut master_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorId" => {
                            detector_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InvitationId" => {
                            invitation_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterId" => {
                            master_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(MasterProperties {
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    invitation_id: invitation_id.ok_or(::serde::de::Error::missing_field("InvitationId"))?,
                    master_id: master_id.ok_or(::serde::de::Error::missing_field("MasterId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct MemberProperties {
    /// Property `DetectorId`.
    pub detector_id: ::Value<String>,
    /// Property `Email`.
    pub email: ::Value<String>,
    /// Property `MemberId`.
    pub member_id: ::Value<String>,
    /// Property `Message`.
    pub message: Option<::Value<String>>,
    /// Property `Status`.
    pub status: Option<::Value<String>>,
}

impl ::serde::Serialize for MemberProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", &self.email)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberId", &self.member_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", &self.message)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MemberProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MemberProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MemberProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut detector_id = None;
                let mut email = None;
                let mut member_id = None;
                let mut message = None;
                let mut status = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorId" => {
                            detector_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Email" => {
                            email = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MemberId" => {
                            member_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Message" => {
                            message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Status" => {
                            status = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(MemberProperties {
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    email: email.ok_or(::serde::de::Error::missing_field("Email"))?,
                    member_id: member_id.ok_or(::serde::de::Error::missing_field("MemberId"))?,
                    message: message,
                    status: status,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ThreatIntelSetProperties {
    /// Property `Activate`.
    pub activate: ::Value<bool>,
    /// Property `DetectorId`.
    pub detector_id: ::Value<String>,
    /// Property `Format`.
    pub format: ::Value<String>,
    /// Property `Location`.
    pub location: ::Value<String>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ThreatIntelSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activate", &self.activate)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThreatIntelSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThreatIntelSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThreatIntelSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThreatIntelSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut activate = None;
                let mut detector_id = None;
                let mut format = None;
                let mut location = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Activate" => {
                            activate = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DetectorId" => {
                            detector_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Format" => {
                            format = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Location" => {
                            location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ThreatIntelSetProperties {
                    activate: activate.ok_or(::serde::de::Error::missing_field("Activate"))?,
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                    location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
