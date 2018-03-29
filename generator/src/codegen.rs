use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use heck::SnakeCase;
use itertools::{Itertools, EitherOrBoth, merge_join_by};

use model::*;

pub fn generate<P: AsRef<Path>>(spec: Specification, base_path: P) -> io::Result<()> {
    let resource_groups = spec.resource_types.into_iter().map(|(res_name, res_spec)| {
        assert!(res_name.starts_with("AWS::"));
        let split = res_name[5..].split("::").collect::<Vec<_>>();
        assert!(split.len() == 2);
        (split[0].to_owned(), split[1].to_owned(), res_spec)
    }).group_by(|&(ref service_name, _, _)| service_name.to_owned());

    let property_groups = spec.property_types.into_iter().flatten().filter_map(|(prop_name, prop_spec)| {
        if prop_name.starts_with("AWS::") {
            let split = prop_name[5..].split("::").collect::<Vec<_>>();
            assert!(split.len() == 2);
            let split2 = split[1].split(".").collect::<Vec<_>>();
            assert!(split2.len() == 2);
            Some((split[0].to_owned(), split2[0].to_owned(), split2[1].to_owned(), prop_spec))
        } else {
            None
        }
    }).group_by(|&(ref service_name, _, _, _)| service_name.to_owned());

    let groups = merge_join_by(resource_groups.into_iter(), property_groups.into_iter(),
        |&(ref service_name_left, _), &(ref service_name_right, _)| service_name_left.cmp(service_name_right));

    let mut services = Vec::new();
    let mut resources = Vec::new();
    for joined_specs in groups {
        let (service_name, resource_specs_opt, property_specs_opt) = factor_joined_specs(joined_specs);
        services.push(service_name.to_owned());
        let file_path = base_path.as_ref().join(format!("{}.rs", service_name.to_lowercase()));
        let mut file = File::create(file_path)?;

        if let Some(resource_specs) = resource_specs_opt {
            for (_, resource_name, resource_spec) in resource_specs {
                resources.push((service_name.to_owned(), resource_name.to_owned()));
                generate_resource_declaration(&service_name, &resource_name, &resource_spec, &mut file)?;
            }
        }

        if let Some(property_specs) = property_specs_opt {
            let properties_by_resource = property_specs.group_by(|&(_, ref resource_name, _, _)| resource_name.to_owned());
            for (resource_name, resource_property_specs) in properties_by_resource.into_iter() {
                writeln!(file, "pub mod {} {{", resource_name.to_snake_case())?;
                for (_, resource_name, property_name, property_spec) in resource_property_specs {
                    generate_property_declaration(&service_name, &resource_name, &property_name, &property_spec, &mut file)?;
                }
                writeln!(file, "}}\n")?;
            }
        }
    }

    {
        let mod_file_path = base_path.as_ref().join("mod.rs");
        let mut mod_file = File::create(mod_file_path)?;
        for service_name in services {
            writeln!(mod_file, "pub mod {};", service_name.to_lowercase())?;
        }
    }

    Ok(())
}

fn factor_joined_specs<X, A, B>(either: EitherOrBoth<(X, A), (X, B)>) -> (X, Option<A>, Option<B>) {
    match either {
        EitherOrBoth::Left((key, left)) => (key, Some(left), None),
        EitherOrBoth::Right((key, right)) => (key, None, Some(right)),
        EitherOrBoth::Both((key, left), (_, right)) => (key, Some(left), Some(right))
    }
}

fn generate_property_declaration(service: &str, resource_name: &str, name: &str, spec: &PropertyType, f: &mut Write) -> io::Result<()> {
    writeln!(f, "    /// The [`AWS::{}::{}.{}`]({}) property type.", service, resource_name, name, spec.documentation)?;
    writeln!(f, "    #[derive(Serialize, Deserialize)]")?;
    writeln!(f, "    pub struct {} {{", name)?;
    for (ref property_name, ref property_spec) in spec.properties.iter() {
        generate_field(None, property_name, property_spec, "    ", f)?;
    }
    writeln!(f, "    }}\n")?;

    Ok(())
}

