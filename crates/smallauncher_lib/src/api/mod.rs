pub mod assets;
pub mod java;
pub mod manifest;
pub mod meta;

use serde::de::DeserializeOwned;

use crate::*;

pub(crate) fn get_from_url<T: DeserializeOwned>(url: &str) -> Result<T, error::Error> {
	Ok(ureq::get(url).call()?.into_json()?)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn java_version_api() {
		let _java_versions: java::JavaVersions = get_from_url(java::JavaVersions::DEFAULT_URL).unwrap();
	}
	#[test]
	fn manifest_api() {
		let _manifest: manifest::Manifest = get_from_url(manifest::Manifest::DEFAULT_URL).unwrap();
	}
	#[test]
	fn all_versions_api() {
		let manifest: manifest::Manifest = get_from_url(manifest::Manifest::DEFAULT_URL).unwrap();
		for version in manifest.versions {
			let _version: meta::Version = get_from_url(&version.url).unwrap();
		}
	}
}
