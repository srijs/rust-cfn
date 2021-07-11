//! Types for the `AppMesh` service.

/// The [`AWS::AppMesh::GatewayRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html) resource type.
#[derive(Debug, Default)]
pub struct GatewayRoute {
    properties: GatewayRouteProperties
}

/// Properties for the `GatewayRoute` resource.
#[derive(Debug, Default)]
pub struct GatewayRouteProperties {
    /// Property [`GatewayRouteName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-gatewayroutename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub gateway_route_name: Option<::Value<String>>,
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::gateway_route::GatewayRouteSpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualGatewayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html#cfn-appmesh-gatewayroute-virtualgatewayname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_gateway_name: ::Value<String>,
}

impl ::serde::Serialize for GatewayRouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref gateway_route_name) = self.gateway_route_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayRouteName", gateway_route_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualGatewayName", &self.virtual_gateway_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GatewayRouteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GatewayRouteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GatewayRouteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut gateway_route_name: Option<::Value<String>> = None;
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::gateway_route::GatewayRouteSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_gateway_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GatewayRouteName" => {
                            gateway_route_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualGatewayName" => {
                            virtual_gateway_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GatewayRouteProperties {
                    gateway_route_name: gateway_route_name,
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_gateway_name: virtual_gateway_name.ok_or(::serde::de::Error::missing_field("VirtualGatewayName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GatewayRoute {
    type Properties = GatewayRouteProperties;
    const TYPE: &'static str = "AWS::AppMesh::GatewayRoute";
    fn properties(&self) -> &GatewayRouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GatewayRouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GatewayRoute {}

impl From<GatewayRouteProperties> for GatewayRoute {
    fn from(properties: GatewayRouteProperties) -> GatewayRoute {
        GatewayRoute { properties }
    }
}

/// The [`AWS::AppMesh::Mesh`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-mesh.html) resource type.
#[derive(Debug, Default)]
pub struct Mesh {
    properties: MeshProperties
}

/// Properties for the `Mesh` resource.
#[derive(Debug, Default)]
pub struct MeshProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-mesh.html#cfn-appmesh-mesh-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-mesh.html#cfn-appmesh-mesh-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: Option<::Value<self::mesh::MeshSpec>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-mesh.html#cfn-appmesh-mesh-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MeshProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref mesh_name) = self.mesh_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", mesh_name)?;
        }
        if let Some(ref spec) = self.spec {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", spec)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MeshProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MeshProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MeshProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MeshProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::mesh::MeshSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MeshProperties {
                    mesh_name: mesh_name,
                    spec: spec,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Mesh {
    type Properties = MeshProperties;
    const TYPE: &'static str = "AWS::AppMesh::Mesh";
    fn properties(&self) -> &MeshProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MeshProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Mesh {}

impl From<MeshProperties> for Mesh {
    fn from(properties: MeshProperties) -> Mesh {
        Mesh { properties }
    }
}

/// The [`AWS::AppMesh::Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html) resource type.
#[derive(Debug, Default)]
pub struct Route {
    properties: RouteProperties
}

/// Properties for the `Route` resource.
#[derive(Debug, Default)]
pub struct RouteProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`RouteName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-routename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub route_name: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::route::RouteSpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualRouterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html#cfn-appmesh-route-virtualroutername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_router_name: ::Value<String>,
}

impl ::serde::Serialize for RouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        if let Some(ref route_name) = self.route_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteName", route_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualRouterName", &self.virtual_router_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut route_name: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::route::RouteSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_router_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteName" => {
                            route_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualRouterName" => {
                            virtual_router_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RouteProperties {
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    route_name: route_name,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_router_name: virtual_router_name.ok_or(::serde::de::Error::missing_field("VirtualRouterName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Route {
    type Properties = RouteProperties;
    const TYPE: &'static str = "AWS::AppMesh::Route";
    fn properties(&self) -> &RouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Route {}

impl From<RouteProperties> for Route {
    fn from(properties: RouteProperties) -> Route {
        Route { properties }
    }
}

/// The [`AWS::AppMesh::VirtualGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html) resource type.
#[derive(Debug, Default)]
pub struct VirtualGateway {
    properties: VirtualGatewayProperties
}

/// Properties for the `VirtualGateway` resource.
#[derive(Debug, Default)]
pub struct VirtualGatewayProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html#cfn-appmesh-virtualgateway-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html#cfn-appmesh-virtualgateway-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html#cfn-appmesh-virtualgateway-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::virtual_gateway::VirtualGatewaySpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html#cfn-appmesh-virtualgateway-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualGatewayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html#cfn-appmesh-virtualgateway-virtualgatewayname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_gateway_name: Option<::Value<String>>,
}

impl ::serde::Serialize for VirtualGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref virtual_gateway_name) = self.virtual_gateway_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualGatewayName", virtual_gateway_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VirtualGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VirtualGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::virtual_gateway::VirtualGatewaySpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_gateway_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualGatewayName" => {
                            virtual_gateway_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VirtualGatewayProperties {
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_gateway_name: virtual_gateway_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VirtualGateway {
    type Properties = VirtualGatewayProperties;
    const TYPE: &'static str = "AWS::AppMesh::VirtualGateway";
    fn properties(&self) -> &VirtualGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VirtualGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VirtualGateway {}

impl From<VirtualGatewayProperties> for VirtualGateway {
    fn from(properties: VirtualGatewayProperties) -> VirtualGateway {
        VirtualGateway { properties }
    }
}

/// The [`AWS::AppMesh::VirtualNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html) resource type.
#[derive(Debug, Default)]
pub struct VirtualNode {
    properties: VirtualNodeProperties
}

/// Properties for the `VirtualNode` resource.
#[derive(Debug, Default)]
pub struct VirtualNodeProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html#cfn-appmesh-virtualnode-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html#cfn-appmesh-virtualnode-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html#cfn-appmesh-virtualnode-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::virtual_node::VirtualNodeSpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html#cfn-appmesh-virtualnode-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualNodeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html#cfn-appmesh-virtualnode-virtualnodename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_node_name: Option<::Value<String>>,
}

impl ::serde::Serialize for VirtualNodeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref virtual_node_name) = self.virtual_node_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualNodeName", virtual_node_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VirtualNodeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNodeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VirtualNodeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::virtual_node::VirtualNodeSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_node_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualNodeName" => {
                            virtual_node_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VirtualNodeProperties {
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_node_name: virtual_node_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VirtualNode {
    type Properties = VirtualNodeProperties;
    const TYPE: &'static str = "AWS::AppMesh::VirtualNode";
    fn properties(&self) -> &VirtualNodeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VirtualNodeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VirtualNode {}

impl From<VirtualNodeProperties> for VirtualNode {
    fn from(properties: VirtualNodeProperties) -> VirtualNode {
        VirtualNode { properties }
    }
}

/// The [`AWS::AppMesh::VirtualRouter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html) resource type.
#[derive(Debug, Default)]
pub struct VirtualRouter {
    properties: VirtualRouterProperties
}

/// Properties for the `VirtualRouter` resource.
#[derive(Debug, Default)]
pub struct VirtualRouterProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html#cfn-appmesh-virtualrouter-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html#cfn-appmesh-virtualrouter-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html#cfn-appmesh-virtualrouter-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::virtual_router::VirtualRouterSpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html#cfn-appmesh-virtualrouter-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualRouterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html#cfn-appmesh-virtualrouter-virtualroutername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_router_name: Option<::Value<String>>,
}

impl ::serde::Serialize for VirtualRouterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref virtual_router_name) = self.virtual_router_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualRouterName", virtual_router_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VirtualRouterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualRouterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualRouterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VirtualRouterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::virtual_router::VirtualRouterSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_router_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualRouterName" => {
                            virtual_router_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VirtualRouterProperties {
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_router_name: virtual_router_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VirtualRouter {
    type Properties = VirtualRouterProperties;
    const TYPE: &'static str = "AWS::AppMesh::VirtualRouter";
    fn properties(&self) -> &VirtualRouterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VirtualRouterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VirtualRouter {}

impl From<VirtualRouterProperties> for VirtualRouter {
    fn from(properties: VirtualRouterProperties) -> VirtualRouter {
        VirtualRouter { properties }
    }
}

/// The [`AWS::AppMesh::VirtualService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html) resource type.
#[derive(Debug, Default)]
pub struct VirtualService {
    properties: VirtualServiceProperties
}

/// Properties for the `VirtualService` resource.
#[derive(Debug, Default)]
pub struct VirtualServiceProperties {
    /// Property [`MeshName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html#cfn-appmesh-virtualservice-meshname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_name: ::Value<String>,
    /// Property [`MeshOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html#cfn-appmesh-virtualservice-meshowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mesh_owner: Option<::Value<String>>,
    /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html#cfn-appmesh-virtualservice-spec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub spec: ::Value<self::virtual_service::VirtualServiceSpec>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html#cfn-appmesh-virtualservice-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VirtualServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html#cfn-appmesh-virtualservice-virtualservicename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtual_service_name: ::Value<String>,
}

impl ::serde::Serialize for VirtualServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshName", &self.mesh_name)?;
        if let Some(ref mesh_owner) = self.mesh_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeshOwner", mesh_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", &self.spec)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualServiceName", &self.virtual_service_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VirtualServiceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualServiceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualServiceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VirtualServiceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mesh_name: Option<::Value<String>> = None;
                let mut mesh_owner: Option<::Value<String>> = None;
                let mut spec: Option<::Value<self::virtual_service::VirtualServiceSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut virtual_service_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MeshName" => {
                            mesh_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MeshOwner" => {
                            mesh_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Spec" => {
                            spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualServiceName" => {
                            virtual_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VirtualServiceProperties {
                    mesh_name: mesh_name.ok_or(::serde::de::Error::missing_field("MeshName"))?,
                    mesh_owner: mesh_owner,
                    spec: spec.ok_or(::serde::de::Error::missing_field("Spec"))?,
                    tags: tags,
                    virtual_service_name: virtual_service_name.ok_or(::serde::de::Error::missing_field("VirtualServiceName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VirtualService {
    type Properties = VirtualServiceProperties;
    const TYPE: &'static str = "AWS::AppMesh::VirtualService";
    fn properties(&self) -> &VirtualServiceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VirtualServiceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VirtualService {}

impl From<VirtualServiceProperties> for VirtualService {
    fn from(properties: VirtualServiceProperties) -> VirtualService {
        VirtualService { properties }
    }
}

pub mod gateway_route {
    //! Property types for the `GatewayRoute` resource.

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteHostnameMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamematch.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteHostnameMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamematch.html#cfn-appmesh-gatewayroute-gatewayroutehostnamematch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Suffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamematch.html#cfn-appmesh-gatewayroute-gatewayroutehostnamematch-suffix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suffix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GatewayRouteHostnameMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref suffix) = self.suffix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Suffix", suffix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteHostnameMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteHostnameMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteHostnameMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteHostnameMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut suffix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Suffix" => {
                                suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteHostnameMatch {
                        exact: exact,
                        suffix: suffix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteHostnameRewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamerewrite.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteHostnameRewrite {
        /// Property [`DefaultTargetHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamerewrite.html#cfn-appmesh-gatewayroute-gatewayroutehostnamerewrite-defaulttargethostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_target_hostname: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GatewayRouteHostnameRewrite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_target_hostname) = self.default_target_hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTargetHostname", default_target_hostname)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteHostnameRewrite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteHostnameRewrite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteHostnameRewrite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteHostnameRewrite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_target_hostname: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultTargetHostname" => {
                                default_target_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteHostnameRewrite {
                        default_target_hostname: default_target_hostname,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteMetadataMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteMetadataMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html#cfn-appmesh-gatewayroute-gatewayroutemetadatamatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html#cfn-appmesh-gatewayroute-gatewayroutemetadatamatch-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html#cfn-appmesh-gatewayroute-gatewayroutemetadatamatch-range).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range: Option<::Value<GatewayRouteRangeMatch>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html#cfn-appmesh-gatewayroute-gatewayroutemetadatamatch-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
        /// Property [`Suffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html#cfn-appmesh-gatewayroute-gatewayroutemetadatamatch-suffix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suffix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GatewayRouteMetadataMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref range) = self.range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Range", range)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            if let Some(ref suffix) = self.suffix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Suffix", suffix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteMetadataMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteMetadataMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteMetadataMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteMetadataMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut range: Option<::Value<GatewayRouteRangeMatch>> = None;
                    let mut regex: Option<::Value<String>> = None;
                    let mut suffix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Range" => {
                                range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Suffix" => {
                                suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteMetadataMatch {
                        exact: exact,
                        prefix: prefix,
                        range: range,
                        regex: regex,
                        suffix: suffix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteRangeMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayrouterangematch.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteRangeMatch {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayrouterangematch.html#cfn-appmesh-gatewayroute-gatewayrouterangematch-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: ::Value<u32>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayrouterangematch.html#cfn-appmesh-gatewayroute-gatewayrouterangematch-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: ::Value<u32>,
    }

    impl ::codec::SerializeValue for GatewayRouteRangeMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", &self.end)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", &self.start)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteRangeMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteRangeMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteRangeMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteRangeMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<u32>> = None;
                    let mut start: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteRangeMatch {
                        end: end.ok_or(::serde::de::Error::missing_field("End"))?,
                        start: start.ok_or(::serde::de::Error::missing_field("Start"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutespec.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteSpec {
        /// Property [`GrpcRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutespec.html#cfn-appmesh-gatewayroute-gatewayroutespec-grpcroute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc_route: Option<::Value<GrpcGatewayRoute>>,
        /// Property [`Http2Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutespec.html#cfn-appmesh-gatewayroute-gatewayroutespec-http2route).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http2_route: Option<::Value<HttpGatewayRoute>>,
        /// Property [`HttpRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutespec.html#cfn-appmesh-gatewayroute-gatewayroutespec-httproute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_route: Option<::Value<HttpGatewayRoute>>,
    }

    impl ::codec::SerializeValue for GatewayRouteSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc_route) = self.grpc_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrpcRoute", grpc_route)?;
            }
            if let Some(ref http2_route) = self.http2_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Http2Route", http2_route)?;
            }
            if let Some(ref http_route) = self.http_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRoute", http_route)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc_route: Option<::Value<GrpcGatewayRoute>> = None;
                    let mut http2_route: Option<::Value<HttpGatewayRoute>> = None;
                    let mut http_route: Option<::Value<HttpGatewayRoute>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GrpcRoute" => {
                                grpc_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Http2Route" => {
                                http2_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpRoute" => {
                                http_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteSpec {
                        grpc_route: grpc_route,
                        http2_route: http2_route,
                        http_route: http_route,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutetarget.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteTarget {
        /// Property [`VirtualService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutetarget.html#cfn-appmesh-gatewayroute-gatewayroutetarget-virtualservice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_service: ::Value<GatewayRouteVirtualService>,
    }

    impl ::codec::SerializeValue for GatewayRouteTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualService", &self.virtual_service)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_service: Option<::Value<GatewayRouteVirtualService>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualService" => {
                                virtual_service = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteTarget {
                        virtual_service: virtual_service.ok_or(::serde::de::Error::missing_field("VirtualService"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GatewayRouteVirtualService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutevirtualservice.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayRouteVirtualService {
        /// Property [`VirtualServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutevirtualservice.html#cfn-appmesh-gatewayroute-gatewayroutevirtualservice-virtualservicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_service_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GatewayRouteVirtualService {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualServiceName", &self.virtual_service_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayRouteVirtualService {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayRouteVirtualService, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayRouteVirtualService;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayRouteVirtualService")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualServiceName" => {
                                virtual_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayRouteVirtualService {
                        virtual_service_name: virtual_service_name.ok_or(::serde::de::Error::missing_field("VirtualServiceName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GrpcGatewayRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroute.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcGatewayRoute {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroute.html#cfn-appmesh-gatewayroute-grpcgatewayroute-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<GrpcGatewayRouteAction>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroute.html#cfn-appmesh-gatewayroute-grpcgatewayroute-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<GrpcGatewayRouteMatch>,
    }

    impl ::codec::SerializeValue for GrpcGatewayRoute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcGatewayRoute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcGatewayRoute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcGatewayRoute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcGatewayRoute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<GrpcGatewayRouteAction>> = None;
                    let mut r#match: Option<::Value<GrpcGatewayRouteMatch>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcGatewayRoute {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GrpcGatewayRouteAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouteaction.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcGatewayRouteAction {
        /// Property [`Rewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouteaction.html#cfn-appmesh-gatewayroute-grpcgatewayrouteaction-rewrite).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rewrite: Option<::Value<GrpcGatewayRouteRewrite>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouteaction.html#cfn-appmesh-gatewayroute-grpcgatewayrouteaction-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<GatewayRouteTarget>,
    }

    impl ::codec::SerializeValue for GrpcGatewayRouteAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rewrite) = self.rewrite {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rewrite", rewrite)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcGatewayRouteAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcGatewayRouteAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcGatewayRouteAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcGatewayRouteAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rewrite: Option<::Value<GrpcGatewayRouteRewrite>> = None;
                    let mut target: Option<::Value<GatewayRouteTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rewrite" => {
                                rewrite = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcGatewayRouteAction {
                        rewrite: rewrite,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GrpcGatewayRouteMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutematch.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcGatewayRouteMatch {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutematch.html#cfn-appmesh-gatewayroute-grpcgatewayroutematch-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: Option<::Value<GatewayRouteHostnameMatch>>,
        /// Property [`Metadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutematch.html#cfn-appmesh-gatewayroute-grpcgatewayroutematch-metadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata: Option<::ValueList<GrpcGatewayRouteMetadata>>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutematch.html#cfn-appmesh-gatewayroute-grpcgatewayroutematch-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GrpcGatewayRouteMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            if let Some(ref metadata) = self.metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metadata", metadata)?;
            }
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcGatewayRouteMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcGatewayRouteMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcGatewayRouteMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcGatewayRouteMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname: Option<::Value<GatewayRouteHostnameMatch>> = None;
                    let mut metadata: Option<::ValueList<GrpcGatewayRouteMetadata>> = None;
                    let mut service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metadata" => {
                                metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcGatewayRouteMatch {
                        hostname: hostname,
                        metadata: metadata,
                        service_name: service_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GrpcGatewayRouteMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutemetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcGatewayRouteMetadata {
        /// Property [`Invert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutemetadata.html#cfn-appmesh-gatewayroute-grpcgatewayroutemetadata-invert).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invert: Option<::Value<bool>>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutemetadata.html#cfn-appmesh-gatewayroute-grpcgatewayroutemetadata-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<GatewayRouteMetadataMatch>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutemetadata.html#cfn-appmesh-gatewayroute-grpcgatewayroutemetadata-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GrpcGatewayRouteMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invert) = self.invert {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Invert", invert)?;
            }
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcGatewayRouteMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcGatewayRouteMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcGatewayRouteMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcGatewayRouteMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invert: Option<::Value<bool>> = None;
                    let mut r#match: Option<::Value<GatewayRouteMetadataMatch>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Invert" => {
                                invert = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcGatewayRouteMetadata {
                        invert: invert,
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.GrpcGatewayRouteRewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouterewrite.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcGatewayRouteRewrite {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouterewrite.html#cfn-appmesh-gatewayroute-grpcgatewayrouterewrite-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: Option<::Value<GatewayRouteHostnameRewrite>>,
    }

    impl ::codec::SerializeValue for GrpcGatewayRouteRewrite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcGatewayRouteRewrite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcGatewayRouteRewrite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcGatewayRouteRewrite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcGatewayRouteRewrite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname: Option<::Value<GatewayRouteHostnameRewrite>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcGatewayRouteRewrite {
                        hostname: hostname,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroute.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRoute {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroute.html#cfn-appmesh-gatewayroute-httpgatewayroute-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<HttpGatewayRouteAction>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroute.html#cfn-appmesh-gatewayroute-httpgatewayroute-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<HttpGatewayRouteMatch>,
    }

    impl ::codec::SerializeValue for HttpGatewayRoute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRoute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRoute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRoute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRoute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<HttpGatewayRouteAction>> = None;
                    let mut r#match: Option<::Value<HttpGatewayRouteMatch>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRoute {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRouteAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteaction.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRouteAction {
        /// Property [`Rewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteaction.html#cfn-appmesh-gatewayroute-httpgatewayrouteaction-rewrite).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rewrite: Option<::Value<HttpGatewayRouteRewrite>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteaction.html#cfn-appmesh-gatewayroute-httpgatewayrouteaction-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<GatewayRouteTarget>,
    }

    impl ::codec::SerializeValue for HttpGatewayRouteAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rewrite) = self.rewrite {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rewrite", rewrite)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRouteAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRouteAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRouteAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRouteAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rewrite: Option<::Value<HttpGatewayRouteRewrite>> = None;
                    let mut target: Option<::Value<GatewayRouteTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rewrite" => {
                                rewrite = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRouteAction {
                        rewrite: rewrite,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRouteHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheader.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRouteHeader {
        /// Property [`Invert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheader.html#cfn-appmesh-gatewayroute-httpgatewayrouteheader-invert).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invert: Option<::Value<bool>>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheader.html#cfn-appmesh-gatewayroute-httpgatewayrouteheader-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<HttpGatewayRouteHeaderMatch>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheader.html#cfn-appmesh-gatewayroute-httpgatewayrouteheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpGatewayRouteHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invert) = self.invert {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Invert", invert)?;
            }
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRouteHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRouteHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRouteHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRouteHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invert: Option<::Value<bool>> = None;
                    let mut r#match: Option<::Value<HttpGatewayRouteHeaderMatch>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Invert" => {
                                invert = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRouteHeader {
                        invert: invert,
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRouteHeaderMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRouteHeaderMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html#cfn-appmesh-gatewayroute-httpgatewayrouteheadermatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html#cfn-appmesh-gatewayroute-httpgatewayrouteheadermatch-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html#cfn-appmesh-gatewayroute-httpgatewayrouteheadermatch-range).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range: Option<::Value<GatewayRouteRangeMatch>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html#cfn-appmesh-gatewayroute-httpgatewayrouteheadermatch-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
        /// Property [`Suffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html#cfn-appmesh-gatewayroute-httpgatewayrouteheadermatch-suffix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suffix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpGatewayRouteHeaderMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref range) = self.range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Range", range)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            if let Some(ref suffix) = self.suffix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Suffix", suffix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRouteHeaderMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRouteHeaderMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRouteHeaderMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRouteHeaderMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut range: Option<::Value<GatewayRouteRangeMatch>> = None;
                    let mut regex: Option<::Value<String>> = None;
                    let mut suffix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Range" => {
                                range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Suffix" => {
                                suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRouteHeaderMatch {
                        exact: exact,
                        prefix: prefix,
                        range: range,
                        regex: regex,
                        suffix: suffix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRouteMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRouteMatch {
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<HttpGatewayRouteHeader>>,
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: Option<::Value<GatewayRouteHostnameMatch>>,
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-method).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<HttpPathMatch>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`QueryParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html#cfn-appmesh-gatewayroute-httpgatewayroutematch-queryparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_parameters: Option<::ValueList<QueryParameter>>,
    }

    impl ::codec::SerializeValue for HttpGatewayRouteMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref query_parameters) = self.query_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryParameters", query_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRouteMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRouteMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRouteMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRouteMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut headers: Option<::ValueList<HttpGatewayRouteHeader>> = None;
                    let mut hostname: Option<::Value<GatewayRouteHostnameMatch>> = None;
                    let mut method: Option<::Value<String>> = None;
                    let mut path: Option<::Value<HttpPathMatch>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut query_parameters: Option<::ValueList<QueryParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryParameters" => {
                                query_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRouteMatch {
                        headers: headers,
                        hostname: hostname,
                        method: method,
                        path: path,
                        prefix: prefix,
                        query_parameters: query_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRoutePathRewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutepathrewrite.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRoutePathRewrite {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutepathrewrite.html#cfn-appmesh-gatewayroute-httpgatewayroutepathrewrite-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpGatewayRoutePathRewrite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRoutePathRewrite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRoutePathRewrite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRoutePathRewrite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRoutePathRewrite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRoutePathRewrite {
                        exact: exact,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRoutePrefixRewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteprefixrewrite.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRoutePrefixRewrite {
        /// Property [`DefaultPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteprefixrewrite.html#cfn-appmesh-gatewayroute-httpgatewayrouteprefixrewrite-defaultprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_prefix: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteprefixrewrite.html#cfn-appmesh-gatewayroute-httpgatewayrouteprefixrewrite-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpGatewayRoutePrefixRewrite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_prefix) = self.default_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultPrefix", default_prefix)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRoutePrefixRewrite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRoutePrefixRewrite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRoutePrefixRewrite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRoutePrefixRewrite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_prefix: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultPrefix" => {
                                default_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRoutePrefixRewrite {
                        default_prefix: default_prefix,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpGatewayRouteRewrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouterewrite.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpGatewayRouteRewrite {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouterewrite.html#cfn-appmesh-gatewayroute-httpgatewayrouterewrite-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: Option<::Value<GatewayRouteHostnameRewrite>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouterewrite.html#cfn-appmesh-gatewayroute-httpgatewayrouterewrite-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<HttpGatewayRoutePathRewrite>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouterewrite.html#cfn-appmesh-gatewayroute-httpgatewayrouterewrite-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<HttpGatewayRoutePrefixRewrite>>,
    }

    impl ::codec::SerializeValue for HttpGatewayRouteRewrite {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpGatewayRouteRewrite {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpGatewayRouteRewrite, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpGatewayRouteRewrite;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpGatewayRouteRewrite")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname: Option<::Value<GatewayRouteHostnameRewrite>> = None;
                    let mut path: Option<::Value<HttpGatewayRoutePathRewrite>> = None;
                    let mut prefix: Option<::Value<HttpGatewayRoutePrefixRewrite>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpGatewayRouteRewrite {
                        hostname: hostname,
                        path: path,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpPathMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httppathmatch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpPathMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httppathmatch.html#cfn-appmesh-gatewayroute-httppathmatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httppathmatch.html#cfn-appmesh-gatewayroute-httppathmatch-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpPathMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpPathMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpPathMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpPathMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpPathMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut regex: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpPathMatch {
                        exact: exact,
                        regex: regex,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.HttpQueryParameterMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpqueryparametermatch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpQueryParameterMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpqueryparametermatch.html#cfn-appmesh-gatewayroute-httpqueryparametermatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpQueryParameterMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpQueryParameterMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpQueryParameterMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpQueryParameterMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpQueryParameterMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpQueryParameterMatch {
                        exact: exact,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::GatewayRoute.QueryParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-queryparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryParameter {
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-queryparameter.html#cfn-appmesh-gatewayroute-queryparameter-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<HttpQueryParameterMatch>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-queryparameter.html#cfn-appmesh-gatewayroute-queryparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueryParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#match: Option<::Value<HttpQueryParameterMatch>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryParameter {
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod mesh {
    //! Property types for the `Mesh` resource.

    /// The [`AWS::AppMesh::Mesh.EgressFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-egressfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct EgressFilter {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-egressfilter.html#cfn-appmesh-mesh-egressfilter-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for EgressFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EgressFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EgressFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EgressFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EgressFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EgressFilter {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Mesh.MeshSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-meshspec.html) property type.
    #[derive(Debug, Default)]
    pub struct MeshSpec {
        /// Property [`EgressFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-meshspec.html#cfn-appmesh-mesh-meshspec-egressfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub egress_filter: Option<::Value<EgressFilter>>,
    }

    impl ::codec::SerializeValue for MeshSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref egress_filter) = self.egress_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressFilter", egress_filter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MeshSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MeshSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MeshSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MeshSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut egress_filter: Option<::Value<EgressFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EgressFilter" => {
                                egress_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MeshSpec {
                        egress_filter: egress_filter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod route {
    //! Property types for the `Route` resource.

    /// The [`AWS::AppMesh::Route.Duration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-duration.html) property type.
    #[derive(Debug, Default)]
    pub struct Duration {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-duration.html#cfn-appmesh-route-duration-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-duration.html#cfn-appmesh-route-duration-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Duration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Duration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Duration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Duration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Duration {
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRetryPolicy {
        /// Property [`GrpcRetryEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html#cfn-appmesh-route-grpcretrypolicy-grpcretryevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc_retry_events: Option<::ValueList<String>>,
        /// Property [`HttpRetryEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html#cfn-appmesh-route-grpcretrypolicy-httpretryevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_retry_events: Option<::ValueList<String>>,
        /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html#cfn-appmesh-route-grpcretrypolicy-maxretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retries: ::Value<u32>,
        /// Property [`PerRetryTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html#cfn-appmesh-route-grpcretrypolicy-perretrytimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_retry_timeout: ::Value<Duration>,
        /// Property [`TcpRetryEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html#cfn-appmesh-route-grpcretrypolicy-tcpretryevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp_retry_events: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for GrpcRetryPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc_retry_events) = self.grpc_retry_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrpcRetryEvents", grpc_retry_events)?;
            }
            if let Some(ref http_retry_events) = self.http_retry_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRetryEvents", http_retry_events)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", &self.max_retries)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRetryTimeout", &self.per_retry_timeout)?;
            if let Some(ref tcp_retry_events) = self.tcp_retry_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TcpRetryEvents", tcp_retry_events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRetryPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRetryPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRetryPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRetryPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc_retry_events: Option<::ValueList<String>> = None;
                    let mut http_retry_events: Option<::ValueList<String>> = None;
                    let mut max_retries: Option<::Value<u32>> = None;
                    let mut per_retry_timeout: Option<::Value<Duration>> = None;
                    let mut tcp_retry_events: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GrpcRetryEvents" => {
                                grpc_retry_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpRetryEvents" => {
                                http_retry_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRetries" => {
                                max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRetryTimeout" => {
                                per_retry_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TcpRetryEvents" => {
                                tcp_retry_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRetryPolicy {
                        grpc_retry_events: grpc_retry_events,
                        http_retry_events: http_retry_events,
                        max_retries: max_retries.ok_or(::serde::de::Error::missing_field("MaxRetries"))?,
                        per_retry_timeout: per_retry_timeout.ok_or(::serde::de::Error::missing_field("PerRetryTimeout"))?,
                        tcp_retry_events: tcp_retry_events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRoute {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html#cfn-appmesh-route-grpcroute-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<GrpcRouteAction>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html#cfn-appmesh-route-grpcroute-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<GrpcRouteMatch>,
        /// Property [`RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html#cfn-appmesh-route-grpcroute-retrypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_policy: Option<::Value<GrpcRetryPolicy>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html#cfn-appmesh-route-grpcroute-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<GrpcTimeout>>,
    }

    impl ::codec::SerializeValue for GrpcRoute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            if let Some(ref retry_policy) = self.retry_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryPolicy", retry_policy)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRoute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRoute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRoute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRoute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<GrpcRouteAction>> = None;
                    let mut r#match: Option<::Value<GrpcRouteMatch>> = None;
                    let mut retry_policy: Option<::Value<GrpcRetryPolicy>> = None;
                    let mut timeout: Option<::Value<GrpcTimeout>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryPolicy" => {
                                retry_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRoute {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                        retry_policy: retry_policy,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRouteAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcrouteaction.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRouteAction {
        /// Property [`WeightedTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcrouteaction.html#cfn-appmesh-route-grpcrouteaction-weightedtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weighted_targets: ::ValueList<WeightedTarget>,
    }

    impl ::codec::SerializeValue for GrpcRouteAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedTargets", &self.weighted_targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRouteAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRouteAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRouteAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRouteAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut weighted_targets: Option<::ValueList<WeightedTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WeightedTargets" => {
                                weighted_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRouteAction {
                        weighted_targets: weighted_targets.ok_or(::serde::de::Error::missing_field("WeightedTargets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRouteMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutematch.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRouteMatch {
        /// Property [`Metadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutematch.html#cfn-appmesh-route-grpcroutematch-metadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata: Option<::ValueList<GrpcRouteMetadata>>,
        /// Property [`MethodName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutematch.html#cfn-appmesh-route-grpcroutematch-methodname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method_name: Option<::Value<String>>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutematch.html#cfn-appmesh-route-grpcroutematch-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GrpcRouteMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metadata) = self.metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metadata", metadata)?;
            }
            if let Some(ref method_name) = self.method_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodName", method_name)?;
            }
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRouteMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRouteMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRouteMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRouteMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metadata: Option<::ValueList<GrpcRouteMetadata>> = None;
                    let mut method_name: Option<::Value<String>> = None;
                    let mut service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Metadata" => {
                                metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MethodName" => {
                                method_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRouteMatch {
                        metadata: metadata,
                        method_name: method_name,
                        service_name: service_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRouteMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRouteMetadata {
        /// Property [`Invert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadata.html#cfn-appmesh-route-grpcroutemetadata-invert).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invert: Option<::Value<bool>>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadata.html#cfn-appmesh-route-grpcroutemetadata-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<GrpcRouteMetadataMatchMethod>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadata.html#cfn-appmesh-route-grpcroutemetadata-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GrpcRouteMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invert) = self.invert {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Invert", invert)?;
            }
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRouteMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRouteMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRouteMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRouteMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invert: Option<::Value<bool>> = None;
                    let mut r#match: Option<::Value<GrpcRouteMetadataMatchMethod>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Invert" => {
                                invert = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRouteMetadata {
                        invert: invert,
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcRouteMetadataMatchMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcRouteMetadataMatchMethod {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html#cfn-appmesh-route-grpcroutemetadatamatchmethod-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html#cfn-appmesh-route-grpcroutemetadatamatchmethod-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html#cfn-appmesh-route-grpcroutemetadatamatchmethod-range).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range: Option<::Value<MatchRange>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html#cfn-appmesh-route-grpcroutemetadatamatchmethod-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
        /// Property [`Suffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html#cfn-appmesh-route-grpcroutemetadatamatchmethod-suffix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suffix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GrpcRouteMetadataMatchMethod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref range) = self.range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Range", range)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            if let Some(ref suffix) = self.suffix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Suffix", suffix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcRouteMetadataMatchMethod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcRouteMetadataMatchMethod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcRouteMetadataMatchMethod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcRouteMetadataMatchMethod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut range: Option<::Value<MatchRange>> = None;
                    let mut regex: Option<::Value<String>> = None;
                    let mut suffix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Range" => {
                                range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Suffix" => {
                                suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcRouteMetadataMatchMethod {
                        exact: exact,
                        prefix: prefix,
                        range: range,
                        regex: regex,
                        suffix: suffix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.GrpcTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpctimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpctimeout.html#cfn-appmesh-route-grpctimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
        /// Property [`PerRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpctimeout.html#cfn-appmesh-route-grpctimeout-perrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_request: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for GrpcTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            if let Some(ref per_request) = self.per_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRequest", per_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;
                    let mut per_request: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRequest" => {
                                per_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcTimeout {
                        idle: idle,
                        per_request: per_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HeaderMatchMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html) property type.
    #[derive(Debug, Default)]
    pub struct HeaderMatchMethod {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html#cfn-appmesh-route-headermatchmethod-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html#cfn-appmesh-route-headermatchmethod-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html#cfn-appmesh-route-headermatchmethod-range).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range: Option<::Value<MatchRange>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html#cfn-appmesh-route-headermatchmethod-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
        /// Property [`Suffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html#cfn-appmesh-route-headermatchmethod-suffix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suffix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HeaderMatchMethod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref range) = self.range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Range", range)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            if let Some(ref suffix) = self.suffix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Suffix", suffix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeaderMatchMethod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeaderMatchMethod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeaderMatchMethod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeaderMatchMethod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut range: Option<::Value<MatchRange>> = None;
                    let mut regex: Option<::Value<String>> = None;
                    let mut suffix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Range" => {
                                range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Suffix" => {
                                suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeaderMatchMethod {
                        exact: exact,
                        prefix: prefix,
                        range: range,
                        regex: regex,
                        suffix: suffix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpPathMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httppathmatch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpPathMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httppathmatch.html#cfn-appmesh-route-httppathmatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
        /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httppathmatch.html#cfn-appmesh-route-httppathmatch-regex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpPathMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            if let Some(ref regex) = self.regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", regex)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpPathMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpPathMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpPathMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpPathMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;
                    let mut regex: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regex" => {
                                regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpPathMatch {
                        exact: exact,
                        regex: regex,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpQueryParameterMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpqueryparametermatch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpQueryParameterMatch {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpqueryparametermatch.html#cfn-appmesh-route-httpqueryparametermatch-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpQueryParameterMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpQueryParameterMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpQueryParameterMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpQueryParameterMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpQueryParameterMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpQueryParameterMatch {
                        exact: exact,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpRetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRetryPolicy {
        /// Property [`HttpRetryEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html#cfn-appmesh-route-httpretrypolicy-httpretryevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_retry_events: Option<::ValueList<String>>,
        /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html#cfn-appmesh-route-httpretrypolicy-maxretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retries: ::Value<u32>,
        /// Property [`PerRetryTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html#cfn-appmesh-route-httpretrypolicy-perretrytimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_retry_timeout: ::Value<Duration>,
        /// Property [`TcpRetryEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html#cfn-appmesh-route-httpretrypolicy-tcpretryevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp_retry_events: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HttpRetryPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_retry_events) = self.http_retry_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRetryEvents", http_retry_events)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", &self.max_retries)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRetryTimeout", &self.per_retry_timeout)?;
            if let Some(ref tcp_retry_events) = self.tcp_retry_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TcpRetryEvents", tcp_retry_events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRetryPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRetryPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRetryPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRetryPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_retry_events: Option<::ValueList<String>> = None;
                    let mut max_retries: Option<::Value<u32>> = None;
                    let mut per_retry_timeout: Option<::Value<Duration>> = None;
                    let mut tcp_retry_events: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpRetryEvents" => {
                                http_retry_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRetries" => {
                                max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRetryTimeout" => {
                                per_retry_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TcpRetryEvents" => {
                                tcp_retry_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpRetryPolicy {
                        http_retry_events: http_retry_events,
                        max_retries: max_retries.ok_or(::serde::de::Error::missing_field("MaxRetries"))?,
                        per_retry_timeout: per_retry_timeout.ok_or(::serde::de::Error::missing_field("PerRetryTimeout"))?,
                        tcp_retry_events: tcp_retry_events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRoute {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html#cfn-appmesh-route-httproute-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<HttpRouteAction>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html#cfn-appmesh-route-httproute-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<HttpRouteMatch>,
        /// Property [`RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html#cfn-appmesh-route-httproute-retrypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_policy: Option<::Value<HttpRetryPolicy>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html#cfn-appmesh-route-httproute-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<HttpTimeout>>,
    }

    impl ::codec::SerializeValue for HttpRoute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            if let Some(ref retry_policy) = self.retry_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryPolicy", retry_policy)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRoute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRoute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRoute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRoute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<HttpRouteAction>> = None;
                    let mut r#match: Option<::Value<HttpRouteMatch>> = None;
                    let mut retry_policy: Option<::Value<HttpRetryPolicy>> = None;
                    let mut timeout: Option<::Value<HttpTimeout>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryPolicy" => {
                                retry_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpRoute {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                        retry_policy: retry_policy,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpRouteAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteaction.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRouteAction {
        /// Property [`WeightedTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteaction.html#cfn-appmesh-route-httprouteaction-weightedtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weighted_targets: ::ValueList<WeightedTarget>,
    }

    impl ::codec::SerializeValue for HttpRouteAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedTargets", &self.weighted_targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRouteAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRouteAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRouteAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRouteAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut weighted_targets: Option<::ValueList<WeightedTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WeightedTargets" => {
                                weighted_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpRouteAction {
                        weighted_targets: weighted_targets.ok_or(::serde::de::Error::missing_field("WeightedTargets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpRouteHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteheader.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRouteHeader {
        /// Property [`Invert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteheader.html#cfn-appmesh-route-httprouteheader-invert).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invert: Option<::Value<bool>>,
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteheader.html#cfn-appmesh-route-httprouteheader-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<HeaderMatchMethod>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteheader.html#cfn-appmesh-route-httprouteheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpRouteHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invert) = self.invert {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Invert", invert)?;
            }
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRouteHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRouteHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRouteHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRouteHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invert: Option<::Value<bool>> = None;
                    let mut r#match: Option<::Value<HeaderMatchMethod>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Invert" => {
                                invert = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpRouteHeader {
                        invert: invert,
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpRouteMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRouteMatch {
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<HttpRouteHeader>>,
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-method).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<HttpPathMatch>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`QueryParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-queryparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_parameters: Option<::ValueList<QueryParameter>>,
        /// Property [`Scheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html#cfn-appmesh-route-httproutematch-scheme).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheme: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpRouteMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref query_parameters) = self.query_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryParameters", query_parameters)?;
            }
            if let Some(ref scheme) = self.scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheme", scheme)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRouteMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRouteMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRouteMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRouteMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut headers: Option<::ValueList<HttpRouteHeader>> = None;
                    let mut method: Option<::Value<String>> = None;
                    let mut path: Option<::Value<HttpPathMatch>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut query_parameters: Option<::ValueList<QueryParameter>> = None;
                    let mut scheme: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryParameters" => {
                                query_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scheme" => {
                                scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpRouteMatch {
                        headers: headers,
                        method: method,
                        path: path,
                        prefix: prefix,
                        query_parameters: query_parameters,
                        scheme: scheme,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.HttpTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httptimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httptimeout.html#cfn-appmesh-route-httptimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
        /// Property [`PerRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httptimeout.html#cfn-appmesh-route-httptimeout-perrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_request: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for HttpTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            if let Some(ref per_request) = self.per_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRequest", per_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;
                    let mut per_request: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRequest" => {
                                per_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpTimeout {
                        idle: idle,
                        per_request: per_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.MatchRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-matchrange.html) property type.
    #[derive(Debug, Default)]
    pub struct MatchRange {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-matchrange.html#cfn-appmesh-route-matchrange-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: ::Value<u32>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-matchrange.html#cfn-appmesh-route-matchrange-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: ::Value<u32>,
    }

    impl ::codec::SerializeValue for MatchRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", &self.end)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", &self.start)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MatchRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MatchRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MatchRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<u32>> = None;
                    let mut start: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MatchRange {
                        end: end.ok_or(::serde::de::Error::missing_field("End"))?,
                        start: start.ok_or(::serde::de::Error::missing_field("Start"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.QueryParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-queryparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryParameter {
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-queryparameter.html#cfn-appmesh-route-queryparameter-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: Option<::Value<HttpQueryParameterMatch>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-queryparameter.html#cfn-appmesh-route-queryparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueryParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#match) = self.r#match {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", r#match)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#match: Option<::Value<HttpQueryParameterMatch>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryParameter {
                        r#match: r#match,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.RouteSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html) property type.
    #[derive(Debug, Default)]
    pub struct RouteSpec {
        /// Property [`GrpcRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html#cfn-appmesh-route-routespec-grpcroute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc_route: Option<::Value<GrpcRoute>>,
        /// Property [`Http2Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html#cfn-appmesh-route-routespec-http2route).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http2_route: Option<::Value<HttpRoute>>,
        /// Property [`HttpRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html#cfn-appmesh-route-routespec-httproute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_route: Option<::Value<HttpRoute>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html#cfn-appmesh-route-routespec-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: Option<::Value<u32>>,
        /// Property [`TcpRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html#cfn-appmesh-route-routespec-tcproute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp_route: Option<::Value<TcpRoute>>,
    }

    impl ::codec::SerializeValue for RouteSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc_route) = self.grpc_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrpcRoute", grpc_route)?;
            }
            if let Some(ref http2_route) = self.http2_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Http2Route", http2_route)?;
            }
            if let Some(ref http_route) = self.http_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRoute", http_route)?;
            }
            if let Some(ref priority) = self.priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
            }
            if let Some(ref tcp_route) = self.tcp_route {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TcpRoute", tcp_route)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RouteSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RouteSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RouteSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc_route: Option<::Value<GrpcRoute>> = None;
                    let mut http2_route: Option<::Value<HttpRoute>> = None;
                    let mut http_route: Option<::Value<HttpRoute>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut tcp_route: Option<::Value<TcpRoute>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GrpcRoute" => {
                                grpc_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Http2Route" => {
                                http2_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpRoute" => {
                                http_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TcpRoute" => {
                                tcp_route = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RouteSpec {
                        grpc_route: grpc_route,
                        http2_route: http2_route,
                        http_route: http_route,
                        priority: priority,
                        tcp_route: tcp_route,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.TcpRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcproute.html) property type.
    #[derive(Debug, Default)]
    pub struct TcpRoute {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcproute.html#cfn-appmesh-route-tcproute-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<TcpRouteAction>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcproute.html#cfn-appmesh-route-tcproute-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<TcpTimeout>>,
    }

    impl ::codec::SerializeValue for TcpRoute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TcpRoute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TcpRoute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TcpRoute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TcpRoute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<TcpRouteAction>> = None;
                    let mut timeout: Option<::Value<TcpTimeout>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TcpRoute {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.TcpRouteAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcprouteaction.html) property type.
    #[derive(Debug, Default)]
    pub struct TcpRouteAction {
        /// Property [`WeightedTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcprouteaction.html#cfn-appmesh-route-tcprouteaction-weightedtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weighted_targets: ::ValueList<WeightedTarget>,
    }

    impl ::codec::SerializeValue for TcpRouteAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedTargets", &self.weighted_targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TcpRouteAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TcpRouteAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TcpRouteAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TcpRouteAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut weighted_targets: Option<::ValueList<WeightedTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WeightedTargets" => {
                                weighted_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TcpRouteAction {
                        weighted_targets: weighted_targets.ok_or(::serde::de::Error::missing_field("WeightedTargets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.TcpTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcptimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct TcpTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcptimeout.html#cfn-appmesh-route-tcptimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for TcpTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TcpTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TcpTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TcpTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TcpTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TcpTimeout {
                        idle: idle,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::Route.WeightedTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-weightedtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct WeightedTarget {
        /// Property [`VirtualNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-weightedtarget.html#cfn-appmesh-route-weightedtarget-virtualnode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_node: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-weightedtarget.html#cfn-appmesh-route-weightedtarget-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: ::Value<u32>,
    }

    impl ::codec::SerializeValue for WeightedTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualNode", &self.virtual_node)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", &self.weight)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WeightedTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WeightedTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WeightedTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WeightedTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_node: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualNode" => {
                                virtual_node = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WeightedTarget {
                        virtual_node: virtual_node.ok_or(::serde::de::Error::missing_field("VirtualNode"))?,
                        weight: weight.ok_or(::serde::de::Error::missing_field("Weight"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod virtual_gateway {
    //! Property types for the `VirtualGateway` resource.

    /// The [`AWS::AppMesh::VirtualGateway.SubjectAlternativeNameMatchers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenamematchers.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectAlternativeNameMatchers {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenamematchers.html#cfn-appmesh-virtualgateway-subjectalternativenamematchers-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SubjectAlternativeNameMatchers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectAlternativeNameMatchers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectAlternativeNameMatchers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectAlternativeNameMatchers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectAlternativeNameMatchers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectAlternativeNameMatchers {
                        exact: exact,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenames.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectAlternativeNames {
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenames.html#cfn-appmesh-virtualgateway-subjectalternativenames-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<SubjectAlternativeNameMatchers>,
    }

    impl ::codec::SerializeValue for SubjectAlternativeNames {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectAlternativeNames {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectAlternativeNames, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectAlternativeNames;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectAlternativeNames")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#match: Option<::Value<SubjectAlternativeNameMatchers>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectAlternativeNames {
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayAccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayaccesslog.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayAccessLog {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayaccesslog.html#cfn-appmesh-virtualgateway-virtualgatewayaccesslog-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<VirtualGatewayFileAccessLog>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayAccessLog {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayAccessLog {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayAccessLog, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayAccessLog;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayAccessLog")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<VirtualGatewayFileAccessLog>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayAccessLog {
                        file: file,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayBackendDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaybackenddefaults.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayBackendDefaults {
        /// Property [`ClientPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaybackenddefaults.html#cfn-appmesh-virtualgateway-virtualgatewaybackenddefaults-clientpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_policy: Option<::Value<VirtualGatewayClientPolicy>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayBackendDefaults {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_policy) = self.client_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientPolicy", client_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayBackendDefaults {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayBackendDefaults, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayBackendDefaults;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayBackendDefaults")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_policy: Option<::Value<VirtualGatewayClientPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientPolicy" => {
                                client_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayBackendDefaults {
                        client_policy: client_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayClientPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayClientPolicy {
        /// Property [`TLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayclientpolicy-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<VirtualGatewayClientPolicyTls>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayClientPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLS", tls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayClientPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayClientPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayClientPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayClientPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tls: Option<::Value<VirtualGatewayClientPolicyTls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TLS" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayClientPolicy {
                        tls: tls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayClientPolicyTls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayClientPolicyTls {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html#cfn-appmesh-virtualgateway-virtualgatewayclientpolicytls-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: Option<::Value<VirtualGatewayClientTlsCertificate>>,
        /// Property [`Enforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html#cfn-appmesh-virtualgateway-virtualgatewayclientpolicytls-enforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce: Option<::Value<bool>>,
        /// Property [`Ports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html#cfn-appmesh-virtualgateway-virtualgatewayclientpolicytls-ports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ports: Option<::ValueList<u32>>,
        /// Property [`Validation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html#cfn-appmesh-virtualgateway-virtualgatewayclientpolicytls-validation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation: ::Value<VirtualGatewayTlsValidationContext>,
    }

    impl ::codec::SerializeValue for VirtualGatewayClientPolicyTls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate) = self.certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
            }
            if let Some(ref enforce) = self.enforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enforce", enforce)?;
            }
            if let Some(ref ports) = self.ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ports", ports)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validation", &self.validation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayClientPolicyTls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayClientPolicyTls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayClientPolicyTls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayClientPolicyTls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate: Option<::Value<VirtualGatewayClientTlsCertificate>> = None;
                    let mut enforce: Option<::Value<bool>> = None;
                    let mut ports: Option<::ValueList<u32>> = None;
                    let mut validation: Option<::Value<VirtualGatewayTlsValidationContext>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enforce" => {
                                enforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ports" => {
                                ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Validation" => {
                                validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayClientPolicyTls {
                        certificate: certificate,
                        enforce: enforce,
                        ports: ports,
                        validation: validation.ok_or(::serde::de::Error::missing_field("Validation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayClientTlsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclienttlscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayClientTlsCertificate {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclienttlscertificate.html#cfn-appmesh-virtualgateway-virtualgatewayclienttlscertificate-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<VirtualGatewayListenerTlsFileCertificate>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclienttlscertificate.html#cfn-appmesh-virtualgateway-virtualgatewayclienttlscertificate-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<VirtualGatewayListenerTlsSdsCertificate>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayClientTlsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayClientTlsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayClientTlsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayClientTlsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayClientTlsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<VirtualGatewayListenerTlsFileCertificate>> = None;
                    let mut sds: Option<::Value<VirtualGatewayListenerTlsSdsCertificate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayClientTlsCertificate {
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayConnectionPool {
        /// Property [`GRPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayconnectionpool-grpc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc: Option<::Value<VirtualGatewayGrpcConnectionPool>>,
        /// Property [`HTTP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayconnectionpool-http).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http: Option<::Value<VirtualGatewayHttpConnectionPool>>,
        /// Property [`HTTP2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayconnectionpool-http2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http2: Option<::Value<VirtualGatewayHttp2ConnectionPool>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc) = self.grpc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GRPC", grpc)?;
            }
            if let Some(ref http) = self.http {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP", http)?;
            }
            if let Some(ref http2) = self.http2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP2", http2)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc: Option<::Value<VirtualGatewayGrpcConnectionPool>> = None;
                    let mut http: Option<::Value<VirtualGatewayHttpConnectionPool>> = None;
                    let mut http2: Option<::Value<VirtualGatewayHttp2ConnectionPool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GRPC" => {
                                grpc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP" => {
                                http = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP2" => {
                                http2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayConnectionPool {
                        grpc: grpc,
                        http: http,
                        http2: http2,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayFileAccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayfileaccesslog.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayFileAccessLog {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayfileaccesslog.html#cfn-appmesh-virtualgateway-virtualgatewayfileaccesslog-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayFileAccessLog {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayFileAccessLog {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayFileAccessLog, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayFileAccessLog;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayFileAccessLog")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayFileAccessLog {
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayGrpcConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaygrpcconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayGrpcConnectionPool {
        /// Property [`MaxRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaygrpcconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewaygrpcconnectionpool-maxrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_requests: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualGatewayGrpcConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRequests", &self.max_requests)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayGrpcConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayGrpcConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayGrpcConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayGrpcConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRequests" => {
                                max_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayGrpcConnectionPool {
                        max_requests: max_requests.ok_or(::serde::de::Error::missing_field("MaxRequests"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayHealthCheckPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayHealthCheckPolicy {
        /// Property [`HealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-healthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub healthy_threshold: ::Value<u32>,
        /// Property [`IntervalMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-intervalmillis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_millis: ::Value<u32>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`TimeoutMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-timeoutmillis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_millis: ::Value<u32>,
        /// Property [`UnhealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html#cfn-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy-unhealthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unhealthy_threshold: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualGatewayHealthCheckPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", &self.healthy_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalMillis", &self.interval_millis)?;
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutMillis", &self.timeout_millis)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", &self.unhealthy_threshold)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayHealthCheckPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayHealthCheckPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayHealthCheckPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayHealthCheckPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut healthy_threshold: Option<::Value<u32>> = None;
                    let mut interval_millis: Option<::Value<u32>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut timeout_millis: Option<::Value<u32>> = None;
                    let mut unhealthy_threshold: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalMillis" => {
                                interval_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutMillis" => {
                                timeout_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayHealthCheckPolicy {
                        healthy_threshold: healthy_threshold.ok_or(::serde::de::Error::missing_field("HealthyThreshold"))?,
                        interval_millis: interval_millis.ok_or(::serde::de::Error::missing_field("IntervalMillis"))?,
                        path: path,
                        port: port,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        timeout_millis: timeout_millis.ok_or(::serde::de::Error::missing_field("TimeoutMillis"))?,
                        unhealthy_threshold: unhealthy_threshold.ok_or(::serde::de::Error::missing_field("UnhealthyThreshold"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayHttp2ConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttp2connectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayHttp2ConnectionPool {
        /// Property [`MaxRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttp2connectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayhttp2connectionpool-maxrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_requests: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualGatewayHttp2ConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRequests", &self.max_requests)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayHttp2ConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayHttp2ConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayHttp2ConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayHttp2ConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRequests" => {
                                max_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayHttp2ConnectionPool {
                        max_requests: max_requests.ok_or(::serde::de::Error::missing_field("MaxRequests"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayHttpConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttpconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayHttpConnectionPool {
        /// Property [`MaxConnections`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttpconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayhttpconnectionpool-maxconnections).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_connections: ::Value<u32>,
        /// Property [`MaxPendingRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttpconnectionpool.html#cfn-appmesh-virtualgateway-virtualgatewayhttpconnectionpool-maxpendingrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_pending_requests: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayHttpConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConnections", &self.max_connections)?;
            if let Some(ref max_pending_requests) = self.max_pending_requests {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxPendingRequests", max_pending_requests)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayHttpConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayHttpConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayHttpConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayHttpConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_connections: Option<::Value<u32>> = None;
                    let mut max_pending_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConnections" => {
                                max_connections = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxPendingRequests" => {
                                max_pending_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayHttpConnectionPool {
                        max_connections: max_connections.ok_or(::serde::de::Error::missing_field("MaxConnections"))?,
                        max_pending_requests: max_pending_requests,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListener {
        /// Property [`ConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html#cfn-appmesh-virtualgateway-virtualgatewaylistener-connectionpool).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_pool: Option<::Value<VirtualGatewayConnectionPool>>,
        /// Property [`HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html#cfn-appmesh-virtualgateway-virtualgatewaylistener-healthcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_check: Option<::Value<VirtualGatewayHealthCheckPolicy>>,
        /// Property [`PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html#cfn-appmesh-virtualgateway-virtualgatewaylistener-portmapping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_mapping: ::Value<VirtualGatewayPortMapping>,
        /// Property [`TLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html#cfn-appmesh-virtualgateway-virtualgatewaylistener-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<VirtualGatewayListenerTls>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListener {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_pool) = self.connection_pool {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionPool", connection_pool)?;
            }
            if let Some(ref health_check) = self.health_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheck", health_check)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortMapping", &self.port_mapping)?;
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLS", tls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListener {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListener, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListener;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListener")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_pool: Option<::Value<VirtualGatewayConnectionPool>> = None;
                    let mut health_check: Option<::Value<VirtualGatewayHealthCheckPolicy>> = None;
                    let mut port_mapping: Option<::Value<VirtualGatewayPortMapping>> = None;
                    let mut tls: Option<::Value<VirtualGatewayListenerTls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionPool" => {
                                connection_pool = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthCheck" => {
                                health_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortMapping" => {
                                port_mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TLS" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListener {
                        connection_pool: connection_pool,
                        health_check: health_check,
                        port_mapping: port_mapping.ok_or(::serde::de::Error::missing_field("PortMapping"))?,
                        tls: tls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertls.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTls {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertls.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertls-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: ::Value<VirtualGatewayListenerTlsCertificate>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertls.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertls-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
        /// Property [`Validation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertls.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertls-validation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation: Option<::Value<VirtualGatewayListenerTlsValidationContext>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", &self.certificate)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            if let Some(ref validation) = self.validation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validation", validation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate: Option<::Value<VirtualGatewayListenerTlsCertificate>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut validation: Option<::Value<VirtualGatewayListenerTlsValidationContext>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Validation" => {
                                validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTls {
                        certificate: certificate.ok_or(::serde::de::Error::missing_field("Certificate"))?,
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                        validation: validation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsAcmCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsacmcertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsAcmCertificate {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsacmcertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsacmcertificate-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsAcmCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsAcmCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsAcmCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsAcmCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsAcmCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsAcmCertificate {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsCertificate {
        /// Property [`ACM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlscertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlscertificate-acm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm: Option<::Value<VirtualGatewayListenerTlsAcmCertificate>>,
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlscertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlscertificate-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<VirtualGatewayListenerTlsFileCertificate>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlscertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlscertificate-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<VirtualGatewayListenerTlsSdsCertificate>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm) = self.acm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ACM", acm)?;
            }
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm: Option<::Value<VirtualGatewayListenerTlsAcmCertificate>> = None;
                    let mut file: Option<::Value<VirtualGatewayListenerTlsFileCertificate>> = None;
                    let mut sds: Option<::Value<VirtualGatewayListenerTlsSdsCertificate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ACM" => {
                                acm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsCertificate {
                        acm: acm,
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsFileCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsFileCertificate {
        /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate-certificatechain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_chain: ::Value<String>,
        /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate-privatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsFileCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", &self.certificate_chain)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", &self.private_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsFileCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsFileCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsFileCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsFileCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_chain: Option<::Value<String>> = None;
                    let mut private_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateChain" => {
                                certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKey" => {
                                private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsFileCertificate {
                        certificate_chain: certificate_chain.ok_or(::serde::de::Error::missing_field("CertificateChain"))?,
                        private_key: private_key.ok_or(::serde::de::Error::missing_field("PrivateKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsSdsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlssdscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsSdsCertificate {
        /// Property [`SecretName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlssdscertificate.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlssdscertificate-secretname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsSdsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretName", &self.secret_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsSdsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsSdsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsSdsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsSdsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secret_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretName" => {
                                secret_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsSdsCertificate {
                        secret_name: secret_name.ok_or(::serde::de::Error::missing_field("SecretName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsValidationContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsValidationContext {
        /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext-subjectalternativenames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_alternative_names: Option<::Value<SubjectAlternativeNames>>,
        /// Property [`Trust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext-trust).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trust: ::Value<VirtualGatewayListenerTlsValidationContextTrust>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsValidationContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref subject_alternative_names) = self.subject_alternative_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trust", &self.trust)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsValidationContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsValidationContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsValidationContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsValidationContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subject_alternative_names: Option<::Value<SubjectAlternativeNames>> = None;
                    let mut trust: Option<::Value<VirtualGatewayListenerTlsValidationContextTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubjectAlternativeNames" => {
                                subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trust" => {
                                trust = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsValidationContext {
                        subject_alternative_names: subject_alternative_names,
                        trust: trust.ok_or(::serde::de::Error::missing_field("Trust"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsValidationContextTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayListenerTlsValidationContextTrust {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<VirtualGatewayTlsValidationContextFileTrust>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust.html#cfn-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<VirtualGatewayTlsValidationContextSdsTrust>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayListenerTlsValidationContextTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayListenerTlsValidationContextTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayListenerTlsValidationContextTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayListenerTlsValidationContextTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayListenerTlsValidationContextTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<VirtualGatewayTlsValidationContextFileTrust>> = None;
                    let mut sds: Option<::Value<VirtualGatewayTlsValidationContextSdsTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayListenerTlsValidationContextTrust {
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylogging.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayLogging {
        /// Property [`AccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylogging.html#cfn-appmesh-virtualgateway-virtualgatewaylogging-accesslog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_log: Option<::Value<VirtualGatewayAccessLog>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayLogging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_log) = self.access_log {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLog", access_log)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayLogging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayLogging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayLogging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayLogging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_log: Option<::Value<VirtualGatewayAccessLog>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessLog" => {
                                access_log = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayLogging {
                        access_log: access_log,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayPortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayportmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayPortMapping {
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayportmapping.html#cfn-appmesh-virtualgateway-virtualgatewayportmapping-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayportmapping.html#cfn-appmesh-virtualgateway-virtualgatewayportmapping-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayPortMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayPortMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayPortMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayPortMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayPortMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayPortMapping {
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewaySpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayspec.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewaySpec {
        /// Property [`BackendDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayspec.html#cfn-appmesh-virtualgateway-virtualgatewayspec-backenddefaults).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backend_defaults: Option<::Value<VirtualGatewayBackendDefaults>>,
        /// Property [`Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayspec.html#cfn-appmesh-virtualgateway-virtualgatewayspec-listeners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub listeners: ::ValueList<VirtualGatewayListener>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayspec.html#cfn-appmesh-virtualgateway-virtualgatewayspec-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<::Value<VirtualGatewayLogging>>,
    }

    impl ::codec::SerializeValue for VirtualGatewaySpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref backend_defaults) = self.backend_defaults {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackendDefaults", backend_defaults)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Listeners", &self.listeners)?;
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewaySpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewaySpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewaySpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewaySpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut backend_defaults: Option<::Value<VirtualGatewayBackendDefaults>> = None;
                    let mut listeners: Option<::ValueList<VirtualGatewayListener>> = None;
                    let mut logging: Option<::Value<VirtualGatewayLogging>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackendDefaults" => {
                                backend_defaults = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Listeners" => {
                                listeners = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewaySpec {
                        backend_defaults: backend_defaults,
                        listeners: listeners.ok_or(::serde::de::Error::missing_field("Listeners"))?,
                        logging: logging,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayTlsValidationContext {
        /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext-subjectalternativenames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_alternative_names: Option<::Value<SubjectAlternativeNames>>,
        /// Property [`Trust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext-trust).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trust: ::Value<VirtualGatewayTlsValidationContextTrust>,
    }

    impl ::codec::SerializeValue for VirtualGatewayTlsValidationContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref subject_alternative_names) = self.subject_alternative_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trust", &self.trust)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayTlsValidationContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayTlsValidationContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayTlsValidationContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayTlsValidationContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subject_alternative_names: Option<::Value<SubjectAlternativeNames>> = None;
                    let mut trust: Option<::Value<VirtualGatewayTlsValidationContextTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubjectAlternativeNames" => {
                                subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trust" => {
                                trust = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayTlsValidationContext {
                        subject_alternative_names: subject_alternative_names,
                        trust: trust.ok_or(::serde::de::Error::missing_field("Trust"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextAcmTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextacmtrust.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayTlsValidationContextAcmTrust {
        /// Property [`CertificateAuthorityArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextacmtrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextacmtrust-certificateauthorityarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_authority_arns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayTlsValidationContextAcmTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArns", &self.certificate_authority_arns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayTlsValidationContextAcmTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayTlsValidationContextAcmTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayTlsValidationContextAcmTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayTlsValidationContextAcmTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_authority_arns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateAuthorityArns" => {
                                certificate_authority_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayTlsValidationContextAcmTrust {
                        certificate_authority_arns: certificate_authority_arns.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextFileTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextfiletrust.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayTlsValidationContextFileTrust {
        /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextfiletrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextfiletrust-certificatechain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_chain: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayTlsValidationContextFileTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", &self.certificate_chain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayTlsValidationContextFileTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayTlsValidationContextFileTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayTlsValidationContextFileTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayTlsValidationContextFileTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_chain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateChain" => {
                                certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayTlsValidationContextFileTrust {
                        certificate_chain: certificate_chain.ok_or(::serde::de::Error::missing_field("CertificateChain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextSdsTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextsdstrust.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayTlsValidationContextSdsTrust {
        /// Property [`SecretName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextsdstrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextsdstrust-secretname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualGatewayTlsValidationContextSdsTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretName", &self.secret_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayTlsValidationContextSdsTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayTlsValidationContextSdsTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayTlsValidationContextSdsTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayTlsValidationContextSdsTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secret_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretName" => {
                                secret_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayTlsValidationContextSdsTrust {
                        secret_name: secret_name.ok_or(::serde::de::Error::missing_field("SecretName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualGatewayTlsValidationContextTrust {
        /// Property [`ACM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust-acm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm: Option<::Value<VirtualGatewayTlsValidationContextAcmTrust>>,
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<VirtualGatewayTlsValidationContextFileTrust>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust.html#cfn-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<VirtualGatewayTlsValidationContextSdsTrust>>,
    }

    impl ::codec::SerializeValue for VirtualGatewayTlsValidationContextTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm) = self.acm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ACM", acm)?;
            }
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualGatewayTlsValidationContextTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualGatewayTlsValidationContextTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualGatewayTlsValidationContextTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualGatewayTlsValidationContextTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm: Option<::Value<VirtualGatewayTlsValidationContextAcmTrust>> = None;
                    let mut file: Option<::Value<VirtualGatewayTlsValidationContextFileTrust>> = None;
                    let mut sds: Option<::Value<VirtualGatewayTlsValidationContextSdsTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ACM" => {
                                acm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualGatewayTlsValidationContextTrust {
                        acm: acm,
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod virtual_node {
    //! Property types for the `VirtualNode` resource.

    /// The [`AWS::AppMesh::VirtualNode.AccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-accesslog.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessLog {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-accesslog.html#cfn-appmesh-virtualnode-accesslog-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<FileAccessLog>>,
    }

    impl ::codec::SerializeValue for AccessLog {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessLog {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessLog, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessLog;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessLog")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<FileAccessLog>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessLog {
                        file: file,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.AwsCloudMapInstanceAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapinstanceattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsCloudMapInstanceAttribute {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapinstanceattribute.html#cfn-appmesh-virtualnode-awscloudmapinstanceattribute-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapinstanceattribute.html#cfn-appmesh-virtualnode-awscloudmapinstanceattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for AwsCloudMapInstanceAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsCloudMapInstanceAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsCloudMapInstanceAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsCloudMapInstanceAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsCloudMapInstanceAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsCloudMapInstanceAttribute {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.AwsCloudMapServiceDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapservicediscovery.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsCloudMapServiceDiscovery {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapservicediscovery.html#cfn-appmesh-virtualnode-awscloudmapservicediscovery-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueList<AwsCloudMapInstanceAttribute>>,
        /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapservicediscovery.html#cfn-appmesh-virtualnode-awscloudmapservicediscovery-namespacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_name: ::Value<String>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapservicediscovery.html#cfn-appmesh-virtualnode-awscloudmapservicediscovery-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AwsCloudMapServiceDiscovery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", &self.namespace_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", &self.service_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsCloudMapServiceDiscovery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsCloudMapServiceDiscovery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsCloudMapServiceDiscovery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsCloudMapServiceDiscovery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<AwsCloudMapInstanceAttribute>> = None;
                    let mut namespace_name: Option<::Value<String>> = None;
                    let mut service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceName" => {
                                namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsCloudMapServiceDiscovery {
                        attributes: attributes,
                        namespace_name: namespace_name.ok_or(::serde::de::Error::missing_field("NamespaceName"))?,
                        service_name: service_name.ok_or(::serde::de::Error::missing_field("ServiceName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.Backend`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backend.html) property type.
    #[derive(Debug, Default)]
    pub struct Backend {
        /// Property [`VirtualService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backend.html#cfn-appmesh-virtualnode-backend-virtualservice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_service: Option<::Value<VirtualServiceBackend>>,
    }

    impl ::codec::SerializeValue for Backend {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref virtual_service) = self.virtual_service {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualService", virtual_service)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Backend {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Backend, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Backend;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Backend")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_service: Option<::Value<VirtualServiceBackend>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualService" => {
                                virtual_service = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Backend {
                        virtual_service: virtual_service,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.BackendDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backenddefaults.html) property type.
    #[derive(Debug, Default)]
    pub struct BackendDefaults {
        /// Property [`ClientPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backenddefaults.html#cfn-appmesh-virtualnode-backenddefaults-clientpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_policy: Option<::Value<ClientPolicy>>,
    }

    impl ::codec::SerializeValue for BackendDefaults {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_policy) = self.client_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientPolicy", client_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BackendDefaults {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BackendDefaults, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BackendDefaults;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BackendDefaults")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_policy: Option<::Value<ClientPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientPolicy" => {
                                client_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BackendDefaults {
                        client_policy: client_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ClientPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientPolicy {
        /// Property [`TLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicy.html#cfn-appmesh-virtualnode-clientpolicy-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<ClientPolicyTls>>,
    }

    impl ::codec::SerializeValue for ClientPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLS", tls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tls: Option<::Value<ClientPolicyTls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TLS" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientPolicy {
                        tls: tls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ClientPolicyTls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientPolicyTls {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html#cfn-appmesh-virtualnode-clientpolicytls-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: Option<::Value<ClientTlsCertificate>>,
        /// Property [`Enforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html#cfn-appmesh-virtualnode-clientpolicytls-enforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce: Option<::Value<bool>>,
        /// Property [`Ports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html#cfn-appmesh-virtualnode-clientpolicytls-ports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ports: Option<::ValueList<u32>>,
        /// Property [`Validation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html#cfn-appmesh-virtualnode-clientpolicytls-validation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation: ::Value<TlsValidationContext>,
    }

    impl ::codec::SerializeValue for ClientPolicyTls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate) = self.certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
            }
            if let Some(ref enforce) = self.enforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enforce", enforce)?;
            }
            if let Some(ref ports) = self.ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ports", ports)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validation", &self.validation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientPolicyTls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientPolicyTls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientPolicyTls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientPolicyTls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate: Option<::Value<ClientTlsCertificate>> = None;
                    let mut enforce: Option<::Value<bool>> = None;
                    let mut ports: Option<::ValueList<u32>> = None;
                    let mut validation: Option<::Value<TlsValidationContext>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enforce" => {
                                enforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ports" => {
                                ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Validation" => {
                                validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientPolicyTls {
                        certificate: certificate,
                        enforce: enforce,
                        ports: ports,
                        validation: validation.ok_or(::serde::de::Error::missing_field("Validation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ClientTlsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clienttlscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientTlsCertificate {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clienttlscertificate.html#cfn-appmesh-virtualnode-clienttlscertificate-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<ListenerTlsFileCertificate>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clienttlscertificate.html#cfn-appmesh-virtualnode-clienttlscertificate-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<ListenerTlsSdsCertificate>>,
    }

    impl ::codec::SerializeValue for ClientTlsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientTlsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientTlsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientTlsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientTlsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<ListenerTlsFileCertificate>> = None;
                    let mut sds: Option<::Value<ListenerTlsSdsCertificate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientTlsCertificate {
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.DnsServiceDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-dnsservicediscovery.html) property type.
    #[derive(Debug, Default)]
    pub struct DnsServiceDiscovery {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-dnsservicediscovery.html#cfn-appmesh-virtualnode-dnsservicediscovery-hostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hostname: ::Value<String>,
        /// Property [`ResponseType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-dnsservicediscovery.html#cfn-appmesh-virtualnode-dnsservicediscovery-responsetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DnsServiceDiscovery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", &self.hostname)?;
            if let Some(ref response_type) = self.response_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseType", response_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DnsServiceDiscovery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DnsServiceDiscovery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DnsServiceDiscovery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DnsServiceDiscovery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname: Option<::Value<String>> = None;
                    let mut response_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseType" => {
                                response_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DnsServiceDiscovery {
                        hostname: hostname.ok_or(::serde::de::Error::missing_field("Hostname"))?,
                        response_type: response_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.Duration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-duration.html) property type.
    #[derive(Debug, Default)]
    pub struct Duration {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-duration.html#cfn-appmesh-virtualnode-duration-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-duration.html#cfn-appmesh-virtualnode-duration-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Duration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Duration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Duration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Duration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Duration {
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.FileAccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-fileaccesslog.html) property type.
    #[derive(Debug, Default)]
    pub struct FileAccessLog {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-fileaccesslog.html#cfn-appmesh-virtualnode-fileaccesslog-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
    }

    impl ::codec::SerializeValue for FileAccessLog {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileAccessLog {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileAccessLog, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileAccessLog;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileAccessLog")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileAccessLog {
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.GrpcTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-grpctimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct GrpcTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-grpctimeout.html#cfn-appmesh-virtualnode-grpctimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
        /// Property [`PerRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-grpctimeout.html#cfn-appmesh-virtualnode-grpctimeout-perrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_request: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for GrpcTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            if let Some(ref per_request) = self.per_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRequest", per_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrpcTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrpcTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrpcTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrpcTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;
                    let mut per_request: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRequest" => {
                                per_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrpcTimeout {
                        idle: idle,
                        per_request: per_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheck {
        /// Property [`HealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-healthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub healthy_threshold: ::Value<u32>,
        /// Property [`IntervalMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-intervalmillis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_millis: ::Value<u32>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`TimeoutMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-timeoutmillis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_millis: ::Value<u32>,
        /// Property [`UnhealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html#cfn-appmesh-virtualnode-healthcheck-unhealthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unhealthy_threshold: ::Value<u32>,
    }

    impl ::codec::SerializeValue for HealthCheck {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", &self.healthy_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalMillis", &self.interval_millis)?;
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutMillis", &self.timeout_millis)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", &self.unhealthy_threshold)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheck {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheck, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheck;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheck")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut healthy_threshold: Option<::Value<u32>> = None;
                    let mut interval_millis: Option<::Value<u32>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut timeout_millis: Option<::Value<u32>> = None;
                    let mut unhealthy_threshold: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalMillis" => {
                                interval_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutMillis" => {
                                timeout_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheck {
                        healthy_threshold: healthy_threshold.ok_or(::serde::de::Error::missing_field("HealthyThreshold"))?,
                        interval_millis: interval_millis.ok_or(::serde::de::Error::missing_field("IntervalMillis"))?,
                        path: path,
                        port: port,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        timeout_millis: timeout_millis.ok_or(::serde::de::Error::missing_field("TimeoutMillis"))?,
                        unhealthy_threshold: unhealthy_threshold.ok_or(::serde::de::Error::missing_field("UnhealthyThreshold"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.HttpTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-httptimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-httptimeout.html#cfn-appmesh-virtualnode-httptimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
        /// Property [`PerRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-httptimeout.html#cfn-appmesh-virtualnode-httptimeout-perrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_request: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for HttpTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            if let Some(ref per_request) = self.per_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRequest", per_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;
                    let mut per_request: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRequest" => {
                                per_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpTimeout {
                        idle: idle,
                        per_request: per_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html) property type.
    #[derive(Debug, Default)]
    pub struct Listener {
        /// Property [`ConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-connectionpool).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_pool: Option<::Value<VirtualNodeConnectionPool>>,
        /// Property [`HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-healthcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_check: Option<::Value<HealthCheck>>,
        /// Property [`OutlierDetection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-outlierdetection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outlier_detection: Option<::Value<OutlierDetection>>,
        /// Property [`PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-portmapping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_mapping: ::Value<PortMapping>,
        /// Property [`TLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<ListenerTls>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html#cfn-appmesh-virtualnode-listener-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<ListenerTimeout>>,
    }

    impl ::codec::SerializeValue for Listener {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_pool) = self.connection_pool {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionPool", connection_pool)?;
            }
            if let Some(ref health_check) = self.health_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheck", health_check)?;
            }
            if let Some(ref outlier_detection) = self.outlier_detection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutlierDetection", outlier_detection)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortMapping", &self.port_mapping)?;
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLS", tls)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Listener {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Listener, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Listener;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Listener")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_pool: Option<::Value<VirtualNodeConnectionPool>> = None;
                    let mut health_check: Option<::Value<HealthCheck>> = None;
                    let mut outlier_detection: Option<::Value<OutlierDetection>> = None;
                    let mut port_mapping: Option<::Value<PortMapping>> = None;
                    let mut tls: Option<::Value<ListenerTls>> = None;
                    let mut timeout: Option<::Value<ListenerTimeout>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionPool" => {
                                connection_pool = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthCheck" => {
                                health_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutlierDetection" => {
                                outlier_detection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortMapping" => {
                                port_mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TLS" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Listener {
                        connection_pool: connection_pool,
                        health_check: health_check,
                        outlier_detection: outlier_detection,
                        port_mapping: port_mapping.ok_or(::serde::de::Error::missing_field("PortMapping"))?,
                        tls: tls,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTimeout {
        /// Property [`GRPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html#cfn-appmesh-virtualnode-listenertimeout-grpc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc: Option<::Value<GrpcTimeout>>,
        /// Property [`HTTP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html#cfn-appmesh-virtualnode-listenertimeout-http).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http: Option<::Value<HttpTimeout>>,
        /// Property [`HTTP2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html#cfn-appmesh-virtualnode-listenertimeout-http2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http2: Option<::Value<HttpTimeout>>,
        /// Property [`TCP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html#cfn-appmesh-virtualnode-listenertimeout-tcp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp: Option<::Value<TcpTimeout>>,
    }

    impl ::codec::SerializeValue for ListenerTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc) = self.grpc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GRPC", grpc)?;
            }
            if let Some(ref http) = self.http {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP", http)?;
            }
            if let Some(ref http2) = self.http2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP2", http2)?;
            }
            if let Some(ref tcp) = self.tcp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TCP", tcp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc: Option<::Value<GrpcTimeout>> = None;
                    let mut http: Option<::Value<HttpTimeout>> = None;
                    let mut http2: Option<::Value<HttpTimeout>> = None;
                    let mut tcp: Option<::Value<TcpTimeout>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GRPC" => {
                                grpc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP" => {
                                http = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP2" => {
                                http2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TCP" => {
                                tcp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTimeout {
                        grpc: grpc,
                        http: http,
                        http2: http2,
                        tcp: tcp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertls.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTls {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertls.html#cfn-appmesh-virtualnode-listenertls-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: ::Value<ListenerTlsCertificate>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertls.html#cfn-appmesh-virtualnode-listenertls-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
        /// Property [`Validation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertls.html#cfn-appmesh-virtualnode-listenertls-validation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation: Option<::Value<ListenerTlsValidationContext>>,
    }

    impl ::codec::SerializeValue for ListenerTls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", &self.certificate)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            if let Some(ref validation) = self.validation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validation", validation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate: Option<::Value<ListenerTlsCertificate>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut validation: Option<::Value<ListenerTlsValidationContext>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Validation" => {
                                validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTls {
                        certificate: certificate.ok_or(::serde::de::Error::missing_field("Certificate"))?,
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                        validation: validation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsAcmCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsacmcertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsAcmCertificate {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsacmcertificate.html#cfn-appmesh-virtualnode-listenertlsacmcertificate-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ListenerTlsAcmCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsAcmCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsAcmCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsAcmCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsAcmCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsAcmCertificate {
                        certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsCertificate {
        /// Property [`ACM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlscertificate.html#cfn-appmesh-virtualnode-listenertlscertificate-acm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm: Option<::Value<ListenerTlsAcmCertificate>>,
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlscertificate.html#cfn-appmesh-virtualnode-listenertlscertificate-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<ListenerTlsFileCertificate>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlscertificate.html#cfn-appmesh-virtualnode-listenertlscertificate-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<ListenerTlsSdsCertificate>>,
    }

    impl ::codec::SerializeValue for ListenerTlsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm) = self.acm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ACM", acm)?;
            }
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm: Option<::Value<ListenerTlsAcmCertificate>> = None;
                    let mut file: Option<::Value<ListenerTlsFileCertificate>> = None;
                    let mut sds: Option<::Value<ListenerTlsSdsCertificate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ACM" => {
                                acm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsCertificate {
                        acm: acm,
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsFileCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsfilecertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsFileCertificate {
        /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsfilecertificate.html#cfn-appmesh-virtualnode-listenertlsfilecertificate-certificatechain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_chain: ::Value<String>,
        /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsfilecertificate.html#cfn-appmesh-virtualnode-listenertlsfilecertificate-privatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for ListenerTlsFileCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", &self.certificate_chain)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", &self.private_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsFileCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsFileCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsFileCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsFileCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_chain: Option<::Value<String>> = None;
                    let mut private_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateChain" => {
                                certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKey" => {
                                private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsFileCertificate {
                        certificate_chain: certificate_chain.ok_or(::serde::de::Error::missing_field("CertificateChain"))?,
                        private_key: private_key.ok_or(::serde::de::Error::missing_field("PrivateKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsSdsCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlssdscertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsSdsCertificate {
        /// Property [`SecretName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlssdscertificate.html#cfn-appmesh-virtualnode-listenertlssdscertificate-secretname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ListenerTlsSdsCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretName", &self.secret_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsSdsCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsSdsCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsSdsCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsSdsCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secret_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretName" => {
                                secret_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsSdsCertificate {
                        secret_name: secret_name.ok_or(::serde::de::Error::missing_field("SecretName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsValidationContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsValidationContext {
        /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontext.html#cfn-appmesh-virtualnode-listenertlsvalidationcontext-subjectalternativenames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_alternative_names: Option<::Value<SubjectAlternativeNames>>,
        /// Property [`Trust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontext.html#cfn-appmesh-virtualnode-listenertlsvalidationcontext-trust).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trust: ::Value<ListenerTlsValidationContextTrust>,
    }

    impl ::codec::SerializeValue for ListenerTlsValidationContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref subject_alternative_names) = self.subject_alternative_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trust", &self.trust)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsValidationContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsValidationContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsValidationContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsValidationContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subject_alternative_names: Option<::Value<SubjectAlternativeNames>> = None;
                    let mut trust: Option<::Value<ListenerTlsValidationContextTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubjectAlternativeNames" => {
                                subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trust" => {
                                trust = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsValidationContext {
                        subject_alternative_names: subject_alternative_names,
                        trust: trust.ok_or(::serde::de::Error::missing_field("Trust"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ListenerTlsValidationContextTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontexttrust.html) property type.
    #[derive(Debug, Default)]
    pub struct ListenerTlsValidationContextTrust {
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontexttrust.html#cfn-appmesh-virtualnode-listenertlsvalidationcontexttrust-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<TlsValidationContextFileTrust>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontexttrust.html#cfn-appmesh-virtualnode-listenertlsvalidationcontexttrust-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<TlsValidationContextSdsTrust>>,
    }

    impl ::codec::SerializeValue for ListenerTlsValidationContextTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ListenerTlsValidationContextTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerTlsValidationContextTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ListenerTlsValidationContextTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ListenerTlsValidationContextTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file: Option<::Value<TlsValidationContextFileTrust>> = None;
                    let mut sds: Option<::Value<TlsValidationContextSdsTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ListenerTlsValidationContextTrust {
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`AccessLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-logging.html#cfn-appmesh-virtualnode-logging-accesslog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_log: Option<::Value<AccessLog>>,
    }

    impl ::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_log) = self.access_log {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLog", access_log)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_log: Option<::Value<AccessLog>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessLog" => {
                                access_log = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        access_log: access_log,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.OutlierDetection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html) property type.
    #[derive(Debug, Default)]
    pub struct OutlierDetection {
        /// Property [`BaseEjectionDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html#cfn-appmesh-virtualnode-outlierdetection-baseejectionduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_ejection_duration: ::Value<Duration>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html#cfn-appmesh-virtualnode-outlierdetection-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: ::Value<Duration>,
        /// Property [`MaxEjectionPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html#cfn-appmesh-virtualnode-outlierdetection-maxejectionpercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ejection_percent: ::Value<u32>,
        /// Property [`MaxServerErrors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html#cfn-appmesh-virtualnode-outlierdetection-maxservererrors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_server_errors: ::Value<u32>,
    }

    impl ::codec::SerializeValue for OutlierDetection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseEjectionDuration", &self.base_ejection_duration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxEjectionPercent", &self.max_ejection_percent)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxServerErrors", &self.max_server_errors)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutlierDetection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutlierDetection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutlierDetection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutlierDetection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_ejection_duration: Option<::Value<Duration>> = None;
                    let mut interval: Option<::Value<Duration>> = None;
                    let mut max_ejection_percent: Option<::Value<u32>> = None;
                    let mut max_server_errors: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseEjectionDuration" => {
                                base_ejection_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxEjectionPercent" => {
                                max_ejection_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxServerErrors" => {
                                max_server_errors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutlierDetection {
                        base_ejection_duration: base_ejection_duration.ok_or(::serde::de::Error::missing_field("BaseEjectionDuration"))?,
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        max_ejection_percent: max_ejection_percent.ok_or(::serde::de::Error::missing_field("MaxEjectionPercent"))?,
                        max_server_errors: max_server_errors.ok_or(::serde::de::Error::missing_field("MaxServerErrors"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-portmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct PortMapping {
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-portmapping.html#cfn-appmesh-virtualnode-portmapping-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-portmapping.html#cfn-appmesh-virtualnode-portmapping-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for PortMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortMapping {
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.ServiceDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-servicediscovery.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceDiscovery {
        /// Property [`AWSCloudMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-servicediscovery.html#cfn-appmesh-virtualnode-servicediscovery-awscloudmap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_cloud_map: Option<::Value<AwsCloudMapServiceDiscovery>>,
        /// Property [`DNS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-servicediscovery.html#cfn-appmesh-virtualnode-servicediscovery-dns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns: Option<::Value<DnsServiceDiscovery>>,
    }

    impl ::codec::SerializeValue for ServiceDiscovery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_cloud_map) = self.aws_cloud_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSCloudMap", aws_cloud_map)?;
            }
            if let Some(ref dns) = self.dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNS", dns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceDiscovery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceDiscovery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceDiscovery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceDiscovery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_cloud_map: Option<::Value<AwsCloudMapServiceDiscovery>> = None;
                    let mut dns: Option<::Value<DnsServiceDiscovery>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AWSCloudMap" => {
                                aws_cloud_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DNS" => {
                                dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceDiscovery {
                        aws_cloud_map: aws_cloud_map,
                        dns: dns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.SubjectAlternativeNameMatchers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenamematchers.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectAlternativeNameMatchers {
        /// Property [`Exact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenamematchers.html#cfn-appmesh-virtualnode-subjectalternativenamematchers-exact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exact: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SubjectAlternativeNameMatchers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exact) = self.exact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exact", exact)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectAlternativeNameMatchers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectAlternativeNameMatchers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectAlternativeNameMatchers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectAlternativeNameMatchers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exact: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exact" => {
                                exact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectAlternativeNameMatchers {
                        exact: exact,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenames.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectAlternativeNames {
        /// Property [`Match`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenames.html#cfn-appmesh-virtualnode-subjectalternativenames-match).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#match: ::Value<SubjectAlternativeNameMatchers>,
    }

    impl ::codec::SerializeValue for SubjectAlternativeNames {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Match", &self.r#match)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectAlternativeNames {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectAlternativeNames, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectAlternativeNames;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectAlternativeNames")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#match: Option<::Value<SubjectAlternativeNameMatchers>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Match" => {
                                r#match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectAlternativeNames {
                        r#match: r#match.ok_or(::serde::de::Error::missing_field("Match"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TcpTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tcptimeout.html) property type.
    #[derive(Debug, Default)]
    pub struct TcpTimeout {
        /// Property [`Idle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tcptimeout.html#cfn-appmesh-virtualnode-tcptimeout-idle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle: Option<::Value<Duration>>,
    }

    impl ::codec::SerializeValue for TcpTimeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle) = self.idle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Idle", idle)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TcpTimeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TcpTimeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TcpTimeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TcpTimeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle: Option<::Value<Duration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Idle" => {
                                idle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TcpTimeout {
                        idle: idle,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TlsValidationContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsValidationContext {
        /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontext.html#cfn-appmesh-virtualnode-tlsvalidationcontext-subjectalternativenames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_alternative_names: Option<::Value<SubjectAlternativeNames>>,
        /// Property [`Trust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontext.html#cfn-appmesh-virtualnode-tlsvalidationcontext-trust).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trust: ::Value<TlsValidationContextTrust>,
    }

    impl ::codec::SerializeValue for TlsValidationContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref subject_alternative_names) = self.subject_alternative_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trust", &self.trust)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsValidationContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsValidationContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsValidationContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsValidationContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subject_alternative_names: Option<::Value<SubjectAlternativeNames>> = None;
                    let mut trust: Option<::Value<TlsValidationContextTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubjectAlternativeNames" => {
                                subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trust" => {
                                trust = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsValidationContext {
                        subject_alternative_names: subject_alternative_names,
                        trust: trust.ok_or(::serde::de::Error::missing_field("Trust"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TlsValidationContextAcmTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextacmtrust.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsValidationContextAcmTrust {
        /// Property [`CertificateAuthorityArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextacmtrust.html#cfn-appmesh-virtualnode-tlsvalidationcontextacmtrust-certificateauthorityarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_authority_arns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for TlsValidationContextAcmTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArns", &self.certificate_authority_arns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsValidationContextAcmTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsValidationContextAcmTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsValidationContextAcmTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsValidationContextAcmTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_authority_arns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateAuthorityArns" => {
                                certificate_authority_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsValidationContextAcmTrust {
                        certificate_authority_arns: certificate_authority_arns.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TlsValidationContextFileTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextfiletrust.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsValidationContextFileTrust {
        /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextfiletrust.html#cfn-appmesh-virtualnode-tlsvalidationcontextfiletrust-certificatechain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_chain: ::Value<String>,
    }

    impl ::codec::SerializeValue for TlsValidationContextFileTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", &self.certificate_chain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsValidationContextFileTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsValidationContextFileTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsValidationContextFileTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsValidationContextFileTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_chain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateChain" => {
                                certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsValidationContextFileTrust {
                        certificate_chain: certificate_chain.ok_or(::serde::de::Error::missing_field("CertificateChain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TlsValidationContextSdsTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextsdstrust.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsValidationContextSdsTrust {
        /// Property [`SecretName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextsdstrust.html#cfn-appmesh-virtualnode-tlsvalidationcontextsdstrust-secretname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for TlsValidationContextSdsTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretName", &self.secret_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsValidationContextSdsTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsValidationContextSdsTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsValidationContextSdsTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsValidationContextSdsTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secret_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretName" => {
                                secret_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsValidationContextSdsTrust {
                        secret_name: secret_name.ok_or(::serde::de::Error::missing_field("SecretName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.TlsValidationContextTrust`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontexttrust.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsValidationContextTrust {
        /// Property [`ACM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontexttrust.html#cfn-appmesh-virtualnode-tlsvalidationcontexttrust-acm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm: Option<::Value<TlsValidationContextAcmTrust>>,
        /// Property [`File`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontexttrust.html#cfn-appmesh-virtualnode-tlsvalidationcontexttrust-file).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file: Option<::Value<TlsValidationContextFileTrust>>,
        /// Property [`SDS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontexttrust.html#cfn-appmesh-virtualnode-tlsvalidationcontexttrust-sds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sds: Option<::Value<TlsValidationContextSdsTrust>>,
    }

    impl ::codec::SerializeValue for TlsValidationContextTrust {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm) = self.acm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ACM", acm)?;
            }
            if let Some(ref file) = self.file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "File", file)?;
            }
            if let Some(ref sds) = self.sds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SDS", sds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsValidationContextTrust {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsValidationContextTrust, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsValidationContextTrust;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsValidationContextTrust")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm: Option<::Value<TlsValidationContextAcmTrust>> = None;
                    let mut file: Option<::Value<TlsValidationContextFileTrust>> = None;
                    let mut sds: Option<::Value<TlsValidationContextSdsTrust>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ACM" => {
                                acm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "File" => {
                                file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SDS" => {
                                sds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsValidationContextTrust {
                        acm: acm,
                        file: file,
                        sds: sds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeConnectionPool {
        /// Property [`GRPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html#cfn-appmesh-virtualnode-virtualnodeconnectionpool-grpc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc: Option<::Value<VirtualNodeGrpcConnectionPool>>,
        /// Property [`HTTP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html#cfn-appmesh-virtualnode-virtualnodeconnectionpool-http).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http: Option<::Value<VirtualNodeHttpConnectionPool>>,
        /// Property [`HTTP2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html#cfn-appmesh-virtualnode-virtualnodeconnectionpool-http2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http2: Option<::Value<VirtualNodeHttp2ConnectionPool>>,
        /// Property [`TCP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html#cfn-appmesh-virtualnode-virtualnodeconnectionpool-tcp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp: Option<::Value<VirtualNodeTcpConnectionPool>>,
    }

    impl ::codec::SerializeValue for VirtualNodeConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc) = self.grpc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GRPC", grpc)?;
            }
            if let Some(ref http) = self.http {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP", http)?;
            }
            if let Some(ref http2) = self.http2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTP2", http2)?;
            }
            if let Some(ref tcp) = self.tcp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TCP", tcp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc: Option<::Value<VirtualNodeGrpcConnectionPool>> = None;
                    let mut http: Option<::Value<VirtualNodeHttpConnectionPool>> = None;
                    let mut http2: Option<::Value<VirtualNodeHttp2ConnectionPool>> = None;
                    let mut tcp: Option<::Value<VirtualNodeTcpConnectionPool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GRPC" => {
                                grpc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP" => {
                                http = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTP2" => {
                                http2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TCP" => {
                                tcp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeConnectionPool {
                        grpc: grpc,
                        http: http,
                        http2: http2,
                        tcp: tcp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeGrpcConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodegrpcconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeGrpcConnectionPool {
        /// Property [`MaxRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodegrpcconnectionpool.html#cfn-appmesh-virtualnode-virtualnodegrpcconnectionpool-maxrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_requests: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualNodeGrpcConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRequests", &self.max_requests)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeGrpcConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeGrpcConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeGrpcConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeGrpcConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRequests" => {
                                max_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeGrpcConnectionPool {
                        max_requests: max_requests.ok_or(::serde::de::Error::missing_field("MaxRequests"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeHttp2ConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttp2connectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeHttp2ConnectionPool {
        /// Property [`MaxRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttp2connectionpool.html#cfn-appmesh-virtualnode-virtualnodehttp2connectionpool-maxrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_requests: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualNodeHttp2ConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRequests", &self.max_requests)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeHttp2ConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeHttp2ConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeHttp2ConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeHttp2ConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRequests" => {
                                max_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeHttp2ConnectionPool {
                        max_requests: max_requests.ok_or(::serde::de::Error::missing_field("MaxRequests"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeHttpConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttpconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeHttpConnectionPool {
        /// Property [`MaxConnections`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttpconnectionpool.html#cfn-appmesh-virtualnode-virtualnodehttpconnectionpool-maxconnections).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_connections: ::Value<u32>,
        /// Property [`MaxPendingRequests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttpconnectionpool.html#cfn-appmesh-virtualnode-virtualnodehttpconnectionpool-maxpendingrequests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_pending_requests: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VirtualNodeHttpConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConnections", &self.max_connections)?;
            if let Some(ref max_pending_requests) = self.max_pending_requests {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxPendingRequests", max_pending_requests)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeHttpConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeHttpConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeHttpConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeHttpConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_connections: Option<::Value<u32>> = None;
                    let mut max_pending_requests: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConnections" => {
                                max_connections = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxPendingRequests" => {
                                max_pending_requests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeHttpConnectionPool {
                        max_connections: max_connections.ok_or(::serde::de::Error::missing_field("MaxConnections"))?,
                        max_pending_requests: max_pending_requests,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeSpec {
        /// Property [`BackendDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html#cfn-appmesh-virtualnode-virtualnodespec-backenddefaults).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backend_defaults: Option<::Value<BackendDefaults>>,
        /// Property [`Backends`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html#cfn-appmesh-virtualnode-virtualnodespec-backends).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backends: Option<::ValueList<Backend>>,
        /// Property [`Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html#cfn-appmesh-virtualnode-virtualnodespec-listeners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub listeners: Option<::ValueList<Listener>>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html#cfn-appmesh-virtualnode-virtualnodespec-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<::Value<Logging>>,
        /// Property [`ServiceDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html#cfn-appmesh-virtualnode-virtualnodespec-servicediscovery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_discovery: Option<::Value<ServiceDiscovery>>,
    }

    impl ::codec::SerializeValue for VirtualNodeSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref backend_defaults) = self.backend_defaults {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackendDefaults", backend_defaults)?;
            }
            if let Some(ref backends) = self.backends {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Backends", backends)?;
            }
            if let Some(ref listeners) = self.listeners {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Listeners", listeners)?;
            }
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            if let Some(ref service_discovery) = self.service_discovery {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceDiscovery", service_discovery)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut backend_defaults: Option<::Value<BackendDefaults>> = None;
                    let mut backends: Option<::ValueList<Backend>> = None;
                    let mut listeners: Option<::ValueList<Listener>> = None;
                    let mut logging: Option<::Value<Logging>> = None;
                    let mut service_discovery: Option<::Value<ServiceDiscovery>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackendDefaults" => {
                                backend_defaults = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Backends" => {
                                backends = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Listeners" => {
                                listeners = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceDiscovery" => {
                                service_discovery = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeSpec {
                        backend_defaults: backend_defaults,
                        backends: backends,
                        listeners: listeners,
                        logging: logging,
                        service_discovery: service_discovery,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualNodeTcpConnectionPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodetcpconnectionpool.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeTcpConnectionPool {
        /// Property [`MaxConnections`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodetcpconnectionpool.html#cfn-appmesh-virtualnode-virtualnodetcpconnectionpool-maxconnections).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_connections: ::Value<u32>,
    }

    impl ::codec::SerializeValue for VirtualNodeTcpConnectionPool {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConnections", &self.max_connections)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeTcpConnectionPool {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeTcpConnectionPool, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeTcpConnectionPool;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeTcpConnectionPool")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_connections: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConnections" => {
                                max_connections = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeTcpConnectionPool {
                        max_connections: max_connections.ok_or(::serde::de::Error::missing_field("MaxConnections"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualNode.VirtualServiceBackend`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualservicebackend.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualServiceBackend {
        /// Property [`ClientPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualservicebackend.html#cfn-appmesh-virtualnode-virtualservicebackend-clientpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_policy: Option<::Value<ClientPolicy>>,
        /// Property [`VirtualServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualservicebackend.html#cfn-appmesh-virtualnode-virtualservicebackend-virtualservicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_service_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualServiceBackend {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_policy) = self.client_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientPolicy", client_policy)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualServiceName", &self.virtual_service_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualServiceBackend {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualServiceBackend, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualServiceBackend;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualServiceBackend")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_policy: Option<::Value<ClientPolicy>> = None;
                    let mut virtual_service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientPolicy" => {
                                client_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualServiceName" => {
                                virtual_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualServiceBackend {
                        client_policy: client_policy,
                        virtual_service_name: virtual_service_name.ok_or(::serde::de::Error::missing_field("VirtualServiceName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod virtual_router {
    //! Property types for the `VirtualRouter` resource.

    /// The [`AWS::AppMesh::VirtualRouter.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct PortMapping {
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for PortMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortMapping {
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualRouter.VirtualRouterListener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterlistener.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualRouterListener {
        /// Property [`PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterlistener.html#cfn-appmesh-virtualrouter-virtualrouterlistener-portmapping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_mapping: ::Value<PortMapping>,
    }

    impl ::codec::SerializeValue for VirtualRouterListener {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortMapping", &self.port_mapping)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualRouterListener {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualRouterListener, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualRouterListener;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualRouterListener")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port_mapping: Option<::Value<PortMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PortMapping" => {
                                port_mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualRouterListener {
                        port_mapping: port_mapping.ok_or(::serde::de::Error::missing_field("PortMapping"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualRouter.VirtualRouterSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterspec.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualRouterSpec {
        /// Property [`Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterspec.html#cfn-appmesh-virtualrouter-virtualrouterspec-listeners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub listeners: ::ValueList<VirtualRouterListener>,
    }

    impl ::codec::SerializeValue for VirtualRouterSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Listeners", &self.listeners)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualRouterSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualRouterSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualRouterSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualRouterSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut listeners: Option<::ValueList<VirtualRouterListener>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Listeners" => {
                                listeners = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualRouterSpec {
                        listeners: listeners.ok_or(::serde::de::Error::missing_field("Listeners"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod virtual_service {
    //! Property types for the `VirtualService` resource.

    /// The [`AWS::AppMesh::VirtualService.VirtualNodeServiceProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualnodeserviceprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualNodeServiceProvider {
        /// Property [`VirtualNodeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualnodeserviceprovider.html#cfn-appmesh-virtualservice-virtualnodeserviceprovider-virtualnodename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_node_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualNodeServiceProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualNodeName", &self.virtual_node_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualNodeServiceProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualNodeServiceProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualNodeServiceProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualNodeServiceProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_node_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualNodeName" => {
                                virtual_node_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualNodeServiceProvider {
                        virtual_node_name: virtual_node_name.ok_or(::serde::de::Error::missing_field("VirtualNodeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualService.VirtualRouterServiceProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualrouterserviceprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualRouterServiceProvider {
        /// Property [`VirtualRouterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualrouterserviceprovider.html#cfn-appmesh-virtualservice-virtualrouterserviceprovider-virtualroutername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_router_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VirtualRouterServiceProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualRouterName", &self.virtual_router_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualRouterServiceProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualRouterServiceProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualRouterServiceProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualRouterServiceProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_router_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualRouterName" => {
                                virtual_router_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualRouterServiceProvider {
                        virtual_router_name: virtual_router_name.ok_or(::serde::de::Error::missing_field("VirtualRouterName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualService.VirtualServiceProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualserviceprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualServiceProvider {
        /// Property [`VirtualNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualserviceprovider.html#cfn-appmesh-virtualservice-virtualserviceprovider-virtualnode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_node: Option<::Value<VirtualNodeServiceProvider>>,
        /// Property [`VirtualRouter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualserviceprovider.html#cfn-appmesh-virtualservice-virtualserviceprovider-virtualrouter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_router: Option<::Value<VirtualRouterServiceProvider>>,
    }

    impl ::codec::SerializeValue for VirtualServiceProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref virtual_node) = self.virtual_node {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualNode", virtual_node)?;
            }
            if let Some(ref virtual_router) = self.virtual_router {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualRouter", virtual_router)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualServiceProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualServiceProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualServiceProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualServiceProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut virtual_node: Option<::Value<VirtualNodeServiceProvider>> = None;
                    let mut virtual_router: Option<::Value<VirtualRouterServiceProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VirtualNode" => {
                                virtual_node = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualRouter" => {
                                virtual_router = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualServiceProvider {
                        virtual_node: virtual_node,
                        virtual_router: virtual_router,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppMesh::VirtualService.VirtualServiceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualservicespec.html) property type.
    #[derive(Debug, Default)]
    pub struct VirtualServiceSpec {
        /// Property [`Provider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualservicespec.html#cfn-appmesh-virtualservice-virtualservicespec-provider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider: Option<::Value<VirtualServiceProvider>>,
    }

    impl ::codec::SerializeValue for VirtualServiceSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref provider) = self.provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", provider)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VirtualServiceSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VirtualServiceSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VirtualServiceSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VirtualServiceSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provider: Option<::Value<VirtualServiceProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Provider" => {
                                provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VirtualServiceSpec {
                        provider: provider,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
