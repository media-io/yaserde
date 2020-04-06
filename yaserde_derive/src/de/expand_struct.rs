use crate::attribute::*;
use crate::de::build_default_value::build_default_value;
use crate::field_type::*;
use proc_macro2::{Span, TokenStream};
use std::collections::BTreeMap;
use syn::spanned::Spanned;
use syn::DataStruct;
use syn::Ident;

pub fn parse(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  prefix: &Option<String>,
  namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let namespaces_matches: TokenStream = namespaces
    .iter()
    .map(|(p, ns)| {
      if prefix.as_ref() == Some(p) {
        Some(quote!(#ns => {}))
      } else {
        None
      }
    })
    .filter_map(|x| x)
    .collect();

  let variables: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &get_value_label(&field.ident);
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { struct_name } => build_default_value(
          label,
          &quote! {#struct_name},
          &quote! {#struct_name::default()},
          &field_attrs.default,
        ),
        FieldType::FieldTypeOption { .. } => {
          if let Some(d) = &field_attrs.default {
            let default_function = Ident::new(&d, field.span());

            Some(quote! {
              #[allow(unused_mut, non_snake_case, non_camel_case_types)]
              let mut #label = #default_function();
            })
          } else {
            Some(quote! {
              #[allow(unused_mut, non_snake_case, non_camel_case_types)]
              let mut #label = None;
            })
          }
        }
        FieldType::FieldTypeVec { data_type } => match *data_type {
          FieldType::FieldTypeStruct { ref struct_name } => build_default_value(
            label,
            &quote! {Vec<#struct_name>},
            &quote! {vec![]},
            &field_attrs.default,
          ),
          FieldType::FieldTypeOption { .. } | FieldType::FieldTypeVec { .. } => {
            unimplemented!();
          }
          simple_type => {
            let type_token = get_simple_type_token(&simple_type);

            build_default_value(
              label,
              &quote! {Vec<#type_token>},
              &quote! {vec![]},
              &field_attrs.default,
            )
          }
        },
        simple_type => {
          let type_token = get_simple_type_token(&simple_type);
          build_default_value(
            label,
            &type_token,
            &quote! {#type_token::default()},
            &field_attrs.default,
          )
        }
      })
    })
    .filter_map(|x| x)
    .collect();

  let field_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label_name = field_attrs
        .rename
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

      let struct_visitor = |struct_name: syn::Path| {
        let struct_id: String = struct_name
          .segments
          .iter()
          .map(|s| s.ident.to_string())
          .collect();

        let visitor_label = build_visitor_ident(&label_name, field.span(), Some(&struct_name));

        Some(quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> Visitor<'de> for #visitor_label {
            type Value = #struct_name;

            fn visit_str(self, v: &str) -> Result<Self::Value, String> {
              let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
              let value : Result<#struct_name, String> = yaserde::de::from_str(&content);
              value
            }
          }
        })
      };

      let simple_type_visitor = |simple_type: FieldType| {
        let field_type = get_simple_type_token(&simple_type);
        let visitor = get_simple_type_visitor(&simple_type);
        let visitor_label = build_visitor_ident(&label_name, field.span(), None);

        Some(quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> Visitor<'de> for #visitor_label {
            type Value = #field_type;

            fn #visitor(self, v: &str) -> Result<Self::Value, String> {
              Ok(#field_type::from_str(v).unwrap())
            }
          }
        })
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { struct_name } => struct_visitor(struct_name),
        FieldType::FieldTypeOption { data_type } => match *data_type {
          FieldType::FieldTypeStruct { struct_name } => struct_visitor(struct_name),
          FieldType::FieldTypeOption { .. } | FieldType::FieldTypeVec { .. } => None,
          simple_type => simple_type_visitor(simple_type),
        },
        FieldType::FieldTypeVec { data_type } => match *data_type {
          FieldType::FieldTypeStruct { struct_name } => struct_visitor(struct_name),
          FieldType::FieldTypeOption { .. } | FieldType::FieldTypeVec { .. } => None,
          simple_type => simple_type_visitor(simple_type),
        },
        simple_type => simple_type_visitor(simple_type),
      })
    })
    .filter_map(|x| x)
    .collect();

  let call_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label = &field.ident;
      let value_label = &get_value_label(&field.ident);

      if field_attrs.attribute || field_attrs.flatten {
        return None;
      }

      let label_name = field_attrs
        .rename
        .clone()
        .unwrap_or_else(|| label.as_ref().unwrap().to_string());

      let visit_struct = |struct_name: syn::Path, action: TokenStream| {
        Some(quote! {
          #label_name => {
            reader.set_map_value();
            let value = #struct_name::deserialize(reader)?;
            #value_label #action;
            let _root = reader.next_event();
          }
        })
      };

      let visit_simple = |simple_type: FieldType, action: TokenStream| {
        build_call_visitor(
          &get_simple_type_token(&simple_type),
          &get_simple_type_visitor(&simple_type),
          &action,
          &field_attrs,
          label,
          &namespaces,
          field.span(),
        )
      };

      let visit_sub = |sub_type: Box<FieldType>, action: TokenStream| match *sub_type {
        FieldType::FieldTypeOption { .. } | FieldType::FieldTypeVec { .. } => unimplemented!(),
        FieldType::FieldTypeStruct { struct_name } => visit_struct(struct_name, action),
        simple_type => visit_simple(simple_type, action),
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { struct_name } => visit_struct(struct_name, quote! {= value}),
        FieldType::FieldTypeOption { data_type } => visit_sub(data_type, quote! {= Some(value)}),
        FieldType::FieldTypeVec { data_type } => visit_sub(data_type, quote! {.push(value)}),
        simple_type => visit_simple(simple_type, quote! {= value}),
      })
    })
    .filter_map(|x| x)
    .collect();

  let call_flatten_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let value_label = &get_value_label(&field.ident);

      if field_attrs.attribute || !field_attrs.flatten {
        return None;
      }

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeStruct { .. } => Some(quote! {
          #value_label = yaserde::de::from_str(&unused_xml_elements)?;
        }),
        FieldType::FieldTypeOption { data_type } => match *data_type {
          FieldType::FieldTypeStruct { .. } => Some(quote! {
            #value_label = yaserde::de::from_str(&unused_xml_elements).ok();
          }),
          field_type => unimplemented!("\"flatten\" is not implemented for {:?}", field_type),
        },
        field_type => unimplemented!("\"flatten\" is not implemented for {:?}", field_type),
      })
    })
    .filter_map(|x| x)
    .collect();

  let attributes_loading: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if !field_attrs.attribute {
        return None;
      }

      let label = &get_value_label(&field.ident);

      let label_name = field_attrs
        .rename
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

      let visitor_label = build_visitor_ident(&label_name, field.span(), None);

      let visit = |action: &TokenStream, visitor: &TokenStream, visitor_label: &Ident| {
        Some(quote! {
          for attr in attributes {
            if attr.name.local_name == #label_name {
              let visitor = #visitor_label{};
              let value = visitor.#visitor(&attr.value)?;
              #label #action;
            }
          }
        })
      };

      let visit_string = || {
        Some(quote! {
          for attr in attributes {
            if attr.name.local_name == #label_name {
              #label = attr.value.to_owned();
            }
          }
        })
      };

      let visit_struct = |struct_name: syn::Path, action: TokenStream| {
        visit(
          &action,
          &quote! {visit_str},
          &build_visitor_ident(&label_name, field.span(), Some(&struct_name)),
        )
      };

      let visit_simple = |simple_type: FieldType, action: TokenStream| {
        visit(
          &action,
          &get_simple_type_visitor(&simple_type),
          &visitor_label,
        )
      };

      let visit_sub = |sub_type: Box<FieldType>, action: TokenStream| match *sub_type {
        FieldType::FieldTypeOption { .. } | FieldType::FieldTypeVec { .. } => unimplemented!(),
        FieldType::FieldTypeStruct { struct_name } => visit_struct(struct_name, action),
        simple_type => visit_simple(simple_type, action),
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeString => visit_string(),
        FieldType::FieldTypeOption { data_type } => visit_sub(data_type, quote! {= Some(value)}),
        FieldType::FieldTypeVec { .. } => unimplemented!(),
        FieldType::FieldTypeStruct { struct_name } => visit_struct(struct_name, quote! {= value}),
        simple_type => visit_simple(simple_type, quote! {= value}),
      })
    })
    .filter_map(|x| x)
    .collect();

  let set_text: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &get_value_label(&field.ident);
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      let set_text = |action: &TokenStream| {
        if field_attrs.text {
          Some(quote! {#label = #action;})
        } else {
          None
        }
      };

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeString => set_text(&quote! {text_content.to_owned()}),
        FieldType::FieldTypeStruct { .. }
        | FieldType::FieldTypeOption { .. }
        | FieldType::FieldTypeVec { .. } => None,
        simple_type => {
          let type_token = get_simple_type_token(&simple_type);
          set_text(&quote! {#type_token::from_str(text_content).unwrap()})
        }
      })
    })
    .filter_map(|x| x)
    .collect();

  let struct_builder: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &field.ident;
      let value_label = &get_value_label(&field.ident);

      get_field_type(field).map(|_| {
        quote! { #label: #value_label, }
      })
    })
    .filter_map(|x| x)
    .collect();

  let (init_unused, write_unused, visit_unused) = if call_flatten_visitors.is_empty() {
    (None, None, None)
  } else {
    build_code_for_unused_xml_events(&call_flatten_visitors)
  };

  quote! {
    use xml::reader::{XmlEvent, EventReader};
    use xml::writer::EventWriter;
    use yaserde::Visitor;
    #[allow(unknown_lints, unused_imports)]
    use std::str::FromStr;
    use log::debug;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let (named_element, struct_namespace) =
          if let XmlEvent::StartElement{name, ..} = reader.peek()?.to_owned() {
            (name.local_name.to_owned(), name.namespace.clone())
          } else {
            (String::from(#root), None)
          };
        debug!("Struct: start to parse {:?}", named_element);

        if reader.depth() == 0 {
          if let Some(ref namespace) = struct_namespace {
            match namespace.as_str() {
              #namespaces_matches
              bad_ns => {
                let msg = format!("bad namespace for {}, found {}", named_element, bad_ns);
                return Err(msg);
              }
            }
          }
        }

        #variables
        #field_visitors
        #init_unused

        let mut depth = 0;

        loop {
          let event = reader.peek()?.to_owned();

          match event {
            XmlEvent::StartElement{ref name, ref attributes, ..} => {
              let mut skipped = false;

              match name.local_name.as_str() {
                #call_visitors
                named_element => {
                  let event = reader.next_event()?;
                  #write_unused

                  if depth > 0 { // Don't skip root element
                    skipped = true;
                    reader.skip_element(|event| {
                      #write_unused
                    })?;
                  }
                }
                // name => {
                //   return Err(format!("unknown key {}", name))
                // }
              }
              if depth == 0 && !skipped { // Look for attributes only at element start
                #attributes_loading
              }
              depth += 1;
            }
            XmlEvent::EndElement{ref name} => {
              if name.local_name == named_element {
                #write_unused
                break;
              }
              let event = reader.next_event()?;
              #write_unused
              depth -= 1;
            }
            XmlEvent::Characters(ref text_content) => {
              #set_text
              let event = reader.next_event()?;
              #write_unused
            }
            event => {
              return Err(format!("unknown event {:?}", event))
            }
          }
        }

        #visit_unused

        Ok(#name{#struct_builder})
      }
    }
  }
}

