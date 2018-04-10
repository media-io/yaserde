
use de::attribute::*;
use de::field_type::*;
use quote::Tokens;
use syn::Ident;
use syn::DataStruct;
use proc_macro2::Span;

pub fn serialize(data_struct: &DataStruct, name: &Ident, root: &String) -> Tokens {
  let struct_inspector : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let renamed_label =
        match field_attrs.rename {
          Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
          None => field.ident
        };
      let label = field.ident;
      let label_name = renamed_label.unwrap().to_string();

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) =>
          Some(quote!{
            let start_event = xml::writer::events::XmlEvent::start_element(#label_name);
            let data_event = xml::writer::events::XmlEvent::characters(&self.#label);
            let end_event = xml::writer::events::XmlEvent::end_element();
            let _ret = writer.write(start_event);
            let _ret = writer.write(data_event);
            let _ret = writer.write(end_event);
          }),
        Some(FieldType::FieldTypeStruct{..}) =>
          Some(quote!{
            let start_event = xml::writer::events::XmlEvent::start_element(#label_name);
            let end_event = xml::writer::events::XmlEvent::end_element();
            let _ret = writer.write(start_event);
            let _ret = writer.write(end_event);
          }),
        Some(FieldType::FieldTypeVec) =>
          Some(quote!{
            for item in &self.#label {
              let start_event = xml::writer::events::XmlEvent::start_element(#label_name);
              let _ret = writer.write(start_event);

              let data_event = xml::writer::events::XmlEvent::characters(item);
              let _ret = writer.write(data_event);

              let end_event = xml::writer::events::XmlEvent::end_element();
              let _ret = writer.write(end_event);
            }
          }),
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  // println!("{:?}", struct_inspector);

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn derive_serialize<W: Write>(&self, writer: &mut xml::EventWriter<W>, parent_attributes: Option<&Vec<xml::attribute::OwnedAttribute>>) -> Result<(), String> {
        let struct_start_event = xml::writer::events::XmlEvent::start_element(#root);
        let _ret = writer.write(struct_start_event);

        #struct_inspector

        let struct_end_event = xml::writer::events::XmlEvent::end_element();
        let _ret = writer.write(struct_end_event);
        Ok(())
      }
    }
  }
}
