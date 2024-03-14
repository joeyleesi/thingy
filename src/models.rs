use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Docs {
    pub globals: Class,
    pub math: Class,
}

#[derive(Deserialize, Debug)]
pub struct Class {
    pub name: String,
    pub description: String,
    pub methods: Vec<Method>,
    pub fields: Vec<Field>,
}

#[derive(Deserialize, Debug)]
pub struct Method {
    pub aliases: Vec<String>,
    pub description: String,
    pub name: String,
    pub parameters: Vec<Vec<Parameter>>,
    pub returns: Vec<String>,
    pub r#static: bool,
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub editable: bool,
    pub children: Vec<Class>,
}

impl Docs {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(format!("File not found: {path}")?.into());
                }
                _ => return Err(e.into()),
            },
        };
        let docs: Docs = serde_json::from_str(&data)?;
        Ok(docs)
    }
}

impl Class {
    pub fn all_types(&self) -> Vec<String> {
        self.methods
            .iter()
            .flat_map(|method| {
                [
                    method.returns.clone(),
                    method
                        .parameters
                        .iter()
                        .flat_map(|overload| overload.iter().map(|param| param.r#type.to_string()))
                        .collect::<Vec<_>>(),
                ]
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}
