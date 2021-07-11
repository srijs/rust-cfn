//! Types for the `AuditManager` service.

/// The [`AWS::AuditManager::Assessment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html) resource type.
#[derive(Debug, Default)]
pub struct Assessment {
    properties: AssessmentProperties
}

/// Properties for the `Assessment` resource.
#[derive(Debug, Default)]
pub struct AssessmentProperties {
    /// Property [`AssessmentReportsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-assessmentreportsdestination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub assessment_reports_destination: Option<::Value<self::assessment::AssessmentReportsDestination>>,
    /// Property [`AwsAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-awsaccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account: Option<::Value<self::assessment::AWSAccount>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FrameworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-frameworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub framework_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-roles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub roles: Option<::ValueList<self::assessment::Role>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-scope).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scope: Option<::Value<self::assessment::Scope>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html#cfn-auditmanager-assessment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssessmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref assessment_reports_destination) = self.assessment_reports_destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentReportsDestination", assessment_reports_destination)?;
        }
        if let Some(ref aws_account) = self.aws_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccount", aws_account)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref framework_id) = self.framework_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkId", framework_id)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref roles) = self.roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", roles)?;
        }
        if let Some(ref scope) = self.scope {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssessmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssessmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssessmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssessmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assessment_reports_destination: Option<::Value<self::assessment::AssessmentReportsDestination>> = None;
                let mut aws_account: Option<::Value<self::assessment::AWSAccount>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut framework_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut roles: Option<::ValueList<self::assessment::Role>> = None;
                let mut scope: Option<::Value<self::assessment::Scope>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssessmentReportsDestination" => {
                            assessment_reports_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AwsAccount" => {
                            aws_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FrameworkId" => {
                            framework_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Roles" => {
                            roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssessmentProperties {
                    assessment_reports_destination: assessment_reports_destination,
                    aws_account: aws_account,
                    description: description,
                    framework_id: framework_id,
                    name: name,
                    roles: roles,
                    scope: scope,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Assessment {
    type Properties = AssessmentProperties;
    const TYPE: &'static str = "AWS::AuditManager::Assessment";
    fn properties(&self) -> &AssessmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssessmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Assessment {}

impl From<AssessmentProperties> for Assessment {
    fn from(properties: AssessmentProperties) -> Assessment {
        Assessment { properties }
    }
}

pub mod assessment {
    //! Property types for the `Assessment` resource.

    /// The [`AWS::AuditManager::Assessment.AWSAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsaccount.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSAccount {
        /// Property [`EmailAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsaccount.html#cfn-auditmanager-assessment-awsaccount-emailaddress).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub email_address: Option<::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsaccount.html#cfn-auditmanager-assessment-awsaccount-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsaccount.html#cfn-auditmanager-assessment-awsaccount-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AWSAccount {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email_address) = self.email_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailAddress", email_address)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSAccount {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSAccount, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSAccount;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSAccount")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email_address: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmailAddress" => {
                                email_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSAccount {
                        email_address: email_address,
                        id: id,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AuditManager::Assessment.AWSService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsservice.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSService {
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsservice.html#cfn-auditmanager-assessment-awsservice-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AWSService {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSService {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSService, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSService;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSService")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSService {
                        service_name: service_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AuditManager::Assessment.AssessmentReportsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-assessmentreportsdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct AssessmentReportsDestination {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-assessmentreportsdestination.html#cfn-auditmanager-assessment-assessmentreportsdestination-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<String>>,
        /// Property [`DestinationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-assessmentreportsdestination.html#cfn-auditmanager-assessment-assessmentreportsdestination-destinationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AssessmentReportsDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref destination_type) = self.destination_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationType", destination_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssessmentReportsDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssessmentReportsDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssessmentReportsDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssessmentReportsDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;
                    let mut destination_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationType" => {
                                destination_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssessmentReportsDestination {
                        destination: destination,
                        destination_type: destination_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AuditManager::Assessment.Delegation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html) property type.
    #[derive(Debug, Default)]
    pub struct Delegation {
        /// Property [`AssessmentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-assessmentid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assessment_id: Option<::Value<String>>,
        /// Property [`AssessmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-assessmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assessment_name: Option<::Value<String>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`ControlSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-controlsetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_set_id: Option<::Value<String>>,
        /// Property [`CreatedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-createdby).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_by: Option<::Value<String>>,
        /// Property [`CreationTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-creationtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_time: Option<::Value<f64>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`LastUpdated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-lastupdated).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated: Option<::Value<f64>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`RoleType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-roletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_type: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html#cfn-auditmanager-assessment-delegation-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Delegation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assessment_id) = self.assessment_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentId", assessment_id)?;
            }
            if let Some(ref assessment_name) = self.assessment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentName", assessment_name)?;
            }
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref control_set_id) = self.control_set_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlSetId", control_set_id)?;
            }
            if let Some(ref created_by) = self.created_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedBy", created_by)?;
            }
            if let Some(ref creation_time) = self.creation_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationTime", creation_time)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref last_updated) = self.last_updated {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdated", last_updated)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref role_type) = self.role_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleType", role_type)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Delegation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Delegation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Delegation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Delegation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assessment_id: Option<::Value<String>> = None;
                    let mut assessment_name: Option<::Value<String>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut control_set_id: Option<::Value<String>> = None;
                    let mut created_by: Option<::Value<String>> = None;
                    let mut creation_time: Option<::Value<f64>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut last_updated: Option<::Value<f64>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut role_type: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssessmentId" => {
                                assessment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssessmentName" => {
                                assessment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ControlSetId" => {
                                control_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedBy" => {
                                created_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreationTime" => {
                                creation_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdated" => {
                                last_updated = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleType" => {
                                role_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Delegation {
                        assessment_id: assessment_id,
                        assessment_name: assessment_name,
                        comment: comment,
                        control_set_id: control_set_id,
                        created_by: created_by,
                        creation_time: creation_time,
                        id: id,
                        last_updated: last_updated,
                        role_arn: role_arn,
                        role_type: role_type,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AuditManager::Assessment.Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-role.html) property type.
    #[derive(Debug, Default)]
    pub struct Role {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-role.html#cfn-auditmanager-assessment-role-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`RoleType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-role.html#cfn-auditmanager-assessment-role-roletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Role {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref role_type) = self.role_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleType", role_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Role {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Role, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Role;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Role")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut role_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleType" => {
                                role_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Role {
                        role_arn: role_arn,
                        role_type: role_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AuditManager::Assessment.Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-scope.html) property type.
    #[derive(Debug, Default)]
    pub struct Scope {
        /// Property [`AwsAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-scope.html#cfn-auditmanager-assessment-scope-awsaccounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_accounts: Option<::ValueList<AWSAccount>>,
        /// Property [`AwsServices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-scope.html#cfn-auditmanager-assessment-scope-awsservices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_services: Option<::ValueList<AWSService>>,
    }

    impl ::codec::SerializeValue for Scope {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_accounts) = self.aws_accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccounts", aws_accounts)?;
            }
            if let Some(ref aws_services) = self.aws_services {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsServices", aws_services)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scope {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scope, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scope;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scope")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_accounts: Option<::ValueList<AWSAccount>> = None;
                    let mut aws_services: Option<::ValueList<AWSService>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccounts" => {
                                aws_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsServices" => {
                                aws_services = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scope {
                        aws_accounts: aws_accounts,
                        aws_services: aws_services,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
