use std::{ffi, fs, io, path};

use crate::*;

pub(crate) fn create_or_open_file<S: AsRef<ffi::OsStr>>(path: &S) -> Result<fs::File, io::Error> {
	let path = path::Path::new(path);
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}
	Ok(fs::File::create(&path)?)
}

pub(crate) fn file_hash<P: AsRef<path::Path>>(hash: &str, path: &P) -> Result<bool, error::Error> {
	use sha1::Digest;
	let mut file = fs::File::open(path)?;
	let mut hasher = sha1::Sha1::new();
	io::copy(&mut file, &mut hasher)?;
	let reader_hash = hex::encode(hasher.finalize());
	Ok(reader_hash == hash)
}

#[inline(always)]
pub(crate) fn from_json_file<T: serde::de::DeserializeOwned, P: AsRef<path::Path>>(path: P) -> Result<T, error::Error> {
	Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[macro_export]
macro_rules! path {
    () => {
        std::path::PathBuf::new()
    };
    ($($segment:expr),+ $(,)?) => {
        {
            let mut path = std::path::PathBuf::new();
            $(
                path.push($segment);
            )+
            path
        }
    };
}

#[macro_export]
macro_rules! osStr {
    () => {
        std::ffi::OsString::new()
    };
    ($($segment:expr),+ $(,)?) => {
        {
            let mut string = std::ffi::OsString::new();
            $(
                string.push($segment);
            )+
            string
        }
    };
}
