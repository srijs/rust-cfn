//! Types for the `DirectoryService` service.

/// The [`AWS::DirectoryService::MicrosoftAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-microsoftad.html) resource type.
#[derive(Debug)]
pub struct MicrosoftAD {
    properties: MicrosoftADProperties
}

/// Properties for the `MicrosoftAD` resource.
#[derive(Debug)]
pub struct MicrosoftADProperties {
    /// Property `CreateAlias`.
    pub create_alias: Option<::Value<bool>>,
    /// Property `EnableSso`.
    pub enable_sso: Option<::Value<bool>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Password`.
    pub password: ::Value<String>,
    /// Property `ShortName`.
    pub short_name: Option<::Value<String>>,
    /// Property `VpcSettings`.
    pub vpc_settings: ::Value<self::microsoft_ad::VpcSettings>,
}

impl ::serde::Serialize for MicrosoftADProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref create_alias) = self.create_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateAlias", create_alias)?;
        }
        if let Some(ref enable_sso) = self.enable_sso {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSso", enable_sso)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
        if let Some(ref short_name) = self.short_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShortName", short_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSettings", &self.vpc_settings)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MicrosoftADProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MicrosoftADProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MicrosoftADProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MicrosoftADProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut create_alias = None;
                let mut enable_sso = None;
                let mut name = None;
                let mut password = None;
                let mut short_name = None;
                let mut vpc_settings = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CreateAlias" => {
                            create_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableSso" => {
                            enable_sso = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ShortName" => {
                            short_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSettings" => {
                            vpc_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MicrosoftADProperties {
                    create_alias: create_alias,
                    enable_sso: enable_sso,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                    short_name: short_name,
                    vpc_settings: vpc_settings.ok_or(::serde::de::Error::missing_field("VpcSettings"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MicrosoftAD {
    type Properties = MicrosoftADProperties;
    const TYPE: &'static str = "AWS::DirectoryService::MicrosoftAD";
    fn properties(&self) -> &MicrosoftADProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MicrosoftADProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MicrosoftAD {}

impl From<MicrosoftADProperties> for MicrosoftAD {
    fn from(properties: MicrosoftADProperties) -> MicrosoftAD {
        MicrosoftAD { properties }
    }
}

/// The [`AWS::DirectoryService::SimpleAD`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-simplead.html) resource type.
#[derive(Debug)]
pub struct SimpleAD {
    properties: SimpleADProperties
}

/// Properties for the `SimpleAD` resource.
#[derive(Debug)]
pub struct SimpleADProperties {
    /// Property `CreateAlias`.
    pub create_alias: Option<::Value<bool>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EnableSso`.
    pub enable_sso: Option<::Value<bool>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Password`.
    pub password: ::Value<String>,
    /// Property `ShortName`.
    pub short_name: Option<::Value<String>>,
    /// Property `Size`.
    pub size: ::Value<String>,
    /// Property `VpcSettings`.
    pub vpc_settings: ::Value<self::simple_ad::VpcSettings>,
}

impl ::serde::Serialize for SimpleADProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref create_alias) = self.create_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateAlias", create_alias)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enable_sso) = self.enable_sso {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSso", enable_sso)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
        if let Some(ref short_name) = self.short_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShortName", short_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSettings", &self.vpc_settings)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SimpleADProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SimpleADProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SimpleADProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SimpleADProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut create_alias = None;
                let mut description = None;
                let mut enable_sso = None;
                let mut name = None;
                let mut password = None;
                let mut short_name = None;
                let mut size = None;
                let mut vpc_settings = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CreateAlias" => {
                            create_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableSso" => {
                            enable_sso = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ShortName" => {
                            short_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Size" => {
                            size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSettings" => {
                            vpc_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SimpleADProperties {
                    create_alias: create_alias,
                    description: description,
                    enable_sso: enable_sso,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                    short_name: short_name,
                    size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                    vpc_settings: vpc_settings.ok_or(::serde::de::Error::missing_field("VpcSettings"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SimpleAD {
    type Properties = SimpleADProperties;
    const TYPE: &'static str = "AWS::DirectoryService::SimpleAD";
    fn properties(&self) -> &SimpleADProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SimpleADProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SimpleAD {}

impl From<SimpleADProperties> for SimpleAD {
    fn from(properties: SimpleADProperties) -> SimpleAD {
        SimpleAD { properties }
    }
}

pub mod microsoft_ad {
    //! Property types for the `MicrosoftAD` resource.

    /// The [`AWS::DirectoryService::MicrosoftAD.VpcSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-microsoftad-vpcsettings.html) property type.
    #[derive(Debug)]
    pub struct VpcSettings {
        /// Property `SubnetIds`.
        pub subnet_ids: ::ValueList<String>,
        /// Property `VpcId`.
        pub vpc_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for VpcSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subnet_ids = None;
                    let mut vpc_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcSettings {
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                        vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod simple_ad {
    //! Property types for the `SimpleAD` resource.

    /// The [`AWS::DirectoryService::SimpleAD.VpcSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-simplead-vpcsettings.html) property type.
    #[derive(Debug)]
    pub struct VpcSettings {
        /// Property `SubnetIds`.
        pub subnet_ids: ::ValueList<String>,
        /// Property `VpcId`.
        pub vpc_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for VpcSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subnet_ids = None;
                    let mut vpc_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcSettings {
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                        vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
