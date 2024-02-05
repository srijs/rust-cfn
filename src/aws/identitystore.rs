//! Types for the `IdentityStore` service.

/// The [`AWS::IdentityStore::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-group.html) resource type.
#[derive(Debug, Default)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug, Default)]
pub struct GroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-group.html#cfn-identitystore-group-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-group.html#cfn-identitystore-group-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: ::Value<String>,
    /// Property [`IdentityStoreId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-group.html#cfn-identitystore-group-identitystoreid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_store_id: ::Value<String>,
}

impl ::serde::Serialize for GroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityStoreId", &self.identity_store_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut identity_store_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityStoreId" => {
                            identity_store_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupProperties {
                    description: description,
                    display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                    identity_store_id: identity_store_id.ok_or(::serde::de::Error::missing_field("IdentityStoreId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Group {
    type Properties = GroupProperties;
    const TYPE: &'static str = "AWS::IdentityStore::Group";
    fn properties(&self) -> &GroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Group {}

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::IdentityStore::GroupMembership`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-groupmembership.html) resource type.
#[derive(Debug, Default)]
pub struct GroupMembership {
    properties: GroupMembershipProperties
}

/// Properties for the `GroupMembership` resource.
#[derive(Debug, Default)]
pub struct GroupMembershipProperties {
    /// Property [`GroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-groupmembership.html#cfn-identitystore-groupmembership-groupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_id: ::Value<String>,
    /// Property [`IdentityStoreId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-groupmembership.html#cfn-identitystore-groupmembership-identitystoreid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_store_id: ::Value<String>,
    /// Property [`MemberId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-groupmembership.html#cfn-identitystore-groupmembership-memberid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub member_id: ::Value<self::group_membership::MemberId>,
}

impl ::serde::Serialize for GroupMembershipProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityStoreId", &self.identity_store_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberId", &self.member_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupMembershipProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupMembershipProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupMembershipProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupMembershipProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_id: Option<::Value<String>> = None;
                let mut identity_store_id: Option<::Value<String>> = None;
                let mut member_id: Option<::Value<self::group_membership::MemberId>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupId" => {
                            group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityStoreId" => {
                            identity_store_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberId" => {
                            member_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupMembershipProperties {
                    group_id: group_id.ok_or(::serde::de::Error::missing_field("GroupId"))?,
                    identity_store_id: identity_store_id.ok_or(::serde::de::Error::missing_field("IdentityStoreId"))?,
                    member_id: member_id.ok_or(::serde::de::Error::missing_field("MemberId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GroupMembership {
    type Properties = GroupMembershipProperties;
    const TYPE: &'static str = "AWS::IdentityStore::GroupMembership";
    fn properties(&self) -> &GroupMembershipProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupMembershipProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GroupMembership {}

impl From<GroupMembershipProperties> for GroupMembership {
    fn from(properties: GroupMembershipProperties) -> GroupMembership {
        GroupMembership { properties }
    }
}

pub mod group_membership {
    //! Property types for the `GroupMembership` resource.

    /// The [`AWS::IdentityStore::GroupMembership.MemberId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-identitystore-groupmembership-memberid.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberId {
        /// Property [`UserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-identitystore-groupmembership-memberid.html#cfn-identitystore-groupmembership-memberid-userid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub user_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for MemberId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserId", &self.user_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut user_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UserId" => {
                                user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberId {
                        user_id: user_id.ok_or(::serde::de::Error::missing_field("UserId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
