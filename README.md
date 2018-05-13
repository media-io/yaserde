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
