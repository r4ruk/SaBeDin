pub mod custom_uuid {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;
    use uuid::Uuid;

    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S:Serializer
    {
        val.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Uuid::from_str(val).map_err(D::Error::custom)
    }
}

pub mod custom_datetime {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(val: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
    S:Serializer {val.to_rfc3339().serialize(serializer)}

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        let parsed = DateTime::parse_from_rfc3339(val);
        let deserialized_time: DateTime<Utc>;
        match parsed {
            Ok(value) => deserialized_time = value.into(),
            Err(_) => deserialized_time = Utc::now(),
        }
        let result: DateTime<Utc> = deserialized_time;
        Ok(result)
    }
}