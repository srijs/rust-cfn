//! Types for the `AmplifyUIBuilder` service.

/// The [`AWS::AmplifyUIBuilder::Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html) resource type.
#[derive(Debug, Default)]
pub struct Component {
    properties: ComponentProperties
}

/// Properties for the `Component` resource.
#[derive(Debug, Default)]
pub struct ComponentProperties {
    /// Property [`BindingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-bindingproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub binding_properties: ::ValueMap<self::component::ComponentBindingPropertiesValue>,
    /// Property [`Children`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-children).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub children: Option<::ValueList<self::component::ComponentChild>>,
    /// Property [`CollectionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-collectionproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub collection_properties: Option<::ValueMap<self::component::ComponentDataConfiguration>>,
    /// Property [`ComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-componenttype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub component_type: ::Value<String>,
    /// Property [`Events`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-events).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub events: Option<::ValueMap<self::component::ComponentEvent>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-overrides).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub overrides: ::ValueMap<self::component::ComponentOverridesValue>,
    /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-properties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub properties: ::ValueMap<self::component::ComponentProperty>,
    /// Property [`SchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-schemaversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema_version: Option<::Value<String>>,
    /// Property [`SourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-sourceid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Variants`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-variants).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub variants: ::ValueList<self::component::ComponentVariant>,
}

