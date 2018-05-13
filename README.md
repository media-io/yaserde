# yaserde
Yet Another Serializer/Deserializer

## Goal
This library will support XML de/ser-ializing with all specific features.

## Supported types

- [x] Struct
- [x] Vec<AnyType>
- [x] Enum
- [ ] Enum with complex types
- [ ] Option

## Decorators

- [x] **root**: rename the based element. Used only at the XML root.
- [x] **rename**: be able to rename a field
- [x] **attribute**: this field is defined as an attribute
- [x] **text**: this field match to the text content
- [ ] **namespace**: defines the namespace of the field

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
