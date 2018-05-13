
use attribute::*;
use field_type::*;
use quote::Tokens;
use syn::Ident;
use syn::DataStruct;
use proc_macro2::Span;
use std::string::ToString;

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
        Some(FieldType::FieldTypeStruct{struct_name: _struct_name}) =>
          Some(quote!{
            .attr(#label_name, &*{
              use std::mem;
              match yaserde::ser::to_string_content(&self.#label) {
                Ok(value) => {
                  unsafe {
                    let ret : &'static str = mem::transmute(&value as &str);
                    mem::forget(value);
                    ret
                  }
                },
                Err(msg) => return Err("Unable to serialize content".to_owned()),
              }
            })
          }),
        _ => {
          None
        }
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
            writer.set_skip_start_end(false);
            match self.#label.serialize(writer) {
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
                  writer.set_skip_start_end(false);
                  match item.serialize(writer) {
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
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        error!("Struct: start to expand {:?}", #root);

        if !writer.skip_start_end() {
          let struct_start_event = XmlEvent::start_element(#root)#build_attributes;
          let _ret = writer.write(struct_start_event);
        }

        #struct_inspector

        if !writer.skip_start_end() {
          let struct_end_event = XmlEvent::end_element();
          let _ret = writer.write(struct_end_event);
        }
        Ok(())
      }
    }
  }
}
