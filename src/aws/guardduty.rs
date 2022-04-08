//! Types for the `GuardDuty` service.

/// The [`AWS::GuardDuty::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html) resource type.
#[derive(Debug, Default)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Debug, Default)]
pub struct DetectorProperties {
    /// Property [`DataSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html#cfn-guardduty-detector-datasources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_sources: Option<::Value<self::detector::CFNDataSourceConfigurations>>,
    /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html#cfn-guardduty-detector-enable).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable: ::Value<bool>,
    /// Property [`FindingPublishingFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html#cfn-guardduty-detector-findingpublishingfrequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub finding_publishing_frequency: Option<::Value<String>>,
}

impl ::serde::Serialize for DetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_sources) = self.data_sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSources", data_sources)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", &self.enable)?;
        if let Some(ref finding_publishing_frequency) = self.finding_publishing_frequency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingPublishingFrequency", finding_publishing_frequency)?;
        }
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
                let mut data_sources: Option<::Value<self::detector::CFNDataSourceConfigurations>> = None;
                let mut enable: Option<::Value<bool>> = None;
                let mut finding_publishing_frequency: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataSources" => {
                            data_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enable" => {
                            enable = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FindingPublishingFrequency" => {
                            finding_publishing_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DetectorProperties {
                    data_sources: data_sources,
                    enable: enable.ok_or(::serde::de::Error::missing_field("Enable"))?,
                    finding_publishing_frequency: finding_publishing_frequency,
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

/// The [`AWS::GuardDuty::Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html) resource type.
#[derive(Debug, Default)]
pub struct Filter {
    properties: FilterProperties
}

/// Properties for the `Filter` resource.
#[derive(Debug, Default)]
pub struct FilterProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-action).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`FindingCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-findingcriteria).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub finding_criteria: ::Value<self::filter::FindingCriteria>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Rank`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html#cfn-guardduty-filter-rank).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rank: ::Value<u32>,
}

