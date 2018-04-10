//! Types for the `DAX` service.

/// The [`AWS::DAX::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug)]
pub struct ClusterProperties {
    /// Property `AvailabilityZones`.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `ClusterName`.
    pub cluster_name: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `IAMRoleARN`.
    pub iam_role_arn: ::Value<String>,
    /// Property `NodeType`.
    pub node_type: ::Value<String>,
    /// Property `NotificationTopicARN`.
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property `ParameterGroupName`.
    pub parameter_group_name: Option<::Value<String>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `ReplicationFactor`.
    pub replication_factor: ::Value<u32>,
    /// Property `SecurityGroupIds`.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SubnetGroupName`.
    pub subnet_group_name: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref cluster_name) = self.cluster_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", cluster_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMRoleARN", &self.iam_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeType", &self.node_type)?;
        if let Some(ref notification_topic_arn) = self.notification_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicARN", notification_topic_arn)?;
        }
        if let Some(ref parameter_group_name) = self.parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupName", parameter_group_name)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationFactor", &self.replication_factor)?;
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_group_name) = self.subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetGroupName", subnet_group_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zones = None;
                let mut cluster_name = None;
                let mut description = None;
                let mut iam_role_arn = None;
                let mut node_type = None;
                let mut notification_topic_arn = None;
                let mut parameter_group_name = None;
                let mut preferred_maintenance_window = None;
                let mut replication_factor = None;
                let mut security_group_ids = None;
                let mut subnet_group_name = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IAMRoleARN" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeType" => {
                            node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTopicARN" => {
                            notification_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterGroupName" => {
                            parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationFactor" => {
                            replication_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetGroupName" => {
                            subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    availability_zones: availability_zones,
                    cluster_name: cluster_name,
                    description: description,
                    iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IAMRoleARN"))?,
                    node_type: node_type.ok_or(::serde::de::Error::missing_field("NodeType"))?,
                    notification_topic_arn: notification_topic_arn,
                    parameter_group_name: parameter_group_name,
                    preferred_maintenance_window: preferred_maintenance_window,
                    replication_factor: replication_factor.ok_or(::serde::de::Error::missing_field("ReplicationFactor"))?,
                    security_group_ids: security_group_ids,
                    subnet_group_name: subnet_group_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::DAX::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::DAX::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-parametergroup.html) resource type.
#[derive(Debug)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Debug)]
pub struct ParameterGroupProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `ParameterGroupName`.
    pub parameter_group_name: Option<::Value<String>>,
    /// Property `ParameterNameValues`.
    pub parameter_name_values: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref parameter_group_name) = self.parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupName", parameter_group_name)?;
        }
        if let Some(ref parameter_name_values) = self.parameter_name_values {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterNameValues", parameter_name_values)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut parameter_group_name = None;
                let mut parameter_name_values = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterGroupName" => {
                            parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterNameValues" => {
                            parameter_name_values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ParameterGroupProperties {
                    description: description,
                    parameter_group_name: parameter_group_name,
                    parameter_name_values: parameter_name_values,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ParameterGroup {
    type Properties = ParameterGroupProperties;
    const TYPE: &'static str = "AWS::DAX::ParameterGroup";
    fn properties(&self) -> &ParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ParameterGroup {}

impl From<ParameterGroupProperties> for ParameterGroup {
    fn from(properties: ParameterGroupProperties) -> ParameterGroup {
        ParameterGroup { properties }
    }
}

/// The [`AWS::DAX::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-subnetgroup.html) resource type.
#[derive(Debug)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Debug)]
pub struct SubnetGroupProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `SubnetGroupName`.
    pub subnet_group_name: Option<::Value<String>>,
    /// Property `SubnetIds`.
    pub subnet_ids: ::ValueList<String>,
}

impl ::serde::Serialize for SubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref subnet_group_name) = self.subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetGroupName", subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut subnet_group_name = None;
                let mut subnet_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetGroupName" => {
                            subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubnetGroupProperties {
                    description: description,
                    subnet_group_name: subnet_group_name,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubnetGroup {
    type Properties = SubnetGroupProperties;
    const TYPE: &'static str = "AWS::DAX::SubnetGroup";
    fn properties(&self) -> &SubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetGroup {}

impl From<SubnetGroupProperties> for SubnetGroup {
    fn from(properties: SubnetGroupProperties) -> SubnetGroup {
        SubnetGroup { properties }
    }
}
