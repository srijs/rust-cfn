//! Types for the `SSM` service.

/// The [`AWS::SSM::Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html) resource type.
#[derive(Debug, Default)]
pub struct Association {
    properties: AssociationProperties
}

/// Properties for the `Association` resource.
#[derive(Debug, Default)]
pub struct AssociationProperties {
    /// Property [`ApplyOnlyAtCronInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-applyonlyatcroninterval).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub apply_only_at_cron_interval: Option<::Value<bool>>,
    /// Property [`AssociationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-associationname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub association_name: Option<::Value<String>>,
    /// Property [`AutomationTargetParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-automationtargetparametername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automation_target_parameter_name: Option<::Value<String>>,
    /// Property [`CalendarNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-calendarnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub calendar_names: Option<::ValueList<String>>,
    /// Property [`ComplianceSeverity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-complianceseverity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compliance_severity: Option<::Value<String>>,
    /// Property [`DocumentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-documentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub document_version: Option<::Value<String>>,
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-instanceid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_id: Option<::Value<String>>,
    /// Property [`MaxConcurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-maxconcurrency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_concurrency: Option<::Value<String>>,
    /// Property [`MaxErrors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-maxerrors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_errors: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-outputlocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_location: Option<::Value<self::association::InstanceAssociationOutputLocation>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueMap<::json::Value>>,
    /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-scheduleexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression: Option<::Value<String>>,
    /// Property [`SyncCompliance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-synccompliance).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sync_compliance: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: Option<::ValueList<self::association::Target>>,
    /// Property [`WaitForSuccessTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html#cfn-ssm-association-waitforsuccesstimeoutseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub wait_for_success_timeout_seconds: Option<::Value<u32>>,
}

