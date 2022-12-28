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

#[test]
fn skip_serializing_for_nested_struct() {
  init();

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(skip_serializing)]
    skipped_serializing: XmlStructChild,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "child")]
  pub struct XmlStructChild {
  }

  let model = XmlStruct {
    skipped_serializing: XmlStructChild{},
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}

#[test]
fn skip_serializing_for_enum() {
  init();

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(skip_serializing)]
    skipped_serializing: XmlEnum,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "child")]
  pub enum XmlEnum {
    Ok
  }

  let model = XmlStruct {
    skipped_serializing: XmlEnum::Ok,
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}

#[test]
fn skip_serializing_for_vec() {
  init();

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(skip_serializing)]
    skipped_serializing: Vec<i8>,
  }

  let model = XmlStruct {
    skipped_serializing: vec![1,2,3],
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}

