pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	VersionNotFound,
	JavaVersionNotSupported,
	Serialize(serde_json::Error),
	Io(std::io::Error),
	Network(ureq::Error),
	OsString(std::ffi::OsString),
	Zip(zip::result::ZipError),
	ParseURL(oauth2::url::ParseError),
}

impl From<ureq::Error> for Error {
	#[inline(always)]
	fn from(value: ureq::Error) -> Self {
		Self::Network(value)
	}
}

impl From<std::io::Error> for Error {
	#[inline(always)]
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<serde_json::Error> for Error {
	#[inline(always)]
	fn from(value: serde_json::Error) -> Self {
		Self::Serialize(value)
	}
}

impl From<std::ffi::OsString> for Error {
	#[inline(always)]
	fn from(value: std::ffi::OsString) -> Self {
		Self::OsString(value)
	}
}

impl From<zip::result::ZipError> for Error {
	#[inline(always)]
	fn from(value: zip::result::ZipError) -> Self {
		Self::Zip(value)
	}
}

impl From<oauth2::url::ParseError> for Error {
	#[inline(always)]
	fn from(value: oauth2::url::ParseError) -> Self {
		Self::ParseURL(value)
	}
}
