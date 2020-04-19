use crate::attribute::YaSerdeAttribute;
use crate::ser::namespace::generate_namespaces_definition;
use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub fn implement_deserializer(
  name: &Ident,
  root: &str,
  attributes: &YaSerdeAttribute,
  attributes_inspector: TokenStream,
  inner_inspector: TokenStream,
) -> TokenStream {
  let namespaces_definition = generate_namespaces_definition(attributes);
  let flatten = attributes.flatten;

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>)
        -> Result<(), String> {
        let skip = writer.skip_start_end();

        if !#flatten && !skip {
          let yaserde_label = writer.get_start_event_name().unwrap_or_else(|| #root.to_string());
          let struct_start_event = XmlEvent::start_element(yaserde_label.as_ref())#namespaces_definition;
          #attributes_inspector
          writer.write(struct_start_event).map_err(|e| e.to_string())?;
        }

        #inner_inspector

        if !#flatten && !skip {
          let struct_end_event = XmlEvent::end_element();
          writer.write(struct_end_event).map_err(|e| e.to_string())?;
        }

        Ok(())
      }
    }
  }
}
