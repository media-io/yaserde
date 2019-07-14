# yaserde &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Coverage Status]][coveralls]

[Build Status]: https://travis-ci.org/media-io/yaserde.svg?branch=master
[travis]: https://travis-ci.org/media-io/yaserde
[Latest Version]: https://img.shields.io/crates/v/yaserde.svg
[crates.io]: https://crates.io/crates/yaserde

[Coverage Status]: https://coveralls.io/repos/github/media-io/yaserde/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/media-io/yaserde?branch=master

**Yet Another Serializer/Deserializer specialized for XML**

## Goal
This library will support XML de/ser-ializing with all specific features.

## Supported types

- [x] Struct
- [x] Vec<AnyType>
- [x] Enum
- [ ] Enum with complex types
- [x] Option
- [x] String
- [x] bool
- [x] number (u8, i8, u32, i32, f32, f64)

## Attributes

- [x] **attribute**: this field is defined as an attribute
- [x] **default**: defines the default function to init the field
- [ ] **flatten**: Flatten the contents of the field
- [x] **namespace**: defines the namespace of the field
- [x] **rename**: be able to rename a field
- [x] **root**: rename the based element. Used only at the XML root.
- [ ] **skip_serializing_if**: Skip the serialization for this field if the condition is true
- [x] **strict**: enforce stronger deserialization
- [x] **text**: this field match to the text content

## Custom De/Ser-rializer

Any type can define a custom deserializer and/or serializer.
To implement it, define the implementation of YaDeserialize/YaSerialize

```rust
impl YaDeserialize for MyType {
  fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
    // deserializer code
  }
}
```

```rust

impl YaSerialize for MyType {
  fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
    // serializer code
  }
}
```
