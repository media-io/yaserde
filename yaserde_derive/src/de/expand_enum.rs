use crate::common::{Field, YaSerdeAttribute, YaSerdeField};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, Ident};

pub fn parse(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  root_attributes: &YaSerdeAttribute,
) -> TokenStream {
  let namespaces_matching = root_attributes.get_namespace_matching(
    &None,
    quote!(enum_namespace),
    quote!(named_element),
    true,
  );

  let match_to_enum: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| parse_variant(variant, name))
    .flatten()
    .collect();

  let flatten = root_attributes.flatten;

  quote! {
    impl ::yaserde::YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
      ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, enum_namespace) =
          if let ::yaserde::xml::reader::XmlEvent::StartElement{ name, .. } = reader.peek()?.to_owned() {
            (name.local_name.to_owned(), name.namespace.clone())
          } else {
            (::std::string::String::from(#root), ::std::option::Option::None)
          };

        let start_depth = reader.depth();
        ::yaserde::log::debug!("Enum {} @ {}: start to parse {:?}", stringify!(#name), start_depth, named_element);

        #namespaces_matching

        #[allow(unused_assignments, unused_mut)]
        let mut enum_value = ::std::option::Option::None;

        loop {
          let event = reader.peek()?.to_owned();
          ::yaserde::log::trace!("Enum {} @ {}: matching {:?}", stringify!(#name), start_depth, event);
          match event {
            ::yaserde::xml::reader::XmlEvent::StartElement { ref name, ref attributes, .. } => {
              match name.local_name.as_str() {
                #match_to_enum
                _named_element => {
                  let _root = reader.next_event();
                }
              }

              if let ::yaserde::xml::reader::XmlEvent::Characters(content) = reader.peek()?.to_owned() {
                match content.as_str() {
                  #match_to_enum
                  _ => {}
                }
              }
            }
            ::yaserde::xml::reader::XmlEvent::EndElement { ref name } => {
              if name.local_name == named_element {
                break;
              }
              let _root = reader.next_event();
            }
            ::yaserde::xml::reader::XmlEvent::Characters(ref text_content) => {
              let _root = reader.next_event();
            }
            ::yaserde::xml::reader::XmlEvent::EndDocument => {
              if #flatten {
                break;
              }

              return ::std::result::Result::Err(
                ::std::format!("End of document, missing some content ?"),
              );
            }
            event => {
              return ::std::result::Result::Err(::std::format!("unknown event {:?}", event))
            }
          }
        }

        ::yaserde::log::debug!("Enum {} @ {}: success", stringify!(#name), start_depth);
        match enum_value {
          ::std::option::Option::Some(value) => ::std::result::Result::Ok(value),
          ::std::option::Option::None => {
            ::std::result::Result::Ok(<#name as ::std::default::Default>::default())
          },
        }
      }
    }
  }
}

fn parse_variant(variant: &syn::Variant, name: &Ident) -> Option<TokenStream> {
  let xml_element_name = YaSerdeAttribute::parse(&variant.attrs).xml_element_name(&variant.ident);

  let variant_name = {
    let label = &variant.ident;
    quote! { #name::#label }
  };

  match variant.fields {
    Fields::Unit => Some(quote! {
      #xml_element_name => {
        enum_value = ::std::option::Option::Some(#variant_name);
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
    .map(|field| YaSerdeField::new(field.clone()))
    .enumerate()
    .filter_map(|(idx, field)| {
      let visitor_label = Ident::new(&format!("__Visitor_{}", idx), field.get_span());

      let make_visitor = |visitor: &Ident, field_type: &TokenStream, fn_body: &TokenStream| {
        quote! {
          #[allow(non_snake_case, non_camel_case_types)]
          struct #visitor_label;
          impl<'de> ::yaserde::Visitor<'de> for #visitor_label {
            type Value = #field_type;

            fn #visitor(
              self,
              v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
              #fn_body
            }
          }
        }
      };

      let simple_type_visitor = |simple_type: Field| {
        let visitor = simple_type.get_simple_type_visitor();
        let field_type = simple_type.into();

        make_visitor(
          &visitor,
          &field_type,
          &quote! { ::std::result::Result::Ok(#field_type::from_str(v).unwrap()) },
        )
      };

      match field.get_type() {
        Field::FieldStruct { struct_name } => {
          let struct_id: String = struct_name
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();

          Some(make_visitor(
            &Ident::new("visit_str", Span::call_site()),
            &quote! { #struct_name },
            &quote! {
              let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
              let value: ::std::result::Result<#struct_name, ::std::string::String> =
                ::yaserde::de::from_str(&content);
              value
            },
          ))
        }
        Field::FieldOption { data_type } | Field::FieldVec { data_type } => match *data_type {
          Field::FieldStruct { .. } => None,
          simple_type => Some(simple_type_visitor(simple_type)),
        },
        simple_type => Some(simple_type_visitor(simple_type)),
      }
    })
    .collect()
}

fn build_unnamed_visitor_calls(
  fields: &syn::FieldsUnnamed,
  variant_name: &TokenStream,
) -> TokenStream {
  fields
    .unnamed
    .iter()
    .map(|field| YaSerdeField::new(field.clone()))
    .enumerate()
    .map(|(idx, field)| {
      let visitor_label = Ident::new(&format!("__Visitor_{}", idx), field.get_span());

      let call_simple_type_visitor = |simple_type: Field, action| {
        let visitor = simple_type.get_simple_type_visitor();
        let field_type: TokenStream = simple_type.into();

        let label_name = format!("field_{}", idx);

        Some(quote! {
          let visitor = #visitor_label{};

          let result = reader.read_inner_value::<#field_type, _>(|reader| {
            if let ::yaserde::xml::reader::XmlEvent::EndElement { .. } = *reader.peek()? {
              return visitor.#visitor("");
            }

            if let ::std::result::Result::Ok(::yaserde::xml::reader::XmlEvent::Characters(s))
              = reader.next_event()
            {
              visitor.#visitor(&s)
            } else {
              ::std::result::Result::Err(
                ::std::format!("unable to parse content for {}", #label_name),
              )
            }
          });

          if let ::std::result::Result::Ok(value) = result {
            #action
          }
        })
      };

      let call_struct_visitor = |struct_name, action| {
        Some(quote! {
          match <#struct_name as ::yaserde::YaDeserialize>::deserialize(reader) {
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

      let set_val = quote! {
        enum_value = ::std::option::Option::Some(#variant_name(value))
      };
      let set_opt = quote! {
        enum_value = ::std::option::Option::Some(#variant_name(::std::option::Option::Some(value)))
      };
      let set_vec = quote! {
        match enum_value {
          Some(ref mut v) => match v {
            #variant_name(ref mut v) => v.push(value),
            _ => {
              return ::std::result::Result::Err(
                ::std::string::String::from("Got sequence of different types"),
              );
            }
          }
          None => {
            enum_value = ::std::option::Option::Some(#variant_name(vec![value]));
          }
        }
      };

      match field.get_type() {
        Field::FieldStruct { struct_name } => call_struct_visitor(struct_name, set_val),
        Field::FieldOption { data_type } => match *data_type {
          Field::FieldStruct { struct_name } => call_struct_visitor(struct_name, set_opt),
          simple_type => call_simple_type_visitor(simple_type, set_opt),
        },
        Field::FieldVec { data_type } => match *data_type {
          Field::FieldStruct { struct_name } => call_struct_visitor(struct_name, set_vec),
          simple_type => call_simple_type_visitor(simple_type, set_vec),
        },

        simple_type => call_simple_type_visitor(simple_type, set_val),
      }
    })
    .flatten()
    .collect()
}
