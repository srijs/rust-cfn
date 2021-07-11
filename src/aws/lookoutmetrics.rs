//! Types for the `LookoutMetrics` service.

/// The [`AWS::LookoutMetrics::Alert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html) resource type.
#[derive(Debug, Default)]
pub struct Alert {
    properties: AlertProperties
}

/// Properties for the `Alert` resource.
#[derive(Debug, Default)]
pub struct AlertProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html#cfn-lookoutmetrics-alert-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: ::Value<::json::Value>,
    /// Property [`AlertDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html#cfn-lookoutmetrics-alert-alertdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alert_description: Option<::Value<String>>,
    /// Property [`AlertName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html#cfn-lookoutmetrics-alert-alertname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alert_name: Option<::Value<String>>,
    /// Property [`AlertSensitivityThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html#cfn-lookoutmetrics-alert-alertsensitivitythreshold).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alert_sensitivity_threshold: ::Value<u32>,
    /// Property [`AnomalyDetectorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html#cfn-lookoutmetrics-alert-anomalydetectorarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub anomaly_detector_arn: ::Value<String>,
}

impl ::serde::Serialize for AlertProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        if let Some(ref alert_description) = self.alert_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlertDescription", alert_description)?;
        }
        if let Some(ref alert_name) = self.alert_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlertName", alert_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlertSensitivityThreshold", &self.alert_sensitivity_threshold)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnomalyDetectorArn", &self.anomaly_detector_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AlertProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AlertProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AlertProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AlertProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<::json::Value>> = None;
                let mut alert_description: Option<::Value<String>> = None;
                let mut alert_name: Option<::Value<String>> = None;
                let mut alert_sensitivity_threshold: Option<::Value<u32>> = None;
                let mut anomaly_detector_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlertDescription" => {
                            alert_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlertName" => {
                            alert_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlertSensitivityThreshold" => {
                            alert_sensitivity_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnomalyDetectorArn" => {
                            anomaly_detector_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AlertProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    alert_description: alert_description,
                    alert_name: alert_name,
                    alert_sensitivity_threshold: alert_sensitivity_threshold.ok_or(::serde::de::Error::missing_field("AlertSensitivityThreshold"))?,
                    anomaly_detector_arn: anomaly_detector_arn.ok_or(::serde::de::Error::missing_field("AnomalyDetectorArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alert {
    type Properties = AlertProperties;
    const TYPE: &'static str = "AWS::LookoutMetrics::Alert";
    fn properties(&self) -> &AlertProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AlertProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alert {}

impl From<AlertProperties> for Alert {
    fn from(properties: AlertProperties) -> Alert {
        Alert { properties }
    }
}

/// The [`AWS::LookoutMetrics::AnomalyDetector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html) resource type.
#[derive(Debug, Default)]
pub struct AnomalyDetector {
    properties: AnomalyDetectorProperties
}

/// Properties for the `AnomalyDetector` resource.
#[derive(Debug, Default)]
pub struct AnomalyDetectorProperties {
    /// Property [`AnomalyDetectorConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html#cfn-lookoutmetrics-anomalydetector-anomalydetectorconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub anomaly_detector_config: ::Value<::json::Value>,
    /// Property [`AnomalyDetectorDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html#cfn-lookoutmetrics-anomalydetector-anomalydetectordescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub anomaly_detector_description: Option<::Value<String>>,
    /// Property [`AnomalyDetectorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html#cfn-lookoutmetrics-anomalydetector-anomalydetectorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub anomaly_detector_name: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html#cfn-lookoutmetrics-anomalydetector-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`MetricSetList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html#cfn-lookoutmetrics-anomalydetector-metricsetlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_set_list: ::ValueList<self::anomaly_detector::MetricSet>,
}

impl ::serde::Serialize for AnomalyDetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnomalyDetectorConfig", &self.anomaly_detector_config)?;
        if let Some(ref anomaly_detector_description) = self.anomaly_detector_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnomalyDetectorDescription", anomaly_detector_description)?;
        }
        if let Some(ref anomaly_detector_name) = self.anomaly_detector_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnomalyDetectorName", anomaly_detector_name)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricSetList", &self.metric_set_list)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnomalyDetectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnomalyDetectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnomalyDetectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnomalyDetectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut anomaly_detector_config: Option<::Value<::json::Value>> = None;
                let mut anomaly_detector_description: Option<::Value<String>> = None;
                let mut anomaly_detector_name: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut metric_set_list: Option<::ValueList<self::anomaly_detector::MetricSet>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AnomalyDetectorConfig" => {
                            anomaly_detector_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnomalyDetectorDescription" => {
                            anomaly_detector_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnomalyDetectorName" => {
                            anomaly_detector_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricSetList" => {
                            metric_set_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnomalyDetectorProperties {
                    anomaly_detector_config: anomaly_detector_config.ok_or(::serde::de::Error::missing_field("AnomalyDetectorConfig"))?,
                    anomaly_detector_description: anomaly_detector_description,
                    anomaly_detector_name: anomaly_detector_name,
                    kms_key_arn: kms_key_arn,
                    metric_set_list: metric_set_list.ok_or(::serde::de::Error::missing_field("MetricSetList"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AnomalyDetector {
    type Properties = AnomalyDetectorProperties;
    const TYPE: &'static str = "AWS::LookoutMetrics::AnomalyDetector";
    fn properties(&self) -> &AnomalyDetectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnomalyDetectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AnomalyDetector {}

impl From<AnomalyDetectorProperties> for AnomalyDetector {
    fn from(properties: AnomalyDetectorProperties) -> AnomalyDetector {
        AnomalyDetector { properties }
    }
}

pub mod anomaly_detector {
    //! Property types for the `AnomalyDetector` resource.

    /// The [`AWS::LookoutMetrics::AnomalyDetector.AppFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-appflowconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AppFlowConfig {
        /// Property [`FlowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-appflowconfig.html#cfn-lookoutmetrics-anomalydetector-appflowconfig-flowname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-appflowconfig.html#cfn-lookoutmetrics-anomalydetector-appflowconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for AppFlowConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowName", &self.flow_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppFlowConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppFlowConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppFlowConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppFlowConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flow_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FlowName" => {
                                flow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AppFlowConfig {
                        flow_name: flow_name.ok_or(::serde::de::Error::missing_field("FlowName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.CloudwatchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-cloudwatchconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchConfig {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-cloudwatchconfig.html#cfn-lookoutmetrics-anomalydetector-cloudwatchconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchConfig {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.CsvFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct CsvFormatDescriptor {
        /// Property [`Charset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-charset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub charset: Option<::Value<String>>,
        /// Property [`ContainsHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-containsheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contains_header: Option<::Value<bool>>,
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
        /// Property [`FileCompression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-filecompression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_compression: Option<::Value<String>>,
        /// Property [`HeaderList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-headerlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_list: Option<::ValueList<String>>,
        /// Property [`QuoteSymbol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-csvformatdescriptor-quotesymbol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quote_symbol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CsvFormatDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref charset) = self.charset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Charset", charset)?;
            }
            if let Some(ref contains_header) = self.contains_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainsHeader", contains_header)?;
            }
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            if let Some(ref file_compression) = self.file_compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileCompression", file_compression)?;
            }
            if let Some(ref header_list) = self.header_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderList", header_list)?;
            }
            if let Some(ref quote_symbol) = self.quote_symbol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuoteSymbol", quote_symbol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CsvFormatDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CsvFormatDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CsvFormatDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CsvFormatDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut charset: Option<::Value<String>> = None;
                    let mut contains_header: Option<::Value<bool>> = None;
                    let mut delimiter: Option<::Value<String>> = None;
                    let mut file_compression: Option<::Value<String>> = None;
                    let mut header_list: Option<::ValueList<String>> = None;
                    let mut quote_symbol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Charset" => {
                                charset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainsHeader" => {
                                contains_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileCompression" => {
                                file_compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderList" => {
                                header_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuoteSymbol" => {
                                quote_symbol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CsvFormatDescriptor {
                        charset: charset,
                        contains_header: contains_header,
                        delimiter: delimiter,
                        file_compression: file_compression,
                        header_list: header_list,
                        quote_symbol: quote_symbol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.FileFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-fileformatdescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct FileFormatDescriptor {
        /// Property [`CsvFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-fileformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-fileformatdescriptor-csvformatdescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_format_descriptor: Option<::Value<CsvFormatDescriptor>>,
        /// Property [`JsonFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-fileformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-fileformatdescriptor-jsonformatdescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_format_descriptor: Option<::Value<JsonFormatDescriptor>>,
    }

    impl ::codec::SerializeValue for FileFormatDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv_format_descriptor) = self.csv_format_descriptor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvFormatDescriptor", csv_format_descriptor)?;
            }
            if let Some(ref json_format_descriptor) = self.json_format_descriptor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonFormatDescriptor", json_format_descriptor)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileFormatDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileFormatDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileFormatDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileFormatDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv_format_descriptor: Option<::Value<CsvFormatDescriptor>> = None;
                    let mut json_format_descriptor: Option<::Value<JsonFormatDescriptor>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CsvFormatDescriptor" => {
                                csv_format_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JsonFormatDescriptor" => {
                                json_format_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileFormatDescriptor {
                        csv_format_descriptor: csv_format_descriptor,
                        json_format_descriptor: json_format_descriptor,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.JsonFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-jsonformatdescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonFormatDescriptor {
        /// Property [`Charset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-jsonformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-jsonformatdescriptor-charset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub charset: Option<::Value<String>>,
        /// Property [`FileCompression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-jsonformatdescriptor.html#cfn-lookoutmetrics-anomalydetector-jsonformatdescriptor-filecompression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_compression: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JsonFormatDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref charset) = self.charset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Charset", charset)?;
            }
            if let Some(ref file_compression) = self.file_compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileCompression", file_compression)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonFormatDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonFormatDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonFormatDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonFormatDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut charset: Option<::Value<String>> = None;
                    let mut file_compression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Charset" => {
                                charset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileCompression" => {
                                file_compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonFormatDescriptor {
                        charset: charset,
                        file_compression: file_compression,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metric.html) property type.
    #[derive(Debug, Default)]
    pub struct Metric {
        /// Property [`AggregationFunction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metric.html#cfn-lookoutmetrics-anomalydetector-metric-aggregationfunction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregation_function: ::Value<String>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metric.html#cfn-lookoutmetrics-anomalydetector-metric-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metric.html#cfn-lookoutmetrics-anomalydetector-metric-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Metric {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationFunction", &self.aggregation_function)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
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
                    let mut aggregation_function: Option<::Value<String>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregationFunction" => {
                                aggregation_function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Metric {
                        aggregation_function: aggregation_function.ok_or(::serde::de::Error::missing_field("AggregationFunction"))?,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.MetricSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricSet {
        /// Property [`DimensionList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-dimensionlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_list: Option<::ValueList<String>>,
        /// Property [`MetricList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-metriclist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_list: ::ValueList<Metric>,
        /// Property [`MetricSetDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-metricsetdescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_set_description: Option<::Value<String>>,
        /// Property [`MetricSetFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-metricsetfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_set_frequency: Option<::Value<String>>,
        /// Property [`MetricSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-metricsetname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_set_name: ::Value<String>,
        /// Property [`MetricSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-metricsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_source: ::Value<MetricSource>,
        /// Property [`Offset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-offset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset: Option<::Value<u32>>,
        /// Property [`TimestampColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-timestampcolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_column: Option<::Value<TimestampColumn>>,
        /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html#cfn-lookoutmetrics-anomalydetector-metricset-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MetricSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimension_list) = self.dimension_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionList", dimension_list)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricList", &self.metric_list)?;
            if let Some(ref metric_set_description) = self.metric_set_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricSetDescription", metric_set_description)?;
            }
            if let Some(ref metric_set_frequency) = self.metric_set_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricSetFrequency", metric_set_frequency)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricSetName", &self.metric_set_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricSource", &self.metric_source)?;
            if let Some(ref offset) = self.offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", offset)?;
            }
            if let Some(ref timestamp_column) = self.timestamp_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampColumn", timestamp_column)?;
            }
            if let Some(ref timezone) = self.timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_list: Option<::ValueList<String>> = None;
                    let mut metric_list: Option<::ValueList<Metric>> = None;
                    let mut metric_set_description: Option<::Value<String>> = None;
                    let mut metric_set_frequency: Option<::Value<String>> = None;
                    let mut metric_set_name: Option<::Value<String>> = None;
                    let mut metric_source: Option<::Value<MetricSource>> = None;
                    let mut offset: Option<::Value<u32>> = None;
                    let mut timestamp_column: Option<::Value<TimestampColumn>> = None;
                    let mut timezone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionList" => {
                                dimension_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricList" => {
                                metric_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricSetDescription" => {
                                metric_set_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricSetFrequency" => {
                                metric_set_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricSetName" => {
                                metric_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricSource" => {
                                metric_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Offset" => {
                                offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampColumn" => {
                                timestamp_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timezone" => {
                                timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricSet {
                        dimension_list: dimension_list,
                        metric_list: metric_list.ok_or(::serde::de::Error::missing_field("MetricList"))?,
                        metric_set_description: metric_set_description,
                        metric_set_frequency: metric_set_frequency,
                        metric_set_name: metric_set_name.ok_or(::serde::de::Error::missing_field("MetricSetName"))?,
                        metric_source: metric_source.ok_or(::serde::de::Error::missing_field("MetricSource"))?,
                        offset: offset,
                        timestamp_column: timestamp_column,
                        timezone: timezone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.MetricSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricSource {
        /// Property [`AppFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html#cfn-lookoutmetrics-anomalydetector-metricsource-appflowconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_flow_config: Option<::Value<AppFlowConfig>>,
        /// Property [`CloudwatchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html#cfn-lookoutmetrics-anomalydetector-metricsource-cloudwatchconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_config: Option<::Value<CloudwatchConfig>>,
        /// Property [`RDSSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html#cfn-lookoutmetrics-anomalydetector-metricsource-rdssourceconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rds_source_config: Option<::Value<RDSSourceConfig>>,
        /// Property [`RedshiftSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html#cfn-lookoutmetrics-anomalydetector-metricsource-redshiftsourceconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_source_config: Option<::Value<RedshiftSourceConfig>>,
        /// Property [`S3SourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html#cfn-lookoutmetrics-anomalydetector-metricsource-s3sourceconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_source_config: Option<::Value<S3SourceConfig>>,
    }

    impl ::codec::SerializeValue for MetricSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_flow_config) = self.app_flow_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppFlowConfig", app_flow_config)?;
            }
            if let Some(ref cloudwatch_config) = self.cloudwatch_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchConfig", cloudwatch_config)?;
            }
            if let Some(ref rds_source_config) = self.rds_source_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RDSSourceConfig", rds_source_config)?;
            }
            if let Some(ref redshift_source_config) = self.redshift_source_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftSourceConfig", redshift_source_config)?;
            }
            if let Some(ref s3_source_config) = self.s3_source_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3SourceConfig", s3_source_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_flow_config: Option<::Value<AppFlowConfig>> = None;
                    let mut cloudwatch_config: Option<::Value<CloudwatchConfig>> = None;
                    let mut rds_source_config: Option<::Value<RDSSourceConfig>> = None;
                    let mut redshift_source_config: Option<::Value<RedshiftSourceConfig>> = None;
                    let mut s3_source_config: Option<::Value<S3SourceConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppFlowConfig" => {
                                app_flow_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudwatchConfig" => {
                                cloudwatch_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RDSSourceConfig" => {
                                rds_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftSourceConfig" => {
                                redshift_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3SourceConfig" => {
                                s3_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricSource {
                        app_flow_config: app_flow_config,
                        cloudwatch_config: cloudwatch_config,
                        rds_source_config: rds_source_config,
                        redshift_source_config: redshift_source_config,
                        s3_source_config: s3_source_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.RDSSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RDSSourceConfig {
        /// Property [`DBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-dbinstanceidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_instance_identifier: ::Value<String>,
        /// Property [`DatabaseHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-databasehost).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_host: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`DatabasePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-databaseport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_port: ::Value<u32>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html#cfn-lookoutmetrics-anomalydetector-rdssourceconfig-vpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configuration: ::Value<VpcConfiguration>,
    }

    impl ::codec::SerializeValue for RDSSourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifier", &self.db_instance_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseHost", &self.database_host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabasePort", &self.database_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", &self.secret_manager_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", &self.vpc_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RDSSourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RDSSourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RDSSourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RDSSourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut db_instance_identifier: Option<::Value<String>> = None;
                    let mut database_host: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut database_port: Option<::Value<u32>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_manager_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut vpc_configuration: Option<::Value<VpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DBInstanceIdentifier" => {
                                db_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseHost" => {
                                database_host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabasePort" => {
                                database_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RDSSourceConfig {
                        db_instance_identifier: db_instance_identifier.ok_or(::serde::de::Error::missing_field("DBInstanceIdentifier"))?,
                        database_host: database_host.ok_or(::serde::de::Error::missing_field("DatabaseHost"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        database_port: database_port.ok_or(::serde::de::Error::missing_field("DatabasePort"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_manager_arn: secret_manager_arn.ok_or(::serde::de::Error::missing_field("SecretManagerArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                        vpc_configuration: vpc_configuration.ok_or(::serde::de::Error::missing_field("VpcConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.RedshiftSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftSourceConfig {
        /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-clusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_identifier: ::Value<String>,
        /// Property [`DatabaseHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-databasehost).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_host: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`DatabasePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-databaseport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_port: ::Value<u32>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html#cfn-lookoutmetrics-anomalydetector-redshiftsourceconfig-vpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configuration: ::Value<VpcConfiguration>,
    }

    impl ::codec::SerializeValue for RedshiftSourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseHost", &self.database_host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabasePort", &self.database_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", &self.secret_manager_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", &self.vpc_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftSourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftSourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftSourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftSourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_identifier: Option<::Value<String>> = None;
                    let mut database_host: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut database_port: Option<::Value<u32>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_manager_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut vpc_configuration: Option<::Value<VpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterIdentifier" => {
                                cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseHost" => {
                                database_host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabasePort" => {
                                database_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftSourceConfig {
                        cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                        database_host: database_host.ok_or(::serde::de::Error::missing_field("DatabaseHost"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        database_port: database_port.ok_or(::serde::de::Error::missing_field("DatabasePort"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_manager_arn: secret_manager_arn.ok_or(::serde::de::Error::missing_field("SecretManagerArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                        vpc_configuration: vpc_configuration.ok_or(::serde::de::Error::missing_field("VpcConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.S3SourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3SourceConfig {
        /// Property [`FileFormatDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html#cfn-lookoutmetrics-anomalydetector-s3sourceconfig-fileformatdescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_format_descriptor: ::Value<FileFormatDescriptor>,
        /// Property [`HistoricalDataPathList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html#cfn-lookoutmetrics-anomalydetector-s3sourceconfig-historicaldatapathlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub historical_data_path_list: Option<::ValueList<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html#cfn-lookoutmetrics-anomalydetector-s3sourceconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TemplatedPathList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html#cfn-lookoutmetrics-anomalydetector-s3sourceconfig-templatedpathlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub templated_path_list: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for S3SourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileFormatDescriptor", &self.file_format_descriptor)?;
            if let Some(ref historical_data_path_list) = self.historical_data_path_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HistoricalDataPathList", historical_data_path_list)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref templated_path_list) = self.templated_path_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplatedPathList", templated_path_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3SourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3SourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3SourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3SourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file_format_descriptor: Option<::Value<FileFormatDescriptor>> = None;
                    let mut historical_data_path_list: Option<::ValueList<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut templated_path_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FileFormatDescriptor" => {
                                file_format_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HistoricalDataPathList" => {
                                historical_data_path_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplatedPathList" => {
                                templated_path_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3SourceConfig {
                        file_format_descriptor: file_format_descriptor.ok_or(::serde::de::Error::missing_field("FileFormatDescriptor"))?,
                        historical_data_path_list: historical_data_path_list,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        templated_path_list: templated_path_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.TimestampColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-timestampcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestampColumn {
        /// Property [`ColumnFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-timestampcolumn.html#cfn-lookoutmetrics-anomalydetector-timestampcolumn-columnformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_format: Option<::Value<String>>,
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-timestampcolumn.html#cfn-lookoutmetrics-anomalydetector-timestampcolumn-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TimestampColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref column_format) = self.column_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnFormat", column_format)?;
            }
            if let Some(ref column_name) = self.column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", column_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestampColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestampColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestampColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestampColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_format: Option<::Value<String>> = None;
                    let mut column_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnFormat" => {
                                column_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestampColumn {
                        column_format: column_format,
                        column_name: column_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutMetrics::AnomalyDetector.VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-vpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfiguration {
        /// Property [`SecurityGroupIdList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-vpcconfiguration.html#cfn-lookoutmetrics-anomalydetector-vpcconfiguration-securitygroupidlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_id_list: ::ValueList<String>,
        /// Property [`SubnetIdList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-vpcconfiguration.html#cfn-lookoutmetrics-anomalydetector-vpcconfiguration-subnetidlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id_list: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIdList", &self.security_group_id_list)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIdList", &self.subnet_id_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_id_list: Option<::ValueList<String>> = None;
                    let mut subnet_id_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIdList" => {
                                security_group_id_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIdList" => {
                                subnet_id_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfiguration {
                        security_group_id_list: security_group_id_list.ok_or(::serde::de::Error::missing_field("SecurityGroupIdList"))?,
                        subnet_id_list: subnet_id_list.ok_or(::serde::de::Error::missing_field("SubnetIdList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