fn generate_resource_declaration(service: &str, name: &str, spec: &ResourceType, f: &mut Write) -> io::Result<()> {
    writeln!(f, "/// The [`AWS::{}::{}`]({}) resource type.", service, name, spec.documentation)?;
    writeln!(f, "pub struct {} {{", name)?;
    writeln!(f, "    properties: {}Properties", name)?;
    writeln!(f, "}}\n")?;

    writeln!(f, "/// Properties for the `{}` resource.", name)?;
    writeln!(f, "#[derive(Serialize, Deserialize)]")?;
    writeln!(f, "pub struct {}Properties {{", name)?;
    let namespace = name.to_snake_case();
    for (ref property_name, ref property_spec) in spec.properties.iter() {
        generate_field(Some(&namespace), property_name, property_spec, "", f)?;
    }
    writeln!(f, "}}\n")?;

    writeln!(f, "impl<'a> ::Resource<'a> for {} {{", name)?;
    writeln!(f, "    type Properties = {}Properties;", name)?;
    writeln!(f, "    const TYPE: &'static str = \"AWS::{}::{}\";", service, name)?;
    writeln!(f, "    fn properties(&self) -> &{}Properties {{", name)?;
    writeln!(f, "        &self.properties")?;
    writeln!(f, "    }}")?;
    writeln!(f, "    fn properties_mut(&mut self) -> &mut {}Properties {{", name)?;
    writeln!(f, "        &mut self.properties")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl ::private::Sealed for {} {{}}\n", name)?;

    writeln!(f, "impl From<{}Properties> for {} {{", name, name)?;
    writeln!(f, "    fn from(properties: {}Properties) -> {} {{", name, name)?;
    writeln!(f, "        {} {{ properties }}", name)?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    Ok(())
}

fn generate_field(namespace_opt: Option<&str>, name: &str, spec: &PropertySpecification, indent: &str, f: &mut Write) -> io::Result<()> {
    let mut field_name = name.to_snake_case();
    
    if field_name == "type" {
        field_name = "type_".into();
    }

    writeln!(f, "    {}#[serde(rename=\"{}\")]", indent, name)?;
    writeln!(f, "    {}pub {}: {},", indent, field_name, generate_field_type(namespace_opt, spec))?;

    Ok(())
}

fn generate_field_type(namespace_opt: Option<&str>, spec: &PropertySpecification) -> String {
    if let Some(ref type_name) = spec.type_ {
        if type_name == "List" {
            if let Some(ref item_type) = spec.item_type {
                if item_type == "Tag" {
                    "::Tags".to_owned()
                } else {
                    format!("Vec<{}>", generate_type(namespace_opt, item_type))
                }
            } else {
                format!("Vec<{}>",
                    generate_primitive_type(spec.primitive_item_type.as_ref().unwrap()))
            }
        } else if type_name == "Map" {
            if let Some(ref item_type) = spec.item_type {
                format!("::std::collections::HashMap<String, {}>", generate_type(namespace_opt, item_type))
            } else {
                format!("::std::collections::HashMap<String, {}>",
                    generate_primitive_type(spec.primitive_item_type.as_ref().unwrap()))
            }
        } else {
            generate_type(namespace_opt, type_name)
        }
    } else {
        format!("{}", generate_primitive_type(spec.primitive_type.as_ref().unwrap()))
    }
}

fn generate_type(namespace_opt: Option<&str>, name: &str) -> String {
    if let Some(namespace) = namespace_opt {
        format!("self::{}::{}", namespace, name)
    } else {
        name.to_owned()
    }
}

fn generate_primitive_type(primitive_type: &PrimitiveType) -> &str {
    match primitive_type {
        &PrimitiveType::String => "String",
        &PrimitiveType::Long => "u64",
        &PrimitiveType::Integer => "u32",
        &PrimitiveType::Double => "f64",
        &PrimitiveType::Boolean => "bool",
        &PrimitiveType::Timestamp => "String",
        &PrimitiveType::Json => "::json::Value"
    }
}
