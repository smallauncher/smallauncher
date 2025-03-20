use std::io;
use std::net;

use crate::*;

pub const CLIENT_ID: &'static str = match option_env!("CLIENT_ID") {
	Some(client_id) => client_id,
	None => "74909cec-49b6-4fee-aa60-1b2a57ef72e1",
};
pub const AUTH_URI: &'static str = "https://login.live.com/oauth20_authorize.srf";

const HTTP_OK: &'static [u8] = b"
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 2

ok
";

#[derive(Debug, serde::Deserialize)]
pub struct Token {
	pub access_token: String,
	pub refresh_token: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthResponse {
	pub access_token: String,
	pub token_type: String,
	pub expires_in: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthResponse {
	pub issue_instant: String,
	pub not_after: String,
	pub token: String,
	pub display_claims: DisplayClaims,
}

#[derive(Debug, serde::Deserialize)]
pub struct DisplayClaims {
	pub xui: Vec<Xui>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Xui {
	pub uhs: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Profile {
	pub id: String,
	pub name: String,
}

pub fn get_authorisation_code() -> error::Result<String> {
	let auth_uri = oauth2::AuthUrl::new(AUTH_URI.to_string()).unwrap();
	let redirect_uri = oauth2::RedirectUrl::new("http://localhost:6565".to_string()).unwrap();
	let client = oauth2::basic::BasicClient::new(oauth2::ClientId::new(CLIENT_ID.to_string()))
		.set_auth_uri(auth_uri)
		.set_redirect_uri(redirect_uri);

	let (auth_url, _) = client
		.authorize_url(oauth2::CsrfToken::new_random)
		.add_scope(oauth2::Scope::new("XboxLive.signin".to_string()))
		.add_scope(oauth2::Scope::new("XboxLive.offline_access".to_string()))
		.set_response_type(&oauth2::ResponseType::new("code".to_string()))
		.url();

	open::that_in_background(auth_url.as_str());

	let listener = net::TcpListener::bind("127.0.0.1:6565")?;
	for stream in listener.incoming() {
		if let Ok(mut stream) = stream {
			let mut reader = io::BufReader::new(&mut stream);
			let mut request_line = String::new();

			io::BufRead::read_line(&mut reader, &mut request_line)?;

			let redirect_url = request_line.split_whitespace().nth(1).unwrap();
			let url = oauth2::url::Url::parse(&format!("http://localhost:6565{redirect_url}"))?;
			let mut pairs = url.query_pairs();
			let code = pairs.next().unwrap().1.to_string();
			io::Write::write(&mut stream, HTTP_OK)?;
			return Ok(code);
		}
	}
	unreachable!()
}

pub fn get_xbl_autentication(access_token: &str) -> error::Result<XboxAuthResponse> {
	let body = serde_json::json!({
		"Properties": {
			"AuthMethod": "RPS",
			"SiteName": "user.auth.xboxlive.com",
			"RpsTicket": format!("d={access_token}"),
		},
		"RelyingParty": "http://auth.xboxlive.com",
		"TokenType": "JWT"
	});
	Ok(ureq::post("https://user.auth.xboxlive.com/user/authenticate")
		.send_json(body)?
		.into_json()?)
}

pub fn get_xsts_autentication(xbl_token: &str) -> error::Result<XboxAuthResponse> {
	let body = serde_json::json!({
		"Properties": {
			"SandboxId": "RETAIL",
			"UserTokens": [xbl_token]
		},
		"RelyingParty": "rp://api.minecraftservices.com/",
		"TokenType": "JWT",
	});
	Ok(ureq::post("https://xsts.auth.xboxlive.com/xsts/authorize").send_json(body)?.into_json()?)
}

pub fn get_microsoft_token(authorisation_code: &str) -> error::Result<Token> {
	Ok(ureq::post("https://login.live.com/oauth20_token.srf")
		.send_form(&[
			("client_id", CLIENT_ID),
			("code", authorisation_code),
			("grant_type", "authorization_code"),
			("redirect_uri", "http://localhost:6565"),
		])?
		.into_json()?)
}

pub fn get_minecraft_auth(xsts_auth: &XboxAuthResponse) -> error::Result<AuthResponse> {
	let user_hash = &xsts_auth.display_claims.xui.iter().next().unwrap().uhs;
	let token = &xsts_auth.token;
	let auth_body = serde_json::json!({
		"identityToken": format!("XBL3.0 x={};{}", user_hash, token)
	});
	Ok(ureq::post("https://api.minecraftservices.com/authentication/login_with_xbox")
		.send_json(auth_body)?
		.into_json()?)
}

pub fn get_user_profile(access_token: &str) -> error::Result<Profile> {
	Ok(ureq::get("https://api.minecraftservices.com/minecraft/profile")
		.set("Authorization", &format!("Bearer {access_token}"))
		.call()?
		.into_json()?)
}
