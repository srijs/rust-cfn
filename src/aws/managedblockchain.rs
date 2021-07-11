//! Types for the `ManagedBlockchain` service.

/// The [`AWS::ManagedBlockchain::Member`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html) resource type.
#[derive(Debug, Default)]
pub struct Member {
    properties: MemberProperties
}

/// Properties for the `Member` resource.
#[derive(Debug, Default)]
pub struct MemberProperties {
    /// Property [`InvitationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html#cfn-managedblockchain-member-invitationid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub invitation_id: Option<::Value<String>>,
    /// Property [`MemberConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html#cfn-managedblockchain-member-memberconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub member_configuration: ::Value<self::member::MemberConfiguration>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html#cfn-managedblockchain-member-networkconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_configuration: Option<::Value<self::member::NetworkConfiguration>>,
    /// Property [`NetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html#cfn-managedblockchain-member-networkid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_id: Option<::Value<String>>,
}

impl ::serde::Serialize for MemberProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref invitation_id) = self.invitation_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvitationId", invitation_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberConfiguration", &self.member_configuration)?;
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        if let Some(ref network_id) = self.network_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkId", network_id)?;
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
                let mut invitation_id: Option<::Value<String>> = None;
                let mut member_configuration: Option<::Value<self::member::MemberConfiguration>> = None;
                let mut network_configuration: Option<::Value<self::member::NetworkConfiguration>> = None;
                let mut network_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InvitationId" => {
                            invitation_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberConfiguration" => {
                            member_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkId" => {
                            network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MemberProperties {
                    invitation_id: invitation_id,
                    member_configuration: member_configuration.ok_or(::serde::de::Error::missing_field("MemberConfiguration"))?,
                    network_configuration: network_configuration,
                    network_id: network_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Member {
    type Properties = MemberProperties;
    const TYPE: &'static str = "AWS::ManagedBlockchain::Member";
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

/// The [`AWS::ManagedBlockchain::Node`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-node.html) resource type.
#[derive(Debug, Default)]
pub struct Node {
    properties: NodeProperties
}

/// Properties for the `Node` resource.
#[derive(Debug, Default)]
pub struct NodeProperties {
    /// Property [`MemberId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-node.html#cfn-managedblockchain-node-memberid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub member_id: Option<::Value<String>>,
    /// Property [`NetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-node.html#cfn-managedblockchain-node-networkid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_id: ::Value<String>,
    /// Property [`NodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-node.html#cfn-managedblockchain-node-nodeconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_configuration: ::Value<self::node::NodeConfiguration>,
}

impl ::serde::Serialize for NodeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref member_id) = self.member_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberId", member_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkId", &self.network_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeConfiguration", &self.node_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NodeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NodeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut member_id: Option<::Value<String>> = None;
                let mut network_id: Option<::Value<String>> = None;
                let mut node_configuration: Option<::Value<self::node::NodeConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MemberId" => {
                            member_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkId" => {
                            network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeConfiguration" => {
                            node_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NodeProperties {
                    member_id: member_id,
                    network_id: network_id.ok_or(::serde::de::Error::missing_field("NetworkId"))?,
                    node_configuration: node_configuration.ok_or(::serde::de::Error::missing_field("NodeConfiguration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Node {
    type Properties = NodeProperties;
    const TYPE: &'static str = "AWS::ManagedBlockchain::Node";
    fn properties(&self) -> &NodeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NodeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Node {}

impl From<NodeProperties> for Node {
    fn from(properties: NodeProperties) -> Node {
        Node { properties }
    }
}

pub mod member {
    //! Property types for the `Member` resource.

    /// The [`AWS::ManagedBlockchain::Member.ApprovalThresholdPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-approvalthresholdpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ApprovalThresholdPolicy {
        /// Property [`ProposalDurationInHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-approvalthresholdpolicy.html#cfn-managedblockchain-member-approvalthresholdpolicy-proposaldurationinhours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub proposal_duration_in_hours: Option<::Value<u32>>,
        /// Property [`ThresholdComparator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-approvalthresholdpolicy.html#cfn-managedblockchain-member-approvalthresholdpolicy-thresholdcomparator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold_comparator: Option<::Value<String>>,
        /// Property [`ThresholdPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-approvalthresholdpolicy.html#cfn-managedblockchain-member-approvalthresholdpolicy-thresholdpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold_percentage: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ApprovalThresholdPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref proposal_duration_in_hours) = self.proposal_duration_in_hours {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProposalDurationInHours", proposal_duration_in_hours)?;
            }
            if let Some(ref threshold_comparator) = self.threshold_comparator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdComparator", threshold_comparator)?;
            }
            if let Some(ref threshold_percentage) = self.threshold_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdPercentage", threshold_percentage)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApprovalThresholdPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApprovalThresholdPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApprovalThresholdPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApprovalThresholdPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut proposal_duration_in_hours: Option<::Value<u32>> = None;
                    let mut threshold_comparator: Option<::Value<String>> = None;
                    let mut threshold_percentage: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProposalDurationInHours" => {
                                proposal_duration_in_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThresholdComparator" => {
                                threshold_comparator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThresholdPercentage" => {
                                threshold_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApprovalThresholdPolicy {
                        proposal_duration_in_hours: proposal_duration_in_hours,
                        threshold_comparator: threshold_comparator,
                        threshold_percentage: threshold_percentage,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.MemberConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberConfiguration {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberconfiguration.html#cfn-managedblockchain-member-memberconfiguration-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`MemberFrameworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberconfiguration.html#cfn-managedblockchain-member-memberconfiguration-memberframeworkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub member_framework_configuration: Option<::Value<MemberFrameworkConfiguration>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberconfiguration.html#cfn-managedblockchain-member-memberconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for MemberConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref member_framework_configuration) = self.member_framework_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberFrameworkConfiguration", member_framework_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut member_framework_configuration: Option<::Value<MemberFrameworkConfiguration>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemberFrameworkConfiguration" => {
                                member_framework_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberConfiguration {
                        description: description,
                        member_framework_configuration: member_framework_configuration,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.MemberFabricConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberfabricconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberFabricConfiguration {
        /// Property [`AdminPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberfabricconfiguration.html#cfn-managedblockchain-member-memberfabricconfiguration-adminpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub admin_password: ::Value<String>,
        /// Property [`AdminUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberfabricconfiguration.html#cfn-managedblockchain-member-memberfabricconfiguration-adminusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub admin_username: ::Value<String>,
    }

    impl ::codec::SerializeValue for MemberFabricConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminPassword", &self.admin_password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUsername", &self.admin_username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberFabricConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberFabricConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberFabricConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberFabricConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut admin_password: Option<::Value<String>> = None;
                    let mut admin_username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdminPassword" => {
                                admin_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdminUsername" => {
                                admin_username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberFabricConfiguration {
                        admin_password: admin_password.ok_or(::serde::de::Error::missing_field("AdminPassword"))?,
                        admin_username: admin_username.ok_or(::serde::de::Error::missing_field("AdminUsername"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.MemberFrameworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberframeworkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberFrameworkConfiguration {
        /// Property [`MemberFabricConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberframeworkconfiguration.html#cfn-managedblockchain-member-memberframeworkconfiguration-memberfabricconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub member_fabric_configuration: Option<::Value<MemberFabricConfiguration>>,
    }

    impl ::codec::SerializeValue for MemberFrameworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref member_fabric_configuration) = self.member_fabric_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberFabricConfiguration", member_fabric_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberFrameworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberFrameworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberFrameworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberFrameworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut member_fabric_configuration: Option<::Value<MemberFabricConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MemberFabricConfiguration" => {
                                member_fabric_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberFrameworkConfiguration {
                        member_fabric_configuration: member_fabric_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Framework`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-framework).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework: ::Value<String>,
        /// Property [`FrameworkVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-frameworkversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework_version: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`NetworkFrameworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-networkframeworkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_framework_configuration: Option<::Value<NetworkFrameworkConfiguration>>,
        /// Property [`VotingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html#cfn-managedblockchain-member-networkconfiguration-votingpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub voting_policy: ::Value<VotingPolicy>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Framework", &self.framework)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkVersion", &self.framework_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref network_framework_configuration) = self.network_framework_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkFrameworkConfiguration", network_framework_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VotingPolicy", &self.voting_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut framework: Option<::Value<String>> = None;
                    let mut framework_version: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut network_framework_configuration: Option<::Value<NetworkFrameworkConfiguration>> = None;
                    let mut voting_policy: Option<::Value<VotingPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Framework" => {
                                framework = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameworkVersion" => {
                                framework_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkFrameworkConfiguration" => {
                                network_framework_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VotingPolicy" => {
                                voting_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        description: description,
                        framework: framework.ok_or(::serde::de::Error::missing_field("Framework"))?,
                        framework_version: framework_version.ok_or(::serde::de::Error::missing_field("FrameworkVersion"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        network_framework_configuration: network_framework_configuration,
                        voting_policy: voting_policy.ok_or(::serde::de::Error::missing_field("VotingPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.NetworkFabricConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkfabricconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkFabricConfiguration {
        /// Property [`Edition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkfabricconfiguration.html#cfn-managedblockchain-member-networkfabricconfiguration-edition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub edition: ::Value<String>,
    }

    impl ::codec::SerializeValue for NetworkFabricConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Edition", &self.edition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkFabricConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkFabricConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkFabricConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkFabricConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut edition: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Edition" => {
                                edition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkFabricConfiguration {
                        edition: edition.ok_or(::serde::de::Error::missing_field("Edition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.NetworkFrameworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkframeworkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkFrameworkConfiguration {
        /// Property [`NetworkFabricConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkframeworkconfiguration.html#cfn-managedblockchain-member-networkframeworkconfiguration-networkfabricconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_fabric_configuration: Option<::Value<NetworkFabricConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkFrameworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref network_fabric_configuration) = self.network_fabric_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkFabricConfiguration", network_fabric_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkFrameworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkFrameworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkFrameworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkFrameworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut network_fabric_configuration: Option<::Value<NetworkFabricConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetworkFabricConfiguration" => {
                                network_fabric_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkFrameworkConfiguration {
                        network_fabric_configuration: network_fabric_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ManagedBlockchain::Member.VotingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-votingpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct VotingPolicy {
        /// Property [`ApprovalThresholdPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-votingpolicy.html#cfn-managedblockchain-member-votingpolicy-approvalthresholdpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub approval_threshold_policy: Option<::Value<ApprovalThresholdPolicy>>,
    }

    impl ::codec::SerializeValue for VotingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref approval_threshold_policy) = self.approval_threshold_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovalThresholdPolicy", approval_threshold_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VotingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VotingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VotingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VotingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut approval_threshold_policy: Option<::Value<ApprovalThresholdPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApprovalThresholdPolicy" => {
                                approval_threshold_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VotingPolicy {
                        approval_threshold_policy: approval_threshold_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod node {
    //! Property types for the `Node` resource.

    /// The [`AWS::ManagedBlockchain::Node.NodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-node-nodeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeConfiguration {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-node-nodeconfiguration.html#cfn-managedblockchain-node-nodeconfiguration-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: ::Value<String>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-node-nodeconfiguration.html#cfn-managedblockchain-node-nodeconfiguration-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for NodeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NodeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut instance_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeConfiguration {
                        availability_zone: availability_zone.ok_or(::serde::de::Error::missing_field("AvailabilityZone"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
