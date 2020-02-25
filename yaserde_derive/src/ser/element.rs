use attribute::*;
use proc_macro2::{Ident, Span, TokenStream};

pub fn enclose_formatted_characters(label: &Ident, label_name: String) -> TokenStream {
  enclose_xml_event(label_name, quote!(format!("{}", &self.#label)))
}

pub fn enclose_formatted_characters_for_value(label: &Ident, label_name: String) -> TokenStream {
  enclose_xml_event(label_name, quote!(format!("{}", #label)))
}

pub fn enclose_characters(label: &Option<Ident>, label_name: String) -> TokenStream {
  enclose_xml_event(label_name, quote!(format!("{}", self.#label)))
}

pub fn enclose_xml_event(label_name: String, yaserde_format: TokenStream) -> TokenStream {
  quote! {
    let start_event = XmlEvent::start_element(#label_name);
    writer.write(start_event).map_err(|e| e.to_string())?;

    let yaserde_value = #yaserde_format;
    let data_event = XmlEvent::characters(&yaserde_value);
    writer.write(data_event).map_err(|e| e.to_string())?;

    let end_event = XmlEvent::end_element();
    writer.write(end_event).map_err(|e| e.to_string())?;
  }
}

pub fn serialize_element(
  label: &Option<Ident>,
  label_name: String,
  conditions: &TokenStream,
) -> Option<TokenStream> {
  let inner = enclose_characters(label, label_name);

  Some(quote! {
    #conditions {
      #inner
    }
  })
}

pub fn condition_generator(label: &Option<Ident>, attributes: &YaSerdeAttribute) -> TokenStream {
  let mut conditions = None;

  if let Some(ref d) = attributes.default {
    let default_function = Ident::new(
      &d,
      label
        .as_ref()
        .map_or(Span::call_site(), |ident| ident.span()),
    );

    conditions = Some(quote!(self.#label != #default_function()))
  }

  if let Some(ref s) = attributes.skip_serializing_if {
    let skip_if_function = Ident::new(
      &s,
      label
        .as_ref()
        .map_or(Span::call_site(), |ident| ident.span()),
    );

    conditions = if let Some(prev_conditions) = conditions {
      Some(quote!(!#skip_if_function() && #prev_conditions))
    } else {
      Some(quote!(!self.#skip_if_function(&self.#label)))
    };
  }

  conditions.map(|c| quote!(if #c)).unwrap_or(quote!())
}
