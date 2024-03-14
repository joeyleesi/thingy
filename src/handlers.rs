use crate::models::Docs;

pub fn generate(docs: Docs, pages: Vec<String>) {
    let all_pages = get_pages(&docs);
    for page in &pages {
        if !all_pages.contains(&page) {
            eprintln!("Page not found: {page}");
        }
    }
    let doc_pages = docs
        .globals
        .fields
        .iter()
        .filter(|field| field.r#type != "Function")
        .flat_map(|field| {
            field
                .children
                .iter()
                .filter(|page| pages.contains(&page.name))
        })
        .collect::<Vec<_>>();
    if doc_pages.len() == 0 {
        eprint!("No pages found!");
    }
    // TODO: impl these as to_string or something less gross than this
    for page in doc_pages {
        println!("import Tabs from '@theme/Tabs'");
        println!("import TabItem from '@theme/TabItem'");
        println!();
        println!("{}", page.description);
        println!();
        for method in &page.methods {
            println!("### <code>{}()</code> \\{{#{}}}", method.name, method.name);
            println!();
            println!("{}", method.description);
            println!();
            if method.parameters.len() > 1 {
                println!("<Tabs>");
            }
            for (i, overload) in method.parameters.iter().enumerate() {
                if method.parameters.len() > 1 {
                    println!(
                        "    <TabItem value=\"overload-{}\" label=\"Overload {}\">",
                        i + 1,
                        i + 1
                    );
                }
                println!("```lua");
                let params = overload
                    .iter()
                    .map(|param| param.name.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}({})", method.name, params);
                println!("```");
                if overload.len() > 0 {
                    println!();
                    println!("**Parameters**");
                    println!("| Name | Type | Description | Default |");
                    println!("| - | - | - | - |");
                    for param in overload {
                        let type_link = format!(
                            "<code>[{}]({})</code>",
                            param.r#type,
                            type_url(&param.r#type)
                        );
                        println!("| {} | {} | - | - |", param.name, type_link);
                    }
                }
                println!();
                println!("**Returns**");
                println!("| Type | Description |");
                println!("| - | - |");
                let type_link = format!(
                    "<code>[{}]({})</code>",
                    method.returns[i],
                    type_url(&method.returns[i])
                );
                println!("| {} | - |", type_link);
                if method.parameters.len() > 1 {
                    println!("    </TabItem>");
                }
            }
            if method.parameters.len() > 1 {
                println!("</Tabs>");
            }
            println!();
            println!("**Example**");
            println!();
            println!("```lua");
            println!("-- TODO");
            println!("```");
            println!();
            println!("---");
            println!();
        }
    }
}

pub fn get_pages(docs: &Docs) -> Vec<String> {
    docs.globals
        .fields
        .iter()
        .filter(|field| field.r#type != "Function")
        .flat_map(|field| field.children.iter().map(|page| page.name.to_string()))
        .collect()
}

pub fn get_types(docs: Docs) {
    let types = docs
        .globals
        .fields
        .iter()
        .filter(|field| field.r#type != "Function")
        .flat_map(|field| field.children.iter().flat_map(|class| class.all_types()))
        .collect::<Vec<_>>();
    let unique_types = unique(types);
    for t in unique_types {
        println!("{t}");
    }
}

fn type_url(r#type: &str) -> &str {
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

fn unique(v: Vec<String>) -> Vec<String> {
    let unique = std::collections::BTreeSet::from_iter(v.into_iter());
    unique.into_iter().collect::<Vec<_>>()
}