impl ::serde::Serialize for ComponentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BindingProperties", &self.binding_properties)?;
        if let Some(ref children) = self.children {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Children", children)?;
        }
        if let Some(ref collection_properties) = self.collection_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionProperties", collection_properties)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentType", &self.component_type)?;
        if let Some(ref events) = self.events {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", events)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", &self.overrides)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", &self.properties)?;
        if let Some(ref schema_version) = self.schema_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersion", schema_version)?;
        }
        if let Some(ref source_id) = self.source_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceId", source_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variants", &self.variants)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComponentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComponentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComponentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut binding_properties: Option<::ValueMap<self::component::ComponentBindingPropertiesValue>> = None;
                let mut children: Option<::ValueList<self::component::ComponentChild>> = None;
                let mut collection_properties: Option<::ValueMap<self::component::ComponentDataConfiguration>> = None;
                let mut component_type: Option<::Value<String>> = None;
                let mut events: Option<::ValueMap<self::component::ComponentEvent>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut overrides: Option<::ValueMap<self::component::ComponentOverridesValue>> = None;
                let mut properties: Option<::ValueMap<self::component::ComponentProperty>> = None;
                let mut schema_version: Option<::Value<String>> = None;
                let mut source_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut variants: Option<::ValueList<self::component::ComponentVariant>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BindingProperties" => {
                            binding_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Children" => {
                            children = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CollectionProperties" => {
                            collection_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComponentType" => {
                            component_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Events" => {
                            events = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Overrides" => {
                            overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Properties" => {
                            properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaVersion" => {
                            schema_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceId" => {
                            source_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Variants" => {
                            variants = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ComponentProperties {
                    binding_properties: binding_properties.ok_or(::serde::de::Error::missing_field("BindingProperties"))?,
                    children: children,
                    collection_properties: collection_properties,
                    component_type: component_type.ok_or(::serde::de::Error::missing_field("ComponentType"))?,
                    events: events,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    overrides: overrides.ok_or(::serde::de::Error::missing_field("Overrides"))?,
                    properties: properties.ok_or(::serde::de::Error::missing_field("Properties"))?,
                    schema_version: schema_version,
                    source_id: source_id,
                    tags: tags,
                    variants: variants.ok_or(::serde::de::Error::missing_field("Variants"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Component {
    type Properties = ComponentProperties;
    const TYPE: &'static str = "AWS::AmplifyUIBuilder::Component";
    fn properties(&self) -> &ComponentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComponentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Component {}

impl From<ComponentProperties> for Component {
    fn from(properties: ComponentProperties) -> Component {
        Component { properties }
    }
}

/// The [`AWS::AmplifyUIBuilder::Theme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html) resource type.
#[derive(Debug, Default)]
pub struct Theme {
    properties: ThemeProperties
}

/// Properties for the `Theme` resource.
#[derive(Debug, Default)]
pub struct ThemeProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-overrides).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub overrides: Option<::ValueList<self::theme::ThemeValues>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-values).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub values: ::ValueList<self::theme::ThemeValues>,
}

impl ::serde::Serialize for ThemeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref overrides) = self.overrides {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", overrides)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThemeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThemeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThemeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThemeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut overrides: Option<::ValueList<self::theme::ThemeValues>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut values: Option<::ValueList<self::theme::ThemeValues>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Overrides" => {
                            overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Values" => {
                            values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThemeProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    overrides: overrides,
                    tags: tags,
                    values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Theme {
    type Properties = ThemeProperties;
    const TYPE: &'static str = "AWS::AmplifyUIBuilder::Theme";
    fn properties(&self) -> &ThemeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThemeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Theme {}

impl From<ThemeProperties> for Theme {
    fn from(properties: ThemeProperties) -> Theme {
        Theme { properties }
    }
}

pub mod component {
    //! Property types for the `Component` resource.

    /// The [`AWS::AmplifyUIBuilder::Component.ActionParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionParameters {
        /// Property [`Anchor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-anchor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub anchor: Option<::Value<ComponentProperty>>,
        /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-fields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fields: Option<::Value<ComponentProperties>>,
        /// Property [`Global`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-global).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub global: Option<::Value<ComponentProperty>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<ComponentProperty>>,
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: Option<::Value<String>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<MutationActionSetStateParameter>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<ComponentProperty>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<ComponentProperty>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html#cfn-amplifyuibuilder-component-actionparameters-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<ComponentProperty>>,
    }

    impl ::codec::SerializeValue for ActionParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref anchor) = self.anchor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Anchor", anchor)?;
            }
            if let Some(ref fields) = self.fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", fields)?;
            }
            if let Some(ref global) = self.global {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Global", global)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref model) = self.model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut anchor: Option<::Value<ComponentProperty>> = None;
                    let mut fields: Option<::Value<ComponentProperties>> = None;
                    let mut global: Option<::Value<ComponentProperty>> = None;
                    let mut id: Option<::Value<ComponentProperty>> = None;
                    let mut model: Option<::Value<String>> = None;
                    let mut state: Option<::Value<MutationActionSetStateParameter>> = None;
                    let mut target: Option<::Value<ComponentProperty>> = None;
                    let mut r#type: Option<::Value<ComponentProperty>> = None;
                    let mut url: Option<::Value<ComponentProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Anchor" => {
                                anchor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Fields" => {
                                fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Global" => {
                                global = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionParameters {
                        anchor: anchor,
                        fields: fields,
                        global: global,
                        id: id,
                        model: model,
                        state: state,
                        target: target,
                        r#type: r#type,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentBindingPropertiesValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentBindingPropertiesValue {
        /// Property [`BindingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalue.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalue-bindingproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub binding_properties: Option<::Value<ComponentBindingPropertiesValueProperties>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalue.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalue-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalue.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalue-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentBindingPropertiesValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref binding_properties) = self.binding_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BindingProperties", binding_properties)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentBindingPropertiesValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentBindingPropertiesValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentBindingPropertiesValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentBindingPropertiesValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut binding_properties: Option<::Value<ComponentBindingPropertiesValueProperties>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BindingProperties" => {
                                binding_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentBindingPropertiesValue {
                        binding_properties: binding_properties,
                        default_value: default_value,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentBindingPropertiesValueProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentBindingPropertiesValueProperties {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: Option<::Value<String>>,
        /// Property [`Predicates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-predicates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predicates: Option<::ValueList<Predicate>>,
        /// Property [`UserAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html#cfn-amplifyuibuilder-component-componentbindingpropertiesvalueproperties-userattribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_attribute: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentBindingPropertiesValueProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref model) = self.model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
            }
            if let Some(ref predicates) = self.predicates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Predicates", predicates)?;
            }
            if let Some(ref user_attribute) = self.user_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttribute", user_attribute)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentBindingPropertiesValueProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentBindingPropertiesValueProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentBindingPropertiesValueProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentBindingPropertiesValueProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut field: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut model: Option<::Value<String>> = None;
                    let mut predicates: Option<::ValueList<Predicate>> = None;
                    let mut user_attribute: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Predicates" => {
                                predicates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAttribute" => {
                                user_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentBindingPropertiesValueProperties {
                        bucket: bucket,
                        default_value: default_value,
                        field: field,
                        key: key,
                        model: model,
                        predicates: predicates,
                        user_attribute: user_attribute,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentChild`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentChild {
        /// Property [`Children`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-children).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub children: Option<::ValueList<ComponentChild>>,
        /// Property [`ComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-componenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_type: ::Value<String>,
        /// Property [`Events`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-events).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events: Option<::Value<ComponentEvents>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-properties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub properties: ::Value<ComponentProperties>,
    }

    impl ::codec::SerializeValue for ComponentChild {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref children) = self.children {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Children", children)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentType", &self.component_type)?;
            if let Some(ref events) = self.events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", events)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", &self.properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentChild {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentChild, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentChild;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentChild")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut children: Option<::ValueList<ComponentChild>> = None;
                    let mut component_type: Option<::Value<String>> = None;
                    let mut events: Option<::Value<ComponentEvents>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut properties: Option<::Value<ComponentProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Children" => {
                                children = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentType" => {
                                component_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Events" => {
                                events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Properties" => {
                                properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentChild {
                        children: children,
                        component_type: component_type.ok_or(::serde::de::Error::missing_field("ComponentType"))?,
                        events: events,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        properties: properties.ok_or(::serde::de::Error::missing_field("Properties"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentConditionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentConditionProperty {
        /// Property [`Else`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-else).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#else: Option<::Value<ComponentProperty>>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Operand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-operand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operand: Option<::Value<String>>,
        /// Property [`OperandType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-operandtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operand_type: Option<::Value<String>>,
        /// Property [`Operator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-operator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator: Option<::Value<String>>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: Option<::Value<String>>,
        /// Property [`Then`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html#cfn-amplifyuibuilder-component-componentconditionproperty-then).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub then: Option<::Value<ComponentProperty>>,
    }

    impl ::codec::SerializeValue for ComponentConditionProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#else) = self.r#else {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Else", r#else)?;
            }
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref operand) = self.operand {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operand", operand)?;
            }
            if let Some(ref operand_type) = self.operand_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperandType", operand_type)?;
            }
            if let Some(ref operator) = self.operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operator", operator)?;
            }
            if let Some(ref property) = self.property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", property)?;
            }
            if let Some(ref then) = self.then {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Then", then)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentConditionProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentConditionProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentConditionProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentConditionProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#else: Option<::Value<ComponentProperty>> = None;
                    let mut field: Option<::Value<String>> = None;
                    let mut operand: Option<::Value<String>> = None;
                    let mut operand_type: Option<::Value<String>> = None;
                    let mut operator: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;
                    let mut then: Option<::Value<ComponentProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Else" => {
                                r#else = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operand" => {
                                operand = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OperandType" => {
                                operand_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operator" => {
                                operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Then" => {
                                then = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentConditionProperty {
                        r#else: r#else,
                        field: field,
                        operand: operand,
                        operand_type: operand_type,
                        operator: operator,
                        property: property,
                        then: then,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentDataConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentDataConfiguration {
        /// Property [`Identifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html#cfn-amplifyuibuilder-component-componentdataconfiguration-identifiers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifiers: Option<::ValueList<String>>,
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html#cfn-amplifyuibuilder-component-componentdataconfiguration-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: ::Value<String>,
        /// Property [`Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html#cfn-amplifyuibuilder-component-componentdataconfiguration-predicate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predicate: Option<::Value<Predicate>>,
        /// Property [`Sort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html#cfn-amplifyuibuilder-component-componentdataconfiguration-sort).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sort: Option<::ValueList<SortProperty>>,
    }

    impl ::codec::SerializeValue for ComponentDataConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref identifiers) = self.identifiers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifiers", identifiers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", &self.model)?;
            if let Some(ref predicate) = self.predicate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Predicate", predicate)?;
            }
            if let Some(ref sort) = self.sort {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sort", sort)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentDataConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentDataConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentDataConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentDataConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut identifiers: Option<::ValueList<String>> = None;
                    let mut model: Option<::Value<String>> = None;
                    let mut predicate: Option<::Value<Predicate>> = None;
                    let mut sort: Option<::ValueList<SortProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Identifiers" => {
                                identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Predicate" => {
                                predicate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sort" => {
                                sort = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentDataConfiguration {
                        identifiers: identifiers,
                        model: model.ok_or(::serde::de::Error::missing_field("Model"))?,
                        predicate: predicate,
                        sort: sort,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentEvent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentevent.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentEvent {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentevent.html#cfn-amplifyuibuilder-component-componentevent-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentevent.html#cfn-amplifyuibuilder-component-componentevent-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<ActionParameters>>,
    }

    impl ::codec::SerializeValue for ComponentEvent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentEvent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentEvent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentEvent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentEvent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<ActionParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentEvent {
                        action: action,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentevents.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentEvents {
    }

    impl ::codec::SerializeValue for ComponentEvents {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentEvents {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentEvents, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentEvents;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentEvents")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ComponentEvents {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentoverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentOverrides {
    }

    impl ::codec::SerializeValue for ComponentOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ComponentOverrides {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentOverridesValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentoverridesvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentOverridesValue {
    }

    impl ::codec::SerializeValue for ComponentOverridesValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentOverridesValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentOverridesValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentOverridesValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentOverridesValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ComponentOverridesValue {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentProperties {
    }

    impl ::codec::SerializeValue for ComponentProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ComponentProperties {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentProperty {
        /// Property [`BindingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-bindingproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub binding_properties: Option<::Value<ComponentPropertyBindingProperties>>,
        /// Property [`Bindings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-bindings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bindings: Option<::Value<FormBindings>>,
        /// Property [`CollectionBindingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-collectionbindingproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub collection_binding_properties: Option<::Value<ComponentPropertyBindingProperties>>,
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: Option<::Value<String>>,
        /// Property [`Concat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-concat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub concat: Option<::ValueList<ComponentProperty>>,
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: Option<::Value<ComponentConditionProperty>>,
        /// Property [`Configured`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-configured).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configured: Option<::Value<bool>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-event).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event: Option<::Value<String>>,
        /// Property [`ImportedValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-importedvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub imported_value: Option<::Value<String>>,
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: Option<::Value<String>>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`UserAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-userattribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_attribute: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html#cfn-amplifyuibuilder-component-componentproperty-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ComponentProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref binding_properties) = self.binding_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BindingProperties", binding_properties)?;
            }
            if let Some(ref bindings) = self.bindings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bindings", bindings)?;
            }
            if let Some(ref collection_binding_properties) = self.collection_binding_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionBindingProperties", collection_binding_properties)?;
            }
            if let Some(ref component_name) = self.component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", component_name)?;
            }
            if let Some(ref concat) = self.concat {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Concat", concat)?;
            }
            if let Some(ref condition) = self.condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
            }
            if let Some(ref configured) = self.configured {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configured", configured)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref event) = self.event {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Event", event)?;
            }
            if let Some(ref imported_value) = self.imported_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImportedValue", imported_value)?;
            }
            if let Some(ref model) = self.model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
            }
            if let Some(ref property) = self.property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", property)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref user_attribute) = self.user_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttribute", user_attribute)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut binding_properties: Option<::Value<ComponentPropertyBindingProperties>> = None;
                    let mut bindings: Option<::Value<FormBindings>> = None;
                    let mut collection_binding_properties: Option<::Value<ComponentPropertyBindingProperties>> = None;
                    let mut component_name: Option<::Value<String>> = None;
                    let mut concat: Option<::ValueList<ComponentProperty>> = None;
                    let mut condition: Option<::Value<ComponentConditionProperty>> = None;
                    let mut configured: Option<::Value<bool>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut event: Option<::Value<String>> = None;
                    let mut imported_value: Option<::Value<String>> = None;
                    let mut model: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut user_attribute: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BindingProperties" => {
                                binding_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bindings" => {
                                bindings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CollectionBindingProperties" => {
                                collection_binding_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Concat" => {
                                concat = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configured" => {
                                configured = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Event" => {
                                event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImportedValue" => {
                                imported_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAttribute" => {
                                user_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentProperty {
                        binding_properties: binding_properties,
                        bindings: bindings,
                        collection_binding_properties: collection_binding_properties,
                        component_name: component_name,
                        concat: concat,
                        condition: condition,
                        configured: configured,
                        default_value: default_value,
                        event: event,
                        imported_value: imported_value,
                        model: model,
                        property: property,
                        r#type: r#type,
                        user_attribute: user_attribute,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentPropertyBindingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentpropertybindingproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentPropertyBindingProperties {
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentpropertybindingproperties.html#cfn-amplifyuibuilder-component-componentpropertybindingproperties-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentpropertybindingproperties.html#cfn-amplifyuibuilder-component-componentpropertybindingproperties-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: ::Value<String>,
    }

    impl ::codec::SerializeValue for ComponentPropertyBindingProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentPropertyBindingProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentPropertyBindingProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentPropertyBindingProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentPropertyBindingProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentPropertyBindingProperties {
                        field: field,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentVariant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariant.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentVariant {
        /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariant.html#cfn-amplifyuibuilder-component-componentvariant-overrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overrides: Option<::Value<ComponentOverrides>>,
        /// Property [`VariantValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariant.html#cfn-amplifyuibuilder-component-componentvariant-variantvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variant_values: Option<::Value<ComponentVariantValues>>,
    }

    impl ::codec::SerializeValue for ComponentVariant {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref overrides) = self.overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", overrides)?;
            }
            if let Some(ref variant_values) = self.variant_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariantValues", variant_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentVariant {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentVariant, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentVariant;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentVariant")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut overrides: Option<::Value<ComponentOverrides>> = None;
                    let mut variant_values: Option<::Value<ComponentVariantValues>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Overrides" => {
                                overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariantValues" => {
                                variant_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentVariant {
                        overrides: overrides,
                        variant_values: variant_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.ComponentVariantValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariantvalues.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentVariantValues {
    }

    impl ::codec::SerializeValue for ComponentVariantValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentVariantValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentVariantValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentVariantValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentVariantValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ComponentVariantValues {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.FormBindings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-formbindings.html) property type.
    #[derive(Debug, Default)]
    pub struct FormBindings {
    }

    impl ::codec::SerializeValue for FormBindings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormBindings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormBindings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormBindings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormBindings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(FormBindings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.MutationActionSetStateParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-mutationactionsetstateparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct MutationActionSetStateParameter {
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-mutationactionsetstateparameter.html#cfn-amplifyuibuilder-component-mutationactionsetstateparameter-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: ::Value<String>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-mutationactionsetstateparameter.html#cfn-amplifyuibuilder-component-mutationactionsetstateparameter-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: ::Value<String>,
        /// Property [`Set`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-mutationactionsetstateparameter.html#cfn-amplifyuibuilder-component-mutationactionsetstateparameter-set).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set: ::Value<ComponentProperty>,
    }

    impl ::codec::SerializeValue for MutationActionSetStateParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", &self.component_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Set", &self.set)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MutationActionSetStateParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MutationActionSetStateParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MutationActionSetStateParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MutationActionSetStateParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_name: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;
                    let mut set: Option<::Value<ComponentProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Set" => {
                                set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MutationActionSetStateParameter {
                        component_name: component_name.ok_or(::serde::de::Error::missing_field("ComponentName"))?,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                        set: set.ok_or(::serde::de::Error::missing_field("Set"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html) property type.
    #[derive(Debug, Default)]
    pub struct Predicate {
        /// Property [`And`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html#cfn-amplifyuibuilder-component-predicate-and).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub and: Option<::ValueList<Predicate>>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html#cfn-amplifyuibuilder-component-predicate-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Operand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html#cfn-amplifyuibuilder-component-predicate-operand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operand: Option<::Value<String>>,
        /// Property [`Operator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html#cfn-amplifyuibuilder-component-predicate-operator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator: Option<::Value<String>>,
        /// Property [`Or`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html#cfn-amplifyuibuilder-component-predicate-or).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub or: Option<::ValueList<Predicate>>,
    }

    impl ::codec::SerializeValue for Predicate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref and) = self.and {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "And", and)?;
            }
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref operand) = self.operand {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operand", operand)?;
            }
            if let Some(ref operator) = self.operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operator", operator)?;
            }
            if let Some(ref or) = self.or {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Or", or)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Predicate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Predicate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Predicate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Predicate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut and: Option<::ValueList<Predicate>> = None;
                    let mut field: Option<::Value<String>> = None;
                    let mut operand: Option<::Value<String>> = None;
                    let mut operator: Option<::Value<String>> = None;
                    let mut or: Option<::ValueList<Predicate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "And" => {
                                and = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operand" => {
                                operand = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operator" => {
                                operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Or" => {
                                or = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Predicate {
                        and: and,
                        field: field,
                        operand: operand,
                        operator: operator,
                        or: or,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Component.SortProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-sortproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct SortProperty {
        /// Property [`Direction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-sortproperty.html#cfn-amplifyuibuilder-component-sortproperty-direction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub direction: ::Value<String>,
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-sortproperty.html#cfn-amplifyuibuilder-component-sortproperty-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: ::Value<String>,
    }

    impl ::codec::SerializeValue for SortProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Direction", &self.direction)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", &self.field)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SortProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SortProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SortProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SortProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut direction: Option<::Value<String>> = None;
                    let mut field: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Direction" => {
                                direction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SortProperty {
                        direction: direction.ok_or(::serde::de::Error::missing_field("Direction"))?,
                        field: field.ok_or(::serde::de::Error::missing_field("Field"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod theme {
    //! Property types for the `Theme` resource.

    /// The [`AWS::AmplifyUIBuilder::Theme.ThemeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct ThemeValue {
        /// Property [`Children`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalue.html#cfn-amplifyuibuilder-theme-themevalue-children).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub children: Option<::ValueList<ThemeValues>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalue.html#cfn-amplifyuibuilder-theme-themevalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ThemeValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref children) = self.children {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Children", children)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThemeValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThemeValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThemeValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThemeValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut children: Option<::ValueList<ThemeValues>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Children" => {
                                children = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ThemeValue {
                        children: children,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Theme.ThemeValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalues.html) property type.
    #[derive(Debug, Default)]
    pub struct ThemeValues {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalues.html#cfn-amplifyuibuilder-theme-themevalues-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalues.html#cfn-amplifyuibuilder-theme-themevalues-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<ThemeValue>>,
    }

    impl ::codec::SerializeValue for ThemeValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThemeValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThemeValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThemeValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThemeValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<ThemeValue>> = None;

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

                    Ok(ThemeValues {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
