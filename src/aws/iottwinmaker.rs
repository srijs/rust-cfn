//! Types for the `IoTTwinMaker` service.

/// The [`AWS::IoTTwinMaker::ComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html) resource type.
#[derive(Debug, Default)]
pub struct ComponentType {
    properties: ComponentTypeProperties
}

/// Properties for the `ComponentType` resource.
#[derive(Debug, Default)]
pub struct ComponentTypeProperties {
    /// Property [`ComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-componenttypeid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub component_type_id: ::Value<String>,
    /// Property [`CompositeComponentTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-compositecomponenttypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub composite_component_types: Option<::ValueMap<self::component_type::CompositeComponentType>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExtendsFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-extendsfrom).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extends_from: Option<::ValueList<String>>,
    /// Property [`Functions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-functions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub functions: Option<::ValueMap<self::component_type::Function>>,
    /// Property [`IsSingleton`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-issingleton).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_singleton: Option<::Value<bool>>,
    /// Property [`PropertyDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-propertydefinitions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub property_definitions: Option<::ValueMap<self::component_type::PropertyDefinition>>,
    /// Property [`PropertyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-propertygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub property_groups: Option<::ValueMap<self::component_type::PropertyGroup>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`WorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html#cfn-iottwinmaker-componenttype-workspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workspace_id: ::Value<String>,
}

