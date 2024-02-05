//! Types for the `NeptuneGraph` service.

/// The [`AWS::NeptuneGraph::Graph`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html) resource type.
#[derive(Debug, Default)]
pub struct Graph {
    properties: GraphProperties
}

/// Properties for the `Graph` resource.
#[derive(Debug, Default)]
pub struct GraphProperties {
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<::Value<bool>>,
    /// Property [`GraphName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-graphname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub graph_name: Option<::Value<String>>,
    /// Property [`ProvisionedMemory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-provisionedmemory).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub provisioned_memory: ::Value<u32>,
    /// Property [`PublicConnectivity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-publicconnectivity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_connectivity: Option<::Value<bool>>,
    /// Property [`ReplicaCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-replicacount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replica_count: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VectorSearchConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html#cfn-neptunegraph-graph-vectorsearchconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vector_search_configuration: Option<::Value<self::graph::VectorSearchConfiguration>>,
}

impl ::serde::Serialize for GraphProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref graph_name) = self.graph_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GraphName", graph_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedMemory", &self.provisioned_memory)?;
        if let Some(ref public_connectivity) = self.public_connectivity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicConnectivity", public_connectivity)?;
        }
        if let Some(ref replica_count) = self.replica_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaCount", replica_count)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vector_search_configuration) = self.vector_search_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VectorSearchConfiguration", vector_search_configuration)?;
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
                let mut deletion_protection: Option<::Value<bool>> = None;
                let mut graph_name: Option<::Value<String>> = None;
                let mut provisioned_memory: Option<::Value<u32>> = None;
                let mut public_connectivity: Option<::Value<bool>> = None;
                let mut replica_count: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vector_search_configuration: Option<::Value<self::graph::VectorSearchConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GraphName" => {
                            graph_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedMemory" => {
                            provisioned_memory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicConnectivity" => {
                            public_connectivity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicaCount" => {
                            replica_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VectorSearchConfiguration" => {
                            vector_search_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GraphProperties {
                    deletion_protection: deletion_protection,
                    graph_name: graph_name,
                    provisioned_memory: provisioned_memory.ok_or(::serde::de::Error::missing_field("ProvisionedMemory"))?,
                    public_connectivity: public_connectivity,
                    replica_count: replica_count,
                    tags: tags,
                    vector_search_configuration: vector_search_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Graph {
    type Properties = GraphProperties;
    const TYPE: &'static str = "AWS::NeptuneGraph::Graph";
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

/// The [`AWS::NeptuneGraph::PrivateGraphEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct PrivateGraphEndpoint {
    properties: PrivateGraphEndpointProperties
}

/// Properties for the `PrivateGraphEndpoint` resource.
#[derive(Debug, Default)]
pub struct PrivateGraphEndpointProperties {
    /// Property [`GraphIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html#cfn-neptunegraph-privategraphendpoint-graphidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub graph_identifier: ::Value<String>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html#cfn-neptunegraph-privategraphendpoint-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html#cfn-neptunegraph-privategraphendpoint-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html#cfn-neptunegraph-privategraphendpoint-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for PrivateGraphEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GraphIdentifier", &self.graph_identifier)?;
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PrivateGraphEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateGraphEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PrivateGraphEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PrivateGraphEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut graph_identifier: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GraphIdentifier" => {
                            graph_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PrivateGraphEndpointProperties {
                    graph_identifier: graph_identifier.ok_or(::serde::de::Error::missing_field("GraphIdentifier"))?,
                    security_group_ids: security_group_ids,
                    subnet_ids: subnet_ids,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PrivateGraphEndpoint {
    type Properties = PrivateGraphEndpointProperties;
    const TYPE: &'static str = "AWS::NeptuneGraph::PrivateGraphEndpoint";
    fn properties(&self) -> &PrivateGraphEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PrivateGraphEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PrivateGraphEndpoint {}

impl From<PrivateGraphEndpointProperties> for PrivateGraphEndpoint {
    fn from(properties: PrivateGraphEndpointProperties) -> PrivateGraphEndpoint {
        PrivateGraphEndpoint { properties }
    }
}

pub mod graph {
    //! Property types for the `Graph` resource.

    /// The [`AWS::NeptuneGraph::Graph.VectorSearchConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-neptunegraph-graph-vectorsearchconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VectorSearchConfiguration {
        /// Property [`VectorSearchDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-neptunegraph-graph-vectorsearchconfiguration.html#cfn-neptunegraph-graph-vectorsearchconfiguration-vectorsearchdimension).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vector_search_dimension: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VectorSearchConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VectorSearchDimension", &self.vector_search_dimension)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VectorSearchConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VectorSearchConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VectorSearchConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VectorSearchConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vector_search_dimension: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VectorSearchDimension" => {
                                vector_search_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VectorSearchConfiguration {
                        vector_search_dimension: vector_search_dimension.ok_or(::serde::de::Error::missing_field("VectorSearchDimension"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
