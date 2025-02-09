use crate::*;

use std::ffi;
use std::fs;
use std::io;
use std::path;
use std::process;

const CLIENT_ID: &'static str = "42";
const LAUNCHER_NAME: &'static str = "miniLauncher";
const LAUNCHER_VERSION: &'static str = "42";

#[cfg(all(not(debug_assertions), target_os = "windows"))]
const DETACHED_PROCESS: u32 = 0x00000008;

fn merge(a: serde_json::Value, b: serde_json::Value) -> serde_json::Value {
	match (a, b) {
		(serde_json::Value::Object(mut a), serde_json::Value::Object(b)) => {
			for (k, v2) in b {
				if let Some(v1) = a.remove(&k) {
					a.insert(k, merge(v1, v2));
				} else {
					a.insert(k, v2);
				}
			}
			serde_json::Value::Object(a)
		}
		(serde_json::Value::Array(mut a), serde_json::Value::Array(b)) => {
			a.extend(b);
			serde_json::Value::Array(a)
		}
		(_, b) => b,
	}
}

pub fn launch_minecraft_version(
	game_path: &path::Path,
	jre_path: &path::Path,
	version_name: &str,
	account: &auth::Account,
) -> Result<(), error::Error> {
	let meta: api::meta::Version = {
		let meta_path = path!(game_path, "versions", &version_name, format!("{version_name}.json"));
		let mut map1: serde_json::Map<String, serde_json::Value> = file::from_json_file(&meta_path)?;
		match map1.remove("inheritsFrom") {
			Some(serde_json::Value::String(version_name)) => {
				let meta_path = path!(game_path, "versions", &version_name, format!("{version_name}.json"));
				let map2: serde_json::Map<String, serde_json::Value> = file::from_json_file(&meta_path)?;
				let v1 = serde_json::Value::Object(map1);
				let v2 = serde_json::Value::Object(map2);
				serde_json::from_value(merge(v2, v1))?
			}
			_ => serde_json::from_value(serde_json::Value::Object(map1))?,
		}
	};

	extract_natives(&meta, &game_path)?;

	let main_class = &meta.main_class;
	let java_component = &meta.java_version.component;
	let jvm_args = generate_jvm_args(&meta, &game_path);
	let game_args = generate_game_args(&meta, &game_path, account);
	#[cfg(target_family = "unix")]
	let jre_bin = path!(jre_path, java_component, "bin", "java");
	#[cfg(target_family = "windows")]
	let jre_bin = path!(jre_path, java_component, "bin", "java.exe");
	let mut command = process::Command::new(jre_bin);
	#[cfg(all(not(debug_assertions), target_os = "windows"))]
	std::os::windows::process::CommandExt::creation_flags(&mut command, DETACHED_PROCESS);
	command.current_dir(game_path).args(jvm_args).arg(main_class).args(game_args);

	log::info!("Spawning command: {:?}", command);
	command.spawn()?;
	Ok(())
}

pub fn extract_natives(meta: &api::meta::Version, game_path: &path::Path) -> Result<(), error::Error> {
	for library in &meta.libraries {
		let Some(downloads) = &library.downloads else {
			continue;
		};
		let Some(classifier) = &downloads.classifiers else {
			continue;
		};
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

		let path = path!(game_path, "libraries", &library_download.path);
		let file = fs::File::open(&path)?;
		let mut zip = zip::ZipArchive::new(file)?;
		for i in 0..zip.len() {
			let mut zip_file = zip.by_index(i)?;
			let name = zip_file.name();
			if !(name.ends_with(".so") || name.ends_with(".dll") || name.ends_with(".dylib")) {
				continue;
			}
			let path = path!(game_path, "bin", name);
			log::info!("Extracting {path:?}");
			let mut file = file::create_or_open_file(&path)?;
			io::copy(&mut zip_file, &mut file)?;
		}
	}
	Ok(())
}

