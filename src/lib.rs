extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_struct(input: TokenStream) -> TokenStream {
  let template = parse_macro_input!(input as LitStr);
  let template_string = template.value();

  // Parse the placeholders and generate field names
  let field_pattern = Regex::new(r"#\{(?<name>\w+)\}").unwrap();
  let field_names: Vec<String> = field_pattern
    .captures_iter(&template_string)
    .map(|n| n["name"].to_string())
    .collect();

  // Generate the struct with fields
  let struct_name = Ident::new("Test", Span::call_site());
  let mut field_declarations = Vec::new();
  for field_name in &field_names {
    let field_ident = Ident::new(&field_name, Span::call_site());
    field_declarations.push(quote!(#field_ident: String,));
  }

  let gen = quote! {
      struct #struct_name {
          #(#field_declarations)*
      }
  };

  gen.into()
}
