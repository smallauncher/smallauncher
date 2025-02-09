use smallauncher_lib::path;
use std::env;
use std::path;

#[cfg(target_family = "unix")]
pub fn get_data_folder() -> Result<path::PathBuf, env::VarError> {
	if let Ok(xdg) = env::var("XDG_DATA_HOME") {
		return Ok(path!(xdg, "smallauncher"));
	}
	let home = env::var("HOME")?;
	Ok(path!(home, ".local", "share", "smallauncher"))
}

#[cfg(target_family = "windows")]
pub fn get_data_folder() -> Result<path::PathBuf, env::VarError> {
	let env = env::var("APPDATA")?;
	Ok(path!(env, "smallauncher"))
}
