//! Types for the `Backup` service.

/// The [`AWS::Backup::BackupPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupplan.html) resource type.
#[derive(Debug, Default)]
pub struct BackupPlan {
    properties: BackupPlanProperties
}

/// Properties for the `BackupPlan` resource.
#[derive(Debug, Default)]
pub struct BackupPlanProperties {
    /// Property [`BackupPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupplan.html#cfn-backup-backupplan-backupplan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_plan: ::Value<self::backup_plan::BackupPlanResourceType>,
    /// Property [`BackupPlanTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupplan.html#cfn-backup-backupplan-backupplantags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_plan_tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for BackupPlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPlan", &self.backup_plan)?;
        if let Some(ref backup_plan_tags) = self.backup_plan_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPlanTags", backup_plan_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BackupPlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupPlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BackupPlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BackupPlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut backup_plan: Option<::Value<self::backup_plan::BackupPlanResourceType>> = None;
                let mut backup_plan_tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BackupPlan" => {
                            backup_plan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupPlanTags" => {
                            backup_plan_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BackupPlanProperties {
                    backup_plan: backup_plan.ok_or(::serde::de::Error::missing_field("BackupPlan"))?,
                    backup_plan_tags: backup_plan_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BackupPlan {
    type Properties = BackupPlanProperties;
    const TYPE: &'static str = "AWS::Backup::BackupPlan";
    fn properties(&self) -> &BackupPlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BackupPlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BackupPlan {}

impl From<BackupPlanProperties> for BackupPlan {
    fn from(properties: BackupPlanProperties) -> BackupPlan {
        BackupPlan { properties }
    }
}

/// The [`AWS::Backup::BackupSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupselection.html) resource type.
#[derive(Debug, Default)]
pub struct BackupSelection {
    properties: BackupSelectionProperties
}

/// Properties for the `BackupSelection` resource.
#[derive(Debug, Default)]
pub struct BackupSelectionProperties {
    /// Property [`BackupPlanId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupselection.html#cfn-backup-backupselection-backupplanid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_plan_id: ::Value<String>,
    /// Property [`BackupSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupselection.html#cfn-backup-backupselection-backupselection).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_selection: ::Value<self::backup_selection::BackupSelectionResourceType>,
}

impl ::serde::Serialize for BackupSelectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPlanId", &self.backup_plan_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupSelection", &self.backup_selection)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BackupSelectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupSelectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BackupSelectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BackupSelectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut backup_plan_id: Option<::Value<String>> = None;
                let mut backup_selection: Option<::Value<self::backup_selection::BackupSelectionResourceType>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BackupPlanId" => {
                            backup_plan_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupSelection" => {
                            backup_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BackupSelectionProperties {
                    backup_plan_id: backup_plan_id.ok_or(::serde::de::Error::missing_field("BackupPlanId"))?,
                    backup_selection: backup_selection.ok_or(::serde::de::Error::missing_field("BackupSelection"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BackupSelection {
    type Properties = BackupSelectionProperties;
    const TYPE: &'static str = "AWS::Backup::BackupSelection";
    fn properties(&self) -> &BackupSelectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BackupSelectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BackupSelection {}

impl From<BackupSelectionProperties> for BackupSelection {
    fn from(properties: BackupSelectionProperties) -> BackupSelection {
        BackupSelection { properties }
    }
}

/// The [`AWS::Backup::BackupVault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html) resource type.
#[derive(Debug, Default)]
pub struct BackupVault {
    properties: BackupVaultProperties
}

/// Properties for the `BackupVault` resource.
#[derive(Debug, Default)]
pub struct BackupVaultProperties {
    /// Property [`AccessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-accesspolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policy: Option<::Value<::json::Value>>,
    /// Property [`BackupVaultName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-backupvaultname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_vault_name: ::Value<String>,
    /// Property [`BackupVaultTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-backupvaulttags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_vault_tags: Option<::ValueMap<String>>,
    /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-encryptionkeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_key_arn: Option<::Value<String>>,
    /// Property [`LockConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-lockconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lock_configuration: Option<::Value<self::backup_vault::LockConfigurationType>>,
    /// Property [`Notifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html#cfn-backup-backupvault-notifications).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notifications: Option<::Value<self::backup_vault::NotificationObjectType>>,
}

impl ::serde::Serialize for BackupVaultProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_policy) = self.access_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicy", access_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupVaultName", &self.backup_vault_name)?;
        if let Some(ref backup_vault_tags) = self.backup_vault_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupVaultTags", backup_vault_tags)?;
        }
        if let Some(ref encryption_key_arn) = self.encryption_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
        }
        if let Some(ref lock_configuration) = self.lock_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LockConfiguration", lock_configuration)?;
        }
        if let Some(ref notifications) = self.notifications {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Notifications", notifications)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BackupVaultProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupVaultProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BackupVaultProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BackupVaultProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_policy: Option<::Value<::json::Value>> = None;
                let mut backup_vault_name: Option<::Value<String>> = None;
                let mut backup_vault_tags: Option<::ValueMap<String>> = None;
                let mut encryption_key_arn: Option<::Value<String>> = None;
                let mut lock_configuration: Option<::Value<self::backup_vault::LockConfigurationType>> = None;
                let mut notifications: Option<::Value<self::backup_vault::NotificationObjectType>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessPolicy" => {
                            access_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupVaultName" => {
                            backup_vault_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupVaultTags" => {
                            backup_vault_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKeyArn" => {
                            encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LockConfiguration" => {
                            lock_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Notifications" => {
                            notifications = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BackupVaultProperties {
                    access_policy: access_policy,
                    backup_vault_name: backup_vault_name.ok_or(::serde::de::Error::missing_field("BackupVaultName"))?,
                    backup_vault_tags: backup_vault_tags,
                    encryption_key_arn: encryption_key_arn,
                    lock_configuration: lock_configuration,
                    notifications: notifications,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BackupVault {
    type Properties = BackupVaultProperties;
    const TYPE: &'static str = "AWS::Backup::BackupVault";
    fn properties(&self) -> &BackupVaultProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BackupVaultProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BackupVault {}

impl From<BackupVaultProperties> for BackupVault {
    fn from(properties: BackupVaultProperties) -> BackupVault {
        BackupVault { properties }
    }
}

/// The [`AWS::Backup::Framework`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html) resource type.
#[derive(Debug, Default)]
pub struct Framework {
    properties: FrameworkProperties
}

/// Properties for the `Framework` resource.
#[derive(Debug, Default)]
pub struct FrameworkProperties {
    /// Property [`FrameworkControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html#cfn-backup-framework-frameworkcontrols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub framework_controls: ::ValueList<self::framework::FrameworkControl>,
    /// Property [`FrameworkDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html#cfn-backup-framework-frameworkdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub framework_description: Option<::Value<String>>,
    /// Property [`FrameworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html#cfn-backup-framework-frameworkname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub framework_name: Option<::Value<String>>,
    /// Property [`FrameworkTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html#cfn-backup-framework-frameworktags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub framework_tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FrameworkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkControls", &self.framework_controls)?;
        if let Some(ref framework_description) = self.framework_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkDescription", framework_description)?;
        }
        if let Some(ref framework_name) = self.framework_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkName", framework_name)?;
        }
        if let Some(ref framework_tags) = self.framework_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkTags", framework_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FrameworkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameworkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FrameworkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FrameworkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut framework_controls: Option<::ValueList<self::framework::FrameworkControl>> = None;
                let mut framework_description: Option<::Value<String>> = None;
                let mut framework_name: Option<::Value<String>> = None;
                let mut framework_tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FrameworkControls" => {
                            framework_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FrameworkDescription" => {
                            framework_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FrameworkName" => {
                            framework_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FrameworkTags" => {
                            framework_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FrameworkProperties {
                    framework_controls: framework_controls.ok_or(::serde::de::Error::missing_field("FrameworkControls"))?,
                    framework_description: framework_description,
                    framework_name: framework_name,
                    framework_tags: framework_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Framework {
    type Properties = FrameworkProperties;
    const TYPE: &'static str = "AWS::Backup::Framework";
    fn properties(&self) -> &FrameworkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FrameworkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Framework {}

impl From<FrameworkProperties> for Framework {
    fn from(properties: FrameworkProperties) -> Framework {
        Framework { properties }
    }
}

/// The [`AWS::Backup::ReportPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html) resource type.
#[derive(Debug, Default)]
pub struct ReportPlan {
    properties: ReportPlanProperties
}

/// Properties for the `ReportPlan` resource.
#[derive(Debug, Default)]
pub struct ReportPlanProperties {
    /// Property [`ReportDeliveryChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html#cfn-backup-reportplan-reportdeliverychannel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub report_delivery_channel: ::Value<self::report_plan::ReportDeliveryChannel>,
    /// Property [`ReportPlanDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html#cfn-backup-reportplan-reportplandescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub report_plan_description: Option<::Value<String>>,
    /// Property [`ReportPlanName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html#cfn-backup-reportplan-reportplanname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub report_plan_name: Option<::Value<String>>,
    /// Property [`ReportPlanTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html#cfn-backup-reportplan-reportplantags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub report_plan_tags: Option<::ValueList<::Tag>>,
    /// Property [`ReportSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html#cfn-backup-reportplan-reportsetting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub report_setting: ::Value<self::report_plan::ReportSetting>,
}

impl ::serde::Serialize for ReportPlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportDeliveryChannel", &self.report_delivery_channel)?;
        if let Some(ref report_plan_description) = self.report_plan_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportPlanDescription", report_plan_description)?;
        }
        if let Some(ref report_plan_name) = self.report_plan_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportPlanName", report_plan_name)?;
        }
        if let Some(ref report_plan_tags) = self.report_plan_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportPlanTags", report_plan_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportSetting", &self.report_setting)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReportPlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportPlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReportPlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReportPlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut report_delivery_channel: Option<::Value<self::report_plan::ReportDeliveryChannel>> = None;
                let mut report_plan_description: Option<::Value<String>> = None;
                let mut report_plan_name: Option<::Value<String>> = None;
                let mut report_plan_tags: Option<::ValueList<::Tag>> = None;
                let mut report_setting: Option<::Value<self::report_plan::ReportSetting>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ReportDeliveryChannel" => {
                            report_delivery_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportPlanDescription" => {
                            report_plan_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportPlanName" => {
                            report_plan_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportPlanTags" => {
                            report_plan_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReportSetting" => {
                            report_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReportPlanProperties {
                    report_delivery_channel: report_delivery_channel.ok_or(::serde::de::Error::missing_field("ReportDeliveryChannel"))?,
                    report_plan_description: report_plan_description,
                    report_plan_name: report_plan_name,
                    report_plan_tags: report_plan_tags,
                    report_setting: report_setting.ok_or(::serde::de::Error::missing_field("ReportSetting"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReportPlan {
    type Properties = ReportPlanProperties;
    const TYPE: &'static str = "AWS::Backup::ReportPlan";
    fn properties(&self) -> &ReportPlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReportPlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReportPlan {}

impl From<ReportPlanProperties> for ReportPlan {
    fn from(properties: ReportPlanProperties) -> ReportPlan {
        ReportPlan { properties }
    }
}

/// The [`AWS::Backup::RestoreTestingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html) resource type.
#[derive(Debug, Default)]
pub struct RestoreTestingPlan {
    properties: RestoreTestingPlanProperties
}

/// Properties for the `RestoreTestingPlan` resource.
#[derive(Debug, Default)]
pub struct RestoreTestingPlanProperties {
    /// Property [`RecoveryPointSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-recoverypointselection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recovery_point_selection: ::Value<self::restore_testing_plan::RestoreTestingRecoveryPointSelection>,
    /// Property [`RestoreTestingPlanName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-restoretestingplanname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_testing_plan_name: ::Value<String>,
    /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-scheduleexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression: ::Value<String>,
    /// Property [`ScheduleExpressionTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-scheduleexpressiontimezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression_timezone: Option<::Value<String>>,
    /// Property [`StartWindowHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-startwindowhours).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_window_hours: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html#cfn-backup-restoretestingplan-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RestoreTestingPlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryPointSelection", &self.recovery_point_selection)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreTestingPlanName", &self.restore_testing_plan_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
        if let Some(ref schedule_expression_timezone) = self.schedule_expression_timezone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpressionTimezone", schedule_expression_timezone)?;
        }
        if let Some(ref start_window_hours) = self.start_window_hours {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartWindowHours", start_window_hours)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RestoreTestingPlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RestoreTestingPlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RestoreTestingPlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RestoreTestingPlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut recovery_point_selection: Option<::Value<self::restore_testing_plan::RestoreTestingRecoveryPointSelection>> = None;
                let mut restore_testing_plan_name: Option<::Value<String>> = None;
                let mut schedule_expression: Option<::Value<String>> = None;
                let mut schedule_expression_timezone: Option<::Value<String>> = None;
                let mut start_window_hours: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RecoveryPointSelection" => {
                            recovery_point_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreTestingPlanName" => {
                            restore_testing_plan_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpression" => {
                            schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpressionTimezone" => {
                            schedule_expression_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartWindowHours" => {
                            start_window_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RestoreTestingPlanProperties {
                    recovery_point_selection: recovery_point_selection.ok_or(::serde::de::Error::missing_field("RecoveryPointSelection"))?,
                    restore_testing_plan_name: restore_testing_plan_name.ok_or(::serde::de::Error::missing_field("RestoreTestingPlanName"))?,
                    schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                    schedule_expression_timezone: schedule_expression_timezone,
                    start_window_hours: start_window_hours,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RestoreTestingPlan {
    type Properties = RestoreTestingPlanProperties;
    const TYPE: &'static str = "AWS::Backup::RestoreTestingPlan";
    fn properties(&self) -> &RestoreTestingPlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RestoreTestingPlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RestoreTestingPlan {}

impl From<RestoreTestingPlanProperties> for RestoreTestingPlan {
    fn from(properties: RestoreTestingPlanProperties) -> RestoreTestingPlan {
        RestoreTestingPlan { properties }
    }
}

/// The [`AWS::Backup::RestoreTestingSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html) resource type.
#[derive(Debug, Default)]
pub struct RestoreTestingSelection {
    properties: RestoreTestingSelectionProperties
}

/// Properties for the `RestoreTestingSelection` resource.
#[derive(Debug, Default)]
pub struct RestoreTestingSelectionProperties {
    /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-iamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role_arn: ::Value<String>,
    /// Property [`ProtectedResourceArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-protectedresourcearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protected_resource_arns: Option<::ValueList<String>>,
    /// Property [`ProtectedResourceConditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-protectedresourceconditions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protected_resource_conditions: Option<::Value<self::restore_testing_selection::ProtectedResourceConditions>>,
    /// Property [`ProtectedResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-protectedresourcetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protected_resource_type: ::Value<String>,
    /// Property [`RestoreMetadataOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-restoremetadataoverrides).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub restore_metadata_overrides: Option<::ValueMap<String>>,
    /// Property [`RestoreTestingPlanName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-restoretestingplanname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_testing_plan_name: ::Value<String>,
    /// Property [`RestoreTestingSelectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-restoretestingselectionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_testing_selection_name: ::Value<String>,
    /// Property [`ValidationWindowHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html#cfn-backup-restoretestingselection-validationwindowhours).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub validation_window_hours: Option<::Value<u32>>,
}

impl ::serde::Serialize for RestoreTestingSelectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", &self.iam_role_arn)?;
        if let Some(ref protected_resource_arns) = self.protected_resource_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtectedResourceArns", protected_resource_arns)?;
        }
        if let Some(ref protected_resource_conditions) = self.protected_resource_conditions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtectedResourceConditions", protected_resource_conditions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtectedResourceType", &self.protected_resource_type)?;
        if let Some(ref restore_metadata_overrides) = self.restore_metadata_overrides {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreMetadataOverrides", restore_metadata_overrides)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreTestingPlanName", &self.restore_testing_plan_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreTestingSelectionName", &self.restore_testing_selection_name)?;
        if let Some(ref validation_window_hours) = self.validation_window_hours {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationWindowHours", validation_window_hours)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RestoreTestingSelectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RestoreTestingSelectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RestoreTestingSelectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RestoreTestingSelectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut iam_role_arn: Option<::Value<String>> = None;
                let mut protected_resource_arns: Option<::ValueList<String>> = None;
                let mut protected_resource_conditions: Option<::Value<self::restore_testing_selection::ProtectedResourceConditions>> = None;
                let mut protected_resource_type: Option<::Value<String>> = None;
                let mut restore_metadata_overrides: Option<::ValueMap<String>> = None;
                let mut restore_testing_plan_name: Option<::Value<String>> = None;
                let mut restore_testing_selection_name: Option<::Value<String>> = None;
                let mut validation_window_hours: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IamRoleArn" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtectedResourceArns" => {
                            protected_resource_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtectedResourceConditions" => {
                            protected_resource_conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtectedResourceType" => {
                            protected_resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreMetadataOverrides" => {
                            restore_metadata_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreTestingPlanName" => {
                            restore_testing_plan_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreTestingSelectionName" => {
                            restore_testing_selection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationWindowHours" => {
                            validation_window_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RestoreTestingSelectionProperties {
                    iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IamRoleArn"))?,
                    protected_resource_arns: protected_resource_arns,
                    protected_resource_conditions: protected_resource_conditions,
                    protected_resource_type: protected_resource_type.ok_or(::serde::de::Error::missing_field("ProtectedResourceType"))?,
                    restore_metadata_overrides: restore_metadata_overrides,
                    restore_testing_plan_name: restore_testing_plan_name.ok_or(::serde::de::Error::missing_field("RestoreTestingPlanName"))?,
                    restore_testing_selection_name: restore_testing_selection_name.ok_or(::serde::de::Error::missing_field("RestoreTestingSelectionName"))?,
                    validation_window_hours: validation_window_hours,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RestoreTestingSelection {
    type Properties = RestoreTestingSelectionProperties;
    const TYPE: &'static str = "AWS::Backup::RestoreTestingSelection";
    fn properties(&self) -> &RestoreTestingSelectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RestoreTestingSelectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RestoreTestingSelection {}

impl From<RestoreTestingSelectionProperties> for RestoreTestingSelection {
    fn from(properties: RestoreTestingSelectionProperties) -> RestoreTestingSelection {
        RestoreTestingSelection { properties }
    }
}

pub mod backup_plan {
    //! Property types for the `BackupPlan` resource.

    /// The [`AWS::Backup::BackupPlan.AdvancedBackupSettingResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-advancedbackupsettingresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedBackupSettingResourceType {
        /// Property [`BackupOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-advancedbackupsettingresourcetype.html#cfn-backup-backupplan-advancedbackupsettingresourcetype-backupoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backup_options: ::Value<::json::Value>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-advancedbackupsettingresourcetype.html#cfn-backup-backupplan-advancedbackupsettingresourcetype-resourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AdvancedBackupSettingResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupOptions", &self.backup_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedBackupSettingResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedBackupSettingResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedBackupSettingResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedBackupSettingResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut backup_options: Option<::Value<::json::Value>> = None;
                    let mut resource_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackupOptions" => {
                                backup_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedBackupSettingResourceType {
                        backup_options: backup_options.ok_or(::serde::de::Error::missing_field("BackupOptions"))?,
                        resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupPlan.BackupPlanResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupplanresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct BackupPlanResourceType {
        /// Property [`AdvancedBackupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupplanresourcetype.html#cfn-backup-backupplan-backupplanresourcetype-advancedbackupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub advanced_backup_settings: Option<::ValueList<AdvancedBackupSettingResourceType>>,
        /// Property [`BackupPlanName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupplanresourcetype.html#cfn-backup-backupplan-backupplanresourcetype-backupplanname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backup_plan_name: ::Value<String>,
        /// Property [`BackupPlanRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupplanresourcetype.html#cfn-backup-backupplan-backupplanresourcetype-backupplanrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backup_plan_rule: ::ValueList<BackupRuleResourceType>,
    }

    impl ::codec::SerializeValue for BackupPlanResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref advanced_backup_settings) = self.advanced_backup_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedBackupSettings", advanced_backup_settings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPlanName", &self.backup_plan_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPlanRule", &self.backup_plan_rule)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BackupPlanResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupPlanResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BackupPlanResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BackupPlanResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut advanced_backup_settings: Option<::ValueList<AdvancedBackupSettingResourceType>> = None;
                    let mut backup_plan_name: Option<::Value<String>> = None;
                    let mut backup_plan_rule: Option<::ValueList<BackupRuleResourceType>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdvancedBackupSettings" => {
                                advanced_backup_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackupPlanName" => {
                                backup_plan_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackupPlanRule" => {
                                backup_plan_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BackupPlanResourceType {
                        advanced_backup_settings: advanced_backup_settings,
                        backup_plan_name: backup_plan_name.ok_or(::serde::de::Error::missing_field("BackupPlanName"))?,
                        backup_plan_rule: backup_plan_rule.ok_or(::serde::de::Error::missing_field("BackupPlanRule"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupPlan.BackupRuleResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct BackupRuleResourceType {
        /// Property [`CompletionWindowMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-completionwindowminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub completion_window_minutes: Option<::Value<f64>>,
        /// Property [`CopyActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-copyactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_actions: Option<::ValueList<CopyActionResourceType>>,
        /// Property [`EnableContinuousBackup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-enablecontinuousbackup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_continuous_backup: Option<::Value<bool>>,
        /// Property [`Lifecycle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-lifecycle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle: Option<::Value<LifecycleResourceType>>,
        /// Property [`RecoveryPointTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-recoverypointtags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recovery_point_tags: Option<::ValueMap<String>>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: ::Value<String>,
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: Option<::Value<String>>,
        /// Property [`ScheduleExpressionTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-scheduleexpressiontimezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression_timezone: Option<::Value<String>>,
        /// Property [`StartWindowMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-startwindowminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_window_minutes: Option<::Value<f64>>,
        /// Property [`TargetBackupVault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html#cfn-backup-backupplan-backupruleresourcetype-targetbackupvault).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_backup_vault: ::Value<String>,
    }

    impl ::codec::SerializeValue for BackupRuleResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref completion_window_minutes) = self.completion_window_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompletionWindowMinutes", completion_window_minutes)?;
            }
            if let Some(ref copy_actions) = self.copy_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyActions", copy_actions)?;
            }
            if let Some(ref enable_continuous_backup) = self.enable_continuous_backup {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableContinuousBackup", enable_continuous_backup)?;
            }
            if let Some(ref lifecycle) = self.lifecycle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lifecycle", lifecycle)?;
            }
            if let Some(ref recovery_point_tags) = self.recovery_point_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryPointTags", recovery_point_tags)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", &self.rule_name)?;
            if let Some(ref schedule_expression) = self.schedule_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
            }
            if let Some(ref schedule_expression_timezone) = self.schedule_expression_timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpressionTimezone", schedule_expression_timezone)?;
            }
            if let Some(ref start_window_minutes) = self.start_window_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartWindowMinutes", start_window_minutes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetBackupVault", &self.target_backup_vault)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BackupRuleResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupRuleResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BackupRuleResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BackupRuleResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut completion_window_minutes: Option<::Value<f64>> = None;
                    let mut copy_actions: Option<::ValueList<CopyActionResourceType>> = None;
                    let mut enable_continuous_backup: Option<::Value<bool>> = None;
                    let mut lifecycle: Option<::Value<LifecycleResourceType>> = None;
                    let mut recovery_point_tags: Option<::ValueMap<String>> = None;
                    let mut rule_name: Option<::Value<String>> = None;
                    let mut schedule_expression: Option<::Value<String>> = None;
                    let mut schedule_expression_timezone: Option<::Value<String>> = None;
                    let mut start_window_minutes: Option<::Value<f64>> = None;
                    let mut target_backup_vault: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CompletionWindowMinutes" => {
                                completion_window_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyActions" => {
                                copy_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableContinuousBackup" => {
                                enable_continuous_backup = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lifecycle" => {
                                lifecycle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecoveryPointTags" => {
                                recovery_point_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpressionTimezone" => {
                                schedule_expression_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartWindowMinutes" => {
                                start_window_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetBackupVault" => {
                                target_backup_vault = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BackupRuleResourceType {
                        completion_window_minutes: completion_window_minutes,
                        copy_actions: copy_actions,
                        enable_continuous_backup: enable_continuous_backup,
                        lifecycle: lifecycle,
                        recovery_point_tags: recovery_point_tags,
                        rule_name: rule_name.ok_or(::serde::de::Error::missing_field("RuleName"))?,
                        schedule_expression: schedule_expression,
                        schedule_expression_timezone: schedule_expression_timezone,
                        start_window_minutes: start_window_minutes,
                        target_backup_vault: target_backup_vault.ok_or(::serde::de::Error::missing_field("TargetBackupVault"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupPlan.CopyActionResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-copyactionresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct CopyActionResourceType {
        /// Property [`DestinationBackupVaultArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-copyactionresourcetype.html#cfn-backup-backupplan-copyactionresourcetype-destinationbackupvaultarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_backup_vault_arn: ::Value<String>,
        /// Property [`Lifecycle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-copyactionresourcetype.html#cfn-backup-backupplan-copyactionresourcetype-lifecycle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle: Option<::Value<LifecycleResourceType>>,
    }

    impl ::codec::SerializeValue for CopyActionResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationBackupVaultArn", &self.destination_backup_vault_arn)?;
            if let Some(ref lifecycle) = self.lifecycle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lifecycle", lifecycle)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CopyActionResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CopyActionResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CopyActionResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CopyActionResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_backup_vault_arn: Option<::Value<String>> = None;
                    let mut lifecycle: Option<::Value<LifecycleResourceType>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationBackupVaultArn" => {
                                destination_backup_vault_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lifecycle" => {
                                lifecycle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CopyActionResourceType {
                        destination_backup_vault_arn: destination_backup_vault_arn.ok_or(::serde::de::Error::missing_field("DestinationBackupVaultArn"))?,
                        lifecycle: lifecycle,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupPlan.LifecycleResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-lifecycleresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct LifecycleResourceType {
        /// Property [`DeleteAfterDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-lifecycleresourcetype.html#cfn-backup-backupplan-lifecycleresourcetype-deleteafterdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delete_after_days: Option<::Value<f64>>,
        /// Property [`MoveToColdStorageAfterDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-lifecycleresourcetype.html#cfn-backup-backupplan-lifecycleresourcetype-movetocoldstorageafterdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub move_to_cold_storage_after_days: Option<::Value<f64>>,
        /// Property [`OptInToArchiveForSupportedResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-lifecycleresourcetype.html#cfn-backup-backupplan-lifecycleresourcetype-optintoarchiveforsupportedresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub opt_in_to_archive_for_supported_resources: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LifecycleResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_after_days) = self.delete_after_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteAfterDays", delete_after_days)?;
            }
            if let Some(ref move_to_cold_storage_after_days) = self.move_to_cold_storage_after_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MoveToColdStorageAfterDays", move_to_cold_storage_after_days)?;
            }
            if let Some(ref opt_in_to_archive_for_supported_resources) = self.opt_in_to_archive_for_supported_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptInToArchiveForSupportedResources", opt_in_to_archive_for_supported_resources)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecycleResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecycleResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecycleResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_after_days: Option<::Value<f64>> = None;
                    let mut move_to_cold_storage_after_days: Option<::Value<f64>> = None;
                    let mut opt_in_to_archive_for_supported_resources: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteAfterDays" => {
                                delete_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MoveToColdStorageAfterDays" => {
                                move_to_cold_storage_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptInToArchiveForSupportedResources" => {
                                opt_in_to_archive_for_supported_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleResourceType {
                        delete_after_days: delete_after_days,
                        move_to_cold_storage_after_days: move_to_cold_storage_after_days,
                        opt_in_to_archive_for_supported_resources: opt_in_to_archive_for_supported_resources,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod backup_selection {
    //! Property types for the `BackupSelection` resource.

    /// The [`AWS::Backup::BackupSelection.BackupSelectionResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct BackupSelectionResourceType {
        /// Property [`Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-conditions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub conditions: Option<::Value<Conditions>>,
        /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-iamrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iam_role_arn: ::Value<String>,
        /// Property [`ListOfTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-listoftags).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub list_of_tags: Option<::ValueList<ConditionResourceType>>,
        /// Property [`NotResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-notresources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub not_resources: Option<::ValueList<String>>,
        /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-resources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resources: Option<::ValueList<String>>,
        /// Property [`SelectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html#cfn-backup-backupselection-backupselectionresourcetype-selectionname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub selection_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for BackupSelectionResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref conditions) = self.conditions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditions", conditions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", &self.iam_role_arn)?;
            if let Some(ref list_of_tags) = self.list_of_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListOfTags", list_of_tags)?;
            }
            if let Some(ref not_resources) = self.not_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotResources", not_resources)?;
            }
            if let Some(ref resources) = self.resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", resources)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectionName", &self.selection_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BackupSelectionResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupSelectionResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BackupSelectionResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BackupSelectionResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conditions: Option<::Value<Conditions>> = None;
                    let mut iam_role_arn: Option<::Value<String>> = None;
                    let mut list_of_tags: Option<::ValueList<ConditionResourceType>> = None;
                    let mut not_resources: Option<::ValueList<String>> = None;
                    let mut resources: Option<::ValueList<String>> = None;
                    let mut selection_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Conditions" => {
                                conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamRoleArn" => {
                                iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ListOfTags" => {
                                list_of_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotResources" => {
                                not_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Resources" => {
                                resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectionName" => {
                                selection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BackupSelectionResourceType {
                        conditions: conditions,
                        iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IamRoleArn"))?,
                        list_of_tags: list_of_tags,
                        not_resources: not_resources,
                        resources: resources,
                        selection_name: selection_name.ok_or(::serde::de::Error::missing_field("SelectionName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupSelection.ConditionParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionParameter {
        /// Property [`ConditionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionparameter.html#cfn-backup-backupselection-conditionparameter-conditionkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_key: Option<::Value<String>>,
        /// Property [`ConditionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionparameter.html#cfn-backup-backupselection-conditionparameter-conditionvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConditionParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition_key) = self.condition_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionKey", condition_key)?;
            }
            if let Some(ref condition_value) = self.condition_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionValue", condition_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_key: Option<::Value<String>> = None;
                    let mut condition_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionKey" => {
                                condition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConditionValue" => {
                                condition_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionParameter {
                        condition_key: condition_key,
                        condition_value: condition_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupSelection.ConditionResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionresourcetype.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionResourceType {
        /// Property [`ConditionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionresourcetype.html#cfn-backup-backupselection-conditionresourcetype-conditionkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_key: ::Value<String>,
        /// Property [`ConditionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionresourcetype.html#cfn-backup-backupselection-conditionresourcetype-conditiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_type: ::Value<String>,
        /// Property [`ConditionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionresourcetype.html#cfn-backup-backupselection-conditionresourcetype-conditionvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConditionResourceType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionKey", &self.condition_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionType", &self.condition_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionValue", &self.condition_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionResourceType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionResourceType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionResourceType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionResourceType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_key: Option<::Value<String>> = None;
                    let mut condition_type: Option<::Value<String>> = None;
                    let mut condition_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionKey" => {
                                condition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConditionType" => {
                                condition_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConditionValue" => {
                                condition_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionResourceType {
                        condition_key: condition_key.ok_or(::serde::de::Error::missing_field("ConditionKey"))?,
                        condition_type: condition_type.ok_or(::serde::de::Error::missing_field("ConditionType"))?,
                        condition_value: condition_value.ok_or(::serde::de::Error::missing_field("ConditionValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupSelection.Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html) property type.
    #[derive(Debug, Default)]
    pub struct Conditions {
        /// Property [`StringEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html#cfn-backup-backupselection-conditions-stringequals).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub string_equals: Option<::ValueList<ConditionParameter>>,
        /// Property [`StringLike`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html#cfn-backup-backupselection-conditions-stringlike).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub string_like: Option<::ValueList<ConditionParameter>>,
        /// Property [`StringNotEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html#cfn-backup-backupselection-conditions-stringnotequals).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub string_not_equals: Option<::ValueList<ConditionParameter>>,
        /// Property [`StringNotLike`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html#cfn-backup-backupselection-conditions-stringnotlike).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub string_not_like: Option<::ValueList<ConditionParameter>>,
    }

    impl ::codec::SerializeValue for Conditions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref string_equals) = self.string_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringEquals", string_equals)?;
            }
            if let Some(ref string_like) = self.string_like {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringLike", string_like)?;
            }
            if let Some(ref string_not_equals) = self.string_not_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringNotEquals", string_not_equals)?;
            }
            if let Some(ref string_not_like) = self.string_not_like {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringNotLike", string_not_like)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Conditions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Conditions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Conditions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Conditions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut string_equals: Option<::ValueList<ConditionParameter>> = None;
                    let mut string_like: Option<::ValueList<ConditionParameter>> = None;
                    let mut string_not_equals: Option<::ValueList<ConditionParameter>> = None;
                    let mut string_not_like: Option<::ValueList<ConditionParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StringEquals" => {
                                string_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringLike" => {
                                string_like = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringNotEquals" => {
                                string_not_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringNotLike" => {
                                string_not_like = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Conditions {
                        string_equals: string_equals,
                        string_like: string_like,
                        string_not_equals: string_not_equals,
                        string_not_like: string_not_like,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod backup_vault {
    //! Property types for the `BackupVault` resource.

    /// The [`AWS::Backup::BackupVault.LockConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct LockConfigurationType {
        /// Property [`ChangeableForDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html#cfn-backup-backupvault-lockconfigurationtype-changeablefordays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub changeable_for_days: Option<::Value<u32>>,
        /// Property [`MaxRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html#cfn-backup-backupvault-lockconfigurationtype-maxretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retention_days: Option<::Value<u32>>,
        /// Property [`MinRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html#cfn-backup-backupvault-lockconfigurationtype-minretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_retention_days: ::Value<u32>,
    }

    impl ::codec::SerializeValue for LockConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref changeable_for_days) = self.changeable_for_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChangeableForDays", changeable_for_days)?;
            }
            if let Some(ref max_retention_days) = self.max_retention_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetentionDays", max_retention_days)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinRetentionDays", &self.min_retention_days)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LockConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LockConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LockConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LockConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut changeable_for_days: Option<::Value<u32>> = None;
                    let mut max_retention_days: Option<::Value<u32>> = None;
                    let mut min_retention_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChangeableForDays" => {
                                changeable_for_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRetentionDays" => {
                                max_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinRetentionDays" => {
                                min_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LockConfigurationType {
                        changeable_for_days: changeable_for_days,
                        max_retention_days: max_retention_days,
                        min_retention_days: min_retention_days.ok_or(::serde::de::Error::missing_field("MinRetentionDays"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::BackupVault.NotificationObjectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-notificationobjecttype.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationObjectType {
        /// Property [`BackupVaultEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-notificationobjecttype.html#cfn-backup-backupvault-notificationobjecttype-backupvaultevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub backup_vault_events: ::ValueList<String>,
        /// Property [`SNSTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-notificationobjecttype.html#cfn-backup-backupvault-notificationobjecttype-snstopicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for NotificationObjectType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupVaultEvents", &self.backup_vault_events)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SNSTopicArn", &self.sns_topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationObjectType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationObjectType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationObjectType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationObjectType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut backup_vault_events: Option<::ValueList<String>> = None;
                    let mut sns_topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackupVaultEvents" => {
                                backup_vault_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SNSTopicArn" => {
                                sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationObjectType {
                        backup_vault_events: backup_vault_events.ok_or(::serde::de::Error::missing_field("BackupVaultEvents"))?,
                        sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SNSTopicArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod framework {
    //! Property types for the `Framework` resource.

    /// The [`AWS::Backup::Framework.ControlInputParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlinputparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ControlInputParameter {
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlinputparameter.html#cfn-backup-framework-controlinputparameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlinputparameter.html#cfn-backup-framework-controlinputparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ControlInputParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ControlInputParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ControlInputParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ControlInputParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ControlInputParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ControlInputParameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::Framework.ControlScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlscope.html) property type.
    #[derive(Debug, Default)]
    pub struct ControlScope {
        /// Property [`ComplianceResourceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlscope.html#cfn-backup-framework-controlscope-complianceresourceids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_resource_ids: Option<::ValueList<String>>,
        /// Property [`ComplianceResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlscope.html#cfn-backup-framework-controlscope-complianceresourcetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_resource_types: Option<::ValueList<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlscope.html#cfn-backup-framework-controlscope-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ControlScope {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compliance_resource_ids) = self.compliance_resource_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceIds", compliance_resource_ids)?;
            }
            if let Some(ref compliance_resource_types) = self.compliance_resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceTypes", compliance_resource_types)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ControlScope {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ControlScope, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ControlScope;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ControlScope")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compliance_resource_ids: Option<::ValueList<String>> = None;
                    let mut compliance_resource_types: Option<::ValueList<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComplianceResourceIds" => {
                                compliance_resource_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceResourceTypes" => {
                                compliance_resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ControlScope {
                        compliance_resource_ids: compliance_resource_ids,
                        compliance_resource_types: compliance_resource_types,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::Framework.FrameworkControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-frameworkcontrol.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameworkControl {
        /// Property [`ControlInputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-frameworkcontrol.html#cfn-backup-framework-frameworkcontrol-controlinputparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_input_parameters: Option<::ValueList<ControlInputParameter>>,
        /// Property [`ControlName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-frameworkcontrol.html#cfn-backup-framework-frameworkcontrol-controlname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_name: ::Value<String>,
        /// Property [`ControlScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-frameworkcontrol.html#cfn-backup-framework-frameworkcontrol-controlscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_scope: Option<::Value<ControlScope>>,
    }

    impl ::codec::SerializeValue for FrameworkControl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref control_input_parameters) = self.control_input_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlInputParameters", control_input_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlName", &self.control_name)?;
            if let Some(ref control_scope) = self.control_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlScope", control_scope)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameworkControl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameworkControl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameworkControl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameworkControl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut control_input_parameters: Option<::ValueList<ControlInputParameter>> = None;
                    let mut control_name: Option<::Value<String>> = None;
                    let mut control_scope: Option<::Value<ControlScope>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ControlInputParameters" => {
                                control_input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ControlName" => {
                                control_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ControlScope" => {
                                control_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameworkControl {
                        control_input_parameters: control_input_parameters,
                        control_name: control_name.ok_or(::serde::de::Error::missing_field("ControlName"))?,
                        control_scope: control_scope,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod report_plan {
    //! Property types for the `ReportPlan` resource.

    /// The [`AWS::Backup::ReportPlan.ReportDeliveryChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportdeliverychannel.html) property type.
    #[derive(Debug, Default)]
    pub struct ReportDeliveryChannel {
        /// Property [`Formats`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportdeliverychannel.html#cfn-backup-reportplan-reportdeliverychannel-formats).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub formats: Option<::ValueList<String>>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportdeliverychannel.html#cfn-backup-reportplan-reportdeliverychannel-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: ::Value<String>,
        /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportdeliverychannel.html#cfn-backup-reportplan-reportdeliverychannel-s3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReportDeliveryChannel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref formats) = self.formats {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Formats", formats)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
            if let Some(ref s3_key_prefix) = self.s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReportDeliveryChannel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportDeliveryChannel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReportDeliveryChannel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReportDeliveryChannel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut formats: Option<::ValueList<String>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Formats" => {
                                formats = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReportDeliveryChannel {
                        formats: formats,
                        s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                        s3_key_prefix: s3_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::ReportPlan.ReportSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ReportSetting {
        /// Property [`Accounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html#cfn-backup-reportplan-reportsetting-accounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accounts: Option<::ValueList<String>>,
        /// Property [`FrameworkArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html#cfn-backup-reportplan-reportsetting-frameworkarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework_arns: Option<::ValueList<String>>,
        /// Property [`OrganizationUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html#cfn-backup-reportplan-reportsetting-organizationunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organization_units: Option<::ValueList<String>>,
        /// Property [`Regions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html#cfn-backup-reportplan-reportsetting-regions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regions: Option<::ValueList<String>>,
        /// Property [`ReportTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html#cfn-backup-reportplan-reportsetting-reporttemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_template: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReportSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accounts) = self.accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Accounts", accounts)?;
            }
            if let Some(ref framework_arns) = self.framework_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkArns", framework_arns)?;
            }
            if let Some(ref organization_units) = self.organization_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationUnits", organization_units)?;
            }
            if let Some(ref regions) = self.regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regions", regions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportTemplate", &self.report_template)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReportSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReportSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReportSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReportSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accounts: Option<::ValueList<String>> = None;
                    let mut framework_arns: Option<::ValueList<String>> = None;
                    let mut organization_units: Option<::ValueList<String>> = None;
                    let mut regions: Option<::ValueList<String>> = None;
                    let mut report_template: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Accounts" => {
                                accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameworkArns" => {
                                framework_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationUnits" => {
                                organization_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regions" => {
                                regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportTemplate" => {
                                report_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReportSetting {
                        accounts: accounts,
                        framework_arns: framework_arns,
                        organization_units: organization_units,
                        regions: regions,
                        report_template: report_template.ok_or(::serde::de::Error::missing_field("ReportTemplate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod restore_testing_plan {
    //! Property types for the `RestoreTestingPlan` resource.

    /// The [`AWS::Backup::RestoreTestingPlan.RestoreTestingRecoveryPointSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html) property type.
    #[derive(Debug, Default)]
    pub struct RestoreTestingRecoveryPointSelection {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html#cfn-backup-restoretestingplan-restoretestingrecoverypointselection-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`ExcludeVaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html#cfn-backup-restoretestingplan-restoretestingrecoverypointselection-excludevaults).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_vaults: Option<::ValueList<String>>,
        /// Property [`IncludeVaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html#cfn-backup-restoretestingplan-restoretestingrecoverypointselection-includevaults).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_vaults: ::ValueList<String>,
        /// Property [`RecoveryPointTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html#cfn-backup-restoretestingplan-restoretestingrecoverypointselection-recoverypointtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recovery_point_types: ::ValueList<String>,
        /// Property [`SelectionWindowDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html#cfn-backup-restoretestingplan-restoretestingrecoverypointselection-selectionwindowdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selection_window_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RestoreTestingRecoveryPointSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref exclude_vaults) = self.exclude_vaults {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeVaults", exclude_vaults)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeVaults", &self.include_vaults)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryPointTypes", &self.recovery_point_types)?;
            if let Some(ref selection_window_days) = self.selection_window_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectionWindowDays", selection_window_days)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RestoreTestingRecoveryPointSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RestoreTestingRecoveryPointSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RestoreTestingRecoveryPointSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RestoreTestingRecoveryPointSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut exclude_vaults: Option<::ValueList<String>> = None;
                    let mut include_vaults: Option<::ValueList<String>> = None;
                    let mut recovery_point_types: Option<::ValueList<String>> = None;
                    let mut selection_window_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeVaults" => {
                                exclude_vaults = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeVaults" => {
                                include_vaults = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecoveryPointTypes" => {
                                recovery_point_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectionWindowDays" => {
                                selection_window_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RestoreTestingRecoveryPointSelection {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        exclude_vaults: exclude_vaults,
                        include_vaults: include_vaults.ok_or(::serde::de::Error::missing_field("IncludeVaults"))?,
                        recovery_point_types: recovery_point_types.ok_or(::serde::de::Error::missing_field("RecoveryPointTypes"))?,
                        selection_window_days: selection_window_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod restore_testing_selection {
    //! Property types for the `RestoreTestingSelection` resource.

    /// The [`AWS::Backup::RestoreTestingSelection.KeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-keyvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyValue {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-keyvalue.html#cfn-backup-restoretestingselection-keyvalue-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-keyvalue.html#cfn-backup-restoretestingselection-keyvalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for KeyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

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

                    Ok(KeyValue {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Backup::RestoreTestingSelection.ProtectedResourceConditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-protectedresourceconditions.html) property type.
    #[derive(Debug, Default)]
    pub struct ProtectedResourceConditions {
        /// Property [`StringEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-protectedresourceconditions.html#cfn-backup-restoretestingselection-protectedresourceconditions-stringequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_equals: Option<::ValueList<KeyValue>>,
        /// Property [`StringNotEquals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-protectedresourceconditions.html#cfn-backup-restoretestingselection-protectedresourceconditions-stringnotequals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_not_equals: Option<::ValueList<KeyValue>>,
    }

    impl ::codec::SerializeValue for ProtectedResourceConditions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref string_equals) = self.string_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringEquals", string_equals)?;
            }
            if let Some(ref string_not_equals) = self.string_not_equals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringNotEquals", string_not_equals)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProtectedResourceConditions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtectedResourceConditions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProtectedResourceConditions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProtectedResourceConditions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut string_equals: Option<::ValueList<KeyValue>> = None;
                    let mut string_not_equals: Option<::ValueList<KeyValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StringEquals" => {
                                string_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringNotEquals" => {
                                string_not_equals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProtectedResourceConditions {
                        string_equals: string_equals,
                        string_not_equals: string_not_equals,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
