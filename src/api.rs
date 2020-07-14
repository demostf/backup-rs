use crate::Error;
use chrono::{DateTime, Utc};
use md5::Digest;
use serde::{Deserialize, Deserializer};
use smol_str::SmolStr;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Demo {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub server: SmolStr,
    pub duration: u16,
    pub nick: SmolStr,
    pub map: SmolStr,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: SmolStr,
    pub blue: SmolStr,
    pub red_score: u8,
    pub blue_score: u8,
    pub player_count: u8,
    pub uploader: u32,
    #[serde(deserialize_with = "hex_to_digest")]
    pub hash: Digest,
    pub backend: SmolStr,
    pub path: String,
}

/// Deserializes a lowercase hex string to a `Vec<u8>`.
pub fn hex_to_digest<'de, D>(deserializer: D) -> Result<Digest, D::Error>
where
    D: Deserializer<'de>,
{
    use hex::FromHex;
    use serde::de::Error;

    let string = String::deserialize(deserializer)?;

    if string.len() == 0 {
        return Ok(Digest([0; 16]));
    }

    <[u8; 16]>::from_hex(&string)
        .map_err(|err| Error::custom(err.to_string()))
        .map(Digest)
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
    let mut req = ureq::get("https://api.demos.tf/demos");
    req.query("page", &format!("{}", page))
        .query("order", &format!("{}", params.order));

    if let Some(backend) = params.backend.as_ref() {
        req.query("backend", backend);
    }

    let resp = req.call();

    Ok(resp.into_json_deserialize()?)
}
