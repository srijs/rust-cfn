//! Types for the `Omics` service.

/// The [`AWS::Omics::AnnotationStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html) resource type.
#[derive(Debug, Default)]
pub struct AnnotationStore {
    properties: AnnotationStoreProperties
}

/// Properties for the `AnnotationStore` resource.
#[derive(Debug, Default)]
pub struct AnnotationStoreProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Reference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-reference).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub reference: Option<::Value<self::annotation_store::ReferenceItem>>,
    /// Property [`SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-sseconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sse_config: Option<::Value<self::annotation_store::SseConfig>>,
    /// Property [`StoreFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-storeformat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub store_format: ::Value<String>,
    /// Property [`StoreOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-storeoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub store_options: Option<::Value<self::annotation_store::StoreOptions>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html#cfn-omics-annotationstore-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for AnnotationStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref reference) = self.reference {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reference", reference)?;
        }
        if let Some(ref sse_config) = self.sse_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseConfig", sse_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoreFormat", &self.store_format)?;
        if let Some(ref store_options) = self.store_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoreOptions", store_options)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnnotationStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnnotationStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnnotationStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnnotationStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut reference: Option<::Value<self::annotation_store::ReferenceItem>> = None;
                let mut sse_config: Option<::Value<self::annotation_store::SseConfig>> = None;
                let mut store_format: Option<::Value<String>> = None;
                let mut store_options: Option<::Value<self::annotation_store::StoreOptions>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Reference" => {
                            reference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SseConfig" => {
                            sse_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoreFormat" => {
                            store_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoreOptions" => {
                            store_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnnotationStoreProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    reference: reference,
                    sse_config: sse_config,
                    store_format: store_format.ok_or(::serde::de::Error::missing_field("StoreFormat"))?,
                    store_options: store_options,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AnnotationStore {
    type Properties = AnnotationStoreProperties;
    const TYPE: &'static str = "AWS::Omics::AnnotationStore";
    fn properties(&self) -> &AnnotationStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnnotationStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AnnotationStore {}

impl From<AnnotationStoreProperties> for AnnotationStore {
    fn from(properties: AnnotationStoreProperties) -> AnnotationStore {
        AnnotationStore { properties }
    }
}

/// The [`AWS::Omics::ReferenceStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html) resource type.
#[derive(Debug, Default)]
pub struct ReferenceStore {
    properties: ReferenceStoreProperties
}

/// Properties for the `ReferenceStore` resource.
#[derive(Debug, Default)]
pub struct ReferenceStoreProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html#cfn-omics-referencestore-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html#cfn-omics-referencestore-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html#cfn-omics-referencestore-sseconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sse_config: Option<::Value<self::reference_store::SseConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html#cfn-omics-referencestore-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ReferenceStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref sse_config) = self.sse_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseConfig", sse_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReferenceStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReferenceStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReferenceStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut sse_config: Option<::Value<self::reference_store::SseConfig>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SseConfig" => {
                            sse_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReferenceStoreProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    sse_config: sse_config,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReferenceStore {
    type Properties = ReferenceStoreProperties;
    const TYPE: &'static str = "AWS::Omics::ReferenceStore";
    fn properties(&self) -> &ReferenceStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReferenceStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReferenceStore {}

impl From<ReferenceStoreProperties> for ReferenceStore {
    fn from(properties: ReferenceStoreProperties) -> ReferenceStore {
        ReferenceStore { properties }
    }
}

/// The [`AWS::Omics::RunGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html) resource type.
#[derive(Debug, Default)]
pub struct RunGroup {
    properties: RunGroupProperties
}

/// Properties for the `RunGroup` resource.
#[derive(Debug, Default)]
pub struct RunGroupProperties {
    /// Property [`MaxCpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-maxcpus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_cpus: Option<::Value<f64>>,
    /// Property [`MaxDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-maxduration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_duration: Option<::Value<f64>>,
    /// Property [`MaxGpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-maxgpus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_gpus: Option<::Value<f64>>,
    /// Property [`MaxRuns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-maxruns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_runs: Option<::Value<f64>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html#cfn-omics-rungroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for RunGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref max_cpus) = self.max_cpus {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCpus", max_cpus)?;
        }
        if let Some(ref max_duration) = self.max_duration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxDuration", max_duration)?;
        }
        if let Some(ref max_gpus) = self.max_gpus {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxGpus", max_gpus)?;
        }
        if let Some(ref max_runs) = self.max_runs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuns", max_runs)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RunGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RunGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RunGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RunGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut max_cpus: Option<::Value<f64>> = None;
                let mut max_duration: Option<::Value<f64>> = None;
                let mut max_gpus: Option<::Value<f64>> = None;
                let mut max_runs: Option<::Value<f64>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MaxCpus" => {
                            max_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxDuration" => {
                            max_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxGpus" => {
                            max_gpus = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxRuns" => {
                            max_runs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RunGroupProperties {
                    max_cpus: max_cpus,
                    max_duration: max_duration,
                    max_gpus: max_gpus,
                    max_runs: max_runs,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RunGroup {
    type Properties = RunGroupProperties;
    const TYPE: &'static str = "AWS::Omics::RunGroup";
    fn properties(&self) -> &RunGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RunGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RunGroup {}

impl From<RunGroupProperties> for RunGroup {
    fn from(properties: RunGroupProperties) -> RunGroup {
        RunGroup { properties }
    }
}

/// The [`AWS::Omics::SequenceStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html) resource type.
#[derive(Debug, Default)]
pub struct SequenceStore {
    properties: SequenceStoreProperties
}

/// Properties for the `SequenceStore` resource.
#[derive(Debug, Default)]
pub struct SequenceStoreProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html#cfn-omics-sequencestore-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FallbackLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html#cfn-omics-sequencestore-fallbacklocation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fallback_location: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html#cfn-omics-sequencestore-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html#cfn-omics-sequencestore-sseconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sse_config: Option<::Value<self::sequence_store::SseConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html#cfn-omics-sequencestore-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for SequenceStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref fallback_location) = self.fallback_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FallbackLocation", fallback_location)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref sse_config) = self.sse_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseConfig", sse_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SequenceStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SequenceStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SequenceStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SequenceStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut fallback_location: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut sse_config: Option<::Value<self::sequence_store::SseConfig>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FallbackLocation" => {
                            fallback_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SseConfig" => {
                            sse_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SequenceStoreProperties {
                    description: description,
                    fallback_location: fallback_location,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    sse_config: sse_config,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SequenceStore {
    type Properties = SequenceStoreProperties;
    const TYPE: &'static str = "AWS::Omics::SequenceStore";
    fn properties(&self) -> &SequenceStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SequenceStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SequenceStore {}

impl From<SequenceStoreProperties> for SequenceStore {
    fn from(properties: SequenceStoreProperties) -> SequenceStore {
        SequenceStore { properties }
    }
}

/// The [`AWS::Omics::VariantStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html) resource type.
#[derive(Debug, Default)]
pub struct VariantStore {
    properties: VariantStoreProperties
}

/// Properties for the `VariantStore` resource.
#[derive(Debug, Default)]
pub struct VariantStoreProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html#cfn-omics-variantstore-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html#cfn-omics-variantstore-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Reference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html#cfn-omics-variantstore-reference).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub reference: ::Value<self::variant_store::ReferenceItem>,
    /// Property [`SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html#cfn-omics-variantstore-sseconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sse_config: Option<::Value<self::variant_store::SseConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html#cfn-omics-variantstore-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for VariantStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reference", &self.reference)?;
        if let Some(ref sse_config) = self.sse_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseConfig", sse_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VariantStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VariantStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VariantStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VariantStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut reference: Option<::Value<self::variant_store::ReferenceItem>> = None;
                let mut sse_config: Option<::Value<self::variant_store::SseConfig>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Reference" => {
                            reference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SseConfig" => {
                            sse_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VariantStoreProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    reference: reference.ok_or(::serde::de::Error::missing_field("Reference"))?,
                    sse_config: sse_config,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VariantStore {
    type Properties = VariantStoreProperties;
    const TYPE: &'static str = "AWS::Omics::VariantStore";
    fn properties(&self) -> &VariantStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VariantStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VariantStore {}

impl From<VariantStoreProperties> for VariantStore {
    fn from(properties: VariantStoreProperties) -> VariantStore {
        VariantStore { properties }
    }
}

/// The [`AWS::Omics::Workflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html) resource type.
#[derive(Debug, Default)]
pub struct Workflow {
    properties: WorkflowProperties
}

/// Properties for the `Workflow` resource.
#[derive(Debug, Default)]
pub struct WorkflowProperties {
    /// Property [`Accelerators`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-accelerators).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accelerators: Option<::Value<String>>,
    /// Property [`DefinitionUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-definitionuri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub definition_uri: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: Option<::Value<String>>,
    /// Property [`Main`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-main).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub main: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ParameterTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-parametertemplate).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parameter_template: Option<::ValueMap<self::workflow::WorkflowParameter>>,
    /// Property [`StorageCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-storagecapacity).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_capacity: Option<::Value<f64>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html#cfn-omics-workflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for WorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accelerators) = self.accelerators {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Accelerators", accelerators)?;
        }
        if let Some(ref definition_uri) = self.definition_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefinitionUri", definition_uri)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref main) = self.main {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Main", main)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref parameter_template) = self.parameter_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterTemplate", parameter_template)?;
        }
        if let Some(ref storage_capacity) = self.storage_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacity", storage_capacity)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkflowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkflowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accelerators: Option<::Value<String>> = None;
                let mut definition_uri: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut main: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameter_template: Option<::ValueMap<self::workflow::WorkflowParameter>> = None;
                let mut storage_capacity: Option<::Value<f64>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Accelerators" => {
                            accelerators = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefinitionUri" => {
                            definition_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Main" => {
                            main = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterTemplate" => {
                            parameter_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageCapacity" => {
                            storage_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkflowProperties {
                    accelerators: accelerators,
                    definition_uri: definition_uri,
                    description: description,
                    engine: engine,
                    main: main,
                    name: name,
                    parameter_template: parameter_template,
                    storage_capacity: storage_capacity,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workflow {
    type Properties = WorkflowProperties;
    const TYPE: &'static str = "AWS::Omics::Workflow";
    fn properties(&self) -> &WorkflowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkflowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workflow {}

impl From<WorkflowProperties> for Workflow {
    fn from(properties: WorkflowProperties) -> Workflow {
        Workflow { properties }
    }
}

pub mod annotation_store {
    //! Property types for the `AnnotationStore` resource.

    /// The [`AWS::Omics::AnnotationStore.ReferenceItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-referenceitem.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferenceItem {
        /// Property [`ReferenceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-referenceitem.html#cfn-omics-annotationstore-referenceitem-referencearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub reference_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReferenceItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceArn", &self.reference_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferenceItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferenceItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferenceItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reference_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceArn" => {
                                reference_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferenceItem {
                        reference_arn: reference_arn.ok_or(::serde::de::Error::missing_field("ReferenceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Omics::AnnotationStore.SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-sseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SseConfig {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-sseconfig.html#cfn-omics-annotationstore-sseconfig-keyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-sseconfig.html#cfn-omics-annotationstore-sseconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseConfig {
                        key_arn: key_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Omics::AnnotationStore.StoreOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-storeoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct StoreOptions {
        /// Property [`TsvStoreOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-storeoptions.html#cfn-omics-annotationstore-storeoptions-tsvstoreoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tsv_store_options: ::Value<TsvStoreOptions>,
    }

    impl ::codec::SerializeValue for StoreOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TsvStoreOptions", &self.tsv_store_options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoreOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoreOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoreOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoreOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tsv_store_options: Option<::Value<TsvStoreOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TsvStoreOptions" => {
                                tsv_store_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoreOptions {
                        tsv_store_options: tsv_store_options.ok_or(::serde::de::Error::missing_field("TsvStoreOptions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Omics::AnnotationStore.TsvStoreOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-tsvstoreoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct TsvStoreOptions {
        /// Property [`AnnotationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-tsvstoreoptions.html#cfn-omics-annotationstore-tsvstoreoptions-annotationtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub annotation_type: Option<::Value<String>>,
        /// Property [`FormatToHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-tsvstoreoptions.html#cfn-omics-annotationstore-tsvstoreoptions-formattoheader).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub format_to_header: Option<::ValueMap<String>>,
        /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-tsvstoreoptions.html#cfn-omics-annotationstore-tsvstoreoptions-schema).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub schema: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for TsvStoreOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref annotation_type) = self.annotation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnnotationType", annotation_type)?;
            }
            if let Some(ref format_to_header) = self.format_to_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormatToHeader", format_to_header)?;
            }
            if let Some(ref schema) = self.schema {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TsvStoreOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TsvStoreOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TsvStoreOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TsvStoreOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut annotation_type: Option<::Value<String>> = None;
                    let mut format_to_header: Option<::ValueMap<String>> = None;
                    let mut schema: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AnnotationType" => {
                                annotation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FormatToHeader" => {
                                format_to_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schema" => {
                                schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TsvStoreOptions {
                        annotation_type: annotation_type,
                        format_to_header: format_to_header,
                        schema: schema,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod reference_store {
    //! Property types for the `ReferenceStore` resource.

    /// The [`AWS::Omics::ReferenceStore.SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-referencestore-sseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SseConfig {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-referencestore-sseconfig.html#cfn-omics-referencestore-sseconfig-keyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-referencestore-sseconfig.html#cfn-omics-referencestore-sseconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseConfig {
                        key_arn: key_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod sequence_store {
    //! Property types for the `SequenceStore` resource.

    /// The [`AWS::Omics::SequenceStore.SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-sequencestore-sseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SseConfig {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-sequencestore-sseconfig.html#cfn-omics-sequencestore-sseconfig-keyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-sequencestore-sseconfig.html#cfn-omics-sequencestore-sseconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseConfig {
                        key_arn: key_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod variant_store {
    //! Property types for the `VariantStore` resource.

    /// The [`AWS::Omics::VariantStore.ReferenceItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-referenceitem.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferenceItem {
        /// Property [`ReferenceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-referenceitem.html#cfn-omics-variantstore-referenceitem-referencearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub reference_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReferenceItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceArn", &self.reference_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferenceItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferenceItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferenceItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reference_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceArn" => {
                                reference_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferenceItem {
                        reference_arn: reference_arn.ok_or(::serde::de::Error::missing_field("ReferenceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Omics::VariantStore.SseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-sseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SseConfig {
        /// Property [`KeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-sseconfig.html#cfn-omics-variantstore-sseconfig-keyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-sseconfig.html#cfn-omics-variantstore-sseconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyArn", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyArn" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseConfig {
                        key_arn: key_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workflow {
    //! Property types for the `Workflow` resource.

    /// The [`AWS::Omics::Workflow.WorkflowParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-workflowparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowParameter {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-workflowparameter.html#cfn-omics-workflow-workflowparameter-description).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Optional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-workflowparameter.html#cfn-omics-workflow-workflowparameter-optional).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub optional: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for WorkflowParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref optional) = self.optional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Optional", optional)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut optional: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Optional" => {
                                optional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowParameter {
                        description: description,
                        optional: optional,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
