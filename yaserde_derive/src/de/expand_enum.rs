use attribute::*;
use field_type::*;
use proc_macro2::{Span, TokenStream};
use std::collections::BTreeMap;
use syn::DataEnum;
use syn::Fields;
use syn::Ident;

pub fn parse(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  _namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let match_to_enum: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| parse_variant(variant, name))
    .filter_map(|f| f)
    .collect();

  quote! {
    use xml::reader::XmlEvent;
    use yaserde::Visitor;
    #[allow(unknown_lints, unused_imports)]
    use std::str::FromStr;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let named_element =
          if let XmlEvent::StartElement{name, ..} = reader.peek()?.to_owned() {
            name.local_name.to_owned()
          } else {
            String::from(#root)
          };
        debug!("Enum: start to parse {:?}", named_element);

        #[allow(unused_assignments, unused_mut)]
        let mut enum_value = None;

        loop {
          match reader.peek()?.to_owned() {
            XmlEvent::StartElement{ref name, ref attributes, ..} => {

              match name.local_name.as_str() {
                #match_to_enum
                named_element => {
                  let _root = reader.next_event();
                }
              }

              if let XmlEvent::Characters(content) = reader.peek()?.to_owned() {
                match content.as_str() {
                  #match_to_enum
                  _ => {}
                }
              }
            }
            XmlEvent::EndElement{ref name} => {
              if name.local_name == named_element {
                break;
              }
              let _root = reader.next_event();
            }
            XmlEvent::Characters(ref text_content) => {
              let _root = reader.next_event();
            }
            event => {
              return Err(format!("unknown event {:?}", event))
            }
          }
        }

        match enum_value {
          Some(value) => Ok(value),
          None => {
            Ok(#name::default())
          },
        }
      }
    }
  }
}

fn parse_variant(variant: &syn::Variant, name: &Ident) -> Option<TokenStream> {
  let xml_element_name = YaSerdeAttribute::parse(&variant.attrs)
    .rename
    .unwrap_or_else(|| variant.ident.to_string());

  let variant_name = {
    let label = &variant.ident;
    quote! { #name::#label }
  };

  match variant.fields {
    Fields::Unit => Some(quote! {
      #xml_element_name => {
        enum_value = Some(#variant_name);
      }
    }),
    Fields::Unnamed(ref fields) => {
      let field_visitors = build_unnamed_field_visitors(fields);
      let call_visitors = build_unnamed_visitor_calls(fields, &variant_name);

      if fields.unnamed.len() > 1 {
        unimplemented!("enum variant with multiple fields")
      }

      Some(
        fields
          .unnamed
          .iter()
          .take(1)
          .map(|_field| {
            quote! {
              #xml_element_name => {
                #field_visitors
                #call_visitors
              }
            }
          })
          .collect(),
      )
    }
    _ => None,
  }
}

fn build_unnamed_field_visitors(fields: &syn::FieldsUnnamed) -> TokenStream {
  fields
    .unnamed
    .iter()
    .enumerate()
    .map(|(idx, field)| {
      let visitor_label = Ident::new(&format!("__Visitor_{}", idx), Span::call_site());

      let make_visitor = |visitor: &Ident, field_type: &TokenStream, fn_body: &TokenStream| {
        Some(quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> Visitor<'de> for #visitor_label {
            type Value = #field_type;

            fn #visitor(self, v: &str) -> Result<Self::Value, String> {
              #fn_body
            }
          }
        })
      };

      let simple_type_visitor = |simple_type| {
        let field_type = get_simple_type_token(&simple_type);
        let visitor = get_simple_type_visitor(&simple_type);

        make_visitor(
          &visitor,
          &field_type,
          &quote! { Ok(#field_type::from_str(v).unwrap()) },
        )
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { struct_name } => {
          let struct_id: String = struct_name
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();

          make_visitor(
            &Ident::new("visit_str", Span::call_site()),
            &quote! { #struct_name },
            &quote! {
              let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
              let value : Result<#struct_name, String> = yaserde::de::from_str(&content);
              value
            },
          )
        }
        FieldType::FieldTypeOption { data_type } | FieldType::FieldTypeVec { data_type } => {
          match *data_type {
            FieldType::FieldTypeStruct { .. } => None,
            simple_type => simple_type_visitor(simple_type),
          }
        }
        simple_type => simple_type_visitor(simple_type),
      })
    })
    .filter_map(|f| f)
    .collect()
}

fn build_unnamed_visitor_calls(
  fields: &syn::FieldsUnnamed,
  variant_name: &TokenStream,
) -> TokenStream {
  fields
    .unnamed
    .iter()
    .enumerate()
    .map(|(idx, field)| {
      let visitor_label = Ident::new(&format!("__Visitor_{}", idx), Span::call_site());

      let call_simple_type_visitor = |simple_type, action| {
        let field_type = get_simple_type_token(&simple_type);
        let visitor = get_simple_type_visitor(&simple_type);

        let label_name = format!("field_{}", idx);

        Some(quote! {
          let visitor = #visitor_label{};

          if let XmlEvent::StartElement {name, ..} = reader.peek()?.clone() {
            if let Some(namespace) = name.namespace {
              match namespace.as_str() {
                bad_ns => {
                  let msg = format!("bad field namespace for {}, found {}",
                    name.local_name.as_str(),
                    bad_ns);
                  return Err(msg);
                }
              }
            }
            reader.set_map_value()
          }

          let result = reader.read_inner_value::<#field_type, _>(|reader| {
            if let XmlEvent::EndElement { .. } = *reader.peek()? {
              return visitor.#visitor("");
            }

            if let Ok(XmlEvent::Characters(s)) = reader.next_event() {
              visitor.#visitor(&s)
            } else {
              Err(format!("unable to parse content for {}", #label_name))
            }
          });

          if let Ok(value) = result {
            #action
          }
        })
      };

      let call_struct_visitor = |struct_name, action| {
        Some(quote! {
          reader.set_map_value();
          match #struct_name::deserialize(reader) {
            Ok(value) => {
              #action;
              let _root = reader.next_event();
            },
            Err(msg) => {
              return Err(msg);
            },
          }
        })
      };

      let set_val = quote! { enum_value = Some(#variant_name(value)) };
      let set_opt = quote! { enum_value = Some(#variant_name(Some(value))) };
      let set_vec = quote! {
        match enum_value {
          Some(ref mut v) => match v {
            #variant_name(ref mut v) => v.push(value),
            _ => return Err(String::from("Got sequence of different types"))
          }
          None => {
            enum_value = Some(#variant_name(vec![value]));
          }
        }
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { struct_name } => call_struct_visitor(struct_name, set_val),
        FieldType::FieldTypeOption { data_type } => match *data_type {
          FieldType::FieldTypeStruct { struct_name } => call_struct_visitor(struct_name, set_opt),
          simple_type => call_simple_type_visitor(simple_type, set_opt),
        },
        FieldType::FieldTypeVec { data_type } => match *data_type {
          FieldType::FieldTypeStruct { struct_name } => call_struct_visitor(struct_name, set_vec),
          simple_type => call_simple_type_visitor(simple_type, set_vec),
        },

        simple_type => call_simple_type_visitor(simple_type, set_val),
      })
    })
    .filter_map(|f| f)
    .collect()
}