impl ::serde::Serialize for ComponentTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentTypeId", &self.component_type_id)?;
        if let Some(ref composite_component_types) = self.composite_component_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompositeComponentTypes", composite_component_types)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref extends_from) = self.extends_from {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendsFrom", extends_from)?;
        }
        if let Some(ref functions) = self.functions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Functions", functions)?;
        }
        if let Some(ref is_singleton) = self.is_singleton {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsSingleton", is_singleton)?;
        }
        if let Some(ref property_definitions) = self.property_definitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyDefinitions", property_definitions)?;
        }
        if let Some(ref property_groups) = self.property_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyGroups", property_groups)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceId", &self.workspace_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComponentTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComponentTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComponentTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut component_type_id: Option<::Value<String>> = None;
                let mut composite_component_types: Option<::ValueMap<self::component_type::CompositeComponentType>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut extends_from: Option<::ValueList<String>> = None;
                let mut functions: Option<::ValueMap<self::component_type::Function>> = None;
                let mut is_singleton: Option<::Value<bool>> = None;
                let mut property_definitions: Option<::ValueMap<self::component_type::PropertyDefinition>> = None;
                let mut property_groups: Option<::ValueMap<self::component_type::PropertyGroup>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut workspace_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComponentTypeId" => {
                            component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CompositeComponentTypes" => {
                            composite_component_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtendsFrom" => {
                            extends_from = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Functions" => {
                            functions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsSingleton" => {
                            is_singleton = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PropertyDefinitions" => {
                            property_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PropertyGroups" => {
                            property_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceId" => {
                            workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ComponentTypeProperties {
                    component_type_id: component_type_id.ok_or(::serde::de::Error::missing_field("ComponentTypeId"))?,
                    composite_component_types: composite_component_types,
                    description: description,
                    extends_from: extends_from,
                    functions: functions,
                    is_singleton: is_singleton,
                    property_definitions: property_definitions,
                    property_groups: property_groups,
                    tags: tags,
                    workspace_id: workspace_id.ok_or(::serde::de::Error::missing_field("WorkspaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ComponentType {
    type Properties = ComponentTypeProperties;
    const TYPE: &'static str = "AWS::IoTTwinMaker::ComponentType";
    fn properties(&self) -> &ComponentTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComponentTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ComponentType {}

impl From<ComponentTypeProperties> for ComponentType {
    fn from(properties: ComponentTypeProperties) -> ComponentType {
        ComponentType { properties }
    }
}

/// The [`AWS::IoTTwinMaker::Entity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html) resource type.
#[derive(Debug, Default)]
pub struct Entity {
    properties: EntityProperties
}

/// Properties for the `Entity` resource.
#[derive(Debug, Default)]
pub struct EntityProperties {
    /// Property [`Components`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-components).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub components: Option<::ValueMap<self::entity::Component>>,
    /// Property [`CompositeComponents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-compositecomponents).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub composite_components: Option<::ValueMap<self::entity::CompositeComponent>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-entityid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub entity_id: Option<::Value<String>>,
    /// Property [`EntityName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-entityname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entity_name: ::Value<String>,
    /// Property [`ParentEntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-parententityid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parent_entity_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`WorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html#cfn-iottwinmaker-entity-workspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workspace_id: ::Value<String>,
}

impl ::serde::Serialize for EntityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref components) = self.components {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Components", components)?;
        }
        if let Some(ref composite_components) = self.composite_components {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompositeComponents", composite_components)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref entity_id) = self.entity_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityId", entity_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityName", &self.entity_name)?;
        if let Some(ref parent_entity_id) = self.parent_entity_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentEntityId", parent_entity_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceId", &self.workspace_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EntityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EntityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EntityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut components: Option<::ValueMap<self::entity::Component>> = None;
                let mut composite_components: Option<::ValueMap<self::entity::CompositeComponent>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut entity_id: Option<::Value<String>> = None;
                let mut entity_name: Option<::Value<String>> = None;
                let mut parent_entity_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut workspace_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Components" => {
                            components = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CompositeComponents" => {
                            composite_components = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntityId" => {
                            entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntityName" => {
                            entity_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentEntityId" => {
                            parent_entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceId" => {
                            workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EntityProperties {
                    components: components,
                    composite_components: composite_components,
                    description: description,
                    entity_id: entity_id,
                    entity_name: entity_name.ok_or(::serde::de::Error::missing_field("EntityName"))?,
                    parent_entity_id: parent_entity_id,
                    tags: tags,
                    workspace_id: workspace_id.ok_or(::serde::de::Error::missing_field("WorkspaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Entity {
    type Properties = EntityProperties;
    const TYPE: &'static str = "AWS::IoTTwinMaker::Entity";
    fn properties(&self) -> &EntityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EntityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Entity {}

impl From<EntityProperties> for Entity {
    fn from(properties: EntityProperties) -> Entity {
        Entity { properties }
    }
}

/// The [`AWS::IoTTwinMaker::Scene`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html) resource type.
#[derive(Debug, Default)]
pub struct Scene {
    properties: SceneProperties
}

/// Properties for the `Scene` resource.
#[derive(Debug, Default)]
pub struct SceneProperties {
    /// Property [`Capabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-capabilities).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capabilities: Option<::ValueList<String>>,
    /// Property [`ContentLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-contentlocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_location: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`SceneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-sceneid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scene_id: ::Value<String>,
    /// Property [`SceneMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-scenemetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scene_metadata: Option<::ValueMap<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`WorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html#cfn-iottwinmaker-scene-workspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workspace_id: ::Value<String>,
}

impl ::serde::Serialize for SceneProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref capabilities) = self.capabilities {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capabilities", capabilities)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentLocation", &self.content_location)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SceneId", &self.scene_id)?;
        if let Some(ref scene_metadata) = self.scene_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SceneMetadata", scene_metadata)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceId", &self.workspace_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SceneProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SceneProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SceneProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SceneProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut capabilities: Option<::ValueList<String>> = None;
                let mut content_location: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut scene_id: Option<::Value<String>> = None;
                let mut scene_metadata: Option<::ValueMap<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut workspace_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Capabilities" => {
                            capabilities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContentLocation" => {
                            content_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SceneId" => {
                            scene_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SceneMetadata" => {
                            scene_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceId" => {
                            workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SceneProperties {
                    capabilities: capabilities,
                    content_location: content_location.ok_or(::serde::de::Error::missing_field("ContentLocation"))?,
                    description: description,
                    scene_id: scene_id.ok_or(::serde::de::Error::missing_field("SceneId"))?,
                    scene_metadata: scene_metadata,
                    tags: tags,
                    workspace_id: workspace_id.ok_or(::serde::de::Error::missing_field("WorkspaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Scene {
    type Properties = SceneProperties;
    const TYPE: &'static str = "AWS::IoTTwinMaker::Scene";
    fn properties(&self) -> &SceneProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SceneProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Scene {}

impl From<SceneProperties> for Scene {
    fn from(properties: SceneProperties) -> Scene {
        Scene { properties }
    }
}

/// The [`AWS::IoTTwinMaker::SyncJob`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html) resource type.
#[derive(Debug, Default)]
pub struct SyncJob {
    properties: SyncJobProperties
}

/// Properties for the `SyncJob` resource.
#[derive(Debug, Default)]
pub struct SyncJobProperties {
    /// Property [`SyncRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html#cfn-iottwinmaker-syncjob-syncrole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_role: ::Value<String>,
    /// Property [`SyncSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html#cfn-iottwinmaker-syncjob-syncsource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_source: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html#cfn-iottwinmaker-syncjob-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`WorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html#cfn-iottwinmaker-syncjob-workspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workspace_id: ::Value<String>,
}

impl ::serde::Serialize for SyncJobProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncRole", &self.sync_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncSource", &self.sync_source)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceId", &self.workspace_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SyncJobProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SyncJobProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SyncJobProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SyncJobProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut sync_role: Option<::Value<String>> = None;
                let mut sync_source: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut workspace_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SyncRole" => {
                            sync_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncSource" => {
                            sync_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceId" => {
                            workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SyncJobProperties {
                    sync_role: sync_role.ok_or(::serde::de::Error::missing_field("SyncRole"))?,
                    sync_source: sync_source.ok_or(::serde::de::Error::missing_field("SyncSource"))?,
                    tags: tags,
                    workspace_id: workspace_id.ok_or(::serde::de::Error::missing_field("WorkspaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SyncJob {
    type Properties = SyncJobProperties;
    const TYPE: &'static str = "AWS::IoTTwinMaker::SyncJob";
    fn properties(&self) -> &SyncJobProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SyncJobProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SyncJob {}

impl From<SyncJobProperties> for SyncJob {
    fn from(properties: SyncJobProperties) -> SyncJob {
        SyncJob { properties }
    }
}

/// The [`AWS::IoTTwinMaker::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html) resource type.
#[derive(Debug, Default)]
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Debug, Default)]
pub struct WorkspaceProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html#cfn-iottwinmaker-workspace-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html#cfn-iottwinmaker-workspace-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html#cfn-iottwinmaker-workspace-s3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_location: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html#cfn-iottwinmaker-workspace-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`WorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html#cfn-iottwinmaker-workspace-workspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workspace_id: ::Value<String>,
}

impl ::serde::Serialize for WorkspaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Location", &self.s3_location)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceId", &self.workspace_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkspaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkspaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkspaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkspaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut s3_location: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut workspace_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Location" => {
                            s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceId" => {
                            workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkspaceProperties {
                    description: description,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    s3_location: s3_location.ok_or(::serde::de::Error::missing_field("S3Location"))?,
                    tags: tags,
                    workspace_id: workspace_id.ok_or(::serde::de::Error::missing_field("WorkspaceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workspace {
    type Properties = WorkspaceProperties;
    const TYPE: &'static str = "AWS::IoTTwinMaker::Workspace";
    fn properties(&self) -> &WorkspaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkspaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workspace {}

impl From<WorkspaceProperties> for Workspace {
    fn from(properties: WorkspaceProperties) -> Workspace {
        Workspace { properties }
    }
}

pub mod component_type {
    //! Property types for the `ComponentType` resource.

    /// The [`AWS::IoTTwinMaker::ComponentType.CompositeComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-compositecomponenttype.html) property type.
    #[derive(Debug, Default)]
    pub struct CompositeComponentType {
        /// Property [`ComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-compositecomponenttype.html#cfn-iottwinmaker-componenttype-compositecomponenttype-componenttypeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_type_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CompositeComponentType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_type_id) = self.component_type_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentTypeId", component_type_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CompositeComponentType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CompositeComponentType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CompositeComponentType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CompositeComponentType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_type_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentTypeId" => {
                                component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CompositeComponentType {
                        component_type_id: component_type_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.DataConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-dataconnector.html) property type.
    #[derive(Debug, Default)]
    pub struct DataConnector {
        /// Property [`IsNative`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-dataconnector.html#cfn-iottwinmaker-componenttype-dataconnector-isnative).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_native: Option<::Value<bool>>,
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-dataconnector.html#cfn-iottwinmaker-componenttype-dataconnector-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<::Value<LambdaFunction>>,
    }

    impl ::codec::SerializeValue for DataConnector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref is_native) = self.is_native {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsNative", is_native)?;
            }
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataConnector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataConnector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataConnector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataConnector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut is_native: Option<::Value<bool>> = None;
                    let mut lambda: Option<::Value<LambdaFunction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsNative" => {
                                is_native = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataConnector {
                        is_native: is_native,
                        lambda: lambda,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html) property type.
    #[derive(Debug, Default)]
    pub struct DataType {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html#cfn-iottwinmaker-componenttype-datatype-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::ValueList<DataValue>>,
        /// Property [`NestedType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html#cfn-iottwinmaker-componenttype-datatype-nestedtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nested_type: Option<::Value<DataType>>,
        /// Property [`Relationship`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html#cfn-iottwinmaker-componenttype-datatype-relationship).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship: Option<::Value<Relationship>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html#cfn-iottwinmaker-componenttype-datatype-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`UnitOfMeasure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html#cfn-iottwinmaker-componenttype-datatype-unitofmeasure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit_of_measure: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            if let Some(ref nested_type) = self.nested_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NestedType", nested_type)?;
            }
            if let Some(ref relationship) = self.relationship {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Relationship", relationship)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref unit_of_measure) = self.unit_of_measure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnitOfMeasure", unit_of_measure)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::ValueList<DataValue>> = None;
                    let mut nested_type: Option<::Value<DataType>> = None;
                    let mut relationship: Option<::Value<Relationship>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut unit_of_measure: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NestedType" => {
                                nested_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Relationship" => {
                                relationship = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnitOfMeasure" => {
                                unit_of_measure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataType {
                        allowed_values: allowed_values,
                        nested_type: nested_type,
                        relationship: relationship,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        unit_of_measure: unit_of_measure,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.DataValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DataValue {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<::Value<bool>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<f64>>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`IntegerValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-integervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_value: Option<::Value<u32>>,
        /// Property [`ListValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-listvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub list_value: Option<::ValueList<DataValue>>,
        /// Property [`LongValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-longvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub long_value: Option<::Value<f64>>,
        /// Property [`MapValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-mapvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub map_value: Option<::ValueMap<DataValue>>,
        /// Property [`RelationshipValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-relationshipvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship_value: Option<::Value<RelationshipValue>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html#cfn-iottwinmaker-componenttype-datavalue-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            if let Some(ref integer_value) = self.integer_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerValue", integer_value)?;
            }
            if let Some(ref list_value) = self.list_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListValue", list_value)?;
            }
            if let Some(ref long_value) = self.long_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LongValue", long_value)?;
            }
            if let Some(ref map_value) = self.map_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapValue", map_value)?;
            }
            if let Some(ref relationship_value) = self.relationship_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationshipValue", relationship_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<::Value<bool>> = None;
                    let mut double_value: Option<::Value<f64>> = None;
                    let mut expression: Option<::Value<String>> = None;
                    let mut integer_value: Option<::Value<u32>> = None;
                    let mut list_value: Option<::ValueList<DataValue>> = None;
                    let mut long_value: Option<::Value<f64>> = None;
                    let mut map_value: Option<::ValueMap<DataValue>> = None;
                    let mut relationship_value: Option<::Value<RelationshipValue>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerValue" => {
                                integer_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ListValue" => {
                                list_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LongValue" => {
                                long_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MapValue" => {
                                map_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationshipValue" => {
                                relationship_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataValue {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        expression: expression,
                        integer_value: integer_value,
                        list_value: list_value,
                        long_value: long_value,
                        map_value: map_value,
                        relationship_value: relationship_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.Error`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-error.html) property type.
    #[derive(Debug, Default)]
    pub struct Error {
        /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-error.html#cfn-iottwinmaker-componenttype-error-code).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code: Option<::Value<String>>,
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-error.html#cfn-iottwinmaker-componenttype-error-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Error {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code) = self.code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", code)?;
            }
            if let Some(ref message) = self.message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Error {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Error, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Error;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Error")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code: Option<::Value<String>> = None;
                    let mut message: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Code" => {
                                code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Error {
                        code: code,
                        message: message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-function.html) property type.
    #[derive(Debug, Default)]
    pub struct Function {
        /// Property [`ImplementedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-function.html#cfn-iottwinmaker-componenttype-function-implementedby).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub implemented_by: Option<::Value<DataConnector>>,
        /// Property [`RequiredProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-function.html#cfn-iottwinmaker-componenttype-function-requiredproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required_properties: Option<::ValueList<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-function.html#cfn-iottwinmaker-componenttype-function-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Function {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref implemented_by) = self.implemented_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImplementedBy", implemented_by)?;
            }
            if let Some(ref required_properties) = self.required_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequiredProperties", required_properties)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Function {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Function, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Function;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Function")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut implemented_by: Option<::Value<DataConnector>> = None;
                    let mut required_properties: Option<::ValueList<String>> = None;
                    let mut scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImplementedBy" => {
                                implemented_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequiredProperties" => {
                                required_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Function {
                        implemented_by: implemented_by,
                        required_properties: required_properties,
                        scope: scope,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.LambdaFunction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-lambdafunction.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaFunction {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-lambdafunction.html#cfn-iottwinmaker-componenttype-lambdafunction-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaFunction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaFunction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaFunction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaFunction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaFunction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaFunction {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.PropertyDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct PropertyDefinition {
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-configurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configurations: Option<::ValueMap<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: Option<::Value<DataType>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<DataValue>>,
        /// Property [`IsExternalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-isexternalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_external_id: Option<::Value<bool>>,
        /// Property [`IsRequiredInEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-isrequiredinentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_required_in_entity: Option<::Value<bool>>,
        /// Property [`IsStoredExternally`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-isstoredexternally).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_stored_externally: Option<::Value<bool>>,
        /// Property [`IsTimeSeries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html#cfn-iottwinmaker-componenttype-propertydefinition-istimeseries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_time_series: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PropertyDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
            if let Some(ref data_type) = self.data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref is_external_id) = self.is_external_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsExternalId", is_external_id)?;
            }
            if let Some(ref is_required_in_entity) = self.is_required_in_entity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsRequiredInEntity", is_required_in_entity)?;
            }
            if let Some(ref is_stored_externally) = self.is_stored_externally {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsStoredExternally", is_stored_externally)?;
            }
            if let Some(ref is_time_series) = self.is_time_series {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsTimeSeries", is_time_series)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PropertyDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PropertyDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PropertyDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PropertyDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configurations: Option<::ValueMap<String>> = None;
                    let mut data_type: Option<::Value<DataType>> = None;
                    let mut default_value: Option<::Value<DataValue>> = None;
                    let mut is_external_id: Option<::Value<bool>> = None;
                    let mut is_required_in_entity: Option<::Value<bool>> = None;
                    let mut is_stored_externally: Option<::Value<bool>> = None;
                    let mut is_time_series: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsExternalId" => {
                                is_external_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsRequiredInEntity" => {
                                is_required_in_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsStoredExternally" => {
                                is_stored_externally = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsTimeSeries" => {
                                is_time_series = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PropertyDefinition {
                        configurations: configurations,
                        data_type: data_type,
                        default_value: default_value,
                        is_external_id: is_external_id,
                        is_required_in_entity: is_required_in_entity,
                        is_stored_externally: is_stored_externally,
                        is_time_series: is_time_series,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.PropertyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertygroup.html) property type.
    #[derive(Debug, Default)]
    pub struct PropertyGroup {
        /// Property [`GroupType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertygroup.html#cfn-iottwinmaker-componenttype-propertygroup-grouptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_type: Option<::Value<String>>,
        /// Property [`PropertyNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertygroup.html#cfn-iottwinmaker-componenttype-propertygroup-propertynames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PropertyGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group_type) = self.group_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupType", group_type)?;
            }
            if let Some(ref property_names) = self.property_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyNames", property_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PropertyGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PropertyGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PropertyGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PropertyGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_type: Option<::Value<String>> = None;
                    let mut property_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupType" => {
                                group_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyNames" => {
                                property_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PropertyGroup {
                        group_type: group_type,
                        property_names: property_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.Relationship`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationship.html) property type.
    #[derive(Debug, Default)]
    pub struct Relationship {
        /// Property [`RelationshipType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationship.html#cfn-iottwinmaker-componenttype-relationship-relationshiptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship_type: Option<::Value<String>>,
        /// Property [`TargetComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationship.html#cfn-iottwinmaker-componenttype-relationship-targetcomponenttypeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_component_type_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Relationship {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref relationship_type) = self.relationship_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationshipType", relationship_type)?;
            }
            if let Some(ref target_component_type_id) = self.target_component_type_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetComponentTypeId", target_component_type_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Relationship {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Relationship, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Relationship;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Relationship")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut relationship_type: Option<::Value<String>> = None;
                    let mut target_component_type_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RelationshipType" => {
                                relationship_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetComponentTypeId" => {
                                target_component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Relationship {
                        relationship_type: relationship_type,
                        target_component_type_id: target_component_type_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.RelationshipValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationshipvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationshipValue {
        /// Property [`TargetComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationshipvalue.html#cfn-iottwinmaker-componenttype-relationshipvalue-targetcomponentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_component_name: Option<::Value<String>>,
        /// Property [`TargetEntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationshipvalue.html#cfn-iottwinmaker-componenttype-relationshipvalue-targetentityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_entity_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RelationshipValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_component_name) = self.target_component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetComponentName", target_component_name)?;
            }
            if let Some(ref target_entity_id) = self.target_entity_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEntityId", target_entity_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationshipValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationshipValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationshipValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationshipValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_component_name: Option<::Value<String>> = None;
                    let mut target_entity_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetComponentName" => {
                                target_component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetEntityId" => {
                                target_entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationshipValue {
                        target_component_name: target_component_name,
                        target_entity_id: target_entity_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::ComponentType.Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-status.html) property type.
    #[derive(Debug, Default)]
    pub struct Status {
        /// Property [`Error`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-status.html#cfn-iottwinmaker-componenttype-status-error).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error: Option<::Value<Error>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-status.html#cfn-iottwinmaker-componenttype-status-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Status {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error) = self.error {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Error", error)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Status {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Status, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Status;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Status")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error: Option<::Value<Error>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Error" => {
                                error = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Status {
                        error: error,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod entity {
    //! Property types for the `Entity` resource.

    /// The [`AWS::IoTTwinMaker::Entity.Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html) property type.
    #[derive(Debug, Default)]
    pub struct Component {
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: Option<::Value<String>>,
        /// Property [`ComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-componenttypeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_type_id: Option<::Value<String>>,
        /// Property [`DefinedIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-definedin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub defined_in: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-properties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub properties: Option<::ValueMap<Property>>,
        /// Property [`PropertyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-propertygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_groups: Option<::ValueMap<PropertyGroup>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html#cfn-iottwinmaker-entity-component-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<Status>>,
    }

    impl ::codec::SerializeValue for Component {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_name) = self.component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", component_name)?;
            }
            if let Some(ref component_type_id) = self.component_type_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentTypeId", component_type_id)?;
            }
            if let Some(ref defined_in) = self.defined_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefinedIn", defined_in)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref properties) = self.properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", properties)?;
            }
            if let Some(ref property_groups) = self.property_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyGroups", property_groups)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Component {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Component, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Component;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Component")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_name: Option<::Value<String>> = None;
                    let mut component_type_id: Option<::Value<String>> = None;
                    let mut defined_in: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut properties: Option<::ValueMap<Property>> = None;
                    let mut property_groups: Option<::ValueMap<PropertyGroup>> = None;
                    let mut status: Option<::Value<Status>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentTypeId" => {
                                component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefinedIn" => {
                                defined_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Properties" => {
                                properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyGroups" => {
                                property_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Component {
                        component_name: component_name,
                        component_type_id: component_type_id,
                        defined_in: defined_in,
                        description: description,
                        properties: properties,
                        property_groups: property_groups,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.CompositeComponent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html) property type.
    #[derive(Debug, Default)]
    pub struct CompositeComponent {
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: Option<::Value<String>>,
        /// Property [`ComponentPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-componentpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_path: Option<::Value<String>>,
        /// Property [`ComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-componenttypeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_type_id: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-properties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub properties: Option<::ValueMap<Property>>,
        /// Property [`PropertyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-propertygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_groups: Option<::ValueMap<PropertyGroup>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html#cfn-iottwinmaker-entity-compositecomponent-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<Status>>,
    }

    impl ::codec::SerializeValue for CompositeComponent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_name) = self.component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", component_name)?;
            }
            if let Some(ref component_path) = self.component_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentPath", component_path)?;
            }
            if let Some(ref component_type_id) = self.component_type_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentTypeId", component_type_id)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref properties) = self.properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", properties)?;
            }
            if let Some(ref property_groups) = self.property_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyGroups", property_groups)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CompositeComponent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CompositeComponent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CompositeComponent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CompositeComponent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_name: Option<::Value<String>> = None;
                    let mut component_path: Option<::Value<String>> = None;
                    let mut component_type_id: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut properties: Option<::ValueMap<Property>> = None;
                    let mut property_groups: Option<::ValueMap<PropertyGroup>> = None;
                    let mut status: Option<::Value<Status>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentPath" => {
                                component_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentTypeId" => {
                                component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Properties" => {
                                properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyGroups" => {
                                property_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CompositeComponent {
                        component_name: component_name,
                        component_path: component_path,
                        component_type_id: component_type_id,
                        description: description,
                        properties: properties,
                        property_groups: property_groups,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html) property type.
    #[derive(Debug, Default)]
    pub struct DataType {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html#cfn-iottwinmaker-entity-datatype-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::ValueList<DataValue>>,
        /// Property [`NestedType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html#cfn-iottwinmaker-entity-datatype-nestedtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nested_type: Option<::Value<DataType>>,
        /// Property [`Relationship`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html#cfn-iottwinmaker-entity-datatype-relationship).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship: Option<::Value<Relationship>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html#cfn-iottwinmaker-entity-datatype-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`UnitOfMeasure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html#cfn-iottwinmaker-entity-datatype-unitofmeasure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit_of_measure: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            if let Some(ref nested_type) = self.nested_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NestedType", nested_type)?;
            }
            if let Some(ref relationship) = self.relationship {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Relationship", relationship)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref unit_of_measure) = self.unit_of_measure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnitOfMeasure", unit_of_measure)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::ValueList<DataValue>> = None;
                    let mut nested_type: Option<::Value<DataType>> = None;
                    let mut relationship: Option<::Value<Relationship>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut unit_of_measure: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NestedType" => {
                                nested_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Relationship" => {
                                relationship = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnitOfMeasure" => {
                                unit_of_measure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataType {
                        allowed_values: allowed_values,
                        nested_type: nested_type,
                        relationship: relationship,
                        r#type: r#type,
                        unit_of_measure: unit_of_measure,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.DataValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DataValue {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<::Value<bool>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<f64>>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`IntegerValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-integervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_value: Option<::Value<u32>>,
        /// Property [`ListValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-listvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub list_value: Option<::ValueList<DataValue>>,
        /// Property [`LongValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-longvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub long_value: Option<::Value<f64>>,
        /// Property [`MapValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-mapvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub map_value: Option<::ValueMap<DataValue>>,
        /// Property [`RelationshipValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-relationshipvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship_value: Option<::Value<RelationshipValue>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html#cfn-iottwinmaker-entity-datavalue-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            if let Some(ref integer_value) = self.integer_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerValue", integer_value)?;
            }
            if let Some(ref list_value) = self.list_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListValue", list_value)?;
            }
            if let Some(ref long_value) = self.long_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LongValue", long_value)?;
            }
            if let Some(ref map_value) = self.map_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapValue", map_value)?;
            }
            if let Some(ref relationship_value) = self.relationship_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationshipValue", relationship_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<::Value<bool>> = None;
                    let mut double_value: Option<::Value<f64>> = None;
                    let mut expression: Option<::Value<String>> = None;
                    let mut integer_value: Option<::Value<u32>> = None;
                    let mut list_value: Option<::ValueList<DataValue>> = None;
                    let mut long_value: Option<::Value<f64>> = None;
                    let mut map_value: Option<::ValueMap<DataValue>> = None;
                    let mut relationship_value: Option<::Value<RelationshipValue>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerValue" => {
                                integer_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ListValue" => {
                                list_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LongValue" => {
                                long_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MapValue" => {
                                map_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationshipValue" => {
                                relationship_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataValue {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        expression: expression,
                        integer_value: integer_value,
                        list_value: list_value,
                        long_value: long_value,
                        map_value: map_value,
                        relationship_value: relationship_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html) property type.
    #[derive(Debug, Default)]
    pub struct Definition {
        /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration: Option<::ValueMap<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: Option<::Value<DataType>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<DataValue>>,
        /// Property [`IsExternalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isexternalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_external_id: Option<::Value<bool>>,
        /// Property [`IsFinal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isfinal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_final: Option<::Value<bool>>,
        /// Property [`IsImported`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isimported).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_imported: Option<::Value<bool>>,
        /// Property [`IsInherited`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isinherited).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_inherited: Option<::Value<bool>>,
        /// Property [`IsRequiredInEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isrequiredinentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_required_in_entity: Option<::Value<bool>>,
        /// Property [`IsStoredExternally`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-isstoredexternally).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_stored_externally: Option<::Value<bool>>,
        /// Property [`IsTimeSeries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html#cfn-iottwinmaker-entity-definition-istimeseries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_time_series: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Definition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configuration) = self.configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
            }
            if let Some(ref data_type) = self.data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref is_external_id) = self.is_external_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsExternalId", is_external_id)?;
            }
            if let Some(ref is_final) = self.is_final {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsFinal", is_final)?;
            }
            if let Some(ref is_imported) = self.is_imported {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsImported", is_imported)?;
            }
            if let Some(ref is_inherited) = self.is_inherited {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsInherited", is_inherited)?;
            }
            if let Some(ref is_required_in_entity) = self.is_required_in_entity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsRequiredInEntity", is_required_in_entity)?;
            }
            if let Some(ref is_stored_externally) = self.is_stored_externally {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsStoredExternally", is_stored_externally)?;
            }
            if let Some(ref is_time_series) = self.is_time_series {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsTimeSeries", is_time_series)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Definition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Definition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Definition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Definition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration: Option<::ValueMap<String>> = None;
                    let mut data_type: Option<::Value<DataType>> = None;
                    let mut default_value: Option<::Value<DataValue>> = None;
                    let mut is_external_id: Option<::Value<bool>> = None;
                    let mut is_final: Option<::Value<bool>> = None;
                    let mut is_imported: Option<::Value<bool>> = None;
                    let mut is_inherited: Option<::Value<bool>> = None;
                    let mut is_required_in_entity: Option<::Value<bool>> = None;
                    let mut is_stored_externally: Option<::Value<bool>> = None;
                    let mut is_time_series: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configuration" => {
                                configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsExternalId" => {
                                is_external_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsFinal" => {
                                is_final = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsImported" => {
                                is_imported = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsInherited" => {
                                is_inherited = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsRequiredInEntity" => {
                                is_required_in_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsStoredExternally" => {
                                is_stored_externally = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsTimeSeries" => {
                                is_time_series = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Definition {
                        configuration: configuration,
                        data_type: data_type,
                        default_value: default_value,
                        is_external_id: is_external_id,
                        is_final: is_final,
                        is_imported: is_imported,
                        is_inherited: is_inherited,
                        is_required_in_entity: is_required_in_entity,
                        is_stored_externally: is_stored_externally,
                        is_time_series: is_time_series,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.Error`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-error.html) property type.
    #[derive(Debug, Default)]
    pub struct Error {
        /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-error.html#cfn-iottwinmaker-entity-error-code).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code: Option<::Value<String>>,
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-error.html#cfn-iottwinmaker-entity-error-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Error {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code) = self.code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", code)?;
            }
            if let Some(ref message) = self.message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Error {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Error, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Error;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Error")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code: Option<::Value<String>> = None;
                    let mut message: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Code" => {
                                code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Error {
                        code: code,
                        message: message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-property.html) property type.
    #[derive(Debug, Default)]
    pub struct Property {
        /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-property.html#cfn-iottwinmaker-entity-property-definition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub definition: Option<::Value<Definition>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-property.html#cfn-iottwinmaker-entity-property-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<DataValue>>,
    }

    impl ::codec::SerializeValue for Property {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref definition) = self.definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", definition)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Property {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Property, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Property;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Property")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut definition: Option<::Value<Definition>> = None;
                    let mut value: Option<::Value<DataValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Definition" => {
                                definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Property {
                        definition: definition,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.PropertyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-propertygroup.html) property type.
    #[derive(Debug, Default)]
    pub struct PropertyGroup {
        /// Property [`GroupType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-propertygroup.html#cfn-iottwinmaker-entity-propertygroup-grouptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_type: Option<::Value<String>>,
        /// Property [`PropertyNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-propertygroup.html#cfn-iottwinmaker-entity-propertygroup-propertynames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PropertyGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group_type) = self.group_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupType", group_type)?;
            }
            if let Some(ref property_names) = self.property_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyNames", property_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PropertyGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PropertyGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PropertyGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PropertyGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_type: Option<::Value<String>> = None;
                    let mut property_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupType" => {
                                group_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyNames" => {
                                property_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PropertyGroup {
                        group_type: group_type,
                        property_names: property_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.Relationship`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationship.html) property type.
    #[derive(Debug, Default)]
    pub struct Relationship {
        /// Property [`RelationshipType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationship.html#cfn-iottwinmaker-entity-relationship-relationshiptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relationship_type: Option<::Value<String>>,
        /// Property [`TargetComponentTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationship.html#cfn-iottwinmaker-entity-relationship-targetcomponenttypeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_component_type_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Relationship {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref relationship_type) = self.relationship_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationshipType", relationship_type)?;
            }
            if let Some(ref target_component_type_id) = self.target_component_type_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetComponentTypeId", target_component_type_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Relationship {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Relationship, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Relationship;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Relationship")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut relationship_type: Option<::Value<String>> = None;
                    let mut target_component_type_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RelationshipType" => {
                                relationship_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetComponentTypeId" => {
                                target_component_type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Relationship {
                        relationship_type: relationship_type,
                        target_component_type_id: target_component_type_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.RelationshipValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationshipvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationshipValue {
        /// Property [`TargetComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationshipvalue.html#cfn-iottwinmaker-entity-relationshipvalue-targetcomponentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_component_name: Option<::Value<String>>,
        /// Property [`TargetEntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationshipvalue.html#cfn-iottwinmaker-entity-relationshipvalue-targetentityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_entity_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RelationshipValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_component_name) = self.target_component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetComponentName", target_component_name)?;
            }
            if let Some(ref target_entity_id) = self.target_entity_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEntityId", target_entity_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationshipValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationshipValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationshipValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationshipValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_component_name: Option<::Value<String>> = None;
                    let mut target_entity_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetComponentName" => {
                                target_component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetEntityId" => {
                                target_entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationshipValue {
                        target_component_name: target_component_name,
                        target_entity_id: target_entity_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTTwinMaker::Entity.Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-status.html) property type.
    #[derive(Debug, Default)]
    pub struct Status {
        /// Property [`Error`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-status.html#cfn-iottwinmaker-entity-status-error).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error: Option<::Value<Error>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-status.html#cfn-iottwinmaker-entity-status-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Status {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error) = self.error {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Error", error)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Status {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Status, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Status;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Status")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error: Option<::Value<Error>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Error" => {
                                error = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Status {
                        error: error,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