impl ::serde::Serialize for FilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingCriteria", &self.finding_criteria)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rank", &self.rank)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut detector_id: Option<::Value<String>> = None;
                let mut finding_criteria: Option<::Value<self::filter::FindingCriteria>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rank: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FindingCriteria" => {
                            finding_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rank" => {
                            rank = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FilterProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    finding_criteria: finding_criteria.ok_or(::serde::de::Error::missing_field("FindingCriteria"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rank: rank.ok_or(::serde::de::Error::missing_field("Rank"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Filter {
    type Properties = FilterProperties;
    const TYPE: &'static str = "AWS::GuardDuty::Filter";
    fn properties(&self) -> &FilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Filter {}

impl From<FilterProperties> for Filter {
    fn from(properties: FilterProperties) -> Filter {
        Filter { properties }
    }
}

/// The [`AWS::GuardDuty::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html) resource type.
#[derive(Debug, Default)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
pub struct Master {
    properties: MasterProperties
}

/// Properties for the `Master` resource.
#[derive(Debug, Default)]
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
    pub invitation_id: Option<::Value<String>>,
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
        if let Some(ref invitation_id) = self.invitation_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvitationId", invitation_id)?;
        }
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
                    invitation_id: invitation_id,
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
#[derive(Debug, Default)]
pub struct Member {
    properties: MemberProperties
}

/// Properties for the `Member` resource.
#[derive(Debug, Default)]
pub struct MemberProperties {
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`DisableEmailNotification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html#cfn-guardduty-member-disableemailnotification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disable_email_notification: Option<::Value<bool>>,
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
        if let Some(ref disable_email_notification) = self.disable_email_notification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableEmailNotification", disable_email_notification)?;
        }
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
                let mut disable_email_notification: Option<::Value<bool>> = None;
                let mut email: Option<::Value<String>> = None;
                let mut member_id: Option<::Value<String>> = None;
                let mut message: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisableEmailNotification" => {
                            disable_email_notification = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    disable_email_notification: disable_email_notification,
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
#[derive(Debug, Default)]
pub struct ThreatIntelSet {
    properties: ThreatIntelSetProperties
}

/// Properties for the `ThreatIntelSet` resource.
#[derive(Debug, Default)]
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

pub mod detector {
    //! Property types for the `Detector` resource.

    /// The [`AWS::GuardDuty::Detector.CFNDataSourceConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfndatasourceconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct CFNDataSourceConfigurations {
        /// Property [`Kubernetes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfndatasourceconfigurations.html#cfn-guardduty-detector-cfndatasourceconfigurations-kubernetes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kubernetes: Option<::Value<CFNKubernetesConfiguration>>,
        /// Property [`S3Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfndatasourceconfigurations.html#cfn-guardduty-detector-cfndatasourceconfigurations-s3logs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_logs: Option<::Value<CFNS3LogsConfiguration>>,
    }

    impl ::codec::SerializeValue for CFNDataSourceConfigurations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kubernetes) = self.kubernetes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Kubernetes", kubernetes)?;
            }
            if let Some(ref s3_logs) = self.s3_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Logs", s3_logs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CFNDataSourceConfigurations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CFNDataSourceConfigurations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CFNDataSourceConfigurations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CFNDataSourceConfigurations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kubernetes: Option<::Value<CFNKubernetesConfiguration>> = None;
                    let mut s3_logs: Option<::Value<CFNS3LogsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Kubernetes" => {
                                kubernetes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Logs" => {
                                s3_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CFNDataSourceConfigurations {
                        kubernetes: kubernetes,
                        s3_logs: s3_logs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GuardDuty::Detector.CFNKubernetesAuditLogsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesauditlogsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CFNKubernetesAuditLogsConfiguration {
        /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesauditlogsconfiguration.html#cfn-guardduty-detector-cfnkubernetesauditlogsconfiguration-enable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CFNKubernetesAuditLogsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable) = self.enable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", enable)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CFNKubernetesAuditLogsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CFNKubernetesAuditLogsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CFNKubernetesAuditLogsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CFNKubernetesAuditLogsConfiguration")
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

                    Ok(CFNKubernetesAuditLogsConfiguration {
                        enable: enable,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GuardDuty::Detector.CFNKubernetesConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CFNKubernetesConfiguration {
        /// Property [`AuditLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesconfiguration.html#cfn-guardduty-detector-cfnkubernetesconfiguration-auditlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audit_logs: Option<::Value<CFNKubernetesAuditLogsConfiguration>>,
    }

    impl ::codec::SerializeValue for CFNKubernetesConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audit_logs) = self.audit_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuditLogs", audit_logs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CFNKubernetesConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CFNKubernetesConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CFNKubernetesConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CFNKubernetesConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audit_logs: Option<::Value<CFNKubernetesAuditLogsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuditLogs" => {
                                audit_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CFNKubernetesConfiguration {
                        audit_logs: audit_logs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GuardDuty::Detector.CFNS3LogsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfns3logsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CFNS3LogsConfiguration {
        /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfns3logsconfiguration.html#cfn-guardduty-detector-cfns3logsconfiguration-enable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CFNS3LogsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable) = self.enable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", enable)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CFNS3LogsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CFNS3LogsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CFNS3LogsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CFNS3LogsConfiguration")
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

                    Ok(CFNS3LogsConfiguration {
                        enable: enable,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod filter {
    //! Property types for the `Filter` resource.

    /// The [`AWS::GuardDuty::Filter.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html) property type.
    #[derive(Debug, Default)]
    pub struct Condition {
        /// Property [`Eq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html#cfn-guardduty-filter-condition-eq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub eq: Option<::ValueList<String>>,
        /// Property [`Gte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html#cfn-guardduty-filter-condition-gte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gte: Option<::Value<u32>>,
        /// Property [`Lt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html#cfn-guardduty-filter-condition-lt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lt: Option<::Value<u32>>,
        /// Property [`Lte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html#cfn-guardduty-filter-condition-lte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lte: Option<::Value<u32>>,
        /// Property [`Neq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html#cfn-guardduty-filter-condition-neq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub neq: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Condition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref eq) = self.eq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Eq", eq)?;
            }
            if let Some(ref gte) = self.gte {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gte", gte)?;
            }
            if let Some(ref lt) = self.lt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lt", lt)?;
            }
            if let Some(ref lte) = self.lte {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lte", lte)?;
            }
            if let Some(ref neq) = self.neq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Neq", neq)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Condition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Condition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Condition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Condition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut eq: Option<::ValueList<String>> = None;
                    let mut gte: Option<::Value<u32>> = None;
                    let mut lt: Option<::Value<u32>> = None;
                    let mut lte: Option<::Value<u32>> = None;
                    let mut neq: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Eq" => {
                                eq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Gte" => {
                                gte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lt" => {
                                lt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lte" => {
                                lte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Neq" => {
                                neq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Condition {
                        eq: eq,
                        gte: gte,
                        lt: lt,
                        lte: lte,
                        neq: neq,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GuardDuty::Filter.FindingCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-findingcriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct FindingCriteria {
        /// Property [`Criterion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-findingcriteria.html#cfn-guardduty-filter-findingcriteria-criterion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub criterion: Option<::Value<::json::Value>>,
        /// Property [`ItemType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-findingcriteria.html#cfn-guardduty-filter-findingcriteria-itemtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub item_type: Option<::Value<Condition>>,
    }

    impl ::codec::SerializeValue for FindingCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref criterion) = self.criterion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criterion", criterion)?;
            }
            if let Some(ref item_type) = self.item_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ItemType", item_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FindingCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FindingCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FindingCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FindingCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut criterion: Option<::Value<::json::Value>> = None;
                    let mut item_type: Option<::Value<Condition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Criterion" => {
                                criterion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ItemType" => {
                                item_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FindingCriteria {
                        criterion: criterion,
                        item_type: item_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
