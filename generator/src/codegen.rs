use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use heck::SnakeCase;
use itertools::Itertools;

use model::*;

pub fn generate<P: AsRef<Path>>(spec: &Specification, base_path: P) -> io::Result<()> {
    let groups = spec.resource_types.iter().map(|(res_name, res_spec)| {
        assert!(res_name.starts_with("AWS::"));
        let split = res_name[5..].split("::").collect::<Vec<_>>();
        assert!(split.len() == 2);
        (split[0], split[1], res_spec)
    }).group_by(|&(service_name, _, _)| service_name);

    let mut services = Vec::new();
    let mut resources = Vec::new();
    for (service_name, resource_types) in groups.into_iter() {
        services.push(service_name.to_owned());
        let file_path = base_path.as_ref().join(format!("{}.rs", service_name.to_lowercase()));
        let mut file = File::create(file_path)?;
        for (_, resource_name, resource_spec) in resource_types {
            resources.push((service_name.to_owned(), resource_name.to_owned()));
            generate_resource(service_name, resource_name, resource_spec, &mut file)?;
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

fn generate_resource(service: &str, name: &str, spec: &ResourceSpecification, f: &mut Write) -> io::Result<()> {
    writeln!(f, "/// The [`AWS::{}::{}`]({}) resource.", service, name, spec.documentation)?;
    writeln!(f, "#[derive(Serialize, Deserialize)]")?;
    writeln!(f, "pub struct {} {{", name)?;
    writeln!(f, "    properties: {}Properties", name)?;
    writeln!(f, "}}\n")?;

    writeln!(f, "/// Properties for the `{}` resource.", name)?;
    writeln!(f, "#[derive(Serialize, Deserialize)]")?;
    writeln!(f, "pub struct {}Properties {{", name)?;
    for (ref property_name, ref property_spec) in spec.properties.iter() {
        generate_property(property_name, property_spec, f)?;
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

    writeln!(f, "impl From<{}Properties> for {} {{", name, name)?;
    writeln!(f, "    fn from(properties: {}Properties) -> {} {{", name, name)?;
    writeln!(f, "        {} {{ properties }}", name)?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    Ok(())
}

fn generate_property(name: &str, spec: &PropertySpecification, f: &mut Write) -> io::Result<()> {
    let mut field_name = name.to_snake_case();
    
    if field_name == "type" {
        field_name = "type_".into();
    }

    writeln!(f, "    #[serde(rename=\"{}\")]", name)?;
    if let Some(ref type_name) = spec.type_ {
        if type_name == "List" {
            if let Some(ref _item_type) = spec.item_type {
                writeln!(f, "    pub {}: Vec<()>,", field_name)?;
            } else {
                writeln!(f, "    pub {}: Vec<{}>,", field_name,
                    generate_primitive_item_type(spec.primitive_item_type.as_ref().unwrap()))?;
            }
        } else if type_name == "Map" {
            if let Some(ref _item_type) = spec.item_type {
                writeln!(f, "    pub {}: ::std::collections::HashMap<String, ()>,", field_name)?;
            } else {
                writeln!(f, "    pub {}: ::std::collections::HashMap<String, {}>,", field_name,
                    generate_primitive_item_type(spec.primitive_item_type.as_ref().unwrap()))?;
            }
        } else {
            writeln!(f, "    pub {}: (),", field_name)?;
        }
    } else {
        writeln!(f, "    pub {}: {},", field_name, generate_primitive_type(spec.primitive_type.as_ref().unwrap()))?;
    }

    Ok(())
}

fn generate_primitive_type(primitive_type: &PrimitiveType) -> &str {
    match primitive_type {
        &PrimitiveType::String => "String",
        &PrimitiveType::Long => "u64",
        &PrimitiveType::Integer => "u32",
        &PrimitiveType::Double => "f64",
        &PrimitiveType::Boolean => "bool",
        &PrimitiveType::Timestamp => "String",
        &PrimitiveType::Json => "String"
    }
}

fn generate_primitive_item_type(primitive_item_type: &PrimitiveItemType) -> &str {
    match primitive_item_type {
        &PrimitiveItemType::String => "String",
        &PrimitiveItemType::Long => "u64",
        &PrimitiveItemType::Integer => "u32",
        &PrimitiveItemType::Double => "f64",
        &PrimitiveItemType::Boolean => "bool",
        &PrimitiveItemType::Timestamp => "String"
    }
}
