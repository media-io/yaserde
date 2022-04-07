# Skip Serializing

## skip_serializing

Attribute `skip_serializing_if` will skip the serialisation of the field if it is set to `true`. Default value is `false`. 

- `#[yaserde(skip_serializing = true)]` - Skip serialization of the field.
- `#[yaserde(skip_serializing = false)]` - The field will be serialized. Default value.
- Not set - The field will be serialized (same as `false`).

### Full example

```rust
use yaserde_derive::YaSerialize;

#[derive(YaSerialize, PartialEq, Debug)]
struct Struct {
    #[yaserde(skip_serializing = true)]
    skip_value: String,
    do_not_skip_value: String,
}

fn main() {
    let obj = Struct {
        skip_value: "skipped".to_string(),
        do_not_skip_value: "not skipped".to_string()
    };

    let yaserde_cfg = yaserde::ser::Config::default();

    //Output: <?xml version="1.0" encoding="utf-8"?><Struct><do_not_skip_value>not skipped</do_not_skip_value></Struct>
    print!("{}", yaserde::ser::to_string_with_config(&obj, &yaserde_cfg).ok().unwrap());
}
```

## skip_serializing_if

Attribute `skip_serializing_if` will skip the serialisation for this field if the condition is true.

To conditionally skip serialization of a field a string needs to be set to `skip_serializing_if`, that refers to a
function name, implemented on for the struct. This function has one parameter (reference to the field value) and returns a `bool` value.
```rust
use yaserde_derive::YaSerialize;

#[derive(YaSerialize, PartialEq, Debug)]
struct Struct {
    #[yaserde(skip_serializing_if = "check_string")]
    string_value: String,
    
    //...
}

impl Struct {
    // Skip serializing the field if the value is 'unset'.
    fn check_string(&self, value: &str) -> bool {
        value == "unset"
    }
    
    //...
}
```

### Full example

```rust
use yaserde_derive::YaSerialize;

#[derive(YaSerialize, PartialEq, Debug)]
enum Enum {
    Enum1,
    Enum2,
}

#[derive(YaSerialize, PartialEq, Debug)]
struct Struct {
    #[yaserde(skip_serializing_if = "check_bool")]
    bool_value: bool,
    #[yaserde(skip_serializing_if = "check_string")]
    string_value: String,
    #[yaserde(skip_serializing_if = "check_i32")]
    i32_value: i32,
    #[yaserde(skip_serializing_if = "check_optional_string")]
    optional_string_value: Option<String>,
    #[yaserde(skip_serializing_if = "check_enum")]
    enum_value: Enum,
}

impl Struct {
    fn check_bool(&self, value: &bool) -> bool {
        value == &false
    }

    fn check_string(&self, value: &str) -> bool {
        value == "unset"
    }

    fn check_i32(&self, value: &i32) -> bool {
        value < &10
    }

    fn check_optional_string(&self, value: &Option<String>) -> bool {
        value == &Some("unset".to_string())
    }

    fn check_enum(&self, value: &Enum) -> bool {
        value == &Enum::Enum1
    }
}

fn main() {
    let obj_no_skip = Struct {
        bool_value: true,
        string_value: "testString".to_string(),
        i32_value: 10,
        optional_string_value: Some("optionalTestString".to_string()),
        enum_value: Enum::Enum2,
    };

    let obj_skip_all = Struct {
        bool_value: false,
        string_value: "unset".to_string(),
        i32_value: 9,
        optional_string_value: Some("unset".to_string()),
        enum_value: Enum::Enum1,
    };

    let yaserde_cfg = yaserde::ser::Config::default();

    //Output: <?xml version=\"1.0\" encoding=\"utf-8\"?><Struct><bool_value>true</bool_value><string_value>testString</string_value><i32_value>10</i32_value><optional_string_value>optionalTestString</optional_string_value><enum_value>Enum2</enum_value></Struct>
    println!("{}", yaserde::ser::to_string_with_config(&obj_no_skip, &yaserde_cfg).ok().unwrap());

    //Output: <?xml version="1.0" encoding="utf-8"?><Struct><enum_value>Enum1</enum_value></Struct>
    //Known issue, enum fields are not working as expected, see: https://github.com/media-io/yaserde/issues/139
    println!("{}", yaserde::ser::to_string_with_config(&obj_skip_all, &yaserde_cfg).ok().unwrap());
}
```

### Known issues
- Currently, `enum` fields are not working with `skip_serializing_if`: https://github.com/media-io/yaserde/issues/139