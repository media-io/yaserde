use crate::common::YaSerdeAttribute;
use proc_macro2::Ident;

pub fn build_label_name(
  label: &Ident,
  field_attrs: &YaSerdeAttribute,
  default_namespace: &Option<String>,
) -> String {
  let prefix = if default_namespace == &field_attrs.prefix {
    "".to_string()
  } else {
    field_attrs
      .prefix
      .clone()
      .map_or("".to_string(), |prefix| prefix + ":")
  };

  let label = field_attrs
    .rename
    .clone()
    .unwrap_or_else(|| label.to_string());

  format!("{}{}", prefix, label)
}
