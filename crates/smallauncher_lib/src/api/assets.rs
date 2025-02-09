use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
	pub objects: collections::HashMap<String, Asset>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
	pub hash: String,
	pub size: usize,
}
