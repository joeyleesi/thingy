use docs::Docs;

pub fn generate(docs: Docs, page: String) {
    let page = match docs.find_page(&page) {
        Some(page) => page,
        None => {
            eprintln!("Page not found: {}", page);
            return;
        }
    };
    let mut page = page.clone();
    page.swap_setters_and_getters();
    println!("{page}");
}

pub fn get_pages(docs: &Docs) -> Vec<String> {
    let mut pages = docs
        .globals
        .fields
        .iter()
        .filter(|field| field.r#type != "Function")
        .flat_map(|field| field.children.iter().map(|page| page.name.to_string()))
        .collect::<Vec<_>>();
    pages.push("math".to_string());
    pages
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

fn unique(v: Vec<String>) -> Vec<String> {
    let unique = std::collections::BTreeSet::from_iter(v.into_iter());
    unique.into_iter().collect::<Vec<_>>()
}