pub fn generate_game_args(meta: &api::meta::Version, game_path: &path::Path, account: &auth::Account) -> Vec<ffi::OsString> {
	let mut args = Vec::new();
	let arguments = &meta.arguments;
	let assets_path = path!(game_path, "assets");
	for arg in &arguments.game {
		match arg {
			api::meta::Argument::String(str) => match str.as_str() {
				"${clientid}" => args.push(osStr!(CLIENT_ID)),
				"${auth_player_name}" => {
					match &account {
						auth::Account::Microsoft { name, .. } => args.push(osStr!(name)),
						auth::Account::Offline { name, .. } => args.push(osStr!(name)),
					};
				}
				"${auth_uuid}" => {
					match &account {
						auth::Account::Microsoft { uuid, .. } => args.push(osStr!(uuid)),
						auth::Account::Offline { uuid, .. } => args.push(osStr!(uuid)),
					};
				}
				"${user_type}" => {
					match &account {
						auth::Account::Microsoft { .. } => args.push(osStr!("msa")),
						auth::Account::Offline { .. } => args.push(osStr!()),
					};
				}
				"${auth_access_token}" => match &account {
					auth::Account::Microsoft { access_token, .. } => args.push(osStr!(access_token)),
					auth::Account::Offline { .. } => args.push(osStr!()),
				},
				"${game_directory}" => args.push(osStr!(game_path)),
				"${version_name}" => args.push(osStr!(&meta.id)),
				"${assets_root}" => args.push(osStr!(&assets_path)),
				"${assets_index_name}" => args.push(osStr!(&meta.asset_index.id)),
				"${version_type}" => args.push(osStr!(meta.r#type.to_string())),
				"--xuid" | "${auth_xuid}" => {}
				str => args.push(osStr!(str)),
			},
			_ => {}
		}
	}
	args.push(osStr!("--userProperties"));
	args.push(osStr!("{}"));
	args
}

pub fn generate_jvm_args(meta: &api::meta::Version, game_path: &path::Path) -> Vec<ffi::OsString> {
	let mut args = Vec::new();
	let arguments = &meta.arguments;
	let natives_dir = path!(game_path, "bin");
	let class_paths = get_class_paths(meta, game_path);
	for arg in &arguments.jvm {
		match arg {
			api::meta::Argument::String(str) => match str.as_str() {
				"-Dminecraft.launcher.version=${launcher_version}" => {
					let value = osStr!("-Dminecraft.launcher.version=", LAUNCHER_VERSION);
					args.push(value);
				}
				"-Dminecraft.launcher.brand=${launcher_name}" => {
					let value = osStr!("-Dminecraft.launcher.brand=", LAUNCHER_NAME);
					args.push(value);
				}
				"-Djava.library.path=${natives_directory}" => {
					let value = osStr!("-Djava.library.path=", &natives_dir);
					args.push(value);
				}
				"-Djna.tmpdir=${natives_directory}" => {
					let value = osStr!("-Djna.tmpdir=", &natives_dir);
					args.push(value);
				}
				"-Dorg.lwjgl.system.SharedLibraryExtractPath=${natives_directory}" => {
					let value = osStr!("-Dorg.lwjgl.system.SharedLibraryExtractPath=", &natives_dir);
					args.push(value);
				}
				"-Dio.netty.native.workdir=${natives_directory}" => {
					let value = osStr!("-Dio.netty.native.workdir=", &natives_dir);
					args.push(value);
				}
				"${classpath}" => args.push(class_paths.clone()),
				_ => args.push(ffi::OsString::from(str)),
			},
			_ => {}
		}
	}
	args
}

pub fn get_class_paths(meta: &api::meta::Version, game_path: &path::Path) -> ffi::OsString {
	#[cfg(target_os = "linux")]
	const OS_NAME: Option<api::meta::OsName> = Some(api::meta::OsName::Linux);
	#[cfg(target_os = "windows")]
	const OS_NAME: Option<api::meta::OsName> = Some(api::meta::OsName::Windows);
	#[cfg(target_arch = "x86")]
	const ARCH: Option<api::meta::Arch> = Some(api::meta::Arch::X86);
	#[cfg(not(target_arch = "x86"))]
	const ARCH: Option<api::meta::Arch> = None;
	#[cfg(target_family = "unix")]
	const SEPARATOR: &'static str = ":";
	#[cfg(target_family = "windows")]
	const SEPARATOR: &'static str = ";";

	let mut list = ffi::OsString::new();
	{
		let path = path!(game_path, "versions", &meta.id, format!("{}.jar", meta.id));
		list.push(path);
	}
	'l: for lib in &meta.libraries {
		match lib {
			api::meta::Library {
				downloads: Some(api::meta::LibraryDownload {
					artifact: Some(artifact), ..
				}),
				rules,
				..
			} => {
				if let Some(rules) = rules {
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
				list.push(SEPARATOR);
				let path = path!(game_path, "libraries", &artifact.path);
				list.push(path);
			}
			lib => {
				let mut original_name: &str = &lib.name;
				let mut name = "";
				let mut version = "";
				let mut path = path!();

				if let Some(i) = original_name.find(":") {
					let mut paths = &original_name[..i];
					original_name = &original_name[(i + 1)..];
					while let Some(i) = paths.find(".") {
						path = path!(path, &paths[..i]);
						paths = &paths[(i + 1)..];
					}
					path = path!(path, &paths);
				}
				if let Some(i) = original_name.find(":") {
					name = &original_name[..i];
					version = &original_name[(i + 1)..];
				}
				list.push(SEPARATOR);
				let path = path!(game_path, "libraries", &path, name, version, format!("{name}-{version}.jar"));
				list.push(path);
			}
		}
	}
	list
}

pub fn check_version_integrity(game_path: &path::Path, version_name: &str) -> bool {
	let path_version = path!(game_path, "versions", version_name, format!("{version_name}.json"));
	let path_client = path!(game_path, "versions", version_name, format!("{version_name}.jar"));
	let Ok(version) = file::from_json_file::<api::meta::Version, _>(&path_version) else {
		return false;
	};
	if !file::file_hash(&version.downloads.client.sha1, &path_client).unwrap_or_default() {
		return false;
	}
	let path_assets = path!(game_path, "assets", "indexes", format!("{}.json", version.asset_index.id));
	let Ok(assets) = file::from_json_file::<api::assets::Assets, _>(&path_assets) else {
		return false;
	};
	for (_, asset) in assets.objects {
		let prefix_hash = &asset.hash[0..2];
		let hash = &asset.hash;
		let path = path!(game_path, "assets", "objects", prefix_hash, hash);
		if !file::file_hash(hash, &path).unwrap_or_default() {
			return false;
		}
	}
	true
}

pub fn list_versions(game_path: &path::Path) -> Result<Vec<String>, error::Error> {
	let path = path!(game_path, "versions");
	fs::read_dir(path)?
		.map(|x| -> Result<String, error::Error> { Ok(x?.file_name().to_string_lossy().to_string()) })
		.collect()
}

pub fn list_all_versions() -> Result<api::manifest::Manifest, error::Error> {
	api::get_from_url(api::manifest::Manifest::DEFAULT_URL)
}
