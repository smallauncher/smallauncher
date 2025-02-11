fn main() {
	if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
		let mut res = winresource::WindowsResource::new();
		res.set_icon("assets/icon.ico");
		if let Err(e) = res.compile() {
			println!("cargo::warning={e}");
		}
	}
}