impl ::serde::Serialize for AssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref apply_only_at_cron_interval) = self.apply_only_at_cron_interval {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyOnlyAtCronInterval", apply_only_at_cron_interval)?;
        }
        if let Some(ref association_name) = self.association_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationName", association_name)?;
        }
        if let Some(ref automation_target_parameter_name) = self.automation_target_parameter_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomationTargetParameterName", automation_target_parameter_name)?;
        }
        if let Some(ref calendar_names) = self.calendar_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CalendarNames", calendar_names)?;
        }
        if let Some(ref compliance_severity) = self.compliance_severity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceSeverity", compliance_severity)?;
        }
        if let Some(ref document_version) = self.document_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentVersion", document_version)?;
        }
        if let Some(ref instance_id) = self.instance_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", instance_id)?;
        }
        if let Some(ref max_concurrency) = self.max_concurrency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrency", max_concurrency)?;
        }
        if let Some(ref max_errors) = self.max_errors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxErrors", max_errors)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref output_location) = self.output_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputLocation", output_location)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref schedule_expression) = self.schedule_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
        }
        if let Some(ref sync_compliance) = self.sync_compliance {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncCompliance", sync_compliance)?;
        }
        if let Some(ref targets) = self.targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
        }
        if let Some(ref wait_for_success_timeout_seconds) = self.wait_for_success_timeout_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitForSuccessTimeoutSeconds", wait_for_success_timeout_seconds)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut apply_only_at_cron_interval: Option<::Value<bool>> = None;
                let mut association_name: Option<::Value<String>> = None;
                let mut automation_target_parameter_name: Option<::Value<String>> = None;
                let mut calendar_names: Option<::ValueList<String>> = None;
                let mut compliance_severity: Option<::Value<String>> = None;
                let mut document_version: Option<::Value<String>> = None;
                let mut instance_id: Option<::Value<String>> = None;
                let mut max_concurrency: Option<::Value<String>> = None;
                let mut max_errors: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut output_location: Option<::Value<self::association::InstanceAssociationOutputLocation>> = None;
                let mut parameters: Option<::ValueMap<::json::Value>> = None;
                let mut schedule_expression: Option<::Value<String>> = None;
                let mut sync_compliance: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::association::Target>> = None;
                let mut wait_for_success_timeout_seconds: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplyOnlyAtCronInterval" => {
                            apply_only_at_cron_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociationName" => {
                            association_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutomationTargetParameterName" => {
                            automation_target_parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CalendarNames" => {
                            calendar_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComplianceSeverity" => {
                            compliance_severity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentVersion" => {
                            document_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxConcurrency" => {
                            max_concurrency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxErrors" => {
                            max_errors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputLocation" => {
                            output_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpression" => {
                            schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncCompliance" => {
                            sync_compliance = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WaitForSuccessTimeoutSeconds" => {
                            wait_for_success_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssociationProperties {
                    apply_only_at_cron_interval: apply_only_at_cron_interval,
                    association_name: association_name,
                    automation_target_parameter_name: automation_target_parameter_name,
                    calendar_names: calendar_names,
                    compliance_severity: compliance_severity,
                    document_version: document_version,
                    instance_id: instance_id,
                    max_concurrency: max_concurrency,
                    max_errors: max_errors,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    output_location: output_location,
                    parameters: parameters,
                    schedule_expression: schedule_expression,
                    sync_compliance: sync_compliance,
                    targets: targets,
                    wait_for_success_timeout_seconds: wait_for_success_timeout_seconds,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Association {
    type Properties = AssociationProperties;
    const TYPE: &'static str = "AWS::SSM::Association";
    fn properties(&self) -> &AssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Association {}

impl From<AssociationProperties> for Association {
    fn from(properties: AssociationProperties) -> Association {
        Association { properties }
    }
}

/// The [`AWS::SSM::Document`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html) resource type.
#[derive(Debug, Default)]
pub struct Document {
    properties: DocumentProperties
}

/// Properties for the `Document` resource.
#[derive(Debug, Default)]
pub struct DocumentProperties {
    /// Property [`Attachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-attachments).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub attachments: Option<::ValueList<self::document::AttachmentsSource>>,
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-content).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub content: ::Value<::json::Value>,
    /// Property [`DocumentFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-documentformat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub document_format: Option<::Value<String>>,
    /// Property [`DocumentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-documenttype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub document_type: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Requires`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-requires).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub requires: Option<::ValueList<self::document::DocumentRequires>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-targettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_type: Option<::Value<String>>,
    /// Property [`VersionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html#cfn-ssm-document-versionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DocumentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attachments) = self.attachments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attachments", attachments)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref document_format) = self.document_format {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentFormat", document_format)?;
        }
        if let Some(ref document_type) = self.document_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentType", document_type)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref requires) = self.requires {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Requires", requires)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_type) = self.target_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", target_type)?;
        }
        if let Some(ref version_name) = self.version_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionName", version_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DocumentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DocumentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DocumentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attachments: Option<::ValueList<self::document::AttachmentsSource>> = None;
                let mut content: Option<::Value<::json::Value>> = None;
                let mut document_format: Option<::Value<String>> = None;
                let mut document_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut requires: Option<::ValueList<self::document::DocumentRequires>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_type: Option<::Value<String>> = None;
                let mut version_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attachments" => {
                            attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentFormat" => {
                            document_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentType" => {
                            document_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Requires" => {
                            requires = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionName" => {
                            version_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DocumentProperties {
                    attachments: attachments,
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    document_format: document_format,
                    document_type: document_type,
                    name: name,
                    requires: requires,
                    tags: tags,
                    target_type: target_type,
                    version_name: version_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Document {
    type Properties = DocumentProperties;
    const TYPE: &'static str = "AWS::SSM::Document";
    fn properties(&self) -> &DocumentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Document {}

impl From<DocumentProperties> for Document {
    fn from(properties: DocumentProperties) -> Document {
        Document { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html) resource type.
#[derive(Debug, Default)]
pub struct MaintenanceWindow {
    properties: MaintenanceWindowProperties
}

/// Properties for the `MaintenanceWindow` resource.
#[derive(Debug, Default)]
pub struct MaintenanceWindowProperties {
    /// Property [`AllowUnassociatedTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-allowunassociatedtargets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_unassociated_targets: ::Value<bool>,
    /// Property [`Cutoff`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-cutoff).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cutoff: ::Value<u32>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Duration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-duration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub duration: ::Value<u32>,
    /// Property [`EndDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-enddate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_date: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: ::Value<String>,
    /// Property [`ScheduleOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-scheduleoffset).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_offset: Option<::Value<u32>>,
    /// Property [`ScheduleTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-scheduletimezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_timezone: Option<::Value<String>>,
    /// Property [`StartDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-startdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_date: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html#cfn-ssm-maintenancewindow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MaintenanceWindowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowUnassociatedTargets", &self.allow_unassociated_targets)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cutoff", &self.cutoff)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Duration", &self.duration)?;
        if let Some(ref end_date) = self.end_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndDate", end_date)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", &self.schedule)?;
        if let Some(ref schedule_offset) = self.schedule_offset {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleOffset", schedule_offset)?;
        }
        if let Some(ref schedule_timezone) = self.schedule_timezone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleTimezone", schedule_timezone)?;
        }
        if let Some(ref start_date) = self.start_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartDate", start_date)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MaintenanceWindowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MaintenanceWindowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MaintenanceWindowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_unassociated_targets: Option<::Value<bool>> = None;
                let mut cutoff: Option<::Value<u32>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut duration: Option<::Value<u32>> = None;
                let mut end_date: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<String>> = None;
                let mut schedule_offset: Option<::Value<u32>> = None;
                let mut schedule_timezone: Option<::Value<String>> = None;
                let mut start_date: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowUnassociatedTargets" => {
                            allow_unassociated_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cutoff" => {
                            cutoff = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Duration" => {
                            duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndDate" => {
                            end_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleOffset" => {
                            schedule_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleTimezone" => {
                            schedule_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartDate" => {
                            start_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MaintenanceWindowProperties {
                    allow_unassociated_targets: allow_unassociated_targets.ok_or(::serde::de::Error::missing_field("AllowUnassociatedTargets"))?,
                    cutoff: cutoff.ok_or(::serde::de::Error::missing_field("Cutoff"))?,
                    description: description,
                    duration: duration.ok_or(::serde::de::Error::missing_field("Duration"))?,
                    end_date: end_date,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schedule: schedule.ok_or(::serde::de::Error::missing_field("Schedule"))?,
                    schedule_offset: schedule_offset,
                    schedule_timezone: schedule_timezone,
                    start_date: start_date,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MaintenanceWindow {
    type Properties = MaintenanceWindowProperties;
    const TYPE: &'static str = "AWS::SSM::MaintenanceWindow";
    fn properties(&self) -> &MaintenanceWindowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MaintenanceWindowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MaintenanceWindow {}

impl From<MaintenanceWindowProperties> for MaintenanceWindow {
    fn from(properties: MaintenanceWindowProperties) -> MaintenanceWindow {
        MaintenanceWindow { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindowTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html) resource type.
#[derive(Debug, Default)]
pub struct MaintenanceWindowTarget {
    properties: MaintenanceWindowTargetProperties
}

/// Properties for the `MaintenanceWindowTarget` resource.
#[derive(Debug, Default)]
pub struct MaintenanceWindowTargetProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`OwnerInformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-ownerinformation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub owner_information: Option<::Value<String>>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-resourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_type: ::Value<String>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: ::ValueList<self::maintenance_window_target::Targets>,
    /// Property [`WindowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html#cfn-ssm-maintenancewindowtarget-windowid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub window_id: ::Value<String>,
}

impl ::serde::Serialize for MaintenanceWindowTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref owner_information) = self.owner_information {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerInformation", owner_information)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowId", &self.window_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MaintenanceWindowTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MaintenanceWindowTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MaintenanceWindowTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut owner_information: Option<::Value<String>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::maintenance_window_target::Targets>> = None;
                let mut window_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerInformation" => {
                            owner_information = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WindowId" => {
                            window_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MaintenanceWindowTargetProperties {
                    description: description,
                    name: name,
                    owner_information: owner_information,
                    resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                    window_id: window_id.ok_or(::serde::de::Error::missing_field("WindowId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MaintenanceWindowTarget {
    type Properties = MaintenanceWindowTargetProperties;
    const TYPE: &'static str = "AWS::SSM::MaintenanceWindowTarget";
    fn properties(&self) -> &MaintenanceWindowTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MaintenanceWindowTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MaintenanceWindowTarget {}

impl From<MaintenanceWindowTargetProperties> for MaintenanceWindowTarget {
    fn from(properties: MaintenanceWindowTargetProperties) -> MaintenanceWindowTarget {
        MaintenanceWindowTarget { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindowTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html) resource type.
#[derive(Debug, Default)]
pub struct MaintenanceWindowTask {
    properties: MaintenanceWindowTaskProperties
}

/// Properties for the `MaintenanceWindowTask` resource.
#[derive(Debug, Default)]
pub struct MaintenanceWindowTaskProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-logginginfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_info: Option<::Value<self::maintenance_window_task::LoggingInfo>>,
    /// Property [`MaxConcurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-maxconcurrency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_concurrency: Option<::Value<String>>,
    /// Property [`MaxErrors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-maxerrors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_errors: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-priority).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority: ::Value<u32>,
    /// Property [`ServiceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-servicerolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_role_arn: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: Option<::ValueList<self::maintenance_window_task::Target>>,
    /// Property [`TaskArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-taskarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_arn: ::Value<String>,
    /// Property [`TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-taskinvocationparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_invocation_parameters: Option<::Value<self::maintenance_window_task::TaskInvocationParameters>>,
    /// Property [`TaskParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-taskparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_parameters: Option<::Value<::json::Value>>,
    /// Property [`TaskType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-tasktype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub task_type: ::Value<String>,
    /// Property [`WindowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html#cfn-ssm-maintenancewindowtask-windowid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub window_id: ::Value<String>,
}

impl ::serde::Serialize for MaintenanceWindowTaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref logging_info) = self.logging_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingInfo", logging_info)?;
        }
        if let Some(ref max_concurrency) = self.max_concurrency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrency", max_concurrency)?;
        }
        if let Some(ref max_errors) = self.max_errors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxErrors", max_errors)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        if let Some(ref service_role_arn) = self.service_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", service_role_arn)?;
        }
        if let Some(ref targets) = self.targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskArn", &self.task_arn)?;
        if let Some(ref task_invocation_parameters) = self.task_invocation_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskInvocationParameters", task_invocation_parameters)?;
        }
        if let Some(ref task_parameters) = self.task_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskParameters", task_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskType", &self.task_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowId", &self.window_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MaintenanceWindowTaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowTaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MaintenanceWindowTaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MaintenanceWindowTaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut logging_info: Option<::Value<self::maintenance_window_task::LoggingInfo>> = None;
                let mut max_concurrency: Option<::Value<String>> = None;
                let mut max_errors: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut priority: Option<::Value<u32>> = None;
                let mut service_role_arn: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::maintenance_window_task::Target>> = None;
                let mut task_arn: Option<::Value<String>> = None;
                let mut task_invocation_parameters: Option<::Value<self::maintenance_window_task::TaskInvocationParameters>> = None;
                let mut task_parameters: Option<::Value<::json::Value>> = None;
                let mut task_type: Option<::Value<String>> = None;
                let mut window_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingInfo" => {
                            logging_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxConcurrency" => {
                            max_concurrency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxErrors" => {
                            max_errors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskArn" => {
                            task_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskInvocationParameters" => {
                            task_invocation_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskParameters" => {
                            task_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskType" => {
                            task_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WindowId" => {
                            window_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MaintenanceWindowTaskProperties {
                    description: description,
                    logging_info: logging_info,
                    max_concurrency: max_concurrency,
                    max_errors: max_errors,
                    name: name,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                    service_role_arn: service_role_arn,
                    targets: targets,
                    task_arn: task_arn.ok_or(::serde::de::Error::missing_field("TaskArn"))?,
                    task_invocation_parameters: task_invocation_parameters,
                    task_parameters: task_parameters,
                    task_type: task_type.ok_or(::serde::de::Error::missing_field("TaskType"))?,
                    window_id: window_id.ok_or(::serde::de::Error::missing_field("WindowId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MaintenanceWindowTask {
    type Properties = MaintenanceWindowTaskProperties;
    const TYPE: &'static str = "AWS::SSM::MaintenanceWindowTask";
    fn properties(&self) -> &MaintenanceWindowTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MaintenanceWindowTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MaintenanceWindowTask {}

impl From<MaintenanceWindowTaskProperties> for MaintenanceWindowTask {
    fn from(properties: MaintenanceWindowTaskProperties) -> MaintenanceWindowTask {
        MaintenanceWindowTask { properties }
    }
}

/// The [`AWS::SSM::Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html) resource type.
#[derive(Debug, Default)]
pub struct Parameter {
    properties: ParameterProperties
}

/// Properties for the `Parameter` resource.
#[derive(Debug, Default)]
pub struct ParameterProperties {
    /// Property [`AllowedPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-allowedpattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_pattern: Option<::Value<String>>,
    /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-datatype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_type: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-tier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tier: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
    /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html#cfn-ssm-parameter-value).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub value: ::Value<String>,
}

impl ::serde::Serialize for ParameterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allowed_pattern) = self.allowed_pattern {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedPattern", allowed_pattern)?;
        }
        if let Some(ref data_type) = self.data_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tier) = self.tier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", tier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ParameterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ParameterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ParameterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_pattern: Option<::Value<String>> = None;
                let mut data_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut policies: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut tier: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut value: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedPattern" => {
                            allowed_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataType" => {
                            data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tier" => {
                            tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Value" => {
                            value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ParameterProperties {
                    allowed_pattern: allowed_pattern,
                    data_type: data_type,
                    description: description,
                    name: name,
                    policies: policies,
                    tags: tags,
                    tier: tier,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Parameter {
    type Properties = ParameterProperties;
    const TYPE: &'static str = "AWS::SSM::Parameter";
    fn properties(&self) -> &ParameterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Parameter {}

impl From<ParameterProperties> for Parameter {
    fn from(properties: ParameterProperties) -> Parameter {
        Parameter { properties }
    }
}

/// The [`AWS::SSM::PatchBaseline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html) resource type.
#[derive(Debug, Default)]
pub struct PatchBaseline {
    properties: PatchBaselineProperties
}

/// Properties for the `PatchBaseline` resource.
#[derive(Debug, Default)]
pub struct PatchBaselineProperties {
    /// Property [`ApprovalRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-approvalrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approval_rules: Option<::Value<self::patch_baseline::RuleGroup>>,
    /// Property [`ApprovedPatches`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-approvedpatches).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approved_patches: Option<::ValueList<String>>,
    /// Property [`ApprovedPatchesComplianceLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-approvedpatchescompliancelevel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approved_patches_compliance_level: Option<::Value<String>>,
    /// Property [`ApprovedPatchesEnableNonSecurity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-approvedpatchesenablenonsecurity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approved_patches_enable_non_security: Option<::Value<bool>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlobalFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-globalfilters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_filters: Option<::Value<self::patch_baseline::PatchFilterGroup>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OperatingSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-operatingsystem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub operating_system: Option<::Value<String>>,
    /// Property [`PatchGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-patchgroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub patch_groups: Option<::ValueList<String>>,
    /// Property [`RejectedPatches`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-rejectedpatches).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rejected_patches: Option<::ValueList<String>>,
    /// Property [`RejectedPatchesAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-rejectedpatchesaction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rejected_patches_action: Option<::Value<String>>,
    /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-sources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sources: Option<::ValueList<self::patch_baseline::PatchSource>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html#cfn-ssm-patchbaseline-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PatchBaselineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref approval_rules) = self.approval_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovalRules", approval_rules)?;
        }
        if let Some(ref approved_patches) = self.approved_patches {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatches", approved_patches)?;
        }
        if let Some(ref approved_patches_compliance_level) = self.approved_patches_compliance_level {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatchesComplianceLevel", approved_patches_compliance_level)?;
        }
        if let Some(ref approved_patches_enable_non_security) = self.approved_patches_enable_non_security {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatchesEnableNonSecurity", approved_patches_enable_non_security)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref global_filters) = self.global_filters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalFilters", global_filters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref operating_system) = self.operating_system {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatingSystem", operating_system)?;
        }
        if let Some(ref patch_groups) = self.patch_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchGroups", patch_groups)?;
        }
        if let Some(ref rejected_patches) = self.rejected_patches {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RejectedPatches", rejected_patches)?;
        }
        if let Some(ref rejected_patches_action) = self.rejected_patches_action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RejectedPatchesAction", rejected_patches_action)?;
        }
        if let Some(ref sources) = self.sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PatchBaselineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchBaselineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PatchBaselineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PatchBaselineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut approval_rules: Option<::Value<self::patch_baseline::RuleGroup>> = None;
                let mut approved_patches: Option<::ValueList<String>> = None;
                let mut approved_patches_compliance_level: Option<::Value<String>> = None;
                let mut approved_patches_enable_non_security: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut global_filters: Option<::Value<self::patch_baseline::PatchFilterGroup>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut operating_system: Option<::Value<String>> = None;
                let mut patch_groups: Option<::ValueList<String>> = None;
                let mut rejected_patches: Option<::ValueList<String>> = None;
                let mut rejected_patches_action: Option<::Value<String>> = None;
                let mut sources: Option<::ValueList<self::patch_baseline::PatchSource>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApprovalRules" => {
                            approval_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApprovedPatches" => {
                            approved_patches = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApprovedPatchesComplianceLevel" => {
                            approved_patches_compliance_level = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApprovedPatchesEnableNonSecurity" => {
                            approved_patches_enable_non_security = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalFilters" => {
                            global_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OperatingSystem" => {
                            operating_system = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PatchGroups" => {
                            patch_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RejectedPatches" => {
                            rejected_patches = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RejectedPatchesAction" => {
                            rejected_patches_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sources" => {
                            sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PatchBaselineProperties {
                    approval_rules: approval_rules,
                    approved_patches: approved_patches,
                    approved_patches_compliance_level: approved_patches_compliance_level,
                    approved_patches_enable_non_security: approved_patches_enable_non_security,
                    description: description,
                    global_filters: global_filters,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    operating_system: operating_system,
                    patch_groups: patch_groups,
                    rejected_patches: rejected_patches,
                    rejected_patches_action: rejected_patches_action,
                    sources: sources,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PatchBaseline {
    type Properties = PatchBaselineProperties;
    const TYPE: &'static str = "AWS::SSM::PatchBaseline";
    fn properties(&self) -> &PatchBaselineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PatchBaselineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PatchBaseline {}

impl From<PatchBaselineProperties> for PatchBaseline {
    fn from(properties: PatchBaselineProperties) -> PatchBaseline {
        PatchBaseline { properties }
    }
}

/// The [`AWS::SSM::ResourceDataSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceDataSync {
    properties: ResourceDataSyncProperties
}

/// Properties for the `ResourceDataSync` resource.
#[derive(Debug, Default)]
pub struct ResourceDataSyncProperties {
    /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-bucketname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_name: Option<::Value<String>>,
    /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-bucketprefix).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_prefix: Option<::Value<String>>,
    /// Property [`BucketRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-bucketregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bucket_region: Option<::Value<String>>,
    /// Property [`KMSKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-kmskeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-s3destination).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_destination: Option<::Value<self::resource_data_sync::S3Destination>>,
    /// Property [`SyncFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-syncformat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_format: Option<::Value<String>>,
    /// Property [`SyncName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-syncname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_name: ::Value<String>,
    /// Property [`SyncSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-syncsource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sync_source: Option<::Value<self::resource_data_sync::SyncSource>>,
    /// Property [`SyncType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html#cfn-ssm-resourcedatasync-synctype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sync_type: Option<::Value<String>>,
}

impl ::serde::Serialize for ResourceDataSyncProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref bucket_name) = self.bucket_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
        }
        if let Some(ref bucket_prefix) = self.bucket_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
        }
        if let Some(ref bucket_region) = self.bucket_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketRegion", bucket_region)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyArn", kms_key_arn)?;
        }
        if let Some(ref s3_destination) = self.s3_destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Destination", s3_destination)?;
        }
        if let Some(ref sync_format) = self.sync_format {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncFormat", sync_format)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncName", &self.sync_name)?;
        if let Some(ref sync_source) = self.sync_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncSource", sync_source)?;
        }
        if let Some(ref sync_type) = self.sync_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncType", sync_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceDataSyncProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceDataSyncProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceDataSyncProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceDataSyncProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bucket_name: Option<::Value<String>> = None;
                let mut bucket_prefix: Option<::Value<String>> = None;
                let mut bucket_region: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut s3_destination: Option<::Value<self::resource_data_sync::S3Destination>> = None;
                let mut sync_format: Option<::Value<String>> = None;
                let mut sync_name: Option<::Value<String>> = None;
                let mut sync_source: Option<::Value<self::resource_data_sync::SyncSource>> = None;
                let mut sync_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BucketName" => {
                            bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketPrefix" => {
                            bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BucketRegion" => {
                            bucket_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Destination" => {
                            s3_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncFormat" => {
                            sync_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncName" => {
                            sync_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncSource" => {
                            sync_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncType" => {
                            sync_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceDataSyncProperties {
                    bucket_name: bucket_name,
                    bucket_prefix: bucket_prefix,
                    bucket_region: bucket_region,
                    kms_key_arn: kms_key_arn,
                    s3_destination: s3_destination,
                    sync_format: sync_format,
                    sync_name: sync_name.ok_or(::serde::de::Error::missing_field("SyncName"))?,
                    sync_source: sync_source,
                    sync_type: sync_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceDataSync {
    type Properties = ResourceDataSyncProperties;
    const TYPE: &'static str = "AWS::SSM::ResourceDataSync";
    fn properties(&self) -> &ResourceDataSyncProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceDataSyncProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceDataSync {}

impl From<ResourceDataSyncProperties> for ResourceDataSync {
    fn from(properties: ResourceDataSyncProperties) -> ResourceDataSync {
        ResourceDataSync { properties }
    }
}

pub mod association {
    //! Property types for the `Association` resource.

    /// The [`AWS::SSM::Association.InstanceAssociationOutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-instanceassociationoutputlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceAssociationOutputLocation {
        /// Property [`S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-instanceassociationoutputlocation.html#cfn-ssm-association-instanceassociationoutputlocation-s3location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_location: Option<::Value<S3OutputLocation>>,
    }

    impl ::codec::SerializeValue for InstanceAssociationOutputLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_location) = self.s3_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Location", s3_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceAssociationOutputLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceAssociationOutputLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceAssociationOutputLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceAssociationOutputLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_location: Option<::Value<S3OutputLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Location" => {
                                s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceAssociationOutputLocation {
                        s3_location: s3_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::Association.S3OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-s3outputlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3OutputLocation {
        /// Property [`OutputS3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-s3outputlocation.html#cfn-ssm-association-s3outputlocation-outputs3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_bucket_name: Option<::Value<String>>,
        /// Property [`OutputS3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-s3outputlocation.html#cfn-ssm-association-s3outputlocation-outputs3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_key_prefix: Option<::Value<String>>,
        /// Property [`OutputS3Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-s3outputlocation.html#cfn-ssm-association-s3outputlocation-outputs3region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3OutputLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref output_s3_bucket_name) = self.output_s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3BucketName", output_s3_bucket_name)?;
            }
            if let Some(ref output_s3_key_prefix) = self.output_s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3KeyPrefix", output_s3_key_prefix)?;
            }
            if let Some(ref output_s3_region) = self.output_s3_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3Region", output_s3_region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3OutputLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3OutputLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3OutputLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3OutputLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut output_s3_bucket_name: Option<::Value<String>> = None;
                    let mut output_s3_key_prefix: Option<::Value<String>> = None;
                    let mut output_s3_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OutputS3BucketName" => {
                                output_s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3KeyPrefix" => {
                                output_s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3Region" => {
                                output_s3_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3OutputLocation {
                        output_s3_bucket_name: output_s3_bucket_name,
                        output_s3_key_prefix: output_s3_key_prefix,
                        output_s3_region: output_s3_region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::Association.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html#cfn-ssm-association-target-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html#cfn-ssm-association-target-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod document {
    //! Property types for the `Document` resource.

    /// The [`AWS::SSM::Document.AttachmentsSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-attachmentssource.html) property type.
    #[derive(Debug, Default)]
    pub struct AttachmentsSource {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-attachmentssource.html#cfn-ssm-document-attachmentssource-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-attachmentssource.html#cfn-ssm-document-attachmentssource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-attachmentssource.html#cfn-ssm-document-attachmentssource-values).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AttachmentsSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttachmentsSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttachmentsSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttachmentsSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttachmentsSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttachmentsSource {
                        key: key,
                        name: name,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::Document.DocumentRequires`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-documentrequires.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentRequires {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-documentrequires.html#cfn-ssm-document-documentrequires-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-documentrequires.html#cfn-ssm-document-documentrequires-version).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentRequires {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentRequires {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentRequires, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentRequires;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentRequires")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentRequires {
                        name: name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod maintenance_window_target {
    //! Property types for the `MaintenanceWindowTarget` resource.

    /// The [`AWS::SSM::MaintenanceWindowTarget.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtarget-targets.html) property type.
    #[derive(Debug, Default)]
    pub struct Targets {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtarget-targets.html#cfn-ssm-maintenancewindowtarget-targets-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtarget-targets.html#cfn-ssm-maintenancewindowtarget-targets-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for Targets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Targets {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Targets, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Targets;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Targets")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Targets {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod maintenance_window_task {
    //! Property types for the `MaintenanceWindowTask` resource.

    /// The [`AWS::SSM::MaintenanceWindowTask.LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingInfo {
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html#cfn-ssm-maintenancewindowtask-logginginfo-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html#cfn-ssm-maintenancewindowtask-logginginfo-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html#cfn-ssm-maintenancewindowtask-logginginfo-s3prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            if let Some(ref s3_prefix) = self.s3_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Prefix", s3_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut region: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Prefix" => {
                                s3_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingInfo {
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_prefix: s3_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindowAutomationParameters {
        /// Property [`DocumentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowautomationparameters-documentversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_version: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowautomationparameters-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowAutomationParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref document_version) = self.document_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentVersion", document_version)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowAutomationParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowAutomationParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowAutomationParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowAutomationParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_version: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentVersion" => {
                                document_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowAutomationParameters {
                        document_version: document_version,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindowLambdaParameters {
        /// Property [`ClientContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowlambdaparameters-clientcontext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_context: Option<::Value<String>>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowlambdaparameters-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<::Value<String>>,
        /// Property [`Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowlambdaparameters-qualifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub qualifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowLambdaParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_context) = self.client_context {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientContext", client_context)?;
            }
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            if let Some(ref qualifier) = self.qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", qualifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowLambdaParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowLambdaParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowLambdaParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowLambdaParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_context: Option<::Value<String>> = None;
                    let mut payload: Option<::Value<String>> = None;
                    let mut qualifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientContext" => {
                                client_context = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Qualifier" => {
                                qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowLambdaParameters {
                        client_context: client_context,
                        payload: payload,
                        qualifier: qualifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindowRunCommandParameters {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`DocumentHash`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-documenthash).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_hash: Option<::Value<String>>,
        /// Property [`DocumentHashType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-documenthashtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_hash_type: Option<::Value<String>>,
        /// Property [`NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-notificationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_config: Option<::Value<NotificationConfig>>,
        /// Property [`OutputS3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-outputs3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_bucket_name: Option<::Value<String>>,
        /// Property [`OutputS3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-outputs3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_key_prefix: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`ServiceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-servicerolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_role_arn: Option<::Value<String>>,
        /// Property [`TimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowruncommandparameters-timeoutseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowRunCommandParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref document_hash) = self.document_hash {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentHash", document_hash)?;
            }
            if let Some(ref document_hash_type) = self.document_hash_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentHashType", document_hash_type)?;
            }
            if let Some(ref notification_config) = self.notification_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfig", notification_config)?;
            }
            if let Some(ref output_s3_bucket_name) = self.output_s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3BucketName", output_s3_bucket_name)?;
            }
            if let Some(ref output_s3_key_prefix) = self.output_s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3KeyPrefix", output_s3_key_prefix)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref service_role_arn) = self.service_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", service_role_arn)?;
            }
            if let Some(ref timeout_seconds) = self.timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutSeconds", timeout_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowRunCommandParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowRunCommandParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowRunCommandParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowRunCommandParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut document_hash: Option<::Value<String>> = None;
                    let mut document_hash_type: Option<::Value<String>> = None;
                    let mut notification_config: Option<::Value<NotificationConfig>> = None;
                    let mut output_s3_bucket_name: Option<::Value<String>> = None;
                    let mut output_s3_key_prefix: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut service_role_arn: Option<::Value<String>> = None;
                    let mut timeout_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentHash" => {
                                document_hash = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentHashType" => {
                                document_hash_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationConfig" => {
                                notification_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3BucketName" => {
                                output_s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3KeyPrefix" => {
                                output_s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceRoleArn" => {
                                service_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutSeconds" => {
                                timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowRunCommandParameters {
                        comment: comment,
                        document_hash: document_hash,
                        document_hash_type: document_hash_type,
                        notification_config: notification_config,
                        output_s3_bucket_name: output_s3_bucket_name,
                        output_s3_key_prefix: output_s3_key_prefix,
                        parameters: parameters,
                        service_role_arn: service_role_arn,
                        timeout_seconds: timeout_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters-input).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html#cfn-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowStepFunctionsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input) = self.input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", input)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowStepFunctionsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowStepFunctionsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowStepFunctionsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowStepFunctionsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Input" => {
                                input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowStepFunctionsParameters {
                        input: input,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationConfig {
        /// Property [`NotificationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html#cfn-ssm-maintenancewindowtask-notificationconfig-notificationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_arn: ::Value<String>,
        /// Property [`NotificationEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html#cfn-ssm-maintenancewindowtask-notificationconfig-notificationevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_events: Option<::ValueList<String>>,
        /// Property [`NotificationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html#cfn-ssm-maintenancewindowtask-notificationconfig-notificationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NotificationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationArn", &self.notification_arn)?;
            if let Some(ref notification_events) = self.notification_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationEvents", notification_events)?;
            }
            if let Some(ref notification_type) = self.notification_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationType", notification_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification_arn: Option<::Value<String>> = None;
                    let mut notification_events: Option<::ValueList<String>> = None;
                    let mut notification_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotificationArn" => {
                                notification_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationEvents" => {
                                notification_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationType" => {
                                notification_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfig {
                        notification_arn: notification_arn.ok_or(::serde::de::Error::missing_field("NotificationArn"))?,
                        notification_events: notification_events,
                        notification_type: notification_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html#cfn-ssm-maintenancewindowtask-target-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html#cfn-ssm-maintenancewindowtask-target-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskInvocationParameters {
        /// Property [`MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html#cfn-ssm-maintenancewindowtask-taskinvocationparameters-maintenancewindowautomationparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maintenance_window_automation_parameters: Option<::Value<MaintenanceWindowAutomationParameters>>,
        /// Property [`MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html#cfn-ssm-maintenancewindowtask-taskinvocationparameters-maintenancewindowlambdaparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maintenance_window_lambda_parameters: Option<::Value<MaintenanceWindowLambdaParameters>>,
        /// Property [`MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html#cfn-ssm-maintenancewindowtask-taskinvocationparameters-maintenancewindowruncommandparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maintenance_window_run_command_parameters: Option<::Value<MaintenanceWindowRunCommandParameters>>,
        /// Property [`MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html#cfn-ssm-maintenancewindowtask-taskinvocationparameters-maintenancewindowstepfunctionsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maintenance_window_step_functions_parameters: Option<::Value<MaintenanceWindowStepFunctionsParameters>>,
    }

    impl ::codec::SerializeValue for TaskInvocationParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maintenance_window_automation_parameters) = self.maintenance_window_automation_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowAutomationParameters", maintenance_window_automation_parameters)?;
            }
            if let Some(ref maintenance_window_lambda_parameters) = self.maintenance_window_lambda_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowLambdaParameters", maintenance_window_lambda_parameters)?;
            }
            if let Some(ref maintenance_window_run_command_parameters) = self.maintenance_window_run_command_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowRunCommandParameters", maintenance_window_run_command_parameters)?;
            }
            if let Some(ref maintenance_window_step_functions_parameters) = self.maintenance_window_step_functions_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowStepFunctionsParameters", maintenance_window_step_functions_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskInvocationParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskInvocationParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskInvocationParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskInvocationParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maintenance_window_automation_parameters: Option<::Value<MaintenanceWindowAutomationParameters>> = None;
                    let mut maintenance_window_lambda_parameters: Option<::Value<MaintenanceWindowLambdaParameters>> = None;
                    let mut maintenance_window_run_command_parameters: Option<::Value<MaintenanceWindowRunCommandParameters>> = None;
                    let mut maintenance_window_step_functions_parameters: Option<::Value<MaintenanceWindowStepFunctionsParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaintenanceWindowAutomationParameters" => {
                                maintenance_window_automation_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaintenanceWindowLambdaParameters" => {
                                maintenance_window_lambda_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaintenanceWindowRunCommandParameters" => {
                                maintenance_window_run_command_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaintenanceWindowStepFunctionsParameters" => {
                                maintenance_window_step_functions_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskInvocationParameters {
                        maintenance_window_automation_parameters: maintenance_window_automation_parameters,
                        maintenance_window_lambda_parameters: maintenance_window_lambda_parameters,
                        maintenance_window_run_command_parameters: maintenance_window_run_command_parameters,
                        maintenance_window_step_functions_parameters: maintenance_window_step_functions_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod patch_baseline {
    //! Property types for the `PatchBaseline` resource.

    /// The [`AWS::SSM::PatchBaseline.PatchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct PatchFilter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html#cfn-ssm-patchbaseline-patchfilter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html#cfn-ssm-patchbaseline-patchfilter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PatchFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchFilter {
                        key: key,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html) property type.
    #[derive(Debug, Default)]
    pub struct PatchFilterGroup {
        /// Property [`PatchFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html#cfn-ssm-patchbaseline-patchfiltergroup-patchfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub patch_filters: Option<::ValueList<PatchFilter>>,
    }

    impl ::codec::SerializeValue for PatchFilterGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref patch_filters) = self.patch_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchFilters", patch_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchFilterGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchFilterGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchFilterGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchFilterGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut patch_filters: Option<::ValueList<PatchFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PatchFilters" => {
                                patch_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchFilterGroup {
                        patch_filters: patch_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.PatchSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html) property type.
    #[derive(Debug, Default)]
    pub struct PatchSource {
        /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html#cfn-ssm-patchbaseline-patchsource-configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html#cfn-ssm-patchbaseline-patchsource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Products`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html#cfn-ssm-patchbaseline-patchsource-products).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub products: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PatchSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configuration) = self.configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref products) = self.products {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Products", products)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut products: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configuration" => {
                                configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Products" => {
                                products = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchSource {
                        configuration: configuration,
                        name: name,
                        products: products,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.PatchStringDate`]() property type.
    #[derive(Debug, Default)]
    pub struct PatchStringDate {
    }

    impl ::codec::SerializeValue for PatchStringDate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchStringDate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchStringDate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchStringDate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchStringDate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(PatchStringDate {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`ApproveAfterDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html#cfn-ssm-patchbaseline-rule-approveafterdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub approve_after_days: Option<::Value<u32>>,
        /// Property [`ApproveUntilDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html#cfn-ssm-patchbaseline-rule-approveuntildate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub approve_until_date: Option<::Value<PatchStringDate>>,
        /// Property [`ComplianceLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html#cfn-ssm-patchbaseline-rule-compliancelevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_level: Option<::Value<String>>,
        /// Property [`EnableNonSecurity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html#cfn-ssm-patchbaseline-rule-enablenonsecurity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_non_security: Option<::Value<bool>>,
        /// Property [`PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html#cfn-ssm-patchbaseline-rule-patchfiltergroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub patch_filter_group: Option<::Value<PatchFilterGroup>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref approve_after_days) = self.approve_after_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApproveAfterDays", approve_after_days)?;
            }
            if let Some(ref approve_until_date) = self.approve_until_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApproveUntilDate", approve_until_date)?;
            }
            if let Some(ref compliance_level) = self.compliance_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceLevel", compliance_level)?;
            }
            if let Some(ref enable_non_security) = self.enable_non_security {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNonSecurity", enable_non_security)?;
            }
            if let Some(ref patch_filter_group) = self.patch_filter_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchFilterGroup", patch_filter_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut approve_after_days: Option<::Value<u32>> = None;
                    let mut approve_until_date: Option<::Value<PatchStringDate>> = None;
                    let mut compliance_level: Option<::Value<String>> = None;
                    let mut enable_non_security: Option<::Value<bool>> = None;
                    let mut patch_filter_group: Option<::Value<PatchFilterGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApproveAfterDays" => {
                                approve_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApproveUntilDate" => {
                                approve_until_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceLevel" => {
                                compliance_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNonSecurity" => {
                                enable_non_security = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatchFilterGroup" => {
                                patch_filter_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        approve_after_days: approve_after_days,
                        approve_until_date: approve_until_date,
                        compliance_level: compliance_level,
                        enable_non_security: enable_non_security,
                        patch_filter_group: patch_filter_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleGroup {
        /// Property [`PatchRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html#cfn-ssm-patchbaseline-rulegroup-patchrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub patch_rules: Option<::ValueList<Rule>>,
    }

    impl ::codec::SerializeValue for RuleGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref patch_rules) = self.patch_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchRules", patch_rules)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut patch_rules: Option<::ValueList<Rule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PatchRules" => {
                                patch_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleGroup {
                        patch_rules: patch_rules,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resource_data_sync {
    //! Property types for the `ResourceDataSync` resource.

    /// The [`AWS::SSM::ResourceDataSync.AwsOrganizationsSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-awsorganizationssource.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsOrganizationsSource {
        /// Property [`OrganizationSourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-awsorganizationssource.html#cfn-ssm-resourcedatasync-awsorganizationssource-organizationsourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organization_source_type: ::Value<String>,
        /// Property [`OrganizationalUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-awsorganizationssource.html#cfn-ssm-resourcedatasync-awsorganizationssource-organizationalunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organizational_units: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AwsOrganizationsSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationSourceType", &self.organization_source_type)?;
            if let Some(ref organizational_units) = self.organizational_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnits", organizational_units)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsOrganizationsSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsOrganizationsSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsOrganizationsSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsOrganizationsSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut organization_source_type: Option<::Value<String>> = None;
                    let mut organizational_units: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OrganizationSourceType" => {
                                organization_source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnits" => {
                                organizational_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsOrganizationsSource {
                        organization_source_type: organization_source_type.ok_or(::serde::de::Error::missing_field("OrganizationSourceType"))?,
                        organizational_units: organizational_units,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::ResourceDataSync.S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Destination {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html#cfn-ssm-resourcedatasync-s3destination-bucketname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html#cfn-ssm-resourcedatasync-s3destination-bucketprefix).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`BucketRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html#cfn-ssm-resourcedatasync-s3destination-bucketregion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket_region: ::Value<String>,
        /// Property [`KMSKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html#cfn-ssm-resourcedatasync-s3destination-kmskeyarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`SyncFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html#cfn-ssm-resourcedatasync-s3destination-syncformat).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sync_format: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketRegion", &self.bucket_region)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncFormat", &self.sync_format)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Destination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Destination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Destination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Destination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut bucket_region: Option<::Value<String>> = None;
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut sync_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketRegion" => {
                                bucket_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KMSKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncFormat" => {
                                sync_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Destination {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                        bucket_region: bucket_region.ok_or(::serde::de::Error::missing_field("BucketRegion"))?,
                        kms_key_arn: kms_key_arn,
                        sync_format: sync_format.ok_or(::serde::de::Error::missing_field("SyncFormat"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::ResourceDataSync.SyncSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html) property type.
    #[derive(Debug, Default)]
    pub struct SyncSource {
        /// Property [`AwsOrganizationsSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html#cfn-ssm-resourcedatasync-syncsource-awsorganizationssource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_organizations_source: Option<::Value<AwsOrganizationsSource>>,
        /// Property [`IncludeFutureRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html#cfn-ssm-resourcedatasync-syncsource-includefutureregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_future_regions: Option<::Value<bool>>,
        /// Property [`SourceRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html#cfn-ssm-resourcedatasync-syncsource-sourceregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_regions: ::ValueList<String>,
        /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html#cfn-ssm-resourcedatasync-syncsource-sourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SyncSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_organizations_source) = self.aws_organizations_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsOrganizationsSource", aws_organizations_source)?;
            }
            if let Some(ref include_future_regions) = self.include_future_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeFutureRegions", include_future_regions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegions", &self.source_regions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", &self.source_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SyncSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SyncSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SyncSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SyncSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_organizations_source: Option<::Value<AwsOrganizationsSource>> = None;
                    let mut include_future_regions: Option<::Value<bool>> = None;
                    let mut source_regions: Option<::ValueList<String>> = None;
                    let mut source_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsOrganizationsSource" => {
                                aws_organizations_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeFutureRegions" => {
                                include_future_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceRegions" => {
                                source_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceType" => {
                                source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SyncSource {
                        aws_organizations_source: aws_organizations_source,
                        include_future_regions: include_future_regions,
                        source_regions: source_regions.ok_or(::serde::de::Error::missing_field("SourceRegions"))?,
                        source_type: source_type.ok_or(::serde::de::Error::missing_field("SourceType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
