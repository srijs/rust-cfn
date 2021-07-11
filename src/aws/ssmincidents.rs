//! Types for the `SSMIncidents` service.

/// The [`AWS::SSMIncidents::ReplicationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-replicationset.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationSet {
    properties: ReplicationSetProperties
}

/// Properties for the `ReplicationSet` resource.
#[derive(Debug, Default)]
pub struct ReplicationSetProperties {
    /// Property [`DeletionProtected`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-replicationset.html#cfn-ssmincidents-replicationset-deletionprotected).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protected: Option<::Value<bool>>,
    /// Property [`Regions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-replicationset.html#cfn-ssmincidents-replicationset-regions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regions: ::ValueList<self::replication_set::ReplicationRegion>,
}

impl ::serde::Serialize for ReplicationSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deletion_protected) = self.deletion_protected {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtected", deletion_protected)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regions", &self.regions)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deletion_protected: Option<::Value<bool>> = None;
                let mut regions: Option<::ValueList<self::replication_set::ReplicationRegion>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeletionProtected" => {
                            deletion_protected = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Regions" => {
                            regions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationSetProperties {
                    deletion_protected: deletion_protected,
                    regions: regions.ok_or(::serde::de::Error::missing_field("Regions"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationSet {
    type Properties = ReplicationSetProperties;
    const TYPE: &'static str = "AWS::SSMIncidents::ReplicationSet";
    fn properties(&self) -> &ReplicationSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationSet {}

impl From<ReplicationSetProperties> for ReplicationSet {
    fn from(properties: ReplicationSetProperties) -> ReplicationSet {
        ReplicationSet { properties }
    }
}

/// The [`AWS::SSMIncidents::ResponsePlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html) resource type.
#[derive(Debug, Default)]
pub struct ResponsePlan {
    properties: ResponsePlanProperties
}

/// Properties for the `ResponsePlan` resource.
#[derive(Debug, Default)]
pub struct ResponsePlanProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: Option<::ValueList<self::response_plan::Action>>,
    /// Property [`ChatChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-chatchannel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub chat_channel: Option<::Value<self::response_plan::ChatChannel>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`Engagements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-engagements).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engagements: Option<::ValueList<String>>,
    /// Property [`IncidentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-incidenttemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub incident_template: ::Value<self::response_plan::IncidentTemplate>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html#cfn-ssmincidents-responseplan-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ResponsePlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions) = self.actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
        }
        if let Some(ref chat_channel) = self.chat_channel {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChatChannel", chat_channel)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref engagements) = self.engagements {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engagements", engagements)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncidentTemplate", &self.incident_template)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResponsePlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponsePlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResponsePlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResponsePlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<self::response_plan::Action>> = None;
                let mut chat_channel: Option<::Value<self::response_plan::ChatChannel>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut engagements: Option<::ValueList<String>> = None;
                let mut incident_template: Option<::Value<self::response_plan::IncidentTemplate>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChatChannel" => {
                            chat_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engagements" => {
                            engagements = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncidentTemplate" => {
                            incident_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResponsePlanProperties {
                    actions: actions,
                    chat_channel: chat_channel,
                    display_name: display_name,
                    engagements: engagements,
                    incident_template: incident_template.ok_or(::serde::de::Error::missing_field("IncidentTemplate"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResponsePlan {
    type Properties = ResponsePlanProperties;
    const TYPE: &'static str = "AWS::SSMIncidents::ResponsePlan";
    fn properties(&self) -> &ResponsePlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResponsePlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResponsePlan {}

impl From<ResponsePlanProperties> for ResponsePlan {
    fn from(properties: ResponsePlanProperties) -> ResponsePlan {
        ResponsePlan { properties }
    }
}

pub mod replication_set {
    //! Property types for the `ReplicationSet` resource.

    /// The [`AWS::SSMIncidents::ReplicationSet.RegionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-regionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RegionConfiguration {
        /// Property [`SseKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-regionconfiguration.html#cfn-ssmincidents-replicationset-regionconfiguration-ssekmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_kms_key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for RegionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseKmsKeyId", &self.sse_kms_key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sse_kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SseKmsKeyId" => {
                                sse_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegionConfiguration {
                        sse_kms_key_id: sse_kms_key_id.ok_or(::serde::de::Error::missing_field("SseKmsKeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ReplicationSet.ReplicationRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-replicationregion.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationRegion {
        /// Property [`RegionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-replicationregion.html#cfn-ssmincidents-replicationset-replicationregion-regionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_configuration: Option<::Value<RegionConfiguration>>,
        /// Property [`RegionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-replicationregion.html#cfn-ssmincidents-replicationset-replicationregion-regionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReplicationRegion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref region_configuration) = self.region_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionConfiguration", region_configuration)?;
            }
            if let Some(ref region_name) = self.region_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionName", region_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationRegion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationRegion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationRegion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationRegion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut region_configuration: Option<::Value<RegionConfiguration>> = None;
                    let mut region_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RegionConfiguration" => {
                                region_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegionName" => {
                                region_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationRegion {
                        region_configuration: region_configuration,
                        region_name: region_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod response_plan {
    //! Property types for the `ResponsePlan` resource.

    /// The [`AWS::SSMIncidents::ResponsePlan.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`SsmAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-action.html#cfn-ssmincidents-responseplan-action-ssmautomation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssm_automation: Option<::Value<SsmAutomation>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ssm_automation) = self.ssm_automation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SsmAutomation", ssm_automation)?;
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
                    let mut ssm_automation: Option<::Value<SsmAutomation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SsmAutomation" => {
                                ssm_automation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        ssm_automation: ssm_automation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ResponsePlan.ChatChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-chatchannel.html) property type.
    #[derive(Debug, Default)]
    pub struct ChatChannel {
        /// Property [`ChatbotSns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-chatchannel.html#cfn-ssmincidents-responseplan-chatchannel-chatbotsns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub chatbot_sns: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ChatChannel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref chatbot_sns) = self.chatbot_sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChatbotSns", chatbot_sns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChatChannel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChatChannel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChatChannel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChatChannel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut chatbot_sns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChatbotSns" => {
                                chatbot_sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChatChannel {
                        chatbot_sns: chatbot_sns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ResponsePlan.IncidentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct IncidentTemplate {
        /// Property [`DedupeString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html#cfn-ssmincidents-responseplan-incidenttemplate-dedupestring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dedupe_string: Option<::Value<String>>,
        /// Property [`Impact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html#cfn-ssmincidents-responseplan-incidenttemplate-impact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub impact: ::Value<u32>,
        /// Property [`NotificationTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html#cfn-ssmincidents-responseplan-incidenttemplate-notificationtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_targets: Option<::ValueList<NotificationTargetItem>>,
        /// Property [`Summary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html#cfn-ssmincidents-responseplan-incidenttemplate-summary).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub summary: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html#cfn-ssmincidents-responseplan-incidenttemplate-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: ::Value<String>,
    }

    impl ::codec::SerializeValue for IncidentTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dedupe_string) = self.dedupe_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DedupeString", dedupe_string)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Impact", &self.impact)?;
            if let Some(ref notification_targets) = self.notification_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTargets", notification_targets)?;
            }
            if let Some(ref summary) = self.summary {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Summary", summary)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", &self.title)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IncidentTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IncidentTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IncidentTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IncidentTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dedupe_string: Option<::Value<String>> = None;
                    let mut impact: Option<::Value<u32>> = None;
                    let mut notification_targets: Option<::ValueList<NotificationTargetItem>> = None;
                    let mut summary: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DedupeString" => {
                                dedupe_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Impact" => {
                                impact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationTargets" => {
                                notification_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Summary" => {
                                summary = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IncidentTemplate {
                        dedupe_string: dedupe_string,
                        impact: impact.ok_or(::serde::de::Error::missing_field("Impact"))?,
                        notification_targets: notification_targets,
                        summary: summary,
                        title: title.ok_or(::serde::de::Error::missing_field("Title"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ResponsePlan.NotificationTargetItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-notificationtargetitem.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationTargetItem {
        /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-notificationtargetitem.html#cfn-ssmincidents-responseplan-notificationtargetitem-snstopicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NotificationTargetItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sns_topic_arn) = self.sns_topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", sns_topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationTargetItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationTargetItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationTargetItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationTargetItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sns_topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SnsTopicArn" => {
                                sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationTargetItem {
                        sns_topic_arn: sns_topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ResponsePlan.SsmAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html) property type.
    #[derive(Debug, Default)]
    pub struct SsmAutomation {
        /// Property [`DocumentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html#cfn-ssmincidents-responseplan-ssmautomation-documentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_name: ::Value<String>,
        /// Property [`DocumentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html#cfn-ssmincidents-responseplan-ssmautomation-documentversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_version: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html#cfn-ssmincidents-responseplan-ssmautomation-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueList<SsmParameter>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html#cfn-ssmincidents-responseplan-ssmautomation-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TargetAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html#cfn-ssmincidents-responseplan-ssmautomation-targetaccount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_account: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SsmAutomation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentName", &self.document_name)?;
            if let Some(ref document_version) = self.document_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentVersion", document_version)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref target_account) = self.target_account {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAccount", target_account)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SsmAutomation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SsmAutomation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SsmAutomation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SsmAutomation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_name: Option<::Value<String>> = None;
                    let mut document_version: Option<::Value<String>> = None;
                    let mut parameters: Option<::ValueList<SsmParameter>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut target_account: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentName" => {
                                document_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentVersion" => {
                                document_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetAccount" => {
                                target_account = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SsmAutomation {
                        document_name: document_name.ok_or(::serde::de::Error::missing_field("DocumentName"))?,
                        document_version: document_version,
                        parameters: parameters,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        target_account: target_account,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMIncidents::ResponsePlan.SsmParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct SsmParameter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmparameter.html#cfn-ssmincidents-responseplan-ssmparameter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmparameter.html#cfn-ssmincidents-responseplan-ssmparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for SsmParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SsmParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SsmParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SsmParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SsmParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SsmParameter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
