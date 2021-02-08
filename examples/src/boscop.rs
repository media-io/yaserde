// related to issue https://github.com/media-io/yaserde/issues/3
use std::io::Read;
use yaserde::YaDeserialize;

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(root = "layout")]
pub struct Layout {
  #[yaserde(attribute)]
  pub version: u32,
  #[yaserde(attribute)]
  pub mode: u32,
  #[yaserde(attribute)]
  pub w: u32,
  #[yaserde(attribute)]
  pub h: u32,
  #[yaserde(attribute)]
  pub orientation: String,
  pub tabpage: Vec<Tabpage>,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
pub struct Tabpage {
  #[yaserde(attribute, rename = "name")]
  pub named: String,
  #[yaserde(attribute)]
  pub scalef: f32,
  #[yaserde(attribute)]
  pub scalet: f32,
  #[yaserde(attribute)]
  pub li_t: String,
  #[yaserde(attribute)]
  pub li_c: String,
  #[yaserde(attribute)]
  pub li_s: u32,
  #[yaserde(attribute)]
  pub li_o: bool,
  #[yaserde(attribute)]
  pub li_b: bool,
  #[yaserde(attribute)]
  pub la_t: String,
  #[yaserde(attribute)]
  pub la_c: String,
  #[yaserde(attribute)]
  pub la_s: u32,
  #[yaserde(attribute)]
  pub la_o: bool,
  #[yaserde(attribute)]
  pub la_b: bool,
  pub control: Vec<Control>,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
pub struct Control {
  #[yaserde(attribute, rename = "name")]
  pub named: String,
  #[yaserde(attribute)]
  pub x: u32,
  #[yaserde(attribute)]
  pub y: u32,
  #[yaserde(attribute)]
  pub w: u32,
  #[yaserde(attribute)]
  pub h: u32,
  #[yaserde(attribute)]
  pub color: String,
  #[yaserde(attribute)]
  pub scalef: f32,
  #[yaserde(attribute)]
  pub scalet: f32,
  #[yaserde(attribute)]
  pub local_off: bool,
  #[yaserde(attribute)]
  pub sp: bool,
  #[yaserde(attribute)]
  pub sr: bool,
  pub midi: Vec<Midi>,
  #[yaserde(attribute)]
  pub response: String,
  #[yaserde(attribute)]
  pub inverted: String,
  #[yaserde(attribute)]
  pub centered: String,
  #[yaserde(attribute)]
  pub norollover: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
pub struct Midi {
  #[yaserde(attribute)]
  pub var: String,
  #[yaserde(attribute, rename = "type")]
  pub typ: String,
  #[yaserde(attribute)]
  pub channel: String,
  #[yaserde(attribute)]
  pub data1: String,
  #[yaserde(attribute)]
  pub data2f: String,
  #[yaserde(attribute)]
  pub data2t: String,
  #[yaserde(attribute)]
  pub sysex: String,
}

#[test]
fn parsing_bbigras_namespace() {
  use std::fs::File;
  use yaserde::de::from_str;

  let filename = "tests/data/boscop.xml";
  let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  let loaded: Layout = from_str(&contents).unwrap();

  assert_eq!(loaded.tabpage.len(), 4);
  assert_eq!(loaded.tabpage[0].control.len(), 13);
  assert_eq!(loaded.tabpage[1].control.len(), 16);
  assert_eq!(loaded.tabpage[2].control.len(), 65);
  assert_eq!(loaded.tabpage[3].control.len(), 40);
}