//! Types for the `ResourceExplorer2` service.

/// The [`AWS::ResourceExplorer2::DefaultViewAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-defaultviewassociation.html) resource type.
#[derive(Debug, Default)]
pub struct DefaultViewAssociation {
    properties: DefaultViewAssociationProperties
}

/// Properties for the `DefaultViewAssociation` resource.
#[derive(Debug, Default)]
pub struct DefaultViewAssociationProperties {
    /// Property [`ViewArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-defaultviewassociation.html#cfn-resourceexplorer2-defaultviewassociation-viewarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub view_arn: ::Value<String>,
}

impl ::serde::Serialize for DefaultViewAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewArn", &self.view_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DefaultViewAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultViewAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DefaultViewAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DefaultViewAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut view_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ViewArn" => {
                            view_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DefaultViewAssociationProperties {
                    view_arn: view_arn.ok_or(::serde::de::Error::missing_field("ViewArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DefaultViewAssociation {
    type Properties = DefaultViewAssociationProperties;
    const TYPE: &'static str = "AWS::ResourceExplorer2::DefaultViewAssociation";
    fn properties(&self) -> &DefaultViewAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DefaultViewAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DefaultViewAssociation {}

impl From<DefaultViewAssociationProperties> for DefaultViewAssociation {
    fn from(properties: DefaultViewAssociationProperties) -> DefaultViewAssociation {
        DefaultViewAssociation { properties }
    }
}

/// The [`AWS::ResourceExplorer2::Index`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-index.html) resource type.
#[derive(Debug, Default)]
pub struct Index {
    properties: IndexProperties
}

/// Properties for the `Index` resource.
#[derive(Debug, Default)]
pub struct IndexProperties {
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-index.html#cfn-resourceexplorer2-index-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-index.html#cfn-resourceexplorer2-index-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for IndexProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IndexProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IndexProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IndexProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IndexProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags: Option<::ValueMap<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IndexProperties {
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Index {
    type Properties = IndexProperties;
    const TYPE: &'static str = "AWS::ResourceExplorer2::Index";
    fn properties(&self) -> &IndexProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IndexProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Index {}

impl From<IndexProperties> for Index {
    fn from(properties: IndexProperties) -> Index {
        Index { properties }
    }
}

/// The [`AWS::ResourceExplorer2::View`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html) resource type.
#[derive(Debug, Default)]
pub struct View {
    properties: ViewProperties
}

/// Properties for the `View` resource.
#[derive(Debug, Default)]
pub struct ViewProperties {
    /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html#cfn-resourceexplorer2-view-filters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filters: Option<::Value<self::view::SearchFilter>>,
    /// Property [`IncludedProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html#cfn-resourceexplorer2-view-includedproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub included_properties: Option<::ValueList<self::view::IncludedProperty>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html#cfn-resourceexplorer2-view-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html#cfn-resourceexplorer2-view-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`ViewName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html#cfn-resourceexplorer2-view-viewname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub view_name: ::Value<String>,
}

impl ::serde::Serialize for ViewProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref filters) = self.filters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", filters)?;
        }
        if let Some(ref included_properties) = self.included_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedProperties", included_properties)?;
        }
        if let Some(ref scope) = self.scope {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewName", &self.view_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ViewProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ViewProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ViewProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ViewProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filters: Option<::Value<self::view::SearchFilter>> = None;
                let mut included_properties: Option<::ValueList<self::view::IncludedProperty>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut view_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Filters" => {
                            filters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncludedProperties" => {
                            included_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ViewName" => {
                            view_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ViewProperties {
                    filters: filters,
                    included_properties: included_properties,
                    scope: scope,
                    tags: tags,
                    view_name: view_name.ok_or(::serde::de::Error::missing_field("ViewName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for View {
    type Properties = ViewProperties;
    const TYPE: &'static str = "AWS::ResourceExplorer2::View";
    fn properties(&self) -> &ViewProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ViewProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for View {}

impl From<ViewProperties> for View {
    fn from(properties: ViewProperties) -> View {
        View { properties }
    }
}

pub mod view {
    //! Property types for the `View` resource.

    /// The [`AWS::ResourceExplorer2::View.IncludedProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-includedproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct IncludedProperty {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-includedproperty.html#cfn-resourceexplorer2-view-includedproperty-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for IncludedProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IncludedProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IncludedProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IncludedProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IncludedProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IncludedProperty {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ResourceExplorer2::View.SearchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-searchfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct SearchFilter {
        /// Property [`FilterString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-searchfilter.html#cfn-resourceexplorer2-view-searchfilter-filterstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_string: ::Value<String>,
    }

    impl ::codec::SerializeValue for SearchFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterString", &self.filter_string)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SearchFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SearchFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SearchFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SearchFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filter_string: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FilterString" => {
                                filter_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SearchFilter {
                        filter_string: filter_string.ok_or(::serde::de::Error::missing_field("FilterString"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
