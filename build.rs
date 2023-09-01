use regex::Regex;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn trim_whitespace(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  s.split_whitespace().for_each(|w| {
    result.push_str(w);
  });
  result
}

fn generate_component_fn(
  template_str: String,
  component_name: String,
  field_names: Vec<String>,
  generated_code: &mut String,
) {
  // generated_code.push_str(&format!("pub struct {}Props {{\n", component_name));
  // field_names.iter().for_each(|field_name| {
  //   generated_code.push_str(&format!("  pub {}: String,\n", field_name))
  // });
  // generated_code.push_str("}\n\n");

  let formatable_str = template_str.replace("#{", "{");
  let formatable_str = trim_whitespace(&formatable_str);
  // generated_code.push_str(&format!(
  //   "pub fn {} (props: {}Props) -> String {{\n",
  //   component_name, component_name,
  // ));
  // generated_code.push_str(&format!("  let {}Props {{\n", component_name));
  // field_names.iter().for_each(|field_name| {
  //   generated_code.push_str(&format!("    {},\n", field_name))
  // });
  // generated_code.push_str(&format!("  }} = props;\n"));
  // generated_code.push_str(&format!("  format!(r#\"{}\"#)\n", formatable_str));
  // generated_code.push_str("}\n\n");

  generated_code.push_str(&format!("pub fn {} (\n", component_name,));
  field_names.iter().for_each(|field_name| {
    generated_code.push_str(&format!("  {}: String,\n", field_name));
  });
  generated_code.push_str(") -> String {\n");
  generated_code.push_str(&format!("  format!(r#\"{}\"#)\n", formatable_str));
  generated_code.push_str("}\n\n");
}

fn main() {
  let mut generated_code = String::new();

  fs::read_dir("./templates")
    .unwrap()
    .map(|entry| entry.unwrap())
    .for_each(|entry| {
      let file_path = entry.path();
      let file_name = entry.file_name().to_str().unwrap().to_string();
      let mut file = File::open(&file_path).unwrap();
      if file_name.ends_with(".html") {
        let component_name = file_name.trim_end_matches(".html");
        let mut template_str = String::new();
        file.read_to_string(&mut template_str).unwrap();

        let field_pattern = Regex::new(r"#\{(?<name>\w+)\}").unwrap();
        let field_names: Vec<String> = field_pattern
          .captures_iter(&template_str)
          .map(|n| n["name"].to_string())
          .collect();

        generate_component_fn(
          template_str,
          component_name.to_string(),
          field_names,
          &mut generated_code,
        );
      }
    });

  let output_path = Path::new("./src/template.rs");
  let mut output_file = File::create(&output_path).unwrap();
  output_file.write_all(generated_code.as_bytes()).unwrap();
}
