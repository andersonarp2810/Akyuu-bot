use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub use crate::choice_parameters::*;

// TODO: rename "variation" to "page" and add an option for a list of "option"s
// that have emojis associated to each "option" in the list
// pages have left/right arrows
// options have emojis (especially useful for kokoro)

#[derive(Serialize, Deserialize)]
pub struct MoveData {
	pub variations: Vec<Variation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variation {
	pub title: Option<String>,
	#[serde(deserialize_with = "de_num_str_as_frames", default)]
	pub startup: Option<String>,
	#[serde(deserialize_with = "de_num_str_as_frames", default)]
	pub active: Option<String>,
	#[serde(deserialize_with = "de_num_str_as_frames", default)]
	pub recovery: Option<String>,
	#[serde(deserialize_with = "de_num_str", default)]
	pub damage: Option<String>,
	#[serde(deserialize_with = "de_num_str", default)]
	pub stun: Option<String>,
	#[serde(deserialize_with = "de_num_str", default)]
	pub cost: Option<String>,

	pub attachment: Option<String>,
	pub thumbnail: Option<String>,
}

fn de_num_str_as_frames<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> Result<Option<String>, D::Error> {
	Ok(match Value::deserialize(deserializer)? {
		Value::String(s) => Some(s),
		Value::Number(n) => Some(n.to_string() + "f"),
		_ => return Err(de::Error::custom("wrong type")),
	})
}

fn de_num_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
	Ok(match Value::deserialize(deserializer)? {
		Value::String(s) => Some(s),
		Value::Number(n) => Some(n.to_string()),
		_ => return Err(de::Error::custom("wrong type")),
	})
}
