//! Types for the `CodeStar` service.

/// The [`AWS::CodeStar::GitHubRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html) resource type.
#[derive(Debug, Default)]
pub struct GitHubRepository {
    properties: GitHubRepositoryProperties
}

/// Properties for the `GitHubRepository` resource.
#[derive(Debug, Default)]
pub struct GitHubRepositoryProperties {
    /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-code).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code: Option<::Value<self::git_hub_repository::Code>>,
    /// Property [`ConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-connectionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_arn: Option<::Value<String>>,
    /// Property [`EnableIssues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-enableissues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_issues: Option<::Value<bool>>,
    /// Property [`IsPrivate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-isprivate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_private: Option<::Value<bool>>,
    /// Property [`RepositoryAccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-repositoryaccesstoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_access_token: Option<::Value<String>>,
    /// Property [`RepositoryDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-repositorydescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_description: Option<::Value<String>>,
    /// Property [`RepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-repositoryname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_name: ::Value<String>,
    /// Property [`RepositoryOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html#cfn-codestar-githubrepository-repositoryowner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub repository_owner: ::Value<String>,
}

impl ::serde::Serialize for GitHubRepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref code) = self.code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", code)?;
        }
        if let Some(ref connection_arn) = self.connection_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionArn", connection_arn)?;
        }
        if let Some(ref enable_issues) = self.enable_issues {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIssues", enable_issues)?;
        }
        if let Some(ref is_private) = self.is_private {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsPrivate", is_private)?;
        }
        if let Some(ref repository_access_token) = self.repository_access_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryAccessToken", repository_access_token)?;
        }
        if let Some(ref repository_description) = self.repository_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryDescription", repository_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryName", &self.repository_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryOwner", &self.repository_owner)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GitHubRepositoryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GitHubRepositoryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GitHubRepositoryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GitHubRepositoryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut code: Option<::Value<self::git_hub_repository::Code>> = None;
                let mut connection_arn: Option<::Value<String>> = None;
                let mut enable_issues: Option<::Value<bool>> = None;
                let mut is_private: Option<::Value<bool>> = None;
                let mut repository_access_token: Option<::Value<String>> = None;
                let mut repository_description: Option<::Value<String>> = None;
                let mut repository_name: Option<::Value<String>> = None;
                let mut repository_owner: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Code" => {
                            code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionArn" => {
                            connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableIssues" => {
                            enable_issues = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsPrivate" => {
                            is_private = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryAccessToken" => {
                            repository_access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryDescription" => {
                            repository_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryName" => {
                            repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RepositoryOwner" => {
                            repository_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GitHubRepositoryProperties {
                    code: code,
                    connection_arn: connection_arn,
                    enable_issues: enable_issues,
                    is_private: is_private,
                    repository_access_token: repository_access_token,
                    repository_description: repository_description,
                    repository_name: repository_name.ok_or(::serde::de::Error::missing_field("RepositoryName"))?,
                    repository_owner: repository_owner.ok_or(::serde::de::Error::missing_field("RepositoryOwner"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GitHubRepository {
    type Properties = GitHubRepositoryProperties;
    const TYPE: &'static str = "AWS::CodeStar::GitHubRepository";
    fn properties(&self) -> &GitHubRepositoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GitHubRepositoryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GitHubRepository {}

impl From<GitHubRepositoryProperties> for GitHubRepository {
    fn from(properties: GitHubRepositoryProperties) -> GitHubRepository {
        GitHubRepository { properties }
    }
}

pub mod git_hub_repository {
    //! Property types for the `GitHubRepository` resource.

    /// The [`AWS::CodeStar::GitHubRepository.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-code.html) property type.
    #[derive(Debug, Default)]
    pub struct Code {
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-code.html#cfn-codestar-githubrepository-code-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: ::Value<S3>,
    }

    impl ::codec::SerializeValue for Code {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", &self.s3)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Code {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Code, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Code;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Code")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3: Option<::Value<S3>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Code {
                        s3: s3.ok_or(::serde::de::Error::missing_field("S3"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeStar::GitHubRepository.S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-s3.html) property type.
    #[derive(Debug, Default)]
    pub struct S3 {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-s3.html#cfn-codestar-githubrepository-s3-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-s3.html#cfn-codestar-githubrepository-s3-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-s3.html#cfn-codestar-githubrepository-s3-objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref object_version) = self.object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersion", object_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut object_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectVersion" => {
                                object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3 {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        object_version: object_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
