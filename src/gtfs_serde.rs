use crate::error::Error;
use chrono::Datelike;
use chrono::NaiveDate;
use itertools::Itertools;
use rgb::RGB8;
use serde::de;
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match &*s {
        "0" => Ok(false),
        "1" => Ok(true),
        &_ => Err(serde::de::Error::custom(format!(
            "Invalid value `{}`, expected 0 or 1",
            s
        ))),
    }
}

pub(crate) fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *value {
        serializer.serialize_u8(1)
    } else {
        serializer.serialize_u8(0)
    }
}

pub(crate) fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(serde::de::Error::custom)
}

pub(crate) fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{}{}{}", date.year(), date.month(), date.day()).as_str())
}

pub(crate) fn deserialize_option_time<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;

    match s {
        None => Ok(None),
        Some(t) => {
            let parts = t.trim_start().split(':').collect_vec();
            if parts.len() != 3 {
                Err(de::Error::custom(Error::InvalidTime(t.to_owned())))
            } else {
                Ok(Some(parse_time(parts).map_err(de::Error::custom)?))
            }
        }
    }
}

pub(crate) fn deserialize_time<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let parts = s.trim_start().split(':').collect_vec();
    if parts.len() != 3 {
        Err(de::Error::custom(Error::InvalidTime(s.to_owned())))
    } else {
        Ok(parse_time(parts).map_err(de::Error::custom)?)
    }
}

fn parse_time(time_parts: Vec<&str>) -> Result<u64, std::num::ParseIntError> {
    let hours: u64 = time_parts[0].parse()?;
    let minutes: u64 = time_parts[1].parse()?;
    let seconds: u64 = time_parts[2].parse()?;

    Ok(hours * 3600 + minutes * 60 + seconds)
}

pub(crate) fn serialize_option_time<S>(time: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        None => serializer.serialize_none(),
        Some(t) => serializer.serialize_str(format!("{}", t).as_str()),
    }
}

pub(crate) fn serialize_time<S>(time: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{}", time).as_str())
}

pub(crate) fn deserialize_option_color<'de, D>(de: D) -> Result<Option<RGB8>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(de).and_then(|s| {
        let s = s.trim();
        if s.is_empty() {
            Ok(None)
        } else {
            parse_color(s).map(Some).map_err(de::Error::custom)
        }
    })
}

pub(crate) fn serialize_option_color<S>(
    color: &Option<RGB8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match color {
        None => serializer.serialize_none(),
        Some(RGB8 { r, g, b }) => {
            serializer.serialize_str(format!("{:02X}{:02X}{:02X}", r, g, b).as_str())
        }
    }
}

fn parse_color(s: &str) -> Result<RGB8, Error> {
    if s.len() != 6 {
        return Err(Error::InvalidColor(s.to_owned()));
    }
    let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| Error::InvalidColor(s.to_owned()))?;
    let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| Error::InvalidColor(s.to_owned()))?;
    let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| Error::InvalidColor(s.to_owned()))?;
    Ok(RGB8::new(r, g, b))
}
