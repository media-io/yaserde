
use attribute::*;
use field_type::*;
use quote::Tokens;
use syn::Ident;
use syn::DataStruct;
use proc_macro2::Span;

pub fn serialize(data_struct: &DataStruct, name: &Ident, root: &String) -> Tokens {
  let build_attributes : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if field_attrs.attribute == false {
        return None;
      }

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
            .attr(#label_name, &self.#label)
          }),
        _ => None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  let struct_inspector : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if field_attrs.attribute == true {
        return None;
      }

      let label = field.ident;
      if field_attrs.text == true {
        return Some(quote!(
          let data_event = XmlEvent::characters(&self.#label);
          let _ret = writer.write(data_event);
        ))
      }

      let renamed_label =
        match field_attrs.rename {
          Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
          None => field.ident
        };
      let label_name = renamed_label.unwrap().to_string();

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) =>
          Some(quote!{
            let start_event = XmlEvent::start_element(#label_name);
            let data_event = XmlEvent::characters(&self.#label);
            let end_event = XmlEvent::end_element();
            let _ret = writer.write(start_event);
            let _ret = writer.write(data_event);
            let _ret = writer.write(end_event);
          }),
        Some(FieldType::FieldTypeStruct{..}) =>
          Some(quote!{
            match self.#label.derive_serialize(writer, false) {
              Ok(()) => {},
              Err(msg) => {
                return Err(msg);
              },
            };
          }),
        Some(FieldType::FieldTypeVec{data_type}) => {
          let dt = Box::into_raw(data_type);
          match unsafe{dt.as_ref()} {
            Some(&FieldType::FieldTypeString) => {
              Some(quote!{
                for item in &self.#label {
                  let start_event = XmlEvent::start_element(#label_name);
                  let _ret = writer.write(start_event);

                  let data_event = XmlEvent::characters(item);
                  let _ret = writer.write(data_event);

                  let end_event = XmlEvent::end_element();
                  let _ret = writer.write(end_event);
                }
              })
            },
            Some(&FieldType::FieldTypeStruct{..}) => {
              Some(quote!{
                for item in &self.#label {
                  match item.derive_serialize(writer, false) {
                    Ok(()) => {},
                    Err(msg) => {
                      return Err(msg);
                    },
                  };
                }
              })
            },
            Some(&FieldType::FieldTypeVec{..}) => {unimplemented!();},
            None => {unimplemented!();},
          }
        },
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
      fn derive_serialize<W: Write>(&self, writer: &mut xml::EventWriter<W>, skip_start_end: bool) -> Result<(), String> {
        if !skip_start_end {
          let struct_start_event = XmlEvent::start_element(#root)#build_attributes;
          let _ret = writer.write(struct_start_event);
        }

        #struct_inspector

        if !skip_start_end {
          let struct_end_event = XmlEvent::end_element();
          let _ret = writer.write(struct_end_event);
        }
        Ok(())
      }
    }
  }
}
