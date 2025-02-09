use serde::{Deserialize, Serialize};

impl Manifest {
	pub const DEFAULT_URL: &'static str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
	#[inline(always)]
	pub fn get_latest_snapshot(&self) -> Option<&Version> {
		self.get_version(&self.latest.snapshot)
	}
	#[inline(always)]
	pub fn get_latest_release(&self) -> Option<&Version> {
		self.get_version(&self.latest.release)
	}
	pub fn get_version(&self, name: &str) -> Option<&Version> {
		for version in &self.versions {
			if version.id == name {
				return Some(version);
			}
		}
		None
	}
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
	pub latest: Latest,
	pub versions: Vec<Version>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	pub id: String,
	pub r#type: String,
	pub url: String,
	pub time: String,
	pub release_time: String,
}
