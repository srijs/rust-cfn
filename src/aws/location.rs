//! Types for the `Location` service.

/// The [`AWS::Location::APIKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html) resource type.
#[derive(Debug, Default)]
pub struct APIKey {
    properties: APIKeyProperties
}

/// Properties for the `APIKey` resource.
#[derive(Debug, Default)]
pub struct APIKeyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExpireTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-expiretime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expire_time: Option<::Value<String>>,
    /// Property [`ForceDelete`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-forcedelete).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub force_delete: Option<::Value<bool>>,
    /// Property [`ForceUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-forceupdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub force_update: Option<::Value<bool>>,
    /// Property [`KeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-keyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_name: ::Value<String>,
    /// Property [`NoExpiry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-noexpiry).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub no_expiry: Option<::Value<bool>>,
    /// Property [`Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-restrictions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub restrictions: ::Value<self::api_key::ApiKeyRestrictions>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html#cfn-location-apikey-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for APIKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref expire_time) = self.expire_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpireTime", expire_time)?;
        }
        if let Some(ref force_delete) = self.force_delete {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceDelete", force_delete)?;
        }
        if let Some(ref force_update) = self.force_update {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceUpdate", force_update)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyName", &self.key_name)?;
        if let Some(ref no_expiry) = self.no_expiry {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoExpiry", no_expiry)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Restrictions", &self.restrictions)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for APIKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<APIKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type APIKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut expire_time: Option<::Value<String>> = None;
                let mut force_delete: Option<::Value<bool>> = None;
                let mut force_update: Option<::Value<bool>> = None;
                let mut key_name: Option<::Value<String>> = None;
                let mut no_expiry: Option<::Value<bool>> = None;
                let mut restrictions: Option<::Value<self::api_key::ApiKeyRestrictions>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExpireTime" => {
                            expire_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ForceDelete" => {
                            force_delete = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ForceUpdate" => {
                            force_update = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyName" => {
                            key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NoExpiry" => {
                            no_expiry = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Restrictions" => {
                            restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(APIKeyProperties {
                    description: description,
                    expire_time: expire_time,
                    force_delete: force_delete,
                    force_update: force_update,
                    key_name: key_name.ok_or(::serde::de::Error::missing_field("KeyName"))?,
                    no_expiry: no_expiry,
                    restrictions: restrictions.ok_or(::serde::de::Error::missing_field("Restrictions"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for APIKey {
    type Properties = APIKeyProperties;
    const TYPE: &'static str = "AWS::Location::APIKey";
    fn properties(&self) -> &APIKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut APIKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for APIKey {}

impl From<APIKeyProperties> for APIKey {
    fn from(properties: APIKeyProperties) -> APIKey {
        APIKey { properties }
    }
}

/// The [`AWS::Location::GeofenceCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html) resource type.
#[derive(Debug, Default)]
pub struct GeofenceCollection {
    properties: GeofenceCollectionProperties
}

/// Properties for the `GeofenceCollection` resource.
#[derive(Debug, Default)]
pub struct GeofenceCollectionProperties {
    /// Property [`CollectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html#cfn-location-geofencecollection-collectionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub collection_name: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html#cfn-location-geofencecollection-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html#cfn-location-geofencecollection-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html#cfn-location-geofencecollection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for GeofenceCollectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionName", &self.collection_name)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GeofenceCollectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GeofenceCollectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GeofenceCollectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GeofenceCollectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collection_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CollectionName" => {
                            collection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GeofenceCollectionProperties {
                    collection_name: collection_name.ok_or(::serde::de::Error::missing_field("CollectionName"))?,
                    description: description,
                    kms_key_id: kms_key_id,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GeofenceCollection {
    type Properties = GeofenceCollectionProperties;
    const TYPE: &'static str = "AWS::Location::GeofenceCollection";
    fn properties(&self) -> &GeofenceCollectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GeofenceCollectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GeofenceCollection {}

impl From<GeofenceCollectionProperties> for GeofenceCollection {
    fn from(properties: GeofenceCollectionProperties) -> GeofenceCollection {
        GeofenceCollection { properties }
    }
}

/// The [`AWS::Location::Map`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html) resource type.
#[derive(Debug, Default)]
pub struct Map {
    properties: MapProperties
}

/// Properties for the `Map` resource.
#[derive(Debug, Default)]
pub struct MapProperties {
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html#cfn-location-map-configuration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration: ::Value<self::map::MapConfiguration>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html#cfn-location-map-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MapName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html#cfn-location-map-mapname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub map_name: ::Value<String>,
    /// Property [`PricingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html#cfn-location-map-pricingplan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pricing_plan: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html#cfn-location-map-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MapProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapName", &self.map_name)?;
        if let Some(ref pricing_plan) = self.pricing_plan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PricingPlan", pricing_plan)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MapProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MapProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MapProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MapProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration: Option<::Value<self::map::MapConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut map_name: Option<::Value<String>> = None;
                let mut pricing_plan: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MapName" => {
                            map_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PricingPlan" => {
                            pricing_plan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MapProperties {
                    configuration: configuration.ok_or(::serde::de::Error::missing_field("Configuration"))?,
                    description: description,
                    map_name: map_name.ok_or(::serde::de::Error::missing_field("MapName"))?,
                    pricing_plan: pricing_plan,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Map {
    type Properties = MapProperties;
    const TYPE: &'static str = "AWS::Location::Map";
    fn properties(&self) -> &MapProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MapProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Map {}

impl From<MapProperties> for Map {
    fn from(properties: MapProperties) -> Map {
        Map { properties }
    }
}

/// The [`AWS::Location::PlaceIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html) resource type.
#[derive(Debug, Default)]
pub struct PlaceIndex {
    properties: PlaceIndexProperties
}

/// Properties for the `PlaceIndex` resource.
#[derive(Debug, Default)]
pub struct PlaceIndexProperties {
    /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-datasource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_source: ::Value<String>,
    /// Property [`DataSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-datasourceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source_configuration: Option<::Value<self::place_index::DataSourceConfiguration>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-indexname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub index_name: ::Value<String>,
    /// Property [`PricingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-pricingplan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pricing_plan: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html#cfn-location-placeindex-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PlaceIndexProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", &self.data_source)?;
        if let Some(ref data_source_configuration) = self.data_source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceConfiguration", data_source_configuration)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
        if let Some(ref pricing_plan) = self.pricing_plan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PricingPlan", pricing_plan)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PlaceIndexProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PlaceIndexProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PlaceIndexProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PlaceIndexProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_source: Option<::Value<String>> = None;
                let mut data_source_configuration: Option<::Value<self::place_index::DataSourceConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut index_name: Option<::Value<String>> = None;
                let mut pricing_plan: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataSource" => {
                            data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceConfiguration" => {
                            data_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IndexName" => {
                            index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PricingPlan" => {
                            pricing_plan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PlaceIndexProperties {
                    data_source: data_source.ok_or(::serde::de::Error::missing_field("DataSource"))?,
                    data_source_configuration: data_source_configuration,
                    description: description,
                    index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                    pricing_plan: pricing_plan,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PlaceIndex {
    type Properties = PlaceIndexProperties;
    const TYPE: &'static str = "AWS::Location::PlaceIndex";
    fn properties(&self) -> &PlaceIndexProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlaceIndexProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PlaceIndex {}

impl From<PlaceIndexProperties> for PlaceIndex {
    fn from(properties: PlaceIndexProperties) -> PlaceIndex {
        PlaceIndex { properties }
    }
}

/// The [`AWS::Location::RouteCalculator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html) resource type.
#[derive(Debug, Default)]
pub struct RouteCalculator {
    properties: RouteCalculatorProperties
}

/// Properties for the `RouteCalculator` resource.
#[derive(Debug, Default)]
pub struct RouteCalculatorProperties {
    /// Property [`CalculatorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html#cfn-location-routecalculator-calculatorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub calculator_name: ::Value<String>,
    /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html#cfn-location-routecalculator-datasource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_source: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html#cfn-location-routecalculator-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PricingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html#cfn-location-routecalculator-pricingplan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pricing_plan: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html#cfn-location-routecalculator-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RouteCalculatorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CalculatorName", &self.calculator_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", &self.data_source)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref pricing_plan) = self.pricing_plan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PricingPlan", pricing_plan)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteCalculatorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteCalculatorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteCalculatorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteCalculatorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut calculator_name: Option<::Value<String>> = None;
                let mut data_source: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut pricing_plan: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CalculatorName" => {
                            calculator_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSource" => {
                            data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PricingPlan" => {
                            pricing_plan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RouteCalculatorProperties {
                    calculator_name: calculator_name.ok_or(::serde::de::Error::missing_field("CalculatorName"))?,
                    data_source: data_source.ok_or(::serde::de::Error::missing_field("DataSource"))?,
                    description: description,
                    pricing_plan: pricing_plan,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RouteCalculator {
    type Properties = RouteCalculatorProperties;
    const TYPE: &'static str = "AWS::Location::RouteCalculator";
    fn properties(&self) -> &RouteCalculatorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteCalculatorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RouteCalculator {}

impl From<RouteCalculatorProperties> for RouteCalculator {
    fn from(properties: RouteCalculatorProperties) -> RouteCalculator {
        RouteCalculator { properties }
    }
}

/// The [`AWS::Location::Tracker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html) resource type.
#[derive(Debug, Default)]
pub struct Tracker {
    properties: TrackerProperties
}

/// Properties for the `Tracker` resource.
#[derive(Debug, Default)]
pub struct TrackerProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventBridgeEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-eventbridgeenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_bridge_enabled: Option<::Value<bool>>,
    /// Property [`KmsKeyEnableGeospatialQueries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-kmskeyenablegeospatialqueries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_enable_geospatial_queries: Option<::Value<bool>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`PositionFiltering`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-positionfiltering).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub position_filtering: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrackerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html#cfn-location-tracker-trackername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tracker_name: ::Value<String>,
}

impl ::serde::Serialize for TrackerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_bridge_enabled) = self.event_bridge_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeEnabled", event_bridge_enabled)?;
        }
        if let Some(ref kms_key_enable_geospatial_queries) = self.kms_key_enable_geospatial_queries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyEnableGeospatialQueries", kms_key_enable_geospatial_queries)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref position_filtering) = self.position_filtering {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PositionFiltering", position_filtering)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackerName", &self.tracker_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrackerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrackerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrackerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrackerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut event_bridge_enabled: Option<::Value<bool>> = None;
                let mut kms_key_enable_geospatial_queries: Option<::Value<bool>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut position_filtering: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tracker_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventBridgeEnabled" => {
                            event_bridge_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyEnableGeospatialQueries" => {
                            kms_key_enable_geospatial_queries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PositionFiltering" => {
                            position_filtering = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrackerName" => {
                            tracker_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrackerProperties {
                    description: description,
                    event_bridge_enabled: event_bridge_enabled,
                    kms_key_enable_geospatial_queries: kms_key_enable_geospatial_queries,
                    kms_key_id: kms_key_id,
                    position_filtering: position_filtering,
                    tags: tags,
                    tracker_name: tracker_name.ok_or(::serde::de::Error::missing_field("TrackerName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Tracker {
    type Properties = TrackerProperties;
    const TYPE: &'static str = "AWS::Location::Tracker";
    fn properties(&self) -> &TrackerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrackerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Tracker {}

impl From<TrackerProperties> for Tracker {
    fn from(properties: TrackerProperties) -> Tracker {
        Tracker { properties }
    }
}

/// The [`AWS::Location::TrackerConsumer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-trackerconsumer.html) resource type.
#[derive(Debug, Default)]
pub struct TrackerConsumer {
    properties: TrackerConsumerProperties
}

/// Properties for the `TrackerConsumer` resource.
#[derive(Debug, Default)]
pub struct TrackerConsumerProperties {
    /// Property [`ConsumerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-trackerconsumer.html#cfn-location-trackerconsumer-consumerarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub consumer_arn: ::Value<String>,
    /// Property [`TrackerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-trackerconsumer.html#cfn-location-trackerconsumer-trackername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tracker_name: ::Value<String>,
}

impl ::serde::Serialize for TrackerConsumerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerArn", &self.consumer_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackerName", &self.tracker_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrackerConsumerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrackerConsumerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrackerConsumerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrackerConsumerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut consumer_arn: Option<::Value<String>> = None;
                let mut tracker_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConsumerArn" => {
                            consumer_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrackerName" => {
                            tracker_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrackerConsumerProperties {
                    consumer_arn: consumer_arn.ok_or(::serde::de::Error::missing_field("ConsumerArn"))?,
                    tracker_name: tracker_name.ok_or(::serde::de::Error::missing_field("TrackerName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TrackerConsumer {
    type Properties = TrackerConsumerProperties;
    const TYPE: &'static str = "AWS::Location::TrackerConsumer";
    fn properties(&self) -> &TrackerConsumerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrackerConsumerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrackerConsumer {}

impl From<TrackerConsumerProperties> for TrackerConsumer {
    fn from(properties: TrackerConsumerProperties) -> TrackerConsumer {
        TrackerConsumer { properties }
    }
}

pub mod api_key {
    //! Property types for the `APIKey` resource.

    /// The [`AWS::Location::APIKey.ApiKeyRestrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-apikey-apikeyrestrictions.html) property type.
    #[derive(Debug, Default)]
    pub struct ApiKeyRestrictions {
        /// Property [`AllowActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-apikey-apikeyrestrictions.html#cfn-location-apikey-apikeyrestrictions-allowactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_actions: ::ValueList<String>,
        /// Property [`AllowReferers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-apikey-apikeyrestrictions.html#cfn-location-apikey-apikeyrestrictions-allowreferers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_referers: Option<::ValueList<String>>,
        /// Property [`AllowResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-apikey-apikeyrestrictions.html#cfn-location-apikey-apikeyrestrictions-allowresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_resources: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ApiKeyRestrictions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowActions", &self.allow_actions)?;
            if let Some(ref allow_referers) = self.allow_referers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowReferers", allow_referers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowResources", &self.allow_resources)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApiKeyRestrictions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiKeyRestrictions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApiKeyRestrictions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApiKeyRestrictions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_actions: Option<::ValueList<String>> = None;
                    let mut allow_referers: Option<::ValueList<String>> = None;
                    let mut allow_resources: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowActions" => {
                                allow_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowReferers" => {
                                allow_referers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowResources" => {
                                allow_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApiKeyRestrictions {
                        allow_actions: allow_actions.ok_or(::serde::de::Error::missing_field("AllowActions"))?,
                        allow_referers: allow_referers,
                        allow_resources: allow_resources.ok_or(::serde::de::Error::missing_field("AllowResources"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod map {
    //! Property types for the `Map` resource.

    /// The [`AWS::Location::Map.MapConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-map-mapconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MapConfiguration {
        /// Property [`CustomLayers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-map-mapconfiguration.html#cfn-location-map-mapconfiguration-customlayers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_layers: Option<::ValueList<String>>,
        /// Property [`PoliticalView`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-map-mapconfiguration.html#cfn-location-map-mapconfiguration-politicalview).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub political_view: Option<::Value<String>>,
        /// Property [`Style`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-map-mapconfiguration.html#cfn-location-map-mapconfiguration-style).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub style: ::Value<String>,
    }

    impl ::codec::SerializeValue for MapConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_layers) = self.custom_layers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomLayers", custom_layers)?;
            }
            if let Some(ref political_view) = self.political_view {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PoliticalView", political_view)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Style", &self.style)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MapConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MapConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MapConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MapConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_layers: Option<::ValueList<String>> = None;
                    let mut political_view: Option<::Value<String>> = None;
                    let mut style: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomLayers" => {
                                custom_layers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PoliticalView" => {
                                political_view = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Style" => {
                                style = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MapConfiguration {
                        custom_layers: custom_layers,
                        political_view: political_view,
                        style: style.ok_or(::serde::de::Error::missing_field("Style"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod place_index {
    //! Property types for the `PlaceIndex` resource.

    /// The [`AWS::Location::PlaceIndex.DataSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-placeindex-datasourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceConfiguration {
        /// Property [`IntendedUse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-placeindex-datasourceconfiguration.html#cfn-location-placeindex-datasourceconfiguration-intendeduse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intended_use: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref intended_use) = self.intended_use {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntendedUse", intended_use)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut intended_use: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntendedUse" => {
                                intended_use = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceConfiguration {
                        intended_use: intended_use,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
