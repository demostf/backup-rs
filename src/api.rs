use crate::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Demo {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub server: String,
    pub duration: u16,
    pub nick: String,
    pub map: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: String,
    pub blue: String,
    pub red_score: u8,
    pub blue_score: u8,
    pub player_count: u8,
    pub uploader: u32,
    #[serde(deserialize_with = "hex_to_digest")]
    pub hash: [u8; 16],
    pub backend: String,
    pub path: String,
}

/// Deserializes a lowercase hex string to a `[u8; 16]`.
pub fn hex_to_digest<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
where
    D: Deserializer<'de>,
{
    use hex::FromHex;
    use serde::de::Error;

    let string = <Cow<str>>::deserialize(deserializer)?;

    if string.len() == 0 {
        return Ok([0; 16]);
    }

    <[u8; 16]>::from_hex(string.as_ref()).map_err(|err| Error::custom(err.to_string()))
}

#[derive(Debug)]
pub enum ListOrder {
    Ascending,
    Descending,
}

impl Default for ListOrder {
    fn default() -> Self {
        ListOrder::Descending
    }
}

impl fmt::Display for ListOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListOrder::Ascending => "ASC".fmt(f),
            ListOrder::Descending => "DESC".fmt(f),
        }
    }
}

#[derive(Debug, Default)]
pub struct ListParams {
    order: ListOrder,
    backend: Option<String>,
}

impl ListParams {
    #[allow(dead_code)]
    pub fn with_backend(self, backend: impl ToString) -> Self {
        ListParams {
            backend: Some(backend.to_string()),
            ..self
        }
    }

    pub fn with_order(self, order: ListOrder) -> Self {
        ListParams { order, ..self }
    }
}

pub fn list_demos(params: ListParams, page: u32) -> Result<Vec<Demo>, Error> {
    let mut req = ureq::get("https://api.demos.tf/demos")
        .query("page", &format!("{}", page))
        .query("order", &format!("{}", params.order));

    if let Some(backend) = params.backend.as_ref() {
        req = req.query("backend", backend);
    }

    let resp = req.call()?;

    Ok(resp.into_json()?)
}
