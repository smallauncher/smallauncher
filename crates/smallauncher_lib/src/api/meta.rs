use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	pub id: String,
	#[serde(default)]
	pub arguments: Arguments,
	pub asset_index: AssetIndex,
	pub downloads: Downloads,
	#[serde(default)]
	pub java_version: JavaVersion,
	pub libraries: Vec<Library>,
	pub main_class: String,
	pub r#type: VersionType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
	pub component: String,
	pub major_version: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
	pub game: Vec<Argument>,
	pub jvm: Vec<Argument>,
}

impl Default for Arguments {
	fn default() -> Self {
		Arguments {
			game: vec![
				Argument::String("--username".to_string()),
				Argument::String("${auth_player_name}".to_string()),
				Argument::String("--version".to_string()),
				Argument::String("${version_name}".to_string()),
				Argument::String("--gameDir".to_string()),
				Argument::String("${game_directory}".to_string()),
				Argument::String("--assetsDir".to_string()),
				Argument::String("${assets_root}".to_string()),
				Argument::String("--assetIndex".to_string()),
				Argument::String("${assets_index_name}".to_string()),
				Argument::String("--uuid".to_string()),
				Argument::String("${auth_uuid}".to_string()),
				Argument::String("--accessToken".to_string()),
				Argument::String("${auth_access_token}".to_string()),
				Argument::String("--clientId".to_string()),
				Argument::String("${clientid}".to_string()),
				Argument::String("--xuid".to_string()),
				Argument::String("${auth_xuid}".to_string()),
				Argument::String("--userType".to_string()),
				Argument::String("${user_type}".to_string()),
				Argument::String("--versionType".to_string()),
				Argument::String("${version_type}".to_string()),
			],
			jvm: vec![
				Argument::String("-Djava.library.path=${natives_directory}".to_string()),
				Argument::String("-Djna.tmpdir=${natives_directory}".to_string()),
				Argument::String("-Dorg.lwjgl.system.SharedLibraryExtractPath=${natives_directory}".to_string()),
				Argument::String("-Dio.netty.native.workdir=${natives_directory}".to_string()),
				Argument::String("-Dminecraft.launcher.brand=${launcher_name}".to_string()),
				Argument::String("-Dminecraft.launcher.version=${launcher_version}".to_string()),
				Argument::String("-cp".to_string()),
				Argument::String("${classpath}".to_string()),
			],
		}
	}
}

impl Default for JavaVersion {
	#[inline(always)]
	fn default() -> Self {
		Self {
			component: "jre-legacy".to_string(),
			major_version: 8,
		}
	}
}

impl fmt::Display for VersionType {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Release => f.write_str("Release"),
			Self::Snapshot => f.write_str("Snapshot"),
			Self::OldBeta => f.write_str("OldBeta"),
			Self::OldAlpha => f.write_str("OldAlpha"),
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Argument {
	String(String),
	Object(ArgumentRule),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArgumentRule {
	pub rules: Vec<Rule>,
	pub value: ArgumentValue,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ArgumentValue {
	String(String),
	List(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
	pub id: String,
	pub sha1: String,
	pub size: usize,
	pub total_size: usize,
	pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
	pub client: Download,
	pub server: Option<Download>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Download {
	pub url: String,
	pub size: usize,
	pub sha1: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
	Release,
	Snapshot,
	OldBeta,
	OldAlpha,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Library {
	pub name: String,
	pub downloads: Option<LibraryDownload>,
	pub natives: Option<serde_json::Value>,
	pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDownload {
	pub artifact: Option<Artifact>,
	pub classifiers: Option<Classifiers>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Classifiers {
	pub natives_linux: Option<Artifact>,
	pub natives_osx: Option<Artifact>,
	pub natives_windows: Option<Artifact>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
	pub url: String,
	pub path: String,
	pub sha1: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
	pub action: Action,
	pub os: Option<Os>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Os {
	pub name: Option<OsName>,
	pub arch: Option<Arch>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Arch {
	X86,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OsName {
	Windows,
	Osx,
	Linux,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
	Allow,
	Disallow,
}
