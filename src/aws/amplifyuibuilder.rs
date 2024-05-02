//! Types for the `AmplifyUIBuilder` service.

/// The [`AWS::AmplifyUIBuilder::Component`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html) resource type.
#[derive(Debug, Default)]
pub struct Component {
    properties: ComponentProperties
}

/// Properties for the `Component` resource.
#[derive(Debug, Default)]
pub struct ComponentProperties {
    /// Property [`AppId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-appid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_id: Option<::Value<String>>,
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
    /// Property [`EnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html#cfn-amplifyuibuilder-component-environmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_name: Option<::Value<String>>,
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
    pub overrides: ::Value<::json::Value>,
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
        if let Some(ref app_id) = self.app_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppId", app_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BindingProperties", &self.binding_properties)?;
        if let Some(ref children) = self.children {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Children", children)?;
        }
        if let Some(ref collection_properties) = self.collection_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionProperties", collection_properties)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentType", &self.component_type)?;
        if let Some(ref environment_name) = self.environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentName", environment_name)?;
        }
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
                let mut app_id: Option<::Value<String>> = None;
                let mut binding_properties: Option<::ValueMap<self::component::ComponentBindingPropertiesValue>> = None;
                let mut children: Option<::ValueList<self::component::ComponentChild>> = None;
                let mut collection_properties: Option<::ValueMap<self::component::ComponentDataConfiguration>> = None;
                let mut component_type: Option<::Value<String>> = None;
                let mut environment_name: Option<::Value<String>> = None;
                let mut events: Option<::ValueMap<self::component::ComponentEvent>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut overrides: Option<::Value<::json::Value>> = None;
                let mut properties: Option<::ValueMap<self::component::ComponentProperty>> = None;
                let mut schema_version: Option<::Value<String>> = None;
                let mut source_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut variants: Option<::ValueList<self::component::ComponentVariant>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppId" => {
                            app_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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
                        "EnvironmentName" => {
                            environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    app_id: app_id,
                    binding_properties: binding_properties.ok_or(::serde::de::Error::missing_field("BindingProperties"))?,
                    children: children,
                    collection_properties: collection_properties,
                    component_type: component_type.ok_or(::serde::de::Error::missing_field("ComponentType"))?,
                    environment_name: environment_name,
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

/// The [`AWS::AmplifyUIBuilder::Form`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html) resource type.
#[derive(Debug, Default)]
pub struct Form {
    properties: FormProperties
}

/// Properties for the `Form` resource.
#[derive(Debug, Default)]
pub struct FormProperties {
    /// Property [`AppId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-appid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_id: Option<::Value<String>>,
    /// Property [`Cta`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-cta).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cta: Option<::Value<self::form::FormCTA>>,
    /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-datatype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_type: ::Value<self::form::FormDataTypeConfig>,
    /// Property [`EnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-environmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_name: Option<::Value<String>>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: ::ValueMap<self::form::FieldConfig>,
    /// Property [`FormActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-formactiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub form_action_type: ::Value<String>,
    /// Property [`LabelDecorator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-labeldecorator).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub label_decorator: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-schemaversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema_version: ::Value<String>,
    /// Property [`SectionalElements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-sectionalelements).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sectional_elements: ::ValueMap<self::form::SectionalElement>,
    /// Property [`Style`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-style).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub style: ::Value<self::form::FormStyle>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html#cfn-amplifyuibuilder-form-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for FormProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_id) = self.app_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppId", app_id)?;
        }
        if let Some(ref cta) = self.cta {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cta", cta)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
        if let Some(ref environment_name) = self.environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentName", environment_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormActionType", &self.form_action_type)?;
        if let Some(ref label_decorator) = self.label_decorator {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelDecorator", label_decorator)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersion", &self.schema_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SectionalElements", &self.sectional_elements)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Style", &self.style)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FormProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FormProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FormProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FormProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_id: Option<::Value<String>> = None;
                let mut cta: Option<::Value<self::form::FormCTA>> = None;
                let mut data_type: Option<::Value<self::form::FormDataTypeConfig>> = None;
                let mut environment_name: Option<::Value<String>> = None;
                let mut fields: Option<::ValueMap<self::form::FieldConfig>> = None;
                let mut form_action_type: Option<::Value<String>> = None;
                let mut label_decorator: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schema_version: Option<::Value<String>> = None;
                let mut sectional_elements: Option<::ValueMap<self::form::SectionalElement>> = None;
                let mut style: Option<::Value<self::form::FormStyle>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppId" => {
                            app_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cta" => {
                            cta = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataType" => {
                            data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentName" => {
                            environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FormActionType" => {
                            form_action_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LabelDecorator" => {
                            label_decorator = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaVersion" => {
                            schema_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SectionalElements" => {
                            sectional_elements = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Style" => {
                            style = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FormProperties {
                    app_id: app_id,
                    cta: cta,
                    data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                    environment_name: environment_name,
                    fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                    form_action_type: form_action_type.ok_or(::serde::de::Error::missing_field("FormActionType"))?,
                    label_decorator: label_decorator,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schema_version: schema_version.ok_or(::serde::de::Error::missing_field("SchemaVersion"))?,
                    sectional_elements: sectional_elements.ok_or(::serde::de::Error::missing_field("SectionalElements"))?,
                    style: style.ok_or(::serde::de::Error::missing_field("Style"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Form {
    type Properties = FormProperties;
    const TYPE: &'static str = "AWS::AmplifyUIBuilder::Form";
    fn properties(&self) -> &FormProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FormProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Form {}

impl From<FormProperties> for Form {
    fn from(properties: FormProperties) -> Form {
        Form { properties }
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
    /// Property [`AppId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-appid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_id: Option<::Value<String>>,
    /// Property [`EnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html#cfn-amplifyuibuilder-theme-environmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_name: Option<::Value<String>>,
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
        if let Some(ref app_id) = self.app_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppId", app_id)?;
        }
        if let Some(ref environment_name) = self.environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentName", environment_name)?;
        }
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
                let mut app_id: Option<::Value<String>> = None;
                let mut environment_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut overrides: Option<::ValueList<self::theme::ThemeValues>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut values: Option<::ValueList<self::theme::ThemeValues>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppId" => {
                            app_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentName" => {
                            environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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
                    app_id: app_id,
                    environment_name: environment_name,
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
        pub fields: Option<::ValueMap<ComponentProperty>>,
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
                    let mut fields: Option<::ValueMap<ComponentProperty>> = None;
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
        pub events: Option<::ValueMap<ComponentEvent>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html#cfn-amplifyuibuilder-component-componentchild-properties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub properties: ::ValueMap<ComponentProperty>,
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
                    let mut events: Option<::ValueMap<ComponentEvent>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut properties: Option<::ValueMap<ComponentProperty>> = None;

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
        pub bindings: Option<::ValueMap<FormBindingElement>>,
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
                    let mut bindings: Option<::ValueMap<FormBindingElement>> = None;
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
        pub overrides: Option<::Value<::json::Value>>,
        /// Property [`VariantValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariant.html#cfn-amplifyuibuilder-component-componentvariant-variantvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variant_values: Option<::ValueMap<String>>,
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
                    let mut overrides: Option<::Value<::json::Value>> = None;
                    let mut variant_values: Option<::ValueMap<String>> = None;

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

    /// The [`AWS::AmplifyUIBuilder::Component.FormBindingElement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-formbindingelement.html) property type.
    #[derive(Debug, Default)]
    pub struct FormBindingElement {
        /// Property [`Element`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-formbindingelement.html#cfn-amplifyuibuilder-component-formbindingelement-element).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub element: ::Value<String>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-formbindingelement.html#cfn-amplifyuibuilder-component-formbindingelement-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: ::Value<String>,
    }

    impl ::codec::SerializeValue for FormBindingElement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Element", &self.element)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormBindingElement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormBindingElement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormBindingElement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormBindingElement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut element: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Element" => {
                                element = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormBindingElement {
                        element: element.ok_or(::serde::de::Error::missing_field("Element"))?,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                    })
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

pub mod form {
    //! Property types for the `Form` resource.

    /// The [`AWS::AmplifyUIBuilder::Form.FieldConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldConfig {
        /// Property [`Excluded`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html#cfn-amplifyuibuilder-form-fieldconfig-excluded).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded: Option<::Value<bool>>,
        /// Property [`InputType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html#cfn-amplifyuibuilder-form-fieldconfig-inputtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_type: Option<::Value<FieldInputConfig>>,
        /// Property [`Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html#cfn-amplifyuibuilder-form-fieldconfig-label).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label: Option<::Value<String>>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html#cfn-amplifyuibuilder-form-fieldconfig-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: Option<::Value<FieldPosition>>,
        /// Property [`Validations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html#cfn-amplifyuibuilder-form-fieldconfig-validations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validations: Option<::ValueList<FieldValidationConfiguration>>,
    }

    impl ::codec::SerializeValue for FieldConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded) = self.excluded {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Excluded", excluded)?;
            }
            if let Some(ref input_type) = self.input_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputType", input_type)?;
            }
            if let Some(ref label) = self.label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Label", label)?;
            }
            if let Some(ref position) = self.position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
            }
            if let Some(ref validations) = self.validations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validations", validations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded: Option<::Value<bool>> = None;
                    let mut input_type: Option<::Value<FieldInputConfig>> = None;
                    let mut label: Option<::Value<String>> = None;
                    let mut position: Option<::Value<FieldPosition>> = None;
                    let mut validations: Option<::ValueList<FieldValidationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Excluded" => {
                                excluded = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputType" => {
                                input_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Label" => {
                                label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Validations" => {
                                validations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldConfig {
                        excluded: excluded,
                        input_type: input_type,
                        label: label,
                        position: position,
                        validations: validations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FieldInputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldInputConfig {
        /// Property [`DefaultChecked`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-defaultchecked).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_checked: Option<::Value<bool>>,
        /// Property [`DefaultCountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-defaultcountrycode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_country_code: Option<::Value<String>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`DescriptiveText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-descriptivetext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub descriptive_text: Option<::Value<String>>,
        /// Property [`FileUploaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-fileuploaderconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_uploader_config: Option<::Value<FileUploaderFieldConfig>>,
        /// Property [`IsArray`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-isarray).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_array: Option<::Value<bool>>,
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-maxvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_value: Option<::Value<f64>>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-minvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_value: Option<::Value<f64>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Placeholder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-placeholder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub placeholder: Option<::Value<String>>,
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-readonly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_only: Option<::Value<bool>>,
        /// Property [`Required`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-required).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required: Option<::Value<bool>>,
        /// Property [`Step`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-step).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub step: Option<::Value<f64>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
        /// Property [`ValueMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html#cfn-amplifyuibuilder-form-fieldinputconfig-valuemappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_mappings: Option<::Value<ValueMappings>>,
    }

    impl ::codec::SerializeValue for FieldInputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_checked) = self.default_checked {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultChecked", default_checked)?;
            }
            if let Some(ref default_country_code) = self.default_country_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCountryCode", default_country_code)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref descriptive_text) = self.descriptive_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DescriptiveText", descriptive_text)?;
            }
            if let Some(ref file_uploader_config) = self.file_uploader_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileUploaderConfig", file_uploader_config)?;
            }
            if let Some(ref is_array) = self.is_array {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsArray", is_array)?;
            }
            if let Some(ref max_value) = self.max_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", max_value)?;
            }
            if let Some(ref min_value) = self.min_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", min_value)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref placeholder) = self.placeholder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Placeholder", placeholder)?;
            }
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref required) = self.required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", required)?;
            }
            if let Some(ref step) = self.step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Step", step)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            if let Some(ref value_mappings) = self.value_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueMappings", value_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldInputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldInputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldInputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldInputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_checked: Option<::Value<bool>> = None;
                    let mut default_country_code: Option<::Value<String>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut descriptive_text: Option<::Value<String>> = None;
                    let mut file_uploader_config: Option<::Value<FileUploaderFieldConfig>> = None;
                    let mut is_array: Option<::Value<bool>> = None;
                    let mut max_value: Option<::Value<f64>> = None;
                    let mut min_value: Option<::Value<f64>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut placeholder: Option<::Value<String>> = None;
                    let mut read_only: Option<::Value<bool>> = None;
                    let mut required: Option<::Value<bool>> = None;
                    let mut step: Option<::Value<f64>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;
                    let mut value_mappings: Option<::Value<ValueMappings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultChecked" => {
                                default_checked = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultCountryCode" => {
                                default_country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DescriptiveText" => {
                                descriptive_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileUploaderConfig" => {
                                file_uploader_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsArray" => {
                                is_array = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Placeholder" => {
                                placeholder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Required" => {
                                required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Step" => {
                                step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueMappings" => {
                                value_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldInputConfig {
                        default_checked: default_checked,
                        default_country_code: default_country_code,
                        default_value: default_value,
                        descriptive_text: descriptive_text,
                        file_uploader_config: file_uploader_config,
                        is_array: is_array,
                        max_value: max_value,
                        min_value: min_value,
                        name: name,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        step: step,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value,
                        value_mappings: value_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FieldPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldposition.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldPosition {
        /// Property [`Below`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldposition.html#cfn-amplifyuibuilder-form-fieldposition-below).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub below: Option<::Value<String>>,
        /// Property [`Fixed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldposition.html#cfn-amplifyuibuilder-form-fieldposition-fixed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed: Option<::Value<String>>,
        /// Property [`RightOf`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldposition.html#cfn-amplifyuibuilder-form-fieldposition-rightof).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub right_of: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FieldPosition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref below) = self.below {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Below", below)?;
            }
            if let Some(ref fixed) = self.fixed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fixed", fixed)?;
            }
            if let Some(ref right_of) = self.right_of {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RightOf", right_of)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldPosition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldPosition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldPosition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldPosition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut below: Option<::Value<String>> = None;
                    let mut fixed: Option<::Value<String>> = None;
                    let mut right_of: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Below" => {
                                below = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Fixed" => {
                                fixed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RightOf" => {
                                right_of = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldPosition {
                        below: below,
                        fixed: fixed,
                        right_of: right_of,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FieldValidationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldValidationConfiguration {
        /// Property [`NumValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html#cfn-amplifyuibuilder-form-fieldvalidationconfiguration-numvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_values: Option<::ValueList<f64>>,
        /// Property [`StrValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html#cfn-amplifyuibuilder-form-fieldvalidationconfiguration-strvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub str_values: Option<::ValueList<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html#cfn-amplifyuibuilder-form-fieldvalidationconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`ValidationMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html#cfn-amplifyuibuilder-form-fieldvalidationconfiguration-validationmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation_message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FieldValidationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref num_values) = self.num_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumValues", num_values)?;
            }
            if let Some(ref str_values) = self.str_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StrValues", str_values)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref validation_message) = self.validation_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationMessage", validation_message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldValidationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldValidationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldValidationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldValidationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut num_values: Option<::ValueList<f64>> = None;
                    let mut str_values: Option<::ValueList<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut validation_message: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumValues" => {
                                num_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StrValues" => {
                                str_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidationMessage" => {
                                validation_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldValidationConfiguration {
                        num_values: num_values,
                        str_values: str_values,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        validation_message: validation_message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FileUploaderFieldConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FileUploaderFieldConfig {
        /// Property [`AcceptedFileTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-acceptedfiletypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accepted_file_types: ::ValueList<String>,
        /// Property [`AccessLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-accesslevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_level: ::Value<String>,
        /// Property [`IsResumable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-isresumable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_resumable: Option<::Value<bool>>,
        /// Property [`MaxFileCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-maxfilecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_count: Option<::Value<f64>>,
        /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-maxsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_size: Option<::Value<f64>>,
        /// Property [`ShowThumbnails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html#cfn-amplifyuibuilder-form-fileuploaderfieldconfig-showthumbnails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub show_thumbnails: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for FileUploaderFieldConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptedFileTypes", &self.accepted_file_types)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLevel", &self.access_level)?;
            if let Some(ref is_resumable) = self.is_resumable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsResumable", is_resumable)?;
            }
            if let Some(ref max_file_count) = self.max_file_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileCount", max_file_count)?;
            }
            if let Some(ref max_size) = self.max_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
            }
            if let Some(ref show_thumbnails) = self.show_thumbnails {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShowThumbnails", show_thumbnails)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileUploaderFieldConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileUploaderFieldConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileUploaderFieldConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileUploaderFieldConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accepted_file_types: Option<::ValueList<String>> = None;
                    let mut access_level: Option<::Value<String>> = None;
                    let mut is_resumable: Option<::Value<bool>> = None;
                    let mut max_file_count: Option<::Value<f64>> = None;
                    let mut max_size: Option<::Value<f64>> = None;
                    let mut show_thumbnails: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcceptedFileTypes" => {
                                accepted_file_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessLevel" => {
                                access_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsResumable" => {
                                is_resumable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileCount" => {
                                max_file_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSize" => {
                                max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShowThumbnails" => {
                                show_thumbnails = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileUploaderFieldConfig {
                        accepted_file_types: accepted_file_types.ok_or(::serde::de::Error::missing_field("AcceptedFileTypes"))?,
                        access_level: access_level.ok_or(::serde::de::Error::missing_field("AccessLevel"))?,
                        is_resumable: is_resumable,
                        max_file_count: max_file_count,
                        max_size: max_size,
                        show_thumbnails: show_thumbnails,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormButton`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formbutton.html) property type.
    #[derive(Debug, Default)]
    pub struct FormButton {
        /// Property [`Children`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formbutton.html#cfn-amplifyuibuilder-form-formbutton-children).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub children: Option<::Value<String>>,
        /// Property [`Excluded`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formbutton.html#cfn-amplifyuibuilder-form-formbutton-excluded).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded: Option<::Value<bool>>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formbutton.html#cfn-amplifyuibuilder-form-formbutton-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: Option<::Value<FieldPosition>>,
    }

    impl ::codec::SerializeValue for FormButton {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref children) = self.children {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Children", children)?;
            }
            if let Some(ref excluded) = self.excluded {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Excluded", excluded)?;
            }
            if let Some(ref position) = self.position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormButton {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormButton, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormButton;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormButton")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut children: Option<::Value<String>> = None;
                    let mut excluded: Option<::Value<bool>> = None;
                    let mut position: Option<::Value<FieldPosition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Children" => {
                                children = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Excluded" => {
                                excluded = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormButton {
                        children: children,
                        excluded: excluded,
                        position: position,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormCTA`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html) property type.
    #[derive(Debug, Default)]
    pub struct FormCTA {
        /// Property [`Cancel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html#cfn-amplifyuibuilder-form-formcta-cancel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cancel: Option<::Value<FormButton>>,
        /// Property [`Clear`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html#cfn-amplifyuibuilder-form-formcta-clear).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clear: Option<::Value<FormButton>>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html#cfn-amplifyuibuilder-form-formcta-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: Option<::Value<String>>,
        /// Property [`Submit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html#cfn-amplifyuibuilder-form-formcta-submit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub submit: Option<::Value<FormButton>>,
    }

    impl ::codec::SerializeValue for FormCTA {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cancel) = self.cancel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cancel", cancel)?;
            }
            if let Some(ref clear) = self.clear {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Clear", clear)?;
            }
            if let Some(ref position) = self.position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
            }
            if let Some(ref submit) = self.submit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Submit", submit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormCTA {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormCTA, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormCTA;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormCTA")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cancel: Option<::Value<FormButton>> = None;
                    let mut clear: Option<::Value<FormButton>> = None;
                    let mut position: Option<::Value<String>> = None;
                    let mut submit: Option<::Value<FormButton>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cancel" => {
                                cancel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Clear" => {
                                clear = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Submit" => {
                                submit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormCTA {
                        cancel: cancel,
                        clear: clear,
                        position: position,
                        submit: submit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormDataTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formdatatypeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FormDataTypeConfig {
        /// Property [`DataSourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formdatatypeconfig.html#cfn-amplifyuibuilder-form-formdatatypeconfig-datasourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_type: ::Value<String>,
        /// Property [`DataTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formdatatypeconfig.html#cfn-amplifyuibuilder-form-formdatatypeconfig-datatypename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for FormDataTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceType", &self.data_source_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTypeName", &self.data_type_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormDataTypeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormDataTypeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormDataTypeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormDataTypeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_type: Option<::Value<String>> = None;
                    let mut data_type_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceType" => {
                                data_source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTypeName" => {
                                data_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormDataTypeConfig {
                        data_source_type: data_source_type.ok_or(::serde::de::Error::missing_field("DataSourceType"))?,
                        data_type_name: data_type_name.ok_or(::serde::de::Error::missing_field("DataTypeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormInputValueProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputvalueproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct FormInputValueProperty {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputvalueproperty.html#cfn-amplifyuibuilder-form-forminputvalueproperty-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FormInputValueProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormInputValueProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormInputValueProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormInputValueProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormInputValueProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormInputValueProperty {
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct FormStyle {
        /// Property [`HorizontalGap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyle.html#cfn-amplifyuibuilder-form-formstyle-horizontalgap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub horizontal_gap: Option<::Value<FormStyleConfig>>,
        /// Property [`OuterPadding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyle.html#cfn-amplifyuibuilder-form-formstyle-outerpadding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outer_padding: Option<::Value<FormStyleConfig>>,
        /// Property [`VerticalGap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyle.html#cfn-amplifyuibuilder-form-formstyle-verticalgap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vertical_gap: Option<::Value<FormStyleConfig>>,
    }

    impl ::codec::SerializeValue for FormStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref horizontal_gap) = self.horizontal_gap {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HorizontalGap", horizontal_gap)?;
            }
            if let Some(ref outer_padding) = self.outer_padding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OuterPadding", outer_padding)?;
            }
            if let Some(ref vertical_gap) = self.vertical_gap {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerticalGap", vertical_gap)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut horizontal_gap: Option<::Value<FormStyleConfig>> = None;
                    let mut outer_padding: Option<::Value<FormStyleConfig>> = None;
                    let mut vertical_gap: Option<::Value<FormStyleConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HorizontalGap" => {
                                horizontal_gap = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OuterPadding" => {
                                outer_padding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VerticalGap" => {
                                vertical_gap = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormStyle {
                        horizontal_gap: horizontal_gap,
                        outer_padding: outer_padding,
                        vertical_gap: vertical_gap,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.FormStyleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyleconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FormStyleConfig {
        /// Property [`TokenReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyleconfig.html#cfn-amplifyuibuilder-form-formstyleconfig-tokenreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_reference: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyleconfig.html#cfn-amplifyuibuilder-form-formstyleconfig-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FormStyleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref token_reference) = self.token_reference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenReference", token_reference)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormStyleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormStyleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormStyleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormStyleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut token_reference: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TokenReference" => {
                                token_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormStyleConfig {
                        token_reference: token_reference,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.SectionalElement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html) property type.
    #[derive(Debug, Default)]
    pub struct SectionalElement {
        /// Property [`Excluded`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-excluded).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded: Option<::Value<bool>>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-level).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub level: Option<::Value<f64>>,
        /// Property [`Orientation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-orientation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub orientation: Option<::Value<String>>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: Option<::Value<FieldPosition>>,
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html#cfn-amplifyuibuilder-form-sectionalelement-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SectionalElement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded) = self.excluded {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Excluded", excluded)?;
            }
            if let Some(ref level) = self.level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", level)?;
            }
            if let Some(ref orientation) = self.orientation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Orientation", orientation)?;
            }
            if let Some(ref position) = self.position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
            }
            if let Some(ref text) = self.text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", text)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SectionalElement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SectionalElement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SectionalElement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SectionalElement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded: Option<::Value<bool>> = None;
                    let mut level: Option<::Value<f64>> = None;
                    let mut orientation: Option<::Value<String>> = None;
                    let mut position: Option<::Value<FieldPosition>> = None;
                    let mut text: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Excluded" => {
                                excluded = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Orientation" => {
                                orientation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SectionalElement {
                        excluded: excluded,
                        level: level,
                        orientation: orientation,
                        position: position,
                        text: text,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.ValueMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ValueMapping {
        /// Property [`DisplayValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemapping.html#cfn-amplifyuibuilder-form-valuemapping-displayvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_value: Option<::Value<FormInputValueProperty>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemapping.html#cfn-amplifyuibuilder-form-valuemapping-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<FormInputValueProperty>,
    }

    impl ::codec::SerializeValue for ValueMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref display_value) = self.display_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayValue", display_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValueMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValueMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValueMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValueMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut display_value: Option<::Value<FormInputValueProperty>> = None;
                    let mut value: Option<::Value<FormInputValueProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisplayValue" => {
                                display_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValueMapping {
                        display_value: display_value,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AmplifyUIBuilder::Form.ValueMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemappings.html) property type.
    #[derive(Debug, Default)]
    pub struct ValueMappings {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemappings.html#cfn-amplifyuibuilder-form-valuemappings-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<ValueMapping>,
    }

    impl ::codec::SerializeValue for ValueMappings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValueMappings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValueMappings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValueMappings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValueMappings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut values: Option<::ValueList<ValueMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValueMappings {
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
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
