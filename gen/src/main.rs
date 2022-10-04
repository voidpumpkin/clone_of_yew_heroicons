// #![allow(clippy::unused_io_amount)]

use std::fs::File;
use std::io::Write;
use std::{fs, path};

use convert_case::{Case, Casing};
use regex::Regex;

const PROPS: &str = "use yew::prelude::*;

#[derive(Clone, Properties, Eq, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}";

fn main() {
    create_folder("./src/".to_string());
    let mut main = File::create("./src/lib.rs").unwrap();

    writeln!(&mut main, "pub mod props;").unwrap();
    let mut props_file = File::create("./src/props.rs").unwrap();
    writeln!(&mut props_file, "{}", PROPS).unwrap();

    for size_folder in fs::read_dir("./heroicons/optimized/").unwrap() {
        let size_folder = size_folder.unwrap();
        let size_folder_path = size_folder.path();
        let size_folder_name = size_folder_path.file_stem().unwrap().to_str().unwrap();

        create_folder(format!("./src/size_{size_folder_name}/"));
        let mut size_module =
            File::create(format!("./src/size_{size_folder_name}/mod.rs")).unwrap();
        process_styles(&mut size_module, size_folder_name);
        writeln!(&mut main, "pub mod size_{size_folder_name};").unwrap();
    }
}

fn create_folder(path: String) {
    if !path::Path::new(&path).exists() {
        fs::create_dir(path).unwrap();
    }
}

fn process_styles(module_file: &mut File, size_folder_name: &str) {
    for style_folder in fs::read_dir(format!("./heroicons/optimized/{size_folder_name}/")).unwrap()
    {
        let style_folder = style_folder.unwrap();
        let style_folder_path = style_folder.path();
        let style_folder_name = style_folder_path.file_stem().unwrap().to_str().unwrap();

        writeln!(module_file, "pub mod {};", style_folder_name).unwrap();

        create_folder(format!(
            "./src/size_{size_folder_name}/{style_folder_name}/"
        ));
        let mut style_module = File::create(format!(
            "./src/size_{size_folder_name}/{style_folder_name}/mod.rs"
        ))
        .unwrap();

        if let Ok(files) = fs::read_dir(format!(
            "./heroicons/optimized/{size_folder_name}/{style_folder_name}"
        )) {
            for file in files {
                let file = file.unwrap();
                let path = file.path();
                let stem = path.as_path().file_stem().unwrap();
                let file_name = stem.to_str().unwrap();

                let svg = fs::read_to_string(file.path()).unwrap();
                let svg = svg.trim();

                let re = Regex::new(r"<svg ").unwrap();
                let svg = re.replace(svg, "<svg {{class}} fill-rule=\"currentColor\" ");

                let re = Regex::new("fill=\"#[\\w]{3,6}\"").unwrap();
                let svg = re.replace_all(&svg, "");

                let re = Regex::new("fill=\"none\"").unwrap();
                let svg = re.replace(&svg, "fill=\"currentColor\"");

                let component = component(file_name.to_case(Case::Pascal), svg.to_string());

                let snake_name = file_name.to_case(Case::Snake);

                writeln!(&mut style_module, "pub mod {};", snake_name).unwrap();
                let mut component_file = File::create(format!(
                    "./src/size_{size_folder_name}/{style_folder_name}/{snake_name}.rs"
                ))
                .unwrap();
                write!(&mut component_file, "{}", component).unwrap();
            }
        }
    }
}

fn component(name: String, svg: String) -> String {
    format!(
        "use yew::prelude::*;
use crate::props::Props;

#[function_component]
pub fn {}Icon(props: &Props) -> Html {{
    let Props {{ class }} = props.clone();

  html! {{
{}
  }}
}}",
        name, svg
    )
}
