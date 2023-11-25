use enums::EnumWithStringValue;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_optional_string_enum<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: EnumWithStringValue,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        Ok(Some(T::from_string_ignore_case(s.as_str())))
    } else {
        Ok(None)
    }
}

pub fn deserialize_string_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: EnumWithStringValue,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(T::from_string(s.as_str()))
}

pub fn serialize_string_enum<S, T>(field: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: EnumWithStringValue,
{
    serializer.serialize_str(field.as_str())
}

pub fn serialize_optional_string_enum<S, T>(
    field: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: EnumWithStringValue,
{
    if let Some(field) = field {
        serializer.serialize_str(field.as_str())
    } else {
        serializer.serialize_none()
    }
}
