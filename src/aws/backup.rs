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
    pub report_delivery_channel: ::Value<::json::Value>,
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
    pub report_setting: ::Value<::json::Value>,
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
                let mut report_delivery_channel: Option<::Value<::json::Value>> = None;
                let mut report_plan_description: Option<::Value<String>> = None;
                let mut report_plan_name: Option<::Value<String>> = None;
                let mut report_plan_tags: Option<::ValueList<::Tag>> = None;
                let mut report_setting: Option<::Value<::json::Value>> = None;

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

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteAfterDays" => {
                                delete_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MoveToColdStorageAfterDays" => {
                                move_to_cold_storage_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleResourceType {
                        delete_after_days: delete_after_days,
                        move_to_cold_storage_after_days: move_to_cold_storage_after_days,
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
        pub conditions: Option<::Value<::json::Value>>,
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
                    let mut conditions: Option<::Value<::json::Value>> = None;
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
        pub changeable_for_days: Option<::Value<f64>>,
        /// Property [`MaxRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html#cfn-backup-backupvault-lockconfigurationtype-maxretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retention_days: Option<::Value<f64>>,
        /// Property [`MinRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html#cfn-backup-backupvault-lockconfigurationtype-minretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_retention_days: ::Value<f64>,
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
                    let mut changeable_for_days: Option<::Value<f64>> = None;
                    let mut max_retention_days: Option<::Value<f64>> = None;
                    let mut min_retention_days: Option<::Value<f64>> = None;

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
        pub control_scope: Option<::Value<::json::Value>>,
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
                    let mut control_scope: Option<::Value<::json::Value>> = None;

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
