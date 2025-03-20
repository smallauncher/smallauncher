use crate::*;

use log::*;
use std::io::Write;
use std::{fs, io, path};

pub const RESOURCES_URL: &'static str = "https://resources.download.minecraft.net";

fn download<W: Write>(url: &str, writer: &mut W) -> Result<u64, error::Error> {
	let mut reader = ureq::get(url).call()?.into_reader();
	Ok(io::copy(&mut reader, writer)?)
}

pub fn download_minecraft_version(minecraft_path: &path::Path, jre_path: &path::Path, name: &str) -> Result<(), error::Error> {
	let manifest: api::manifest::Manifest = api::get_from_url(api::manifest::Manifest::DEFAULT_URL)?;
	let Some(version) = manifest.get_version(name) else {
		return Err(error::Error::VersionNotFound);
	};
	let meta: api::meta::Version = api::get_from_url(&version.url)?;
	let assets: api::assets::Assets = api::get_from_url(&meta.asset_index.url)?;
	{
		let data_meta = serde_json::to_string_pretty(&meta)?;
		let data_assets = serde_json::to_string_pretty(&assets)?;

		let path_client = path!(minecraft_path, "versions", &version.id, format!("{0}.jar", version.id));
		let path_meta = path!(minecraft_path, "versions", &version.id, format!("{0}.json", version.id));
		let path_assets = path!(minecraft_path, "assets", "indexes", format!("{0}.json", meta.asset_index.id));

		let mut file_meta = file::create_or_open_file(&path_meta)?;
		let mut file_assets = file::create_or_open_file(&path_assets)?;

		if !file::file_hash(&meta.downloads.client.sha1, &path_client).unwrap_or_default() {
			let mut file_client = file::create_or_open_file(&path_client)?;
			info!("Downloading client: {path_client:?}");
			download(&meta.downloads.client.url, &mut file_client)?;
		}

		file_meta.write(data_meta.as_bytes())?;
		file_assets.write(data_assets.as_bytes())?;
	}
	{
		//this file not is used per smallauncher to launch the game, but some mod installers (forge) expect this file exists
		let profile_path = path!(minecraft_path, "launcher_profiles.json");
		if !profile_path.exists() {
			fs::write(profile_path, r#"{"profiles":{}}"#)?;
		}
	}
	{
		let java_versions: api::java::JavaVersions = api::get_from_url(api::java::JavaVersions::DEFAULT_URL)?;
		let java_version = meta.java_version;
		#[cfg(target_os = "linux")]
		let version = java_versions.linux;
		#[cfg(target_os = "windows")]
		let version = java_versions.windows_x64;
		#[cfg(target_os = "macos")]
		let version = java_versions.mac_os;
		let java_component = java_version.component;
		let list = match java_component.as_str() {
			"java-runtime-alpha" => version.java_runtime_alpha,
			"java-runtime-beta" => version.java_runtime_beta,
			"java-runtime-delta" => version.java_runtime_delta,
			"java-runtime-gamma" => version.java_runtime_gamma,
			"java-runtime-gamma-snapshot" => version.java_runtime_gamma_snapshot,
			"jre-legacy" => version.jre_legacy,
			_ => return Err(error::Error::JavaVersionNotSupported),
		};
		let Some(version) = list.get(0) else {
			return Err(error::Error::JavaVersionNotSupported);
		};
		let files: api::java::Files = api::get_from_url(&version.manifest.url)?;
		for (path, file) in files.files {
			let path = path!(jre_path, &java_component, path);
			match file {
				api::java::File::Directory => fs::create_dir_all(path)?,
				api::java::File::Link { .. } => {}
				#[cfg(target_family = "unix")]
				api::java::File::File { downloads, executable } => {
					let file_download = downloads.raw;
					use std::os::unix::fs::PermissionsExt;
					if !file::file_hash(&file_download.sha1, &path).unwrap_or_default() {
						let mut file = file::create_or_open_file(&path)?;
						info!("Downloading java file: {path:?}");
						download(&file_download.url, &mut file)?;
						if executable {
							let perm = fs::Permissions::from_mode(0o755);
							file.set_permissions(perm)?;
						}
					}
				}
				#[cfg(not(target_family = "unix"))]
				api::java::File::File { downloads, .. } => {
					let file_download = downloads.raw;
					if !file::file_hash(&file_download.sha1, &path).unwrap_or_default() {
						let mut file = file::create_or_open_file(&path)?;
						info!("Downloading java file: {path:?}");
						download(&file_download.url, &mut file)?;
					}
				}
			}
		}
	}

	#[cfg(target_os = "linux")]
	const OS_NAME: Option<api::meta::OsName> = Some(api::meta::OsName::Linux);
	#[cfg(target_os = "windows")]
	const OS_NAME: Option<api::meta::OsName> = Some(api::meta::OsName::Windows);
	#[cfg(target_arch = "x86")]
	const ARCH: Option<api::meta::Arch> = Some(api::meta::Arch::X86);
	#[cfg(not(target_arch = "x86"))]
	const ARCH: Option<api::meta::Arch> = None;

	'l: for library in &meta.libraries {
		let Some(downloads) = &library.downloads else {
			continue;
		};

		if let Some(classifier) = &downloads.classifiers {
			#[cfg(target_os = "linux")]
			let Some(library_download) = &classifier.natives_linux
			else {
				continue;
			};
			#[cfg(target_os = "windows")]
			let Some(library_download) = &classifier.natives_windows
			else {
				continue;
			};
			#[cfg(target_os = "macos")]
			let Some(library_download) = &classifier.natives_osx
			else {
				continue;
			};
			let path = path!(minecraft_path, "libraries", &library_download.path);
			if !file::file_hash(&library_download.sha1, &path).unwrap_or_default() {
				let mut file = file::create_or_open_file(&path)?;
				info!("Downloading native library: {}", library_download.path);
				download(&library_download.url, &mut file)?;
			}
		};

		if let Some(rules) = &library.rules {
			for api::meta::Rule { os, action } in rules {
				match (os, action) {
					(Some(api::meta::Os { name: OS_NAME, arch: ARCH }), api::meta::Action::Allow) => {}
					(Some(api::meta::Os { name: OS_NAME, arch: ARCH }), api::meta::Action::Disallow) => continue 'l,
					(None, api::meta::Action::Allow) => {}
					(None, api::meta::Action::Disallow) => continue 'l,
					(_, api::meta::Action::Disallow) => {}
					(_, api::meta::Action::Allow) => continue 'l,
				}
			}
		}

		if let Some(library_download) = &downloads.artifact {
			let path = path!(minecraft_path, "libraries", &library_download.path);
			if !file::file_hash(&library_download.sha1, &path).unwrap_or_default() {
				let mut file = file::create_or_open_file(&path)?;
				info!("Downloading library: {}", library_download.path);
				download(&library_download.url, &mut file)?;
			}
		};
	}
	for (name, asset) in assets.objects {
		let prefix_hash = &asset.hash[0..2];
		let hash = &asset.hash;
		let path = path!(minecraft_path, "assets", "objects", prefix_hash, hash);
		let url = format!("{RESOURCES_URL}/{prefix_hash}/{hash}");
		if !file::file_hash(hash, &path).unwrap_or_default() {
			info!("Downloading asset: {name}");
			let mut file = file::create_or_open_file(&path)?;
			download(&url, &mut file)?;
		}
	}
	Ok(())
}
