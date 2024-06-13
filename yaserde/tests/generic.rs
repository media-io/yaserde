#[macro_use]
extern crate yaserde;

use yaserde::{YaDeserialize, YaSerialize};

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn generic() {
  init();

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct Base<G>
  where
    G: YaSerialize + YaDeserialize + Default,
  {
    background: G,
  }

  #[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
  pub struct Generic {
    #[yaserde(attribute)]
    color: String,
  }

  let content = r#"<base><background color="blue" /></base>"#;
  let model = Base {
    background: Generic {
      color: "blue".to_string(),
    },
  };

  serialize_and_validate!(model, content);

  log::debug!("deserialize_and_validate @ {}:{}", file!(), line!());
  let loaded: Result<Base<Generic>, String> = yaserde::de::from_str(content);
  assert_eq!(loaded, Ok(model));
}
