//! Types for the `Detective` service.

/// The [`AWS::Detective::Graph`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-graph.html) resource type.
#[derive(Debug, Default)]
pub struct Graph {
    properties: GraphProperties
}

/// Properties for the `Graph` resource.
#[derive(Debug, Default)]
pub struct GraphProperties {
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-graph.html#cfn-detective-graph-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for GraphProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GraphProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GraphProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GraphProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GraphProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GraphProperties {
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Graph {
    type Properties = GraphProperties;
    const TYPE: &'static str = "AWS::Detective::Graph";
    fn properties(&self) -> &GraphProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GraphProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Graph {}

impl From<GraphProperties> for Graph {
    fn from(properties: GraphProperties) -> Graph {
        Graph { properties }
    }
}

/// The [`AWS::Detective::MemberInvitation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html) resource type.
#[derive(Debug, Default)]
pub struct MemberInvitation {
    properties: MemberInvitationProperties
}

/// Properties for the `MemberInvitation` resource.
#[derive(Debug, Default)]
pub struct MemberInvitationProperties {
    /// Property [`DisableEmailNotification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html#cfn-detective-memberinvitation-disableemailnotification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disable_email_notification: Option<::Value<bool>>,
    /// Property [`GraphArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html#cfn-detective-memberinvitation-grapharn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub graph_arn: ::Value<String>,
    /// Property [`MemberEmailAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html#cfn-detective-memberinvitation-memberemailaddress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub member_email_address: ::Value<String>,
    /// Property [`MemberId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html#cfn-detective-memberinvitation-memberid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub member_id: ::Value<String>,
    /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html#cfn-detective-memberinvitation-message).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub message: Option<::Value<String>>,
}

impl ::serde::Serialize for MemberInvitationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref disable_email_notification) = self.disable_email_notification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableEmailNotification", disable_email_notification)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GraphArn", &self.graph_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberEmailAddress", &self.member_email_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberId", &self.member_id)?;
        if let Some(ref message) = self.message {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MemberInvitationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberInvitationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MemberInvitationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MemberInvitationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut disable_email_notification: Option<::Value<bool>> = None;
                let mut graph_arn: Option<::Value<String>> = None;
                let mut member_email_address: Option<::Value<String>> = None;
                let mut member_id: Option<::Value<String>> = None;
                let mut message: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DisableEmailNotification" => {
                            disable_email_notification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GraphArn" => {
                            graph_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberEmailAddress" => {
                            member_email_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberId" => {
                            member_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Message" => {
                            message = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MemberInvitationProperties {
                    disable_email_notification: disable_email_notification,
                    graph_arn: graph_arn.ok_or(::serde::de::Error::missing_field("GraphArn"))?,
                    member_email_address: member_email_address.ok_or(::serde::de::Error::missing_field("MemberEmailAddress"))?,
                    member_id: member_id.ok_or(::serde::de::Error::missing_field("MemberId"))?,
                    message: message,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MemberInvitation {
    type Properties = MemberInvitationProperties;
    const TYPE: &'static str = "AWS::Detective::MemberInvitation";
    fn properties(&self) -> &MemberInvitationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MemberInvitationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MemberInvitation {}

impl From<MemberInvitationProperties> for MemberInvitation {
    fn from(properties: MemberInvitationProperties) -> MemberInvitation {
        MemberInvitation { properties }
    }
}
