use std::fs;
use std::path;

use crate::*;

pub mod microsoft;
pub mod offline;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Account {
	Microsoft { name: String, uuid: String, access_token: String },
	Offline { name: String, uuid: String },
}

pub fn auth_microsoft() -> error::Result<Account> {
	let authorisation_code = microsoft::get_authorisation_code()?;
	let token = microsoft::get_microsoft_token(&authorisation_code)?;
	let xbl = microsoft::get_xbl_autentication(&token.access_token)?;
	let xsts = microsoft::get_xsts_autentication(&xbl.token)?;
	let auth = microsoft::get_minecraft_auth(&xsts)?;
	let profile = microsoft::get_user_profile(&auth.access_token)?;

	Ok(Account::Microsoft {
		name: profile.name,
		uuid: profile.id,
		access_token: auth.access_token,
	})
}

pub fn auth_offline(name: &str) -> Account {
	let name = name.to_string();
	let uuid = offline::offline_uuid(&name);
	Account::Offline { name, uuid }
}

pub fn save(auth_path: &path::Path, account: &Account) -> error::Result<()> {
	let data = serde_json::to_string_pretty(account)?;
	let name = match &account {
		Account::Microsoft { name, .. } => name,
		Account::Offline { name, .. } => name,
	};
	if !auth_path.exists() {
		fs::create_dir_all(auth_path)?;
	}
	let path = auth_path.join(format!("{name}.json"));
	fs::write(path, data.as_bytes())?;
	Ok(())
}

pub fn load(auth_path: &path::Path, name: &str) -> error::Result<Option<Account>> {
	if !auth_path.exists() {
		fs::create_dir_all(auth_path)?;
	}
	let path = auth_path.join(format!("{name}.json"));
	if !path.exists() {
		return Ok(None);
	}
	Ok(Some(serde_json::from_str(&fs::read_to_string(path)?)?))
}
