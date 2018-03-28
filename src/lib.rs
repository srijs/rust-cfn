extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

//pub mod aws;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Resources")]
    resources: Resources
}

impl Template {
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

#[derive(Serialize, Deserialize)]
pub struct Resources(HashMap<String, ResourceInner>);

impl Resources {
    pub fn get<'a, R: Resource<'a>>(&'a self, name: &str) -> Option<R> {
        self.0.get(name).and_then(|inner| {
            if inner.tag == R::TYPE {
                R::Properties::deserialize(&inner.properties).ok().map(|properties| {
                    properties.into()
                })
            } else {
                None
            }
        })
    }

    pub fn has<'a, R: Resource<'a>>(&'a self, name: &str) -> bool {
        self.0.get(name).map(|inner| inner.tag == R::TYPE).unwrap_or(false)
    }

    pub fn set<'a, R: Resource<'a>>(&mut self, name: &str, resource: R) {
        let inner = ResourceInner {
            tag: R::TYPE.to_owned(),
            properties: serde_json::to_value(resource.properties()).unwrap()
        };
        self.0.insert(name.to_owned(), inner);
    }
}

#[derive(Serialize, Deserialize)]
struct ResourceInner {
    #[serde(rename = "Type")]
    tag: String,
    #[serde(rename = "Properties")]
    properties: serde_json::Value
}

pub trait Resource<'a>: Sized {
    const TYPE: &'static str;
    type Properties: Into<Self> + serde::Serialize + serde::de::Deserialize<'a>;

    fn properties(&self) -> &Self::Properties;
    fn properties_mut(&mut self) -> &mut Self::Properties;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
