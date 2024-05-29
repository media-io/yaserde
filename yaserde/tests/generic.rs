use yaserde::{serialize_and_validate, YaSerialize};

extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

#[test]
fn parametrized_field() {
    #[derive(Debug, PartialEq, YaSerialize)]
    #[yaserde(rename = "outer")]
    pub struct Outer<T: YaSerialize> {
        pub inner: T,
    }

    #[derive(Debug, PartialEq, YaSerialize)]
    pub struct Inner {
        #[yaserde(text)]
        pub body: String,
    }

    let content = "<outer><inner>Test</inner></outer>";
    let model = Outer { inner: Inner { body: "Test".to_owned() }};
    serialize_and_validate!(model, content);
}
