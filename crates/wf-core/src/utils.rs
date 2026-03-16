use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_mongo_date_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    if v.is_null() {
        return Ok(None);
    }

    if let Value::Number(n) = &v {
        if let Some(ms) = n.as_i64() {
            if let Some(dt) = ms_to_dt(ms) {
                return Ok(Some(dt));
            } else {
                return Err(serde::de::Error::custom("invalid timestamp"));
            }
        }
    }

    if let Value::String(s) = &v {
        if let Ok(ms) = s.parse::<i64>() {
            if let Some(dt) = ms_to_dt(ms) {
                return Ok(Some(dt));
            } else {
                return Err(serde::de::Error::custom("invalid timestamp"));
            }
        }
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }
    }

    if let Value::Object(map) = &v {
        if let Some(Value::String(num_s)) = map.get("$numberLong") {
            if let Ok(ms) = num_s.parse::<i64>() {
                if let Some(dt) = ms_to_dt(ms) {
                    return Ok(Some(dt));
                } else {
                    return Err(serde::de::Error::custom("invalid timestamp"));
                }
            }
        }
        if let Some(Value::Number(num)) = map.get("$numberLong") {
            if let Some(ms) = num.as_i64() {
                if let Some(dt) = ms_to_dt(ms) {
                    return Ok(Some(dt));
                } else {
                    return Err(serde::de::Error::custom("invalid timestamp"));
                }
            }
        }
        if let Some(Value::String(s)) = map.get("$date") {
            if let Ok(ms) = s.parse::<i64>() {
                if let Some(dt) = ms_to_dt(ms) {
                    return Ok(Some(dt));
                } else {
                    return Err(serde::de::Error::custom("invalid timestamp"));
                }
            }
            if let Some(Value::String(ds)) = map.get("$date") {
                if let Ok(dt) = DateTime::parse_from_rfc3339(ds) {
                    return Ok(Some(dt.with_timezone(&Utc)));
                }
            }
        }
    }

    Err(serde::de::Error::custom("unsupported date format"))
}

fn ms_to_dt(ms: i64) -> Option<DateTime<Utc>> {
    let secs = ms / 1000;
    let nsec = ((ms % 1000).abs() as u32) * 1_000_000;
    Utc.timestamp_opt(secs, nsec).single()
}
