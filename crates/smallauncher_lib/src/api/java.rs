use std::collections;

use crate::*;

use serde::{Deserialize, Serialize};

impl JavaVersions {
	pub const DEFAULT_URL: &'static str =
		"https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct JavaVersions {
	pub linux: PlatformVersion,
	pub linux_i386: PlatformVersion,
	pub mac_os: PlatformVersion,
	pub mac_os_arm64: PlatformVersion,
	pub windows_arm64: PlatformVersion,
	pub windows_x64: PlatformVersion,
	pub windows_x86: PlatformVersion,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct PlatformVersion {
	pub java_runtime_alpha: Vec<Version>,
	pub java_runtime_beta: Vec<Version>,
	pub java_runtime_delta: Vec<Version>,
	pub java_runtime_gamma: Vec<Version>,
	pub java_runtime_gamma_snapshot: Vec<Version>,
	pub jre_legacy: Vec<Version>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	pub manifest: api::meta::Download,
	pub version: VersionName,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VersionName {
	pub name: String,
	pub released: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum File {
	Directory,
	Link { target: String },
	File { executable: bool, downloads: Downloads },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Files {
	pub files: collections::HashMap<String, File>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Downloads {
	pub raw: Download,
	pub lzma: Option<Download>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Download {
	pub url: String,
	pub size: usize,
	pub sha1: String,
}
