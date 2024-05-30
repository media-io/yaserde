use yaserde::ser::to_string;
use yaserde::*;
use yaserde::de::from_str;

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Header {}

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
#[yaserde(
  rename = "Envelope",
  namespace = "s: http://schemas.xmlsoap.org/soap/envelope/",
  prefix = "s"
)]
pub struct SoapEnvelope<BODY>
where
  BODY: YaSerialize + YaDeserialize + Default,
{
  #[yaserde(rename = "encodingStyle", prefix = "s", attribute)]
  pub encoding_style: String,
  #[yaserde(rename = "u", prefix = "xmlns", attribute)]
  pub tnsattr: Option<String>,
  #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
  pub urnattr: Option<String>,
  #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
  pub xsiattr: Option<String>,
  #[yaserde(rename = "Header", prefix = "s")]
  pub header: Option<Header>,
  #[yaserde(rename = "Body", prefix = "s")]
  pub body: BODY,
}

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct SoapPlay {
  #[yaserde(rename = "Play", prefix = "u", default)]
  pub body: Play,
}

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
#[yaserde(rename = "Play", prefix = "u")]
pub struct Play {
  #[yaserde(flatten, default)]
  pub parameters: Play2,
}

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
#[yaserde(
  rename = "Play",
  namespace = "u: urn:schemas-upnp-org:service:AVTransport:1",
  prefix = "u"
)]
pub struct Play2 {
  #[yaserde(rename = "InstanceID", default)]
  pub instance_id: i32,
  #[yaserde(rename = "Speed", default)]
  pub speed: i32,
}

#[derive(PrimitiveYaSerde,Debug, Default, Eq, PartialEq)]
struct Meters(i32);
//noinspection RsMainFunctionNotFound
#[test]
fn test_for_generic_newtype() {
  let a = SoapEnvelope {
    encoding_style: "".to_string(),
    tnsattr: None,
    urnattr: None,
    xsiattr: None,
    header: None,
    body: Meters(10) ,
  };

  let s = to_string(&a).unwrap();
  let b: SoapEnvelope<Meters> = from_str(&s).unwrap();

  assert_eq!(a, b);
  println!("{:#?}", b);
}

#[test]
fn test_for_generic_nested_struct() {
  let a = SoapEnvelope {
    encoding_style: "".to_string(),
    tnsattr: None,
    urnattr: None,
    xsiattr: None,
    header: None,
    body: SoapPlay {
      body: Play {
        parameters: Play2 {
          instance_id: 20,
          speed: 1,
        },
      },
    },
  };

  let s = to_string(&a).unwrap();
  let b: SoapEnvelope<SoapPlay> = from_str(&s).unwrap();

  assert_eq!(a, b);
  println!("{:#?}", b);
}
