use proc_macro2::{Ident, Span, TokenStream};

pub fn enclose_formatted_characters(label: &Ident, label_name: String) -> TokenStream {
  quote! {
    let start_event = XmlEvent::start_element(#label_name);
    let _ret = writer.write(start_event);

    let yas_value = format!("{}", &self.#label);
    let data_event = XmlEvent::characters(&yas_value);
    let _ret = writer.write(data_event);

    let end_event = XmlEvent::end_element();
    let _ret = writer.write(end_event);
  }
}

pub fn enclose_formatted_characters_for_value(label: &Ident, label_name: String) -> TokenStream {
  quote! {
    let start_event = XmlEvent::start_element(#label_name);
    let _ret = writer.write(start_event);

    let value = format!("{}", #label);
    let data_event = XmlEvent::characters(&value);
    let _ret = writer.write(data_event);

    let end_event = XmlEvent::end_element();
    let _ret = writer.write(end_event);
  }
}

pub fn enclose_characters(label: &Option<Ident>, label_name: String) -> TokenStream {
  quote! {
    let start_event = XmlEvent::start_element(#label_name);
    let _ret = writer.write(start_event);

    let value = format!("{}", self.#label);
    let data_event = XmlEvent::characters(&value);
    let _ret = writer.write(data_event);

    let end_event = XmlEvent::end_element();
    let _ret = writer.write(end_event);
  }
}

pub fn serialize_element(
  label: &Option<Ident>,
  label_name: String,
  default: &Option<String>,
) -> Option<TokenStream> {
  let inner = enclose_characters(label, label_name);

  if let Some(ref d) = default {
    let default_function = Ident::new(&d, Span::call_site());
    Some(quote! {
      if self.#label != #default_function() {
        #inner
      }
    })
  } else {
    Some(quote! {
      #inner
    })
  }
}