fn build_call_visitor(
  field_type: &TokenStream,
  visitor: &TokenStream,
  action: &TokenStream,
  field_attrs: &YaSerdeAttribute,
  label: &Option<Ident>,
  namespaces: &BTreeMap<String, String>,
  span: Span,
) -> Option<TokenStream> {
  let prefix = field_attrs.prefix.clone();

  // let label = &field.ident;
  let value_label = get_value_label(label);
  let label_name = field_attrs
    .rename
    .clone()
    .unwrap_or_else(|| label.as_ref().unwrap().to_string());

  let visitor_label = build_visitor_ident(&label_name, span, None);

  let namespaces_matches: TokenStream = namespaces
    .iter()
    .map(|(p, ns)| {
      if prefix == Some(p.to_string()) {
        Some(quote!(#ns => {}))
      } else {
        None
      }
    })
    .filter_map(|x| x)
    .collect();

  Some(quote! {
    #label_name => {
      let visitor = #visitor_label{};

      if let XmlEvent::StartElement {name, ..} = reader.peek()?.clone() {
        if let Some(namespace) = name.namespace {
          match namespace.as_str() {
            #namespaces_matches
            bad_ns => {
              let msg = format!("bad field namespace for {}, found {}", name.local_name.as_str(), bad_ns);
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
        #value_label#action
      }
    }
  })
}

fn get_value_label(ident: &Option<syn::Ident>) -> Option<syn::Ident> {
  ident
    .clone()
    .map(|ident| syn::Ident::new(&format!("__{}_value", ident.to_string()), ident.span()))
}

fn build_visitor_ident(label: &str, span: Span, struct_name: Option<&syn::Path>) -> Ident {
  let struct_id = struct_name.map_or_else(
    || "".to_string(),
    |struct_name| {
      struct_name
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect()
    },
  );

  Ident::new(
    &format!("__Visitor_{}_{}", label.replace(".", "_"), struct_id),
    span,
  )
}

fn build_code_for_unused_xml_events(
  call_flatten_visitors: &TokenStream,
) -> (
  Option<TokenStream>,
  Option<TokenStream>,
  Option<TokenStream>,
) {
  (
    Some(quote! {
      let mut buf = Vec::new();
      let mut writer = Some(EventWriter::new(&mut buf));
    }),
    Some(quote! {
      if let Some(ref mut w) = writer {
        if w.write(event.as_writer_event().unwrap()).is_err() {
          writer = None;
        }
      }
    }),
    Some(quote! {
      if writer.is_some() {
        let unused_xml_elements = String::from_utf8(buf).unwrap();
        #call_flatten_visitors
      }
    }),
  )
}
