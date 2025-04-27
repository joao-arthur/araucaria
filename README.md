# Araucaria

> Araucaria is a genus of coniferous trees, that grow **slowly** and can live
> more than a **thousand** years

A frontend for a validation library written in rust.

## Installation

```toml
[dependencies]
araucaria = { git = "https://github.com/joao-arthur/araucaria", rev = "0cc2ef2758917372094cf060babf2024f18ce4e9" }
```

## usage

```rust
use std::{collections::BTreeMap, sync::LazyLock};

use araucaria::validation::{
    DateValidation,
    EmailValidation,
    StrValidation,
    ObjValidation,
    Validation
};

pub static CREATE_USER_SCHEMA: LazyLock<Validation> = LazyLock::new(|| {
    Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
        (
            "first_name".into(),
            Validation::Str(StrValidation::default().chars_len_btwn(1, 256))
        ),
        (
            "birthdate".into(),
            Validation::Date(DateValidation::default().ge("1970-01-01".into()))
        ),
        (
            "email".into(),
            Validation::Email(EmailValidation::default())
        ),
        (
            "username".into(),
            Validation::Str(StrValidation::default().chars_len_btwn(1, 64))
        ),
        (
            "password".into(),
            Validation::Str(
                StrValidation::default()
                    .chars_len_btwn(1, 64)
                    .uppercase_len_gt(1)
                    .lowercase_len_gt(1)
                    .numbers_len_gt(1)
                    .symbols_len_gt(1),
            ),
        ),
    ])))
});
```

There's nothing behind the curtains 🙂. Think of this library like a
**definition language**. The code above is equivalent to:

```rust
use std::{collections::BTreeMap, sync::LazyLock};

use araucaria::{
    operation::{Operand, OperandValue, Operation},
    validation::{
        DateValidation,
        EmailValidation,
        ObjValidation,
        StrValidation,
        Validation
    }
};

pub static CREATE_USER_SCHEMA: LazyLock<Validation> = LazyLock::new(|| {
    Validation::Obj(ObjValidation {
        required: true,
        validation: BTreeMap::from([
            (
                "first_name".into(),
                Validation::Str(StrValidation {
                    required: true,
                    operation: None,
                    bytes_len: None,
                    chars_len: Some(Operation::Btwn(
                        Operand::Value(OperandValue::USize(1)),
                        Operand::Value(OperandValue::USize(256)))
                    ),
                    graphemes_len: None,
                    lowercase_len: None,
                    uppercase_len: None,
                    numbers_len: None,
                    symbols_len: None,
                }),
            ),
            (
                "birthdate".into(),
                Validation::Date(DateValidation {
                    required: true,
                    operation: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("2028-07-22")))
                    )
                }),
            ),
            (
                "email".into(),
                Validation::Email(EmailValidation { required: true })
            ),
            (
                "username".into(),
                Validation::Str(StrValidation {
                    required: true,
                    operation: None,
                    bytes_len: None,
                    chars_len: Some(Operation::Btwn(
                        Operand::Value(OperandValue::USize(1)),
                        Operand::Value(OperandValue::USize(64)))
                    ),
                    graphemes_len: None,
                    lowercase_len: None,
                    uppercase_len: None,
                    numbers_len: None,
                    symbols_len: None,
                }),
            ),
            (
                "password".into(),
                Validation::Str(StrValidation {
                    required: true,
                    operation: None,
                    bytes_len: None,
                    chars_len: Some(Operation::Btwn(
                        Operand::Value(OperandValue::USize(1)),
                        Operand::Value(OperandValue::USize(64)))
                    ),
                    graphemes_len: None,
                    lowercase_len: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("1")))
                    ),
                    uppercase_len: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("1")))
                    ),
                    numbers_len: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("1")))
                    ),
                    symbols_len: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("1")))
                    ),
                }),
            ),
        ]),
    })
});
```

## 🚧 TODO

- `Validation::Arr`
- `validate_arr`
- `validate` should not fail when only optional fields are missing
- Segregate `araucaria_plugins` modules by features
- Create a function that receives `serde_json::Value` and `T: Deserialize`, and
  returns a `T` instance
- `value_from_json_and_schema`
  - parse `.0` float as integer
  - parse number as `date_time` (unixtime)
