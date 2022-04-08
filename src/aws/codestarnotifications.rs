//! Types for the `CodeStarNotifications` service.

/// The [`AWS::CodeStarNotifications::NotificationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html) resource type.
#[derive(Debug, Default)]
pub struct NotificationRule {
    properties: NotificationRuleProperties
}

/// Properties for the `NotificationRule` resource.
#[derive(Debug, Default)]
pub struct NotificationRuleProperties {
    /// Property [`CreatedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-createdby).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub created_by: Option<::Value<String>>,
    /// Property [`DetailType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-detailtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub detail_type: ::Value<String>,
    /// Property [`EventTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-eventtypeid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_type_id: Option<::Value<String>>,
    /// Property [`EventTypeIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-eventtypeids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_type_ids: ::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-resource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource: ::Value<String>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TargetAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-targetaddress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_address: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: ::ValueList<self::notification_rule::Target>,
}

impl ::serde::Serialize for NotificationRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref created_by) = self.created_by {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedBy", created_by)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailType", &self.detail_type)?;
        if let Some(ref event_type_id) = self.event_type_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventTypeId", event_type_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventTypeIds", &self.event_type_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_address) = self.target_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAddress", target_address)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotificationRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotificationRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut created_by: Option<::Value<String>> = None;
                let mut detail_type: Option<::Value<String>> = None;
                let mut event_type_id: Option<::Value<String>> = None;
                let mut event_type_ids: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut resource: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut target_address: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::notification_rule::Target>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CreatedBy" => {
                            created_by = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetailType" => {
                            detail_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventTypeId" => {
                            event_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventTypeIds" => {
                            event_type_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetAddress" => {
                            target_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotificationRuleProperties {
                    created_by: created_by,
                    detail_type: detail_type.ok_or(::serde::de::Error::missing_field("DetailType"))?,
                    event_type_id: event_type_id,
                    event_type_ids: event_type_ids.ok_or(::serde::de::Error::missing_field("EventTypeIds"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                    status: status,
                    tags: tags,
                    target_address: target_address,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NotificationRule {
    type Properties = NotificationRuleProperties;
    const TYPE: &'static str = "AWS::CodeStarNotifications::NotificationRule";
    fn properties(&self) -> &NotificationRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotificationRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NotificationRule {}

impl From<NotificationRuleProperties> for NotificationRule {
    fn from(properties: NotificationRuleProperties) -> NotificationRule {
        NotificationRule { properties }
    }
}

pub mod notification_rule {
    //! Property types for the `NotificationRule` resource.

    /// The [`AWS::CodeStarNotifications::NotificationRule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`TargetAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html#cfn-codestarnotifications-notificationrule-target-targetaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_address: ::Value<String>,
        /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html#cfn-codestarnotifications-notificationrule-target-targettype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAddress", &self.target_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", &self.target_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_address: Option<::Value<String>> = None;
                    let mut target_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetAddress" => {
                                target_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetType" => {
                                target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        target_address: target_address.ok_or(::serde::de::Error::missing_field("TargetAddress"))?,
                        target_type: target_type.ok_or(::serde::de::Error::missing_field("TargetType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
