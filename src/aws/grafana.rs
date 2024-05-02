//! Types for the `Grafana` service.

/// The [`AWS::Grafana::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html) resource type.
#[derive(Debug, Default)]
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Debug, Default)]
pub struct WorkspaceProperties {
    /// Property [`AccountAccessType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-accountaccesstype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_access_type: ::Value<String>,
    /// Property [`AuthenticationProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-authenticationproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authentication_providers: ::ValueList<String>,
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-clienttoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`DataSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-datasources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_sources: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GrafanaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-grafanaversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub grafana_version: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NetworkAccessControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-networkaccesscontrol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_access_control: Option<::Value<self::workspace::NetworkAccessControl>>,
    /// Property [`NotificationDestinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-notificationdestinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_destinations: Option<::ValueList<String>>,
    /// Property [`OrganizationRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-organizationrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organization_role_name: Option<::Value<String>>,
    /// Property [`OrganizationalUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-organizationalunits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organizational_units: Option<::ValueList<String>>,
    /// Property [`PermissionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-permissiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permission_type: ::Value<String>,
    /// Property [`PluginAdminEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-pluginadminenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub plugin_admin_enabled: Option<::Value<bool>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`SamlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-samlconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub saml_configuration: Option<::Value<self::workspace::SamlConfiguration>>,
    /// Property [`StackSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-stacksetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stack_set_name: Option<::Value<String>>,
    /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html#cfn-grafana-workspace-vpcconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_configuration: Option<::Value<self::workspace::VpcConfiguration>>,
}

