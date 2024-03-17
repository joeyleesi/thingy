use crate::markdown::Table;
use crate::{header, lua};
use serde::Deserialize;
use std::{fmt, fs, io, path};

const INDENT: &str = "    ";

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
    pub parameters: Vec<Overload>,
    pub returns: Vec<String>,
    pub r#static: bool,
}

#[derive(Deserialize, Debug)]
pub struct Overload(Vec<Parameter>);

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
    pub fn from_file(path: path::PathBuf) -> Result<Self, io::Error> {
        let f = fs::File::open(path)?;
        let f = io::BufReader::new(f);
        let docs = serde_json::from_reader(f)?;
        Ok(docs)
    }

    pub fn find_page(&self, page: &str) -> Option<&Class> {
        if page == "math" {
            return Some(&self.math);
        }
        return self
            .globals
            .fields
            .iter()
            .find_map(|field| field.children.iter().find(|class| class.name == page));
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
                        .flat_map(|overload| {
                            overload.0.iter().map(|param| param.r#type.to_string())
                        })
                        .collect::<Vec<_>>(),
                ]
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self
            .methods
            .iter()
            .any(|method| method.parameters.len() > 1)
        {
            write!(f, "import Tabs from '@theme/Tabs'")?;
            write!(f, "\n")?;
            write!(f, "import TabItem from '@theme/TabItem'")?;
            write!(f, "\n\n")?;
        }
        write!(f, "{}", self.description.replace("\n", "\n\n"))?;
        write!(f, "\n\n")?;
        write!(f, "## Methods")?;
        for method in &self.methods {
            write!(f, "\n\n")?;
            write!(f, "{}", method)?;
        }
        write!(f, "\n\n")?;
        write!(f, "## Fields")?;
        for field in &self.fields {
            write!(f, "\n\n")?;
            write!(f, "{}", field)?;
        }
        Ok(())
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parameter_headers = vec![
            "Name".to_string(),
            "Type".to_string(),
            "Description".to_string(),
            "Default".to_string(),
        ];
        let return_headers = vec!["Type".to_string(), "Description".to_string()];
        let has_overloads = self.parameters.len() > 1;
        header!(f, "{}()", self.name)?;
        write!(f, "\n\n")?;
        write!(f, "{}", self.description.replace("\n", "\n\n"))?;
        write!(f, "\n\n")?;
        if has_overloads {
            write!(f, "<Tabs>")?;
            write!(f, "\n")?;
        }
        for (i, overload) in self.parameters.iter().enumerate() {
            if has_overloads {
                write!(f, "{INDENT}")?;
                write!(
                    f,
                    "<TabItem value=\"overload-{i}\" label=\"Overload {i}\">",
                    i = i + 1
                )?;
                write!(f, "\n\n")?;
            }
            lua!(f, "{}({})", self.name, overload)?;
            if overload.0.len() > 0 {
                write!(f, "\n\n")?;
                let params = Table::new(
                    "Parameters:".to_string(),
                    parameter_headers.clone(),
                    overload.to_rows(),
                );
                write!(f, "{}", params)?;
            }
            write!(f, "\n\n")?;
            let returns = Table::new(
                "Returns:".to_string(),
                return_headers.clone(),
                vec![vec![
                    format!(
                        "<code>[{}]({})</code>",
                        self.returns[i],
                        type_to_slug(&self.returns[i])
                    ),
                    "-".to_string(),
                ]],
            );
            write!(f, "{}", returns)?;
            write!(f, "\n\n")?;
            write!(f, "**Example:**")?;
            write!(f, "\n\n")?;
            lua!(f, "--todo")?;
            write!(f, "\n\n")?;

            if has_overloads {
                write!(f, "{INDENT}")?;
                write!(f, "</TabItem>")?;
                write!(f, "\n")?;
            }
        }
        if has_overloads {
            write!(f, "</Tabs>")?;
            write!(f, "\n\n")?;
        }
        write!(f, "---")?;
        Ok(())
    }
}

