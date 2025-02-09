pub fn offline_uuid(username: &str) -> String {
	let mut hash = md5::compute(format!("OfflinePlayer:{}", username)).0;
	hash[6] = hash[6] & 0x0f | 0x30; // uuid version 3
	hash[8] = hash[8] & 0x3f | 0x80; // RFC4122 variant

	uuid::Uuid::from_bytes(hash).to_string()
}
