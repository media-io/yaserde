use crate::common::{Field, YaSerdeAttribute, YaSerdeField};
use crate::de::build_default_value::build_default_value;
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataStruct, Ident};

pub fn parse(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  root_attributes: &YaSerdeAttribute,
) -> TokenStream {
  let namespaces_matching = root_attributes.get_namespace_matching(
    &None,
    quote!(struct_namespace),
    quote!(named_element),
    true,
  );

  let variables: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .map(|field| match field.get_type() {
      Field::FieldStruct { struct_name } => build_default_value(
        &field,
        Some(quote!(#struct_name)),
        quote!(<#struct_name as ::std::default::Default>::default()),
      ),
      Field::FieldOption { .. } => {
        build_default_value(&field, None, quote!(::std::option::Option::None))
      }
      Field::FieldVec { data_type } => match *data_type {
        Field::FieldStruct { ref struct_name } => build_default_value(
          &field,
          Some(quote!(::std::vec::Vec<#struct_name>)),
          quote!(::std::vec![]),
        ),
        Field::FieldOption { .. } | Field::FieldVec { .. } => {
          unimplemented!();
        }
        simple_type => {
          let type_token: TokenStream = simple_type.into();

          build_default_value(
            &field,
            Some(quote!(::std::vec::Vec<#type_token>)),
            quote!(::std::vec![]),
          )
        }
      },
      simple_type => {
        let type_token: TokenStream = simple_type.into();
        let value_builder = quote!(<#type_token as ::std::default::Default>::default());

        build_default_value(&field, Some(type_token), value_builder)
      }
    })
    .flatten()
    .collect();

  let field_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .map(|field| {
      let struct_visitor = |struct_name: syn::Path| {
        let struct_id: String = struct_name
          .segments
          .iter()
          .map(|s| s.ident.to_string())
          .collect();

        let visitor_label = field.get_visitor_ident(Some(&struct_name));

        Some(quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> ::yaserde::Visitor<'de> for #visitor_label {
            type Value = #struct_name;

            fn visit_str(
              self,
              v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
              let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
              ::yaserde::de::from_str(&content)
            }
          }
        })
      };

      let simple_type_visitor = |simple_type: Field| {
        let visitor = simple_type.get_simple_type_visitor();
        let visitor_label = field.get_visitor_ident(None);
        let field_type = TokenStream::from(simple_type);

        let map_if_bool = if field_type.to_string() == "bool" {
          quote!(match v {
            "1" => "true",
            "0" => "false",
            _ => v,
          })
        } else {
          quote!(v)
        };

        Some(quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> ::yaserde::Visitor<'de> for #visitor_label {
            type Value = #field_type;

            fn #visitor(
              self,
              v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
              ::std::result::Result::Ok(#field_type::from_str(#map_if_bool).unwrap())
            }
          }
        })
      };

      match field.get_type() {
        Field::FieldStruct { struct_name } => struct_visitor(struct_name),
        Field::FieldOption { data_type } => match *data_type {
          Field::FieldStruct { struct_name } => struct_visitor(struct_name),
          Field::FieldOption { .. } | Field::FieldVec { .. } => None,
          simple_type => simple_type_visitor(simple_type),
        },
        Field::FieldVec { data_type } => match *data_type {
          Field::FieldStruct { struct_name } => struct_visitor(struct_name),
          Field::FieldOption { .. } | Field::FieldVec { .. } => None,
          simple_type => simple_type_visitor(simple_type),
        },
        simple_type => simple_type_visitor(simple_type),
      }
    })
    .flatten()
    .collect();

  let call_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .filter(|field| !field.is_attribute() || !field.is_flatten())
    .map(|field| {
      let value_label = field.get_value_label();
      let label_name = field.renamed_label_without_namespace();

      let visit_struct = |struct_name: syn::Path, action: TokenStream| {
        Some(quote! {
          #label_name => {
            if depth == 0 {
              // Don't count current struct's StartElement as substruct's StartElement
              let _root = reader.next_event();
            }
            if let Ok(::yaserde::xml::reader::XmlEvent::StartElement { .. }) = reader.peek() {
              // If substruct's start element found then deserialize substruct
              let value = <#struct_name as ::yaserde::YaDeserialize>::deserialize(reader)?;
              #value_label #action;
            }
          }
        })
      };

      let visit_simple = |simple_type: Field, action: TokenStream| {
        let field_visitor = simple_type.get_simple_type_visitor();
        let field_type = TokenStream::from(simple_type);
        build_call_visitor(
          &field_type,
          &field_visitor,
          &action,
          &field,
          &root_attributes,
        )
      };

      let visit_sub = |sub_type: Box<Field>, action: TokenStream| match *sub_type {
        Field::FieldOption { .. } | Field::FieldVec { .. } => unimplemented!(),
        Field::FieldStruct { struct_name } => visit_struct(struct_name, action),
        simple_type => visit_simple(simple_type, action),
      };

      match field.get_type() {
        Field::FieldStruct { struct_name } => visit_struct(struct_name, quote! { = value }),
        Field::FieldOption { data_type } => {
          visit_sub(data_type, quote! { = ::std::option::Option::Some(value) })
        }
        Field::FieldVec { data_type } => visit_sub(data_type, quote! { .push(value) }),
        simple_type => visit_simple(simple_type, quote! { = value }),
      }
    })
    .flatten()
    .collect();

  let call_flatten_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .filter(|field| !field.is_attribute() && field.is_flatten())
    .map(|field| {
      let value_label = field.get_value_label();

      match field.get_type() {
        Field::FieldStruct { .. } => Some(quote! {
          #value_label = ::yaserde::de::from_str(&unused_xml_elements)?;
        }),
        Field::FieldOption { data_type } => match *data_type {
          Field::FieldStruct { .. } => Some(quote! {
            #value_label = ::yaserde::de::from_str(&unused_xml_elements).ok();
          }),
          field_type => unimplemented!(r#""flatten" is not implemented for {:?}"#, field_type),
        },
        field_type => unimplemented!(r#""flatten" is not implemented for {:?}"#, field_type),
      }
    })
    .flatten()
    .collect();

  let attributes_loading: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .filter(|field| field.is_attribute())
    .map(|field| {
      let label = field.get_value_label();
      let label_name = field.renamed_label_without_namespace();
      let visitor_label = build_visitor_ident(&label_name, field.get_span(), None);

      let visit = |action: &TokenStream, visitor: &Ident, visitor_label: &Ident| {
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
          &Ident::new("visit_str", Span::call_site()),
          &build_visitor_ident(&label_name, field.get_span(), Some(&struct_name)),
        )
      };

      let visit_simple = |simple_type: Field, action: TokenStream| {
        visit(
          &action,
          &simple_type.get_simple_type_visitor(),
          &visitor_label,
        )
      };

      let visit_sub = |sub_type: Box<Field>, action: TokenStream| match *sub_type {
        Field::FieldOption { .. } | Field::FieldVec { .. } => unimplemented!(),
        Field::FieldStruct { struct_name } => visit_struct(struct_name, action),
        simple_type => visit_simple(simple_type, action),
      };

      match field.get_type() {
        Field::FieldString => visit_string(),
        Field::FieldOption { data_type } => {
          visit_sub(data_type, quote! { = ::std::option::Option::Some(value) })
        }
        Field::FieldVec { .. } => unimplemented!(),
        Field::FieldStruct { struct_name } => visit_struct(struct_name, quote! { = value }),
        simple_type => visit_simple(simple_type, quote! { = value }),
      }
    })
    .flatten()
    .collect();

  let set_text: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .map(|field| {
      let label = field.get_value_label();

      let set_text = |action: &TokenStream| {
        if field.is_text_content() {
          Some(quote! { #label = #action; })
        } else {
          None
        }
      };

      match field.get_type() {
        Field::FieldString => set_text(&quote! { text_content.to_owned() }),
        Field::FieldOption { data_type } => match *data_type {
          Field::FieldString => set_text(
            &quote! { if text_content.is_empty() { None } else { Some(text_content.to_owned()) }},
          ),
          _ => None,
        },
        Field::FieldStruct { .. } | Field::FieldVec { .. } => None,
        simple_type => {
          let type_token = TokenStream::from(simple_type);
          set_text(&quote! { #type_token::from_str(text_content).unwrap() })
        }
      }
    })
    .flatten()
    .collect();

  let struct_builder: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .map(|field| {
      let label = &field.label();
      let value_label = field.get_value_label();

      quote! { #label: #value_label, }
    })
    .collect();

  let (init_unused, write_unused, visit_unused) = if call_flatten_visitors.is_empty() {
    (None, None, None)
  } else {
    build_code_for_unused_xml_events(&call_flatten_visitors)
  };

  let flatten = root_attributes.flatten;

  quote! {
    impl ::yaserde::YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
      ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
          if let ::yaserde::xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned() {
            (name.local_name.to_owned(), name.namespace.clone())
          } else {
            (::std::string::String::from(#root), ::std::option::Option::None)
          };
        let start_depth = reader.depth();
        ::yaserde::log::debug!("Struct {} @ {}: start to parse {:?}", stringify!(#name), start_depth,
               named_element);

        if reader.depth() == 0 {
          #namespaces_matching
        }

        #variables
        #field_visitors
        #init_unused

        let mut depth = 0;

        loop {
          let event = reader.peek()?.to_owned();
          ::yaserde::log::trace!(
            "Struct {} @ {}: matching {:?}",
            stringify!(#name), start_depth, event,
          );
          match event {
            ::yaserde::xml::reader::XmlEvent::StartElement{ref name, ref attributes, ..} => {
              if depth == 0 && name.local_name == #root {
                // Consume root element. We must do this first. In the case it shares a name with a child element, we don't
                // want to prematurely match the child element below.
                let event = reader.next_event()?;
                #write_unused
              } else {
                match name.local_name.as_str() {
                  #call_visitors
                  _ => {
                    let event = reader.next_event()?;
                    #write_unused

                    if depth > 0 { // Don't skip root element
                      reader.skip_element(|event| {
                        #write_unused
                      })?;
                    }
                  }
                }
              }
              if depth == 0 { // Look for attributes only at element start
                #attributes_loading
              }
              depth += 1;
            }
            ::yaserde::xml::reader::XmlEvent::EndElement { ref name } => {
              if name.local_name == named_element {
                #write_unused
                break;
              }
              let event = reader.next_event()?;
              #write_unused
              depth -= 1;
            }
            ::yaserde::xml::reader::XmlEvent::EndDocument => {
              if #flatten {
                break;
              }
            }
            ::yaserde::xml::reader::XmlEvent::Characters(ref text_content) => {
              #set_text
              let event = reader.next_event()?;
              #write_unused
            }
            event => {
              return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
            }
          }
        }

        #visit_unused

        ::yaserde::log::debug!("Struct {} @ {}: success", stringify!(#name), start_depth);
        ::std::result::Result::Ok(#name{#struct_builder})
      }
    }
  }
}

fn build_call_visitor(
  field_type: &TokenStream,
  visitor: &Ident,
  action: &TokenStream,
  field: &YaSerdeField,
  root_attributes: &YaSerdeAttribute,
) -> Option<TokenStream> {
  let value_label = field.get_value_label();
  let label_name = field.renamed_label_without_namespace();
  let visitor_label = build_visitor_ident(&label_name, field.get_span(), None);

  let namespaces_matching = field.get_namespace_matching(
    root_attributes,
    quote!(name.namespace.as_ref()),
    quote!(name.local_name.as_str()),
  );

  Some(quote! {
    #label_name => {
      let visitor = #visitor_label{};

      #namespaces_matching

      let result = reader.read_inner_value::<#field_type, _>(|reader| {
        if let ::std::result::Result::Ok(::yaserde::xml::reader::XmlEvent::Characters(s)) = reader.peek() {
          let val = visitor.#visitor(&s);
          let _event = reader.next_event()?;
          val
        } else {
          ::std::result::Result::Err(::std::format!("unable to parse content for {}", #label_name))
        }
      });

      if let ::std::result::Result::Ok(value) = result {
        #value_label#action
      }
    }
  })
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
    &format!(
      "__Visitor_{}_{}",
      label.replace(".", "_").to_camel_case(),
      struct_id
    ),
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
      let mut buf = ::std::vec![];
      let mut writer = ::std::option::Option::Some(::yaserde::xml::writer::EventWriter::new(&mut buf));
    }),
    Some(quote! {
      if let ::std::option::Option::Some(ref mut w) = writer {
        if w.write(event.as_writer_event().unwrap()).is_err() {
          writer = ::std::option::Option::None;
        }
      }
    }),
    Some(quote! {
      if writer.is_some() {
        let unused_xml_elements = ::std::string::String::from_utf8(buf).unwrap();
        #call_flatten_visitors
      }
    }),
  )
}
