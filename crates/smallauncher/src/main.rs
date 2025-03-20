mod env;

use simple_logger;
use smallauncher_lib::*;
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
	match args.next().unwrap().as_str() {
		"-a" | "--authenticate" => {
			let auth = smallauncher_lib::auth::auth_microsoft().unwrap();
			smallauncher_lib::auth::save(&auth_path, &auth).unwrap();
		}
		"-d" | "--download" => {
			let version = args.next().unwrap_or_else(|| help());
			download::download_minecraft_version(&game_path, &jre_path, &version).unwrap();
			log::info!("Download completed!")
		}
		"-c" | "--check" => {
			let version = args.next().unwrap_or_else(|| help());
			if launch::check_version_integrity(&game_path, &version) {
				println!("Version {version} is integrity!");
			} else {
				println!("Version {version} not is integrity!");
			}
		}
		"-r" | "--run" => {
			let version = args.next().unwrap_or_else(|| help());
			let username = args.next().unwrap_or_else(|| help());
			let auth = match smallauncher_lib::auth::load(&auth_path, &username).unwrap() {
				Some(auth) => auth,
				None => smallauncher_lib::auth::auth_offline(&username),
			};
			launch::launch_minecraft_version(&game_path, &jre_path, &version, &auth).unwrap();
		}
		"-l" | "--list" => {
			let list = launch::list_versions(&game_path).unwrap();
			for version in list {
				println!("{version}");
			}
		}
		"-la" | "--list-all" => {
			let versions = launch::list_all_versions().unwrap();
			for version in versions.versions {
				println!("{}", version.id);
			}
		}
		"-s" | "--search" => {
			let search = args.next().unwrap_or_else(|| help());
			let versions = launch::list_all_versions().unwrap();
			for version in versions.versions {
				if version.id.contains(&search) {
					println!("{}", version.id);
				}
			}
		}
		_ => help(),
	}
}

fn main() {
	#[cfg(not(debug_assertions))]
	simple_logger::init_with_level(log::Level::Info).unwrap();
	#[cfg(debug_assertions)]
	simple_logger::init_with_level(log::Level::Debug).unwrap();

	let args = std::env::args();
	commands(args);
}
