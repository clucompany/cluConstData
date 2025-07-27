use cluConstData::const_data;

const_data! {
	pub(crate) const URL: &str = "https://", "api.example.com";
	const TIMEOUT_MS: u32 = 3000;
	const HEADERS: &[&str] = &["Accept"], &["Content-Type"];
}

fn main() {
	println!("Endpoint: {URL}");
	println!("Timeout: {TIMEOUT_MS} ms");
	println!("Headers: {HEADERS:?}");
}
