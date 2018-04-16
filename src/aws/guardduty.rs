//! Types for the `GuardDuty` service.

/// The [`AWS::GuardDuty::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html) resource type.
#[derive(Debug)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Debug)]
pub struct DetectorProperties {
    /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html#cfn-guardduty-detector-enable).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable: ::Value<bool>,
}

impl ::serde::Serialize for DetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                let mut enable: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enable" => {
                            enable = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Detector {
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
    /// Property [`Activate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html#cfn-guardduty-ipset-activate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub activate: ::Value<bool>,
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html#cfn-guardduty-ipset-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html#cfn-guardduty-ipset-format).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub format: ::Value<String>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html#cfn-guardduty-ipset-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html#cfn-guardduty-ipset-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for IPSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activate", &self.activate)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut activate: Option<::Value<bool>> = None;
                let mut detector_id: Option<::Value<String>> = None;
                let mut format: Option<::Value<String>> = None;
                let mut location: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Activate" => {
                            activate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Format" => {
                            format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for IPSet {
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
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html#cfn-guardduty-master-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`InvitationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html#cfn-guardduty-master-invitationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub invitation_id: ::Value<String>,
    /// Property [`MasterId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html#cfn-guardduty-master-masterid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_id: ::Value<String>,
}

impl ::serde::Serialize for MasterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                let mut detector_id: Option<::Value<String>> = None;
                let mut invitation_id: Option<::Value<String>> = None;
                let mut master_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InvitationId" => {
                            invitation_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterId" => {
                            master_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Master {
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
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`Email`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-email).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub email: ::Value<String>,
    /// Property [`MemberId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-memberid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub member_id: ::Value<String>,
    /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-message).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub message: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
}

impl ::serde::Serialize for MemberProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", &self.email)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberId", &self.member_id)?;
        if let Some(ref message) = self.message {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
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
                let mut detector_id: Option<::Value<String>> = None;
                let mut email: Option<::Value<String>> = None;
                let mut member_id: Option<::Value<String>> = None;
                let mut message: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Email" => {
                            email = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberId" => {
                            member_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Message" => {
                            message = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Member {
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
    /// Property [`Activate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html#cfn-guardduty-threatintelset-activate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub activate: ::Value<bool>,
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html#cfn-guardduty-threatintelset-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html#cfn-guardduty-threatintelset-format).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub format: ::Value<String>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html#cfn-guardduty-threatintelset-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html#cfn-guardduty-threatintelset-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ThreatIntelSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activate", &self.activate)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut activate: Option<::Value<bool>> = None;
                let mut detector_id: Option<::Value<String>> = None;
                let mut format: Option<::Value<String>> = None;
                let mut location: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Activate" => {
                            activate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Format" => {
                            format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ThreatIntelSet {
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
