use yaserde_derive::{YaDeserialize, YaSerialize};

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(YaSerialize, YaDeserialize, PartialEq, Debug)]
#[yaserde(rename = "teststruct")]
struct TestStruct {
  #[yaserde(cdata)]
  pub msgdata: String,
}

#[test]
fn test_cdata_serialization() {
  init();
  let test_data = TestStruct {
    msgdata: "<tag>Some unescaped content</tag>".to_string(),
  };
  let xml_output = yaserde::ser::to_string(&test_data).expect("Serialization failed");
  let expected_output = r#"<?xml version="1.0" encoding="utf-8"?><teststruct><msgdata><![CDATA[<tag>Some unescaped content</tag>]]></msgdata></teststruct>"#;
  assert_eq!(xml_output, expected_output);
}
