use serde::{Deserialize, Deserializer, Serializer};
use enums::EnumWithStringValue;

pub fn deserialize_optional_string_enum<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where D: Deserializer<'de>,
          T: EnumWithStringValue {
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Some(T::from_string_ignore_case(s.as_str())))
}

pub fn deserialize_string_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de>,
          T: EnumWithStringValue {
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(T::from_string(s.as_str()))
}

pub fn serialize_string_enum<S, T>(field: &T, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, T: EnumWithStringValue {
    serializer.serialize_str(field.as_str())
}

pub fn serialize_optional_string_enum<S, T>(field: &Option<T>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, T: EnumWithStringValue {
    if let Some(field) = field {
        serializer.serialize_str(field.as_str())
    } else {
        serializer.serialize_none()
    }
}
// Currently toml is broken and can't handle u64 deserialization  https://github.com/toml-rs/toml/issues/438
pub fn serialize_u64<S>(field: &u64, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(format!("{}", field).as_str())
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de> {
    let value: &str = Deserialize::deserialize(deserializer)?;
    Ok(value.parse::<u64>().unwrap())
}