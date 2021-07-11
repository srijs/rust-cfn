//! Types for the `CUR` service.

/// The [`AWS::CUR::ReportDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ReportDefinition {
    properties: ReportDefinitionProperties
}

/// Properties for the `ReportDefinition` resource.
#[derive(Debug, Default)]
pub struct ReportDefinitionProperties {
    /// Property [`AdditionalArtifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-additionalartifacts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_artifacts: Option<::ValueList<String>>,
    /// Property [`AdditionalSchemaElements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-additionalschemaelements).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_schema_elements: Option<::ValueList<String>>,
    /// Property [`BillingViewArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-billingviewarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub billing_view_arn: Option<::Value<String>>,
    /// Property [`Compression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-compression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compression: ::Value<String>,
    /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-format).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub format: ::Value<String>,
    /// Property [`RefreshClosedReports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-refreshclosedreports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub refresh_closed_reports: ::Value<bool>,
    /// Property [`ReportName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-reportname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub report_name: ::Value<String>,
    /// Property [`ReportVersioning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-reportversioning).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub report_versioning: ::Value<String>,
    /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-s3bucket).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_bucket: ::Value<String>,
    /// Property [`S3Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-s3prefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_prefix: ::Value<String>,
    /// Property [`S3Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-s3region).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_region: ::Value<String>,
    /// Property [`TimeUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html#cfn-cur-reportdefinition-timeunit).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub time_unit: ::Value<String>,
}

impl ::serde::Serialize for ReportDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_artifacts) = self.additional_artifacts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalArtifacts", additional_artifacts)?;
        }
        if let Some(ref additional_schema_elements) = self.additional_schema_elements {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalSchemaElements", additional_schema_elements)?;
        }
        if let Some(ref billing_view_arn) = self.billing_view_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingViewArn", billing_view_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compression", &self.compression)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshClosedReports", &self.refresh_closed_reports)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportName", &self.report_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportVersioning", &self.report_versioning)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Prefix", &self.s3_prefix)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Region", &self.s3_region)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeUnit", &self.time_unit)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReportDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReportDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReportDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_artifacts: Option<::ValueList<String>> = None;
                let mut additional_schema_elements: Option<::ValueList<String>> = None;
                let mut billing_view_arn: Option<::Value<String>> = None;
                let mut compression: Option<::Value<String>> = None;
                let mut format: Option<::Value<String>> = None;
                let mut refresh_closed_reports: Option<::Value<bool>> = None;
                let mut report_name: Option<::Value<String>> = None;
                let mut report_versioning: Option<::Value<String>> = None;
                let mut s3_bucket: Option<::Value<String>> = None;
                let mut s3_prefix: Option<::Value<String>> = None;
                let mut s3_region: Option<::Value<String>> = None;
                let mut time_unit: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalArtifacts" => {
                            additional_artifacts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdditionalSchemaElements" => {
                            additional_schema_elements = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingViewArn" => {
                            billing_view_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Compression" => {
                            compression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Format" => {
                            format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RefreshClosedReports" => {
                            refresh_closed_reports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportName" => {
                            report_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportVersioning" => {
                            report_versioning = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Bucket" => {
                            s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Prefix" => {
                            s3_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Region" => {
                            s3_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeUnit" => {
                            time_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReportDefinitionProperties {
                    additional_artifacts: additional_artifacts,
                    additional_schema_elements: additional_schema_elements,
                    billing_view_arn: billing_view_arn,
                    compression: compression.ok_or(::serde::de::Error::missing_field("Compression"))?,
                    format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                    refresh_closed_reports: refresh_closed_reports.ok_or(::serde::de::Error::missing_field("RefreshClosedReports"))?,
                    report_name: report_name.ok_or(::serde::de::Error::missing_field("ReportName"))?,
                    report_versioning: report_versioning.ok_or(::serde::de::Error::missing_field("ReportVersioning"))?,
                    s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                    s3_prefix: s3_prefix.ok_or(::serde::de::Error::missing_field("S3Prefix"))?,
                    s3_region: s3_region.ok_or(::serde::de::Error::missing_field("S3Region"))?,
                    time_unit: time_unit.ok_or(::serde::de::Error::missing_field("TimeUnit"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReportDefinition {
    type Properties = ReportDefinitionProperties;
    const TYPE: &'static str = "AWS::CUR::ReportDefinition";
    fn properties(&self) -> &ReportDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReportDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReportDefinition {}

impl From<ReportDefinitionProperties> for ReportDefinition {
    fn from(properties: ReportDefinitionProperties) -> ReportDefinition {
        ReportDefinition { properties }
    }
}