impl Overload {
    fn to_rows(&self) -> Vec<Vec<String>> {
        self.0
            .iter()
            .map(|param| {
                vec![
                    param.name.to_string(),
                    format!(
                        "<code>[{}]({})</code>",
                        param.r#type,
                        type_to_slug(&param.r#type)
                    ),
                    "-".to_string(),
                    "-".to_string(),
                ]
            })
            .collect()
    }
}

impl fmt::Display for Overload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params = self
            .0
            .iter()
            .map(|param| param.name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", params)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        header!(f, "{}", self.name)?;
        write!(f, "\n\n")?;
        write!(f, "{}", self.description.replace("\n", "\n\n"))?;
        write!(f, "\n\n")?;
        write!(f, "---")?;
        Ok(())
    }
}

fn type_to_slug(r#type: &str) -> &str {
    match r#type {
        "Action" => "/globals/Action-Wheel/Action",
        "ActionWheelAPI" => "/globals/Action-Wheel",
        "Animation" => "/globals/Animations/Animation",
        "AnimationAPI" => "/globals/Animations",
        "AnyType" => "#",
        "AvatarAPI" => "/globals/Avatar",
        "Biome" => "/globals/World/Biome",
        "BlockState" => "/globals/World/BlockState",
        "BlockTask" => "/globals/Models/BlockTask",
        "Boolean" => "#",
        "Buffer" => "#",
        "ConfigAPI" => "/globals/Config",
        "EntityAPI" => "/globals/Player/Entity",
        "EntityNameplateCustomization" => "/globals/Nameplate/EntityNameplateCustomization",
        "EntityTask" => "/globals/Models/EntityTask",
        "Event" => "/globals/Events/Event",
        "Function" => "#",
        "Future" => "#",
        "HostAPI" => "/globals/Host",
        "HttpRequestBuilder" => "#",
        "InputStream" => "#",
        "Integer" => "#",
        "ItemStack" => "/globals/World/ItemStack",
        "ItemTask" => "/globals/Models/ItemTask",
        "JsonArray" => "#",
        "JsonBuilder" => "#",
        "JsonObject" => "#",
        "JsonSerializer" => "#",
        "Keybind" => "/globals/Keybinds/Keybind",
        "Matrix2" => "/globals/Matrices/Matrix2",
        "Matrix3" => "/globals/Matrices/Matrix3",
        "Matrix4" => "/globals/Matrices/Matrix4",
        "ModelPart" => "/globals/Models",
        "NameplateCustomization" => "/globals/Nameplate/NameplateCustomization",
        "NameplateCustomizationGroup" => "/globals/Nameplate/NameplateCustomizationGroup",
        "Number" => "#",
        "OutputStream" => "#",
        "Page" => "/globals/Action-Wheel/Page",
        "Particle" => "/globals/Particles/Particle",
        "ParticleAPI" => "/globals/Particles",
        "RenderTask" => "/globals/Models/RenderTask",
        "RendererAPI" => "/globals/Renderer",
        "Sound" => "/globals/Sounds",
        "SoundAPI" => "/globals/Sounds/Sound",
        "SpriteTask" => "/globals/Models/SpriteTask",
        "String" => "#",
        "Table" => "#",
        "TextTask" => "/globals/Models/TextTask",
        "Texture" => "/globals/Textures/Texture",
        "TextureAtlas" => "/globals/Textures/TextureAtlas",
        "VanillaPart" => "/globals/Vanilla-Model/VanillaPart",
        "Varargs" => "#",
        "Vector2" => "/globals/Vectors/Vector2",
        "Vector3" => "/globals/Vectors/Vector3",
        "Vector4" => "/globals/Vectors/Vector4",
        "Vertex" => "/globals/Models/Vertex",
        "ViewerAPI" => "/globals/Player/Viewer",
        "java.util.Objects" => "#",
        "java.util.Set" => "#",
        "net.minecraft.class_2561" => "#",
        "nil" => "#",
        "org.figuramc.figura.lua.ReadOnlyLuaTable" => "#",
        // should this throw an error?
        _ => "#",
    }
}
