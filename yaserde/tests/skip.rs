#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn skip_serializing() {
  init();

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(skip_serializing)]
    skipped_serializing: String,
  }

  let model = XmlStruct {
    skipped_serializing: "skipped serializing".to_string(),
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}
