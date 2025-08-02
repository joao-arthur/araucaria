# Araucaria

> Araucaria is a genus of coniferous trees, that grow **slowly** and can live
> more than a **thousand** years

A frontend for a validation library written in rust.

## Installation

```toml
[dependencies]
araucaria = { git = "https://github.com/joao-arthur/araucaria", rev = "0cc2ef2758917372094cf060babf2024f18ce4e9" }
```

## Usage

```rust
use std::{collections::BTreeMap, sync::LazyLock};

use araucaria::schema::{
    DateSchema,
    EmailSchema,
    StrSchema,
    ObjSchema,
    Schema
};

pub static CREATE_USER_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::from(ObjSchema::from([
        (
            "first_name".into(),
            Schema::from(StrSchema::default().chars_len_btwn(1, 256))
        ),
        (
            "birthdate".into(),
            Schema::from(DateSchema::default().unix_epoch())
        ),
        (
            "email".into(),
            Schema::from(EmailSchema::default())
        ),
        (
            "username".into(),
            Schema::from(StrSchema::default().chars_len_btwn(1, 64))
        ),
        (
            "password".into(),
            Schema::from(
                StrSchema::default()
                    .chars_len_btwn(1, 64)
                    .uppercase_len_gt(1)
                    .lowercase_len_gt(1)
                    .numbers_len_gt(1)
                    .symbols_len_gt(1),
            ),
        ),
    ]))
});
```

There's nothing behind the curtains 🙂. Think of this library like a
**definition language**. The code above is equivalent to:

```rust
use std::{collections::BTreeMap, sync::LazyLock};

use araucaria::{
    operation::{Operand, OperandValue, Operation},
    schema::{
        DateSchema,
        EmailSchema,
        ObjSchema,
        StrSchema,
        Schema
    }
};

pub static CREATE_USER_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::Obj(ObjSchema {
        required: true,
        validation: BTreeMap::from([
            (
                "first_name".into(),
                Schema::Str(StrSchema {
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
                Schema::Date(DateSchema {
                    required: true,
                    operation: Some(Operation::Gt(
                        Operand::Value(OperandValue::from("2028-07-22")))
                    )
                }),
            ),
            (
                "email".into(),
                Schema::Email(EmailSchema { required: true })
            ),
            (
                "username".into(),
                Schema::Str(StrSchema {
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
                Schema::Str(StrSchema {
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

## Validation and Localization

You need to install
[araucaria_plugins](https://github.com/joao-arthur/araucaria_plugins).

## 🚧 TODO

- `Schema::Arr`
- readme documentation
- mdBook documentation
- ObjSchema:: pick key
