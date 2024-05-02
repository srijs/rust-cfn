//! Types for the `Shield` service.

/// The [`AWS::Shield::DRTAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html) resource type.
#[derive(Debug, Default)]
pub struct DRTAccess {
    properties: DRTAccessProperties
}

/// Properties for the `DRTAccess` resource.
#[derive(Debug, Default)]
pub struct DRTAccessProperties {
    /// Property [`LogBucketList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-logbucketlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_bucket_list: Option<::ValueList<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html#cfn-shield-drtaccess-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for DRTAccessProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref log_bucket_list) = self.log_bucket_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogBucketList", log_bucket_list)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DRTAccessProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DRTAccessProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DRTAccessProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DRTAccessProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut log_bucket_list: Option<::ValueList<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LogBucketList" => {
                            log_bucket_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DRTAccessProperties {
                    log_bucket_list: log_bucket_list,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DRTAccess {
    type Properties = DRTAccessProperties;
    const TYPE: &'static str = "AWS::Shield::DRTAccess";
    fn properties(&self) -> &DRTAccessProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DRTAccessProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DRTAccess {}

impl From<DRTAccessProperties> for DRTAccess {
    fn from(properties: DRTAccessProperties) -> DRTAccess {
        DRTAccess { properties }
    }
}

/// The [`AWS::Shield::ProactiveEngagement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html) resource type.
#[derive(Debug, Default)]
pub struct ProactiveEngagement {
    properties: ProactiveEngagementProperties
}

/// Properties for the `ProactiveEngagement` resource.
#[derive(Debug, Default)]
pub struct ProactiveEngagementProperties {
    /// Property [`EmergencyContactList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-emergencycontactlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub emergency_contact_list: ::ValueList<self::proactive_engagement::EmergencyContact>,
    /// Property [`ProactiveEngagementStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-proactiveengagementstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub proactive_engagement_status: ::Value<String>,
}

impl ::serde::Serialize for ProactiveEngagementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmergencyContactList", &self.emergency_contact_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProactiveEngagementStatus", &self.proactive_engagement_status)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProactiveEngagementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProactiveEngagementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProactiveEngagementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProactiveEngagementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut emergency_contact_list: Option<::ValueList<self::proactive_engagement::EmergencyContact>> = None;
                let mut proactive_engagement_status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EmergencyContactList" => {
                            emergency_contact_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProactiveEngagementStatus" => {
                            proactive_engagement_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProactiveEngagementProperties {
                    emergency_contact_list: emergency_contact_list.ok_or(::serde::de::Error::missing_field("EmergencyContactList"))?,
                    proactive_engagement_status: proactive_engagement_status.ok_or(::serde::de::Error::missing_field("ProactiveEngagementStatus"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ProactiveEngagement {
    type Properties = ProactiveEngagementProperties;
    const TYPE: &'static str = "AWS::Shield::ProactiveEngagement";
    fn properties(&self) -> &ProactiveEngagementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProactiveEngagementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ProactiveEngagement {}

impl From<ProactiveEngagementProperties> for ProactiveEngagement {
    fn from(properties: ProactiveEngagementProperties) -> ProactiveEngagement {
        ProactiveEngagement { properties }
    }
}

/// The [`AWS::Shield::Protection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html) resource type.
#[derive(Debug, Default)]
pub struct Protection {
    properties: ProtectionProperties
}

/// Properties for the `Protection` resource.
#[derive(Debug, Default)]
pub struct ProtectionProperties {
    /// Property [`ApplicationLayerAutomaticResponseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_layer_automatic_response_configuration: Option<::Value<self::protection::ApplicationLayerAutomaticResponseConfiguration>>,
    /// Property [`HealthCheckArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-healthcheckarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_arns: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProtectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref application_layer_automatic_response_configuration) = self.application_layer_automatic_response_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationLayerAutomaticResponseConfiguration", application_layer_automatic_response_configuration)?;
        }
        if let Some(ref health_check_arns) = self.health_check_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckArns", health_check_arns)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProtectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProtectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProtectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_layer_automatic_response_configuration: Option<::Value<self::protection::ApplicationLayerAutomaticResponseConfiguration>> = None;
                let mut health_check_arns: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut resource_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationLayerAutomaticResponseConfiguration" => {
                            application_layer_automatic_response_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckArns" => {
                            health_check_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProtectionProperties {
                    application_layer_automatic_response_configuration: application_layer_automatic_response_configuration,
                    health_check_arns: health_check_arns,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Protection {
    type Properties = ProtectionProperties;
    const TYPE: &'static str = "AWS::Shield::Protection";
    fn properties(&self) -> &ProtectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProtectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Protection {}

impl From<ProtectionProperties> for Protection {
    fn from(properties: ProtectionProperties) -> Protection {
        Protection { properties }
    }
}

/// The [`AWS::Shield::ProtectionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html) resource type.
#[derive(Debug, Default)]
pub struct ProtectionGroup {
    properties: ProtectionGroupProperties
}

/// Properties for the `ProtectionGroup` resource.
#[derive(Debug, Default)]
pub struct ProtectionGroupProperties {
    /// Property [`Aggregation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-aggregation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aggregation: ::Value<String>,
    /// Property [`Members`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-members).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub members: Option<::ValueList<String>>,
    /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-pattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pattern: ::Value<String>,
    /// Property [`ProtectionGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-protectiongroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protection_group_id: ::Value<String>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-resourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html#cfn-shield-protectiongroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProtectionGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aggregation", &self.aggregation)?;
        if let Some(ref members) = self.members {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Members", members)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", &self.pattern)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtectionGroupId", &self.protection_group_id)?;
        if let Some(ref resource_type) = self.resource_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", resource_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProtectionGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtectionGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProtectionGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProtectionGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aggregation: Option<::Value<String>> = None;
                let mut members: Option<::ValueList<String>> = None;
                let mut pattern: Option<::Value<String>> = None;
                let mut protection_group_id: Option<::Value<String>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Aggregation" => {
                            aggregation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Members" => {
                            members = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Pattern" => {
                            pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtectionGroupId" => {
                            protection_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProtectionGroupProperties {
                    aggregation: aggregation.ok_or(::serde::de::Error::missing_field("Aggregation"))?,
                    members: members,
                    pattern: pattern.ok_or(::serde::de::Error::missing_field("Pattern"))?,
                    protection_group_id: protection_group_id.ok_or(::serde::de::Error::missing_field("ProtectionGroupId"))?,
                    resource_type: resource_type,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ProtectionGroup {
    type Properties = ProtectionGroupProperties;
    const TYPE: &'static str = "AWS::Shield::ProtectionGroup";
    fn properties(&self) -> &ProtectionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProtectionGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ProtectionGroup {}

impl From<ProtectionGroupProperties> for ProtectionGroup {
    fn from(properties: ProtectionGroupProperties) -> ProtectionGroup {
        ProtectionGroup { properties }
    }
}

pub mod proactive_engagement {
    //! Property types for the `ProactiveEngagement` resource.

    /// The [`AWS::Shield::ProactiveEngagement.EmergencyContact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html) property type.
    #[derive(Debug, Default)]
    pub struct EmergencyContact {
        /// Property [`ContactNotes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-contactnotes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_notes: Option<::Value<String>>,
        /// Property [`EmailAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-emailaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_address: ::Value<String>,
        /// Property [`PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-phonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EmergencyContact {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contact_notes) = self.contact_notes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactNotes", contact_notes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailAddress", &self.email_address)?;
            if let Some(ref phone_number) = self.phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumber", phone_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EmergencyContact {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EmergencyContact, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EmergencyContact;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EmergencyContact")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_notes: Option<::Value<String>> = None;
                    let mut email_address: Option<::Value<String>> = None;
                    let mut phone_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactNotes" => {
                                contact_notes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailAddress" => {
                                email_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneNumber" => {
                                phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EmergencyContact {
                        contact_notes: contact_notes,
                        email_address: email_address.ok_or(::serde::de::Error::missing_field("EmailAddress"))?,
                        phone_number: phone_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod protection {
    //! Property types for the `Protection` resource.

    /// The [`AWS::Shield::Protection.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`Block`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-block).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block: Option<::Value<::json::Value>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block) = self.block {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Block", block)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block: Option<::Value<::json::Value>> = None;
                    let mut count: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Block" => {
                                block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        block: block,
                        count: count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Shield::Protection.ApplicationLayerAutomaticResponseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationLayerAutomaticResponseConfiguration {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<Action>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for ApplicationLayerAutomaticResponseConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationLayerAutomaticResponseConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationLayerAutomaticResponseConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationLayerAutomaticResponseConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationLayerAutomaticResponseConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<Action>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationLayerAutomaticResponseConfiguration {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
