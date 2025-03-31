mod env;

use smallauncher_lib::*;
use std::io;
use std::process;

fn help() -> ! {
	println!("smallauncher -d   --download <version>");
	println!("smallauncher -c   --check    <version>");
	println!("smallauncher -r   --run      <version> <username>");
	println!("smallauncher -s   --search   <version>");
	println!("smallauncher -l   --list");
	println!("smallauncher -la  --list-all");
	println!("smallauncher -a   --authenticate");
	process::exit(0);
}

fn commands(mut args: std::env::Args) {
	if args.len() <= 1 {
		help();
	}
	args.next().expect("bin path");
	let base_path = env::get_data_folder().expect("error get folder game");
	let game_path = path!(&base_path, "minecraft");
	let jre_path = path!(&base_path, "jre");
	let auth_path = path!(&base_path, "auth");
	let command = match args.next() {
		Some(cmd) => cmd,
		None => {
			println!("No command provided.");
			help();
		}
	};

	match command.as_str() {
		"-a" | "--authenticate" => match smallauncher_lib::auth::auth_microsoft() {
			Ok(auth) => match smallauncher_lib::auth::save(&auth_path, &auth) {
				Ok(_) => println!("Authentication successful."),
				Err(e) => println!("Failed to save authentication: {:?}", e),
			},
			Err(e) => println!("Authentication failed: {:?}", e),
		},
		"-d" | "--download" => match args.next() {
			Some(version) => match download::download_minecraft_version(&game_path, &jre_path, &version) {
				Ok(_) => println!("Download completed!"),
				Err(e) => println!("Download failed: {:?}", e),
			},
			None => {
				println!("Version not specified.");
				help();
			}
		},
		"-c" | "--check" => match args.next() {
			Some(version) => {
				if launch::check_version_integrity(&game_path, &version) {
					println!("Version {version} is integrity!");
				} else {
					println!("Version {version} not is integrity!");
				}
			}
			None => {
				println!("Version not specified.");
				help();
			}
		},
		"-r" | "--run" => match args.next() {
			Some(version) => match args.next() {
				Some(username) => {
					let auth = match smallauncher_lib::auth::load(&auth_path, &username) {
						Ok(Some(auth)) => auth,
						Ok(None) => smallauncher_lib::auth::auth_offline(&username),
						Err(e) => {
							println!("Failed to load authentication: {:?}", e);
							return;
						}
					};

					match launch::launch_minecraft_version(&game_path, &jre_path, &version, &auth) {
						Ok(_) => println!("Game launched successfully."),
						Err(e) => println!("Failed to launch game: {:?}", e),
					}
				}
				None => {
					println!("Username not specified.");
					help();
				}
			},
			None => {
				println!("Version not specified.");
				help();
			}
		},
		"-l" | "--list" => match launch::list_versions(&game_path) {
			Ok(list) => {
				if !list.is_empty() {
					for version in list {
						println!("{version}");
					}
				} else {
					println!("No installed versions found.");
				}
			}
			Err(error::Error::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
				println!("No installed versions found. The versions directory does not exist.");
			}
			Err(e) => println!("Error listing versions: {:?}", e),
		},
		"-la" | "--list-all" => match launch::list_all_versions() {
			Ok(versions) => {
				if versions.versions.is_empty() {
					println!("No versions available.");
				} else {
					for version in versions.versions {
						println!("{}", version.id);
					}
				}
			}
			Err(e) => println!("Failed to list all versions: {:?}", e),
		},
		"-s" | "--search" => match args.next() {
			Some(search) => match launch::list_all_versions() {
				Ok(versions) => {
					let mut found = false;
					for version in versions.versions {
						if version.id.contains(&search) {
							found = true;
							println!("{}", version.id);
						}
					}
					if !found {
						println!("No versions matching '{}' found.", search);
					}
				}
				Err(e) => println!("Failed to search versions: {:?}", e),
			},
			None => {
				println!("Search term not specified.");
				help();
			}
		},
		_ => help(),
	}
}

fn main() {
	#[cfg(not(debug_assertions))]
	if let Err(e) = simple_logger::init_with_level(log::Level::Info) {
		eprintln!("Failed to initialize logger: {:?}", e);
	}

	#[cfg(debug_assertions)]
	if let Err(e) = simple_logger::init_with_level(log::Level::Debug) {
		eprintln!("Failed to initialize logger: {:?}", e);
	}

	let args = std::env::args();
	commands(args);
}
