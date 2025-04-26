# Araucaria

> Araucaria is a genus of coniferous trees, that grow **slowly** and can live more than a **thousand** years

A frontend for a validation library written in rust.

## 🚧 TODO

- `Validation::Arr`
- `validate_arr`
- `validate_enum`
- `validate_string` normalize strings
- `validate` should not fail when only optional fields are missing
- `Validation::Date`, `Validation::Time`, `Validation::DateTime` replace regex with chrono
- Segregate `araucaria_plugins` modules by features
- Create a function that receives `serde_json::Value` and `T: Deserialize`, and returns a `T` instance 
- `value_from_json_and_schema`
    - parse `.0` float as integer
    - parse number as `date_time` (unixtime)
