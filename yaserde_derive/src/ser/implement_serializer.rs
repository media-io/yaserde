use crate::common::YaSerdeAttribute;
use crate::ser::namespace::generate_namespaces_definition;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn implement_serializer(
  name: &Ident,
  root: &str,
  attributes: &YaSerdeAttribute,
  append_attributes: TokenStream,
  inner_inspector: TokenStream,
) -> TokenStream {
  let namespaces_definition = generate_namespaces_definition(attributes);
  let flatten = attributes.flatten;

  quote! {
    impl ::yaserde::YaSerialize for #name {
      #[allow(unused_variables)]
      fn serialize<W: ::std::io::Write>(
        &self,
        writer: &mut ::yaserde::ser::Serializer<W>,
      ) -> ::std::result::Result<(), ::std::string::String> {
        let skip = writer.skip_start_end();

        if !#flatten && !skip {
          let mut child_attributes = ::std::vec![];
          let mut child_attributes_namespace = ::yaserde::xml::namespace::Namespace::empty();

          let yaserde_label = writer.get_start_event_name().unwrap_or_else(|| #root.to_string());
          let struct_start_event =
            ::yaserde::xml::writer::XmlEvent::start_element(yaserde_label.as_ref()) #namespaces_definition;
          #append_attributes

          let event: ::yaserde::xml::writer::events::XmlEvent = struct_start_event.into();

          if let ::yaserde::xml::writer::events::XmlEvent::StartElement {
            name,
            attributes,
            namespace,
          } = event {
            let mut attributes: ::std::vec::Vec<::yaserde::xml::attribute::OwnedAttribute> =
              attributes.into_owned().to_vec().iter().map(|k| k.to_owned()).collect();
            attributes.extend(child_attributes);

            let all_attributes = attributes.iter().map(|ca| ca.borrow()).collect();

            let mut all_namespaces = namespace.into_owned();
            all_namespaces.extend(&child_attributes_namespace);

            writer.write(::yaserde::xml::writer::events::XmlEvent::StartElement{
              name,
              attributes: ::std::borrow::Cow::Owned(all_attributes),
              namespace: ::std::borrow::Cow::Owned(all_namespaces)
            }).map_err(|e| e.to_string())?;
          } else {
            unreachable!()
          }
        }

        #inner_inspector

        if !#flatten && !skip {
          let struct_end_event = ::yaserde::xml::writer::XmlEvent::end_element();
          writer.write(struct_end_event).map_err(|e| e.to_string())?;
        }

        ::std::result::Result::Ok(())
      }

      fn serialize_attributes(
        &self,
        mut source_attributes: ::std::vec::Vec<::yaserde::xml::attribute::OwnedAttribute>,
        mut source_namespace: ::yaserde::xml::namespace::Namespace,
      ) -> ::std::result::Result<
        (::std::vec::Vec<::yaserde::xml::attribute::OwnedAttribute>, ::yaserde::xml::namespace::Namespace),
        ::std::string::String
      > {
        let mut child_attributes = ::std::vec::Vec::<::yaserde::xml::attribute::OwnedAttribute>::new();
        let mut child_attributes_namespace = ::yaserde::xml::namespace::Namespace::empty();

        let struct_start_event =
          ::yaserde::xml::writer::XmlEvent::start_element("temporary_element_to_generate_attributes")
          #namespaces_definition;

        #append_attributes
        let event: ::yaserde::xml::writer::events::XmlEvent = struct_start_event.into();

        if let ::yaserde::xml::writer::events::XmlEvent::StartElement { attributes, namespace, .. } = event {
          source_namespace.extend(&namespace.into_owned());
          source_namespace.extend(&child_attributes_namespace);

          let a: ::std::vec::Vec<::yaserde::xml::attribute::OwnedAttribute> =
            attributes.into_owned().to_vec().iter().map(|k| k.to_owned()).collect();
          source_attributes.extend(a);
          source_attributes.extend(child_attributes);

          ::std::result::Result::Ok((source_attributes, source_namespace))
        } else {
          unreachable!();
        }
      }
    }
  }
}