impl ::serde::Serialize for WorkspaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountAccessType", &self.account_access_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationProviders", &self.authentication_providers)?;
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        if let Some(ref data_sources) = self.data_sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSources", data_sources)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref grafana_version) = self.grafana_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrafanaVersion", grafana_version)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref network_access_control) = self.network_access_control {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkAccessControl", network_access_control)?;
        }
        if let Some(ref notification_destinations) = self.notification_destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationDestinations", notification_destinations)?;
        }
        if let Some(ref organization_role_name) = self.organization_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationRoleName", organization_role_name)?;
        }
        if let Some(ref organizational_units) = self.organizational_units {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnits", organizational_units)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionType", &self.permission_type)?;
        if let Some(ref plugin_admin_enabled) = self.plugin_admin_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PluginAdminEnabled", plugin_admin_enabled)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref saml_configuration) = self.saml_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamlConfiguration", saml_configuration)?;
        }
        if let Some(ref stack_set_name) = self.stack_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetName", stack_set_name)?;
        }
        if let Some(ref vpc_configuration) = self.vpc_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkspaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkspaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkspaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkspaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_access_type: Option<::Value<String>> = None;
                let mut authentication_providers: Option<::ValueList<String>> = None;
                let mut client_token: Option<::Value<String>> = None;
                let mut data_sources: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut grafana_version: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_access_control: Option<::Value<self::workspace::NetworkAccessControl>> = None;
                let mut notification_destinations: Option<::ValueList<String>> = None;
                let mut organization_role_name: Option<::Value<String>> = None;
                let mut organizational_units: Option<::ValueList<String>> = None;
                let mut permission_type: Option<::Value<String>> = None;
                let mut plugin_admin_enabled: Option<::Value<bool>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut saml_configuration: Option<::Value<self::workspace::SamlConfiguration>> = None;
                let mut stack_set_name: Option<::Value<String>> = None;
                let mut vpc_configuration: Option<::Value<self::workspace::VpcConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountAccessType" => {
                            account_access_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthenticationProviders" => {
                            authentication_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSources" => {
                            data_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GrafanaVersion" => {
                            grafana_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkAccessControl" => {
                            network_access_control = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationDestinations" => {
                            notification_destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationRoleName" => {
                            organization_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationalUnits" => {
                            organizational_units = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionType" => {
                            permission_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PluginAdminEnabled" => {
                            plugin_admin_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamlConfiguration" => {
                            saml_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackSetName" => {
                            stack_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfiguration" => {
                            vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkspaceProperties {
                    account_access_type: account_access_type.ok_or(::serde::de::Error::missing_field("AccountAccessType"))?,
                    authentication_providers: authentication_providers.ok_or(::serde::de::Error::missing_field("AuthenticationProviders"))?,
                    client_token: client_token,
                    data_sources: data_sources,
                    description: description,
                    grafana_version: grafana_version,
                    name: name,
                    network_access_control: network_access_control,
                    notification_destinations: notification_destinations,
                    organization_role_name: organization_role_name,
                    organizational_units: organizational_units,
                    permission_type: permission_type.ok_or(::serde::de::Error::missing_field("PermissionType"))?,
                    plugin_admin_enabled: plugin_admin_enabled,
                    role_arn: role_arn,
                    saml_configuration: saml_configuration,
                    stack_set_name: stack_set_name,
                    vpc_configuration: vpc_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workspace {
    type Properties = WorkspaceProperties;
    const TYPE: &'static str = "AWS::Grafana::Workspace";
    fn properties(&self) -> &WorkspaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkspaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workspace {}

impl From<WorkspaceProperties> for Workspace {
    fn from(properties: WorkspaceProperties) -> Workspace {
        Workspace { properties }
    }
}

pub mod workspace {
    //! Property types for the `Workspace` resource.

    /// The [`AWS::Grafana::Workspace.AssertionAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct AssertionAttributes {
        /// Property [`Email`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-email).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email: Option<::Value<String>>,
        /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-groups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub groups: Option<::Value<String>>,
        /// Property [`Login`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-login).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub login: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Org`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-org).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub org: Option<::Value<String>>,
        /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html#cfn-grafana-workspace-assertionattributes-role).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AssertionAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email) = self.email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", email)?;
            }
            if let Some(ref groups) = self.groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
            }
            if let Some(ref login) = self.login {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Login", login)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref org) = self.org {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Org", org)?;
            }
            if let Some(ref role) = self.role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", role)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssertionAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssertionAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssertionAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssertionAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email: Option<::Value<String>> = None;
                    let mut groups: Option<::Value<String>> = None;
                    let mut login: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut org: Option<::Value<String>> = None;
                    let mut role: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Email" => {
                                email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Groups" => {
                                groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Login" => {
                                login = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Org" => {
                                org = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Role" => {
                                role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssertionAttributes {
                        email: email,
                        groups: groups,
                        login: login,
                        name: name,
                        org: org,
                        role: role,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Grafana::Workspace.IdpMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-idpmetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct IdpMetadata {
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-idpmetadata.html#cfn-grafana-workspace-idpmetadata-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`Xml`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-idpmetadata.html#cfn-grafana-workspace-idpmetadata-xml).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub xml: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IdpMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            if let Some(ref xml) = self.xml {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Xml", xml)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdpMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdpMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdpMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdpMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut url: Option<::Value<String>> = None;
                    let mut xml: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Xml" => {
                                xml = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdpMetadata {
                        url: url,
                        xml: xml,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Grafana::Workspace.NetworkAccessControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-networkaccesscontrol.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkAccessControl {
        /// Property [`PrefixListIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-networkaccesscontrol.html#cfn-grafana-workspace-networkaccesscontrol-prefixlistids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix_list_ids: Option<::ValueList<String>>,
        /// Property [`VpceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-networkaccesscontrol.html#cfn-grafana-workspace-networkaccesscontrol-vpceids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpce_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for NetworkAccessControl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref prefix_list_ids) = self.prefix_list_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrefixListIds", prefix_list_ids)?;
            }
            if let Some(ref vpce_ids) = self.vpce_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpceIds", vpce_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkAccessControl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkAccessControl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkAccessControl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkAccessControl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut prefix_list_ids: Option<::ValueList<String>> = None;
                    let mut vpce_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrefixListIds" => {
                                prefix_list_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpceIds" => {
                                vpce_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkAccessControl {
                        prefix_list_ids: prefix_list_ids,
                        vpce_ids: vpce_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Grafana::Workspace.RoleValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-rolevalues.html) property type.
    #[derive(Debug, Default)]
    pub struct RoleValues {
        /// Property [`Admin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-rolevalues.html#cfn-grafana-workspace-rolevalues-admin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub admin: Option<::ValueList<String>>,
        /// Property [`Editor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-rolevalues.html#cfn-grafana-workspace-rolevalues-editor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub editor: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RoleValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref admin) = self.admin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Admin", admin)?;
            }
            if let Some(ref editor) = self.editor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Editor", editor)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoleValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoleValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoleValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoleValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut admin: Option<::ValueList<String>> = None;
                    let mut editor: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Admin" => {
                                admin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Editor" => {
                                editor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoleValues {
                        admin: admin,
                        editor: editor,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Grafana::Workspace.SamlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SamlConfiguration {
        /// Property [`AllowedOrganizations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html#cfn-grafana-workspace-samlconfiguration-allowedorganizations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_organizations: Option<::ValueList<String>>,
        /// Property [`AssertionAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html#cfn-grafana-workspace-samlconfiguration-assertionattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assertion_attributes: Option<::Value<AssertionAttributes>>,
        /// Property [`IdpMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html#cfn-grafana-workspace-samlconfiguration-idpmetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idp_metadata: ::Value<IdpMetadata>,
        /// Property [`LoginValidityDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html#cfn-grafana-workspace-samlconfiguration-loginvalidityduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub login_validity_duration: Option<::Value<f64>>,
        /// Property [`RoleValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html#cfn-grafana-workspace-samlconfiguration-rolevalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_values: Option<::Value<RoleValues>>,
    }

    impl ::codec::SerializeValue for SamlConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_organizations) = self.allowed_organizations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOrganizations", allowed_organizations)?;
            }
            if let Some(ref assertion_attributes) = self.assertion_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssertionAttributes", assertion_attributes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdpMetadata", &self.idp_metadata)?;
            if let Some(ref login_validity_duration) = self.login_validity_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoginValidityDuration", login_validity_duration)?;
            }
            if let Some(ref role_values) = self.role_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleValues", role_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SamlConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SamlConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SamlConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SamlConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_organizations: Option<::ValueList<String>> = None;
                    let mut assertion_attributes: Option<::Value<AssertionAttributes>> = None;
                    let mut idp_metadata: Option<::Value<IdpMetadata>> = None;
                    let mut login_validity_duration: Option<::Value<f64>> = None;
                    let mut role_values: Option<::Value<RoleValues>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedOrganizations" => {
                                allowed_organizations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssertionAttributes" => {
                                assertion_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdpMetadata" => {
                                idp_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoginValidityDuration" => {
                                login_validity_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleValues" => {
                                role_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SamlConfiguration {
                        allowed_organizations: allowed_organizations,
                        assertion_attributes: assertion_attributes,
                        idp_metadata: idp_metadata.ok_or(::serde::de::Error::missing_field("IdpMetadata"))?,
                        login_validity_duration: login_validity_duration,
                        role_values: role_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Grafana::Workspace.VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-vpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-vpcconfiguration.html#cfn-grafana-workspace-vpcconfiguration-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-vpcconfiguration.html#cfn-grafana-workspace-vpcconfiguration-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
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
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfiguration {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
