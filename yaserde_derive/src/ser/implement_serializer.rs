use crate::common::YaSerdeAttribute;
use crate::ser::namespace::generate_namespaces_definition;
use proc_macro2::Ident;
use proc_macro2::TokenStream;

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
    use xml::writer::XmlEvent;

    impl<'a> YaSerialize<'a> for #name {
      #[allow(unused_variables)]
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>)
        -> Result<(), String> {
        let skip = writer.skip_start_end();

        if !#flatten && !skip {
          let mut child_attributes = vec![];
          let mut child_attributes_namespace = xml::namespace::Namespace::empty();

          let yaserde_label = writer.get_start_event_name().unwrap_or_else(|| #root.to_string());
          let struct_start_event = XmlEvent::start_element(yaserde_label.as_ref())#namespaces_definition;
          #append_attributes

          let event : xml::writer::events::XmlEvent = struct_start_event.into();

          if let xml::writer::events::XmlEvent::StartElement{name, attributes, namespace} = event {
            let mut attributes: Vec<xml::attribute::OwnedAttribute> = attributes.into_owned().to_vec().iter().map(|k| k.to_owned()).collect();
            attributes.extend(child_attributes);

            let all_attributes = attributes.iter().map(|ca| ca.borrow()).collect();

            let mut all_namespaces = namespace.into_owned();
            all_namespaces.extend(&child_attributes_namespace);

            writer.write(xml::writer::events::XmlEvent::StartElement{
              name,
              attributes: std::borrow::Cow::Owned(all_attributes),
              namespace: std::borrow::Cow::Owned(all_namespaces)
            }).map_err(|e| e.to_string())?;
          } else {
            unreachable!()
          }
        }

        #inner_inspector

        if !#flatten && !skip {
          let struct_end_event = XmlEvent::end_element();
          writer.write(struct_end_event).map_err(|e| e.to_string())?;
        }

        Ok(())
      }

      fn serialize_attributes(&self, mut source_attributes: Vec<xml::attribute::OwnedAttribute>, mut source_namespace: xml::namespace::Namespace) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
        let mut child_attributes : Vec<xml::attribute::OwnedAttribute> = vec![];
        let mut child_attributes_namespace = xml::namespace::Namespace::empty();

        let struct_start_event = XmlEvent::start_element("temporary_element_to_generate_attributes")#namespaces_definition;
        #append_attributes
        let event : xml::writer::events::XmlEvent = struct_start_event.into();

        if let xml::writer::events::XmlEvent::StartElement{attributes, namespace, ..} = event {
          source_namespace.extend(&namespace.into_owned());
          source_namespace.extend(&child_attributes_namespace);

          let a: Vec<xml::attribute::OwnedAttribute> = attributes.into_owned().to_vec().iter().map(|k| k.to_owned()).collect();
          source_attributes.extend(a);
          source_attributes.extend(child_attributes);

          Ok((source_attributes, source_namespace))
        } else {
          unreachable!();
        }
      }
    }
  }
}
