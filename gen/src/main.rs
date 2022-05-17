use std::fs;
use std::fs::File;
use std::io::Write;

use convert_case::{Case, Casing};
use regex::Regex;

fn main() {
    let mut main = File::create("../src/lib.rs").unwrap();
    main.write("pub mod props;".as_bytes()).unwrap();

    for style in fs::read_dir("../heroicons/optimized/").unwrap() {
        let style = style.unwrap();
        let style_path = style.path();
        let style_name = style_path.file_stem().unwrap().to_str().unwrap();

        writeln!(&mut main, "pub mod {};", style_name).unwrap();

        let mut style_mod = File::create(format!("../src/{}.rs", style_name)).unwrap();

        style_mod
            .write(
                "use yew::prelude::*;

use crate::props::Props;

"
                .as_bytes(),
            )
            .unwrap();

        if let Ok(files) = fs::read_dir(format!("../heroicons/optimized/{}/", style_name)) {
            for file in files {
                let file = file.unwrap();
                let path = file.path();
                let stem = path.as_path().file_stem().unwrap();
                let file_name = stem.to_str().unwrap();

                let svg = fs::read_to_string(file.path()).unwrap();

                let re = Regex::new(r"<svg ").unwrap();
                let svg = re.replace(
                    &svg,
                    "<svg class={{ props.class.clone() }} fill-rule=\"currentColor\" ",
                );

                let re = Regex::new("fill=\"#[\\w]{3,6}\"").unwrap();
                let svg = re.replace_all(&svg, "");

                let re = Regex::new("fill=\"none\"").unwrap();
                let svg = re.replace(&svg, "fill=\"currentColor\"");

                let snake_case = file_name.to_case(Case::Snake);

                let component =
                    component(file_name.to_case(Case::Pascal), snake_case, svg.to_string());

                style_mod.write(component.as_bytes()).unwrap();
            }
        }
    }
}

fn component(name: String, fn_name: String, svg: String) -> String {
    format!(
        "#[function_component({}Icon)]
pub fn {}(props: &Props) -> Html {{
  html! {{
    {}
  }}
}}

",
        name, fn_name, svg
    )
}
