//! Types for the `IoTSiteWise` service.

/// The [`AWS::IoTSiteWise::AccessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-accesspolicy.html) resource type.
#[derive(Debug, Default)]
pub struct AccessPolicy {
    properties: AccessPolicyProperties
}

/// Properties for the `AccessPolicy` resource.
#[derive(Debug, Default)]
pub struct AccessPolicyProperties {
    /// Property [`AccessPolicyIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-accesspolicy.html#cfn-iotsitewise-accesspolicy-accesspolicyidentity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policy_identity: ::Value<self::access_policy::AccessPolicyIdentity>,
    /// Property [`AccessPolicyPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-accesspolicy.html#cfn-iotsitewise-accesspolicy-accesspolicypermission).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policy_permission: ::Value<String>,
    /// Property [`AccessPolicyResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-accesspolicy.html#cfn-iotsitewise-accesspolicy-accesspolicyresource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policy_resource: ::Value<self::access_policy::AccessPolicyResource>,
}

impl ::serde::Serialize for AccessPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicyIdentity", &self.access_policy_identity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicyPermission", &self.access_policy_permission)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicyResource", &self.access_policy_resource)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_policy_identity: Option<::Value<self::access_policy::AccessPolicyIdentity>> = None;
                let mut access_policy_permission: Option<::Value<String>> = None;
                let mut access_policy_resource: Option<::Value<self::access_policy::AccessPolicyResource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessPolicyIdentity" => {
                            access_policy_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccessPolicyPermission" => {
                            access_policy_permission = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccessPolicyResource" => {
                            access_policy_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccessPolicyProperties {
                    access_policy_identity: access_policy_identity.ok_or(::serde::de::Error::missing_field("AccessPolicyIdentity"))?,
                    access_policy_permission: access_policy_permission.ok_or(::serde::de::Error::missing_field("AccessPolicyPermission"))?,
                    access_policy_resource: access_policy_resource.ok_or(::serde::de::Error::missing_field("AccessPolicyResource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessPolicy {
    type Properties = AccessPolicyProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::AccessPolicy";
    fn properties(&self) -> &AccessPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessPolicy {}

impl From<AccessPolicyProperties> for AccessPolicy {
    fn from(properties: AccessPolicyProperties) -> AccessPolicy {
        AccessPolicy { properties }
    }
}

/// The [`AWS::IoTSiteWise::Asset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html) resource type.
#[derive(Debug, Default)]
pub struct Asset {
    properties: AssetProperties
}

/// Properties for the `Asset` resource.
#[derive(Debug, Default)]
pub struct AssetProperties {
    /// Property [`AssetHierarchies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html#cfn-iotsitewise-asset-assethierarchies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_hierarchies: Option<::ValueList<self::asset::AssetHierarchy>>,
    /// Property [`AssetModelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html#cfn-iotsitewise-asset-assetmodelid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_id: ::Value<String>,
    /// Property [`AssetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html#cfn-iotsitewise-asset-assetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_name: ::Value<String>,
    /// Property [`AssetProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html#cfn-iotsitewise-asset-assetproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_properties: Option<::ValueList<self::asset::AssetProperty>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html#cfn-iotsitewise-asset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref asset_hierarchies) = self.asset_hierarchies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetHierarchies", asset_hierarchies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelId", &self.asset_model_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetName", &self.asset_name)?;
        if let Some(ref asset_properties) = self.asset_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetProperties", asset_properties)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut asset_hierarchies: Option<::ValueList<self::asset::AssetHierarchy>> = None;
                let mut asset_model_id: Option<::Value<String>> = None;
                let mut asset_name: Option<::Value<String>> = None;
                let mut asset_properties: Option<::ValueList<self::asset::AssetProperty>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssetHierarchies" => {
                            asset_hierarchies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetModelId" => {
                            asset_model_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetName" => {
                            asset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetProperties" => {
                            asset_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssetProperties {
                    asset_hierarchies: asset_hierarchies,
                    asset_model_id: asset_model_id.ok_or(::serde::de::Error::missing_field("AssetModelId"))?,
                    asset_name: asset_name.ok_or(::serde::de::Error::missing_field("AssetName"))?,
                    asset_properties: asset_properties,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Asset {
    type Properties = AssetProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::Asset";
    fn properties(&self) -> &AssetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Asset {}

impl From<AssetProperties> for Asset {
    fn from(properties: AssetProperties) -> Asset {
        Asset { properties }
    }
}

/// The [`AWS::IoTSiteWise::AssetModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html) resource type.
#[derive(Debug, Default)]
pub struct AssetModel {
    properties: AssetModelProperties
}

/// Properties for the `AssetModel` resource.
#[derive(Debug, Default)]
pub struct AssetModelProperties {
    /// Property [`AssetModelCompositeModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-assetmodelcompositemodels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_composite_models: Option<::ValueList<self::asset_model::AssetModelCompositeModel>>,
    /// Property [`AssetModelDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-assetmodeldescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_description: Option<::Value<String>>,
    /// Property [`AssetModelHierarchies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-assetmodelhierarchies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_hierarchies: Option<::ValueList<self::asset_model::AssetModelHierarchy>>,
    /// Property [`AssetModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-assetmodelname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_name: ::Value<String>,
    /// Property [`AssetModelProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-assetmodelproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_model_properties: Option<::ValueList<self::asset_model::AssetModelProperty>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html#cfn-iotsitewise-assetmodel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssetModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref asset_model_composite_models) = self.asset_model_composite_models {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelCompositeModels", asset_model_composite_models)?;
        }
        if let Some(ref asset_model_description) = self.asset_model_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelDescription", asset_model_description)?;
        }
        if let Some(ref asset_model_hierarchies) = self.asset_model_hierarchies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelHierarchies", asset_model_hierarchies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelName", &self.asset_model_name)?;
        if let Some(ref asset_model_properties) = self.asset_model_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetModelProperties", asset_model_properties)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetModelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetModelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssetModelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssetModelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut asset_model_composite_models: Option<::ValueList<self::asset_model::AssetModelCompositeModel>> = None;
                let mut asset_model_description: Option<::Value<String>> = None;
                let mut asset_model_hierarchies: Option<::ValueList<self::asset_model::AssetModelHierarchy>> = None;
                let mut asset_model_name: Option<::Value<String>> = None;
                let mut asset_model_properties: Option<::ValueList<self::asset_model::AssetModelProperty>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssetModelCompositeModels" => {
                            asset_model_composite_models = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetModelDescription" => {
                            asset_model_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetModelHierarchies" => {
                            asset_model_hierarchies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetModelName" => {
                            asset_model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssetModelProperties" => {
                            asset_model_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssetModelProperties {
                    asset_model_composite_models: asset_model_composite_models,
                    asset_model_description: asset_model_description,
                    asset_model_hierarchies: asset_model_hierarchies,
                    asset_model_name: asset_model_name.ok_or(::serde::de::Error::missing_field("AssetModelName"))?,
                    asset_model_properties: asset_model_properties,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AssetModel {
    type Properties = AssetModelProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::AssetModel";
    fn properties(&self) -> &AssetModelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssetModelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AssetModel {}

impl From<AssetModelProperties> for AssetModel {
    fn from(properties: AssetModelProperties) -> AssetModel {
        AssetModel { properties }
    }
}

/// The [`AWS::IoTSiteWise::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html) resource type.
#[derive(Debug, Default)]
pub struct Dashboard {
    properties: DashboardProperties
}

/// Properties for the `Dashboard` resource.
#[derive(Debug, Default)]
pub struct DashboardProperties {
    /// Property [`DashboardDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html#cfn-iotsitewise-dashboard-dashboarddefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_definition: ::Value<String>,
    /// Property [`DashboardDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html#cfn-iotsitewise-dashboard-dashboarddescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_description: ::Value<String>,
    /// Property [`DashboardName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html#cfn-iotsitewise-dashboard-dashboardname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_name: ::Value<String>,
    /// Property [`ProjectId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html#cfn-iotsitewise-dashboard-projectid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html#cfn-iotsitewise-dashboard-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DashboardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardDefinition", &self.dashboard_definition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardDescription", &self.dashboard_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardName", &self.dashboard_name)?;
        if let Some(ref project_id) = self.project_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectId", project_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DashboardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DashboardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DashboardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dashboard_definition: Option<::Value<String>> = None;
                let mut dashboard_description: Option<::Value<String>> = None;
                let mut dashboard_name: Option<::Value<String>> = None;
                let mut project_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DashboardDefinition" => {
                            dashboard_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardDescription" => {
                            dashboard_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardName" => {
                            dashboard_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectId" => {
                            project_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DashboardProperties {
                    dashboard_definition: dashboard_definition.ok_or(::serde::de::Error::missing_field("DashboardDefinition"))?,
                    dashboard_description: dashboard_description.ok_or(::serde::de::Error::missing_field("DashboardDescription"))?,
                    dashboard_name: dashboard_name.ok_or(::serde::de::Error::missing_field("DashboardName"))?,
                    project_id: project_id,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dashboard {
    type Properties = DashboardProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::Dashboard";
    fn properties(&self) -> &DashboardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DashboardProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dashboard {}

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties }
    }
}

/// The [`AWS::IoTSiteWise::Gateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html) resource type.
#[derive(Debug, Default)]
pub struct Gateway {
    properties: GatewayProperties
}

/// Properties for the `Gateway` resource.
#[derive(Debug, Default)]
pub struct GatewayProperties {
    /// Property [`GatewayCapabilitySummaries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html#cfn-iotsitewise-gateway-gatewaycapabilitysummaries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gateway_capability_summaries: Option<::ValueList<self::gateway::GatewayCapabilitySummary>>,
    /// Property [`GatewayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html#cfn-iotsitewise-gateway-gatewayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gateway_name: ::Value<String>,
    /// Property [`GatewayPlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html#cfn-iotsitewise-gateway-gatewayplatform).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub gateway_platform: ::Value<self::gateway::GatewayPlatform>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html#cfn-iotsitewise-gateway-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for GatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref gateway_capability_summaries) = self.gateway_capability_summaries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayCapabilitySummaries", gateway_capability_summaries)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayName", &self.gateway_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayPlatform", &self.gateway_platform)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut gateway_capability_summaries: Option<::ValueList<self::gateway::GatewayCapabilitySummary>> = None;
                let mut gateway_name: Option<::Value<String>> = None;
                let mut gateway_platform: Option<::Value<self::gateway::GatewayPlatform>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GatewayCapabilitySummaries" => {
                            gateway_capability_summaries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GatewayName" => {
                            gateway_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GatewayPlatform" => {
                            gateway_platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GatewayProperties {
                    gateway_capability_summaries: gateway_capability_summaries,
                    gateway_name: gateway_name.ok_or(::serde::de::Error::missing_field("GatewayName"))?,
                    gateway_platform: gateway_platform.ok_or(::serde::de::Error::missing_field("GatewayPlatform"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Gateway {
    type Properties = GatewayProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::Gateway";
    fn properties(&self) -> &GatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Gateway {}

impl From<GatewayProperties> for Gateway {
    fn from(properties: GatewayProperties) -> Gateway {
        Gateway { properties }
    }
}

/// The [`AWS::IoTSiteWise::Portal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html) resource type.
#[derive(Debug, Default)]
pub struct Portal {
    properties: PortalProperties
}

/// Properties for the `Portal` resource.
#[derive(Debug, Default)]
pub struct PortalProperties {
    /// Property [`Alarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-alarms).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarms: Option<::Value<::json::Value>>,
    /// Property [`NotificationSenderEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-notificationsenderemail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_sender_email: Option<::Value<String>>,
    /// Property [`PortalAuthMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-portalauthmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portal_auth_mode: Option<::Value<String>>,
    /// Property [`PortalContactEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-portalcontactemail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub portal_contact_email: ::Value<String>,
    /// Property [`PortalDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-portaldescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub portal_description: Option<::Value<String>>,
    /// Property [`PortalName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-portalname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub portal_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html#cfn-iotsitewise-portal-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PortalProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref alarms) = self.alarms {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alarms", alarms)?;
        }
        if let Some(ref notification_sender_email) = self.notification_sender_email {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationSenderEmail", notification_sender_email)?;
        }
        if let Some(ref portal_auth_mode) = self.portal_auth_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalAuthMode", portal_auth_mode)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalContactEmail", &self.portal_contact_email)?;
        if let Some(ref portal_description) = self.portal_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalDescription", portal_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalName", &self.portal_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortalProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortalProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortalProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortalProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alarms: Option<::Value<::json::Value>> = None;
                let mut notification_sender_email: Option<::Value<String>> = None;
                let mut portal_auth_mode: Option<::Value<String>> = None;
                let mut portal_contact_email: Option<::Value<String>> = None;
                let mut portal_description: Option<::Value<String>> = None;
                let mut portal_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Alarms" => {
                            alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationSenderEmail" => {
                            notification_sender_email = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalAuthMode" => {
                            portal_auth_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalContactEmail" => {
                            portal_contact_email = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalDescription" => {
                            portal_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalName" => {
                            portal_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortalProperties {
                    alarms: alarms,
                    notification_sender_email: notification_sender_email,
                    portal_auth_mode: portal_auth_mode,
                    portal_contact_email: portal_contact_email.ok_or(::serde::de::Error::missing_field("PortalContactEmail"))?,
                    portal_description: portal_description,
                    portal_name: portal_name.ok_or(::serde::de::Error::missing_field("PortalName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Portal {
    type Properties = PortalProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::Portal";
    fn properties(&self) -> &PortalProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortalProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Portal {}

impl From<PortalProperties> for Portal {
    fn from(properties: PortalProperties) -> Portal {
        Portal { properties }
    }
}

/// The [`AWS::IoTSiteWise::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`AssetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html#cfn-iotsitewise-project-assetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub asset_ids: Option<::ValueList<String>>,
    /// Property [`PortalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html#cfn-iotsitewise-project-portalid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portal_id: ::Value<String>,
    /// Property [`ProjectDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html#cfn-iotsitewise-project-projectdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub project_description: Option<::Value<String>>,
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html#cfn-iotsitewise-project-projectname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub project_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html#cfn-iotsitewise-project-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref asset_ids) = self.asset_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetIds", asset_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalId", &self.portal_id)?;
        if let Some(ref project_description) = self.project_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectDescription", project_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", &self.project_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut asset_ids: Option<::ValueList<String>> = None;
                let mut portal_id: Option<::Value<String>> = None;
                let mut project_description: Option<::Value<String>> = None;
                let mut project_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssetIds" => {
                            asset_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalId" => {
                            portal_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectDescription" => {
                            project_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    asset_ids: asset_ids,
                    portal_id: portal_id.ok_or(::serde::de::Error::missing_field("PortalId"))?,
                    project_description: project_description,
                    project_name: project_name.ok_or(::serde::de::Error::missing_field("ProjectName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::IoTSiteWise::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

pub mod access_policy {
    //! Property types for the `AccessPolicy` resource.

    /// The [`AWS::IoTSiteWise::AccessPolicy.AccessPolicyIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyidentity.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessPolicyIdentity {
        /// Property [`IamRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyidentity.html#cfn-iotsitewise-accesspolicy-accesspolicyidentity-iamrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_role: Option<::Value<IamRole>>,
        /// Property [`IamUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyidentity.html#cfn-iotsitewise-accesspolicy-accesspolicyidentity-iamuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_user: Option<::Value<IamUser>>,
        /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyidentity.html#cfn-iotsitewise-accesspolicy-accesspolicyidentity-user).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user: Option<::Value<User>>,
    }

    impl ::codec::SerializeValue for AccessPolicyIdentity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iam_role) = self.iam_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRole", iam_role)?;
            }
            if let Some(ref iam_user) = self.iam_user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamUser", iam_user)?;
            }
            if let Some(ref user) = self.user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", user)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessPolicyIdentity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPolicyIdentity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessPolicyIdentity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessPolicyIdentity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iam_role: Option<::Value<IamRole>> = None;
                    let mut iam_user: Option<::Value<IamUser>> = None;
                    let mut user: Option<::Value<User>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IamRole" => {
                                iam_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamUser" => {
                                iam_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessPolicyIdentity {
                        iam_role: iam_role,
                        iam_user: iam_user,
                        user: user,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.AccessPolicyResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyresource.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessPolicyResource {
        /// Property [`Portal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyresource.html#cfn-iotsitewise-accesspolicy-accesspolicyresource-portal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub portal: Option<::Value<Portal>>,
        /// Property [`Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyresource.html#cfn-iotsitewise-accesspolicy-accesspolicyresource-project).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub project: Option<::Value<Project>>,
    }

    impl ::codec::SerializeValue for AccessPolicyResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref portal) = self.portal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Portal", portal)?;
            }
            if let Some(ref project) = self.project {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Project", project)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessPolicyResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPolicyResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessPolicyResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessPolicyResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut portal: Option<::Value<Portal>> = None;
                    let mut project: Option<::Value<Project>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Portal" => {
                                portal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Project" => {
                                project = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessPolicyResource {
                        portal: portal,
                        project: project,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.IamRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamrole.html) property type.
    #[derive(Debug, Default)]
    pub struct IamRole {
        /// Property [`arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamrole.html#cfn-iotsitewise-accesspolicy-iamrole-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IamRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IamRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IamRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IamRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IamRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IamRole {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.IamUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamuser.html) property type.
    #[derive(Debug, Default)]
    pub struct IamUser {
        /// Property [`arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamuser.html#cfn-iotsitewise-accesspolicy-iamuser-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IamUser {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IamUser {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IamUser, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IamUser;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IamUser")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IamUser {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.Portal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-portal.html) property type.
    #[derive(Debug, Default)]
    pub struct Portal {
        /// Property [`id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-portal.html#cfn-iotsitewise-accesspolicy-portal-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Portal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "id", id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Portal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Portal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Portal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Portal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Portal {
                        id: id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-project.html) property type.
    #[derive(Debug, Default)]
    pub struct Project {
        /// Property [`id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-project.html#cfn-iotsitewise-accesspolicy-project-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Project {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "id", id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Project {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Project, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Project;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Project")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Project {
                        id: id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AccessPolicy.User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-user.html) property type.
    #[derive(Debug, Default)]
    pub struct User {
        /// Property [`id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-user.html#cfn-iotsitewise-accesspolicy-user-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for User {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "id", id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for User {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<User, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = User;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type User")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(User {
                        id: id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod asset {
    //! Property types for the `Asset` resource.

    /// The [`AWS::IoTSiteWise::Asset.AssetHierarchy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assethierarchy.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetHierarchy {
        /// Property [`ChildAssetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assethierarchy.html#cfn-iotsitewise-asset-assethierarchy-childassetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_asset_id: ::Value<String>,
        /// Property [`LogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assethierarchy.html#cfn-iotsitewise-asset-assethierarchy-logicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AssetHierarchy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildAssetId", &self.child_asset_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalId", &self.logical_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetHierarchy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetHierarchy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetHierarchy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetHierarchy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_asset_id: Option<::Value<String>> = None;
                    let mut logical_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildAssetId" => {
                                child_asset_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalId" => {
                                logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetHierarchy {
                        child_asset_id: child_asset_id.ok_or(::serde::de::Error::missing_field("ChildAssetId"))?,
                        logical_id: logical_id.ok_or(::serde::de::Error::missing_field("LogicalId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::Asset.AssetProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assetproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetProperty {
        /// Property [`Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assetproperty.html#cfn-iotsitewise-asset-assetproperty-alias).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alias: Option<::Value<String>>,
        /// Property [`LogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assetproperty.html#cfn-iotsitewise-asset-assetproperty-logicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_id: ::Value<String>,
        /// Property [`NotificationState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assetproperty.html#cfn-iotsitewise-asset-assetproperty-notificationstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AssetProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alias) = self.alias {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alias", alias)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalId", &self.logical_id)?;
            if let Some(ref notification_state) = self.notification_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationState", notification_state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alias: Option<::Value<String>> = None;
                    let mut logical_id: Option<::Value<String>> = None;
                    let mut notification_state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alias" => {
                                alias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalId" => {
                                logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationState" => {
                                notification_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetProperty {
                        alias: alias,
                        logical_id: logical_id.ok_or(::serde::de::Error::missing_field("LogicalId"))?,
                        notification_state: notification_state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod asset_model {
    //! Property types for the `AssetModel` resource.

    /// The [`AWS::IoTSiteWise::AssetModel.AssetModelCompositeModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetModelCompositeModel {
        /// Property [`CompositeModelProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html#cfn-iotsitewise-assetmodel-assetmodelcompositemodel-compositemodelproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub composite_model_properties: Option<::ValueList<AssetModelProperty>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html#cfn-iotsitewise-assetmodel-assetmodelcompositemodel-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html#cfn-iotsitewise-assetmodel-assetmodelcompositemodel-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html#cfn-iotsitewise-assetmodel-assetmodelcompositemodel-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AssetModelCompositeModel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref composite_model_properties) = self.composite_model_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompositeModelProperties", composite_model_properties)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetModelCompositeModel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetModelCompositeModel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetModelCompositeModel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetModelCompositeModel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut composite_model_properties: Option<::ValueList<AssetModelProperty>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CompositeModelProperties" => {
                                composite_model_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetModelCompositeModel {
                        composite_model_properties: composite_model_properties,
                        description: description,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.AssetModelHierarchy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelhierarchy.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetModelHierarchy {
        /// Property [`ChildAssetModelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelhierarchy.html#cfn-iotsitewise-assetmodel-assetmodelhierarchy-childassetmodelid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_asset_model_id: ::Value<String>,
        /// Property [`LogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelhierarchy.html#cfn-iotsitewise-assetmodel-assetmodelhierarchy-logicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelhierarchy.html#cfn-iotsitewise-assetmodel-assetmodelhierarchy-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AssetModelHierarchy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildAssetModelId", &self.child_asset_model_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalId", &self.logical_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetModelHierarchy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetModelHierarchy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetModelHierarchy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetModelHierarchy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_asset_model_id: Option<::Value<String>> = None;
                    let mut logical_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildAssetModelId" => {
                                child_asset_model_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalId" => {
                                logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetModelHierarchy {
                        child_asset_model_id: child_asset_model_id.ok_or(::serde::de::Error::missing_field("ChildAssetModelId"))?,
                        logical_id: logical_id.ok_or(::serde::de::Error::missing_field("LogicalId"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.AssetModelProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetModelProperty {
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: ::Value<String>,
        /// Property [`DataTypeSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-datatypespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type_spec: Option<::Value<String>>,
        /// Property [`LogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-logicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<PropertyType>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html#cfn-iotsitewise-assetmodel-assetmodelproperty-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AssetModelProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
            if let Some(ref data_type_spec) = self.data_type_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTypeSpec", data_type_spec)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalId", &self.logical_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetModelProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetModelProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetModelProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetModelProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_type: Option<::Value<String>> = None;
                    let mut data_type_spec: Option<::Value<String>> = None;
                    let mut logical_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<PropertyType>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTypeSpec" => {
                                data_type_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalId" => {
                                logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetModelProperty {
                        data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                        data_type_spec: data_type_spec,
                        logical_id: logical_id.ok_or(::serde::de::Error::missing_field("LogicalId"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-attribute.html) property type.
    #[derive(Debug, Default)]
    pub struct Attribute {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-attribute.html#cfn-iotsitewise-assetmodel-attribute-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Attribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Attribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Attribute {
                        default_value: default_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.ExpressionVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-expressionvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct ExpressionVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-expressionvariable.html#cfn-iotsitewise-assetmodel-expressionvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-expressionvariable.html#cfn-iotsitewise-assetmodel-expressionvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<VariableValue>,
    }

    impl ::codec::SerializeValue for ExpressionVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExpressionVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExpressionVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExpressionVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExpressionVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<VariableValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExpressionVariable {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metric.html) property type.
    #[derive(Debug, Default)]
    pub struct Metric {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metric.html#cfn-iotsitewise-assetmodel-metric-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metric.html#cfn-iotsitewise-assetmodel-metric-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: ::ValueList<ExpressionVariable>,
        /// Property [`Window`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metric.html#cfn-iotsitewise-assetmodel-metric-window).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub window: ::Value<MetricWindow>,
    }

    impl ::codec::SerializeValue for Metric {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", &self.variables)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Window", &self.window)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Metric {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Metric, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Metric;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Metric")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut variables: Option<::ValueList<ExpressionVariable>> = None;
                    let mut window: Option<::Value<MetricWindow>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Window" => {
                                window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Metric {
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        variables: variables.ok_or(::serde::de::Error::missing_field("Variables"))?,
                        window: window.ok_or(::serde::de::Error::missing_field("Window"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.MetricWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metricwindow.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricWindow {
        /// Property [`Tumbling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metricwindow.html#cfn-iotsitewise-assetmodel-metricwindow-tumbling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tumbling: Option<::Value<TumblingWindow>>,
    }

    impl ::codec::SerializeValue for MetricWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tumbling) = self.tumbling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tumbling", tumbling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tumbling: Option<::Value<TumblingWindow>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Tumbling" => {
                                tumbling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricWindow {
                        tumbling: tumbling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.PropertyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html) property type.
    #[derive(Debug, Default)]
    pub struct PropertyType {
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html#cfn-iotsitewise-assetmodel-propertytype-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: Option<::Value<Attribute>>,
        /// Property [`Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html#cfn-iotsitewise-assetmodel-propertytype-metric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric: Option<::Value<Metric>>,
        /// Property [`Transform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html#cfn-iotsitewise-assetmodel-propertytype-transform).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transform: Option<::Value<Transform>>,
        /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html#cfn-iotsitewise-assetmodel-propertytype-typename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PropertyType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute) = self.attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", attribute)?;
            }
            if let Some(ref metric) = self.metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metric", metric)?;
            }
            if let Some(ref transform) = self.transform {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Transform", transform)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", &self.type_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PropertyType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PropertyType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PropertyType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PropertyType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute: Option<::Value<Attribute>> = None;
                    let mut metric: Option<::Value<Metric>> = None;
                    let mut transform: Option<::Value<Transform>> = None;
                    let mut type_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metric" => {
                                metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Transform" => {
                                transform = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TypeName" => {
                                type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PropertyType {
                        attribute: attribute,
                        metric: metric,
                        transform: transform,
                        type_name: type_name.ok_or(::serde::de::Error::missing_field("TypeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.Transform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-transform.html) property type.
    #[derive(Debug, Default)]
    pub struct Transform {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-transform.html#cfn-iotsitewise-assetmodel-transform-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-transform.html#cfn-iotsitewise-assetmodel-transform-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: ::ValueList<ExpressionVariable>,
    }

    impl ::codec::SerializeValue for Transform {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", &self.variables)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Transform {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Transform, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Transform;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Transform")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut variables: Option<::ValueList<ExpressionVariable>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Transform {
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        variables: variables.ok_or(::serde::de::Error::missing_field("Variables"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.TumblingWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-tumblingwindow.html) property type.
    #[derive(Debug, Default)]
    pub struct TumblingWindow {
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-tumblingwindow.html#cfn-iotsitewise-assetmodel-tumblingwindow-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: ::Value<String>,
        /// Property [`Offset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-tumblingwindow.html#cfn-iotsitewise-assetmodel-tumblingwindow-offset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TumblingWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            if let Some(ref offset) = self.offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", offset)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TumblingWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TumblingWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TumblingWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TumblingWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interval: Option<::Value<String>> = None;
                    let mut offset: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Offset" => {
                                offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TumblingWindow {
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        offset: offset,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::AssetModel.VariableValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-variablevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct VariableValue {
        /// Property [`HierarchyLogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-variablevalue.html#cfn-iotsitewise-assetmodel-variablevalue-hierarchylogicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hierarchy_logical_id: Option<::Value<String>>,
        /// Property [`PropertyLogicalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-variablevalue.html#cfn-iotsitewise-assetmodel-variablevalue-propertylogicalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_logical_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for VariableValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hierarchy_logical_id) = self.hierarchy_logical_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HierarchyLogicalId", hierarchy_logical_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyLogicalId", &self.property_logical_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VariableValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VariableValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VariableValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VariableValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hierarchy_logical_id: Option<::Value<String>> = None;
                    let mut property_logical_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HierarchyLogicalId" => {
                                hierarchy_logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyLogicalId" => {
                                property_logical_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VariableValue {
                        hierarchy_logical_id: hierarchy_logical_id,
                        property_logical_id: property_logical_id.ok_or(::serde::de::Error::missing_field("PropertyLogicalId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod gateway {
    //! Property types for the `Gateway` resource.

    /// The [`AWS::IoTSiteWise::Gateway.GatewayCapabilitySummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewaycapabilitysummary.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayCapabilitySummary {
        /// Property [`CapabilityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewaycapabilitysummary.html#cfn-iotsitewise-gateway-gatewaycapabilitysummary-capabilityconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capability_configuration: Option<::Value<String>>,
        /// Property [`CapabilityNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewaycapabilitysummary.html#cfn-iotsitewise-gateway-gatewaycapabilitysummary-capabilitynamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capability_namespace: ::Value<String>,
    }

    impl ::codec::SerializeValue for GatewayCapabilitySummary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capability_configuration) = self.capability_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapabilityConfiguration", capability_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapabilityNamespace", &self.capability_namespace)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayCapabilitySummary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayCapabilitySummary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayCapabilitySummary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayCapabilitySummary")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capability_configuration: Option<::Value<String>> = None;
                    let mut capability_namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CapabilityConfiguration" => {
                                capability_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapabilityNamespace" => {
                                capability_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayCapabilitySummary {
                        capability_configuration: capability_configuration,
                        capability_namespace: capability_namespace.ok_or(::serde::de::Error::missing_field("CapabilityNamespace"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::Gateway.GatewayPlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewayplatform.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayPlatform {
        /// Property [`Greengrass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewayplatform.html#cfn-iotsitewise-gateway-gatewayplatform-greengrass).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub greengrass: Option<::Value<Greengrass>>,
        /// Property [`GreengrassV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewayplatform.html#cfn-iotsitewise-gateway-gatewayplatform-greengrassv2).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub greengrass_v2: Option<::Value<GreengrassV2>>,
    }

    impl ::codec::SerializeValue for GatewayPlatform {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref greengrass) = self.greengrass {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Greengrass", greengrass)?;
            }
            if let Some(ref greengrass_v2) = self.greengrass_v2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GreengrassV2", greengrass_v2)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayPlatform {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayPlatform, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayPlatform;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayPlatform")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut greengrass: Option<::Value<Greengrass>> = None;
                    let mut greengrass_v2: Option<::Value<GreengrassV2>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Greengrass" => {
                                greengrass = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GreengrassV2" => {
                                greengrass_v2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayPlatform {
                        greengrass: greengrass,
                        greengrass_v2: greengrass_v2,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::Gateway.Greengrass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-greengrass.html) property type.
    #[derive(Debug, Default)]
    pub struct Greengrass {
        /// Property [`GroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-greengrass.html#cfn-iotsitewise-gateway-greengrass-grouparn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub group_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Greengrass {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupArn", &self.group_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Greengrass {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Greengrass, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Greengrass;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Greengrass")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupArn" => {
                                group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Greengrass {
                        group_arn: group_arn.ok_or(::serde::de::Error::missing_field("GroupArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTSiteWise::Gateway.GreengrassV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-greengrassv2.html) property type.
    #[derive(Debug, Default)]
    pub struct GreengrassV2 {
        /// Property [`CoreDeviceThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-greengrassv2.html#cfn-iotsitewise-gateway-greengrassv2-coredevicethingname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub core_device_thing_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GreengrassV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreDeviceThingName", &self.core_device_thing_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GreengrassV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GreengrassV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GreengrassV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GreengrassV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut core_device_thing_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CoreDeviceThingName" => {
                                core_device_thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GreengrassV2 {
                        core_device_thing_name: core_device_thing_name.ok_or(::serde::de::Error::missing_field("CoreDeviceThingName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
