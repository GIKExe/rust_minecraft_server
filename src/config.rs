use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize, Clone)]
pub struct BindConfig {
	#[serde(default = "default_host")]
	pub host: String,
	#[serde(default = "default_timeout")]
	pub timeout: u64,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
	#[serde(default)]
	pub online_mode: bool,
	#[serde(default = "default_compression")]
	pub compression_threshold: Option<usize>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize, Clone)]
pub struct Config {
	#[serde(default)]
	pub bind: BindConfig,
	#[serde(default)]
	pub server: ServerConfig,
}

fn default_host() -> String {
	"127.0.0.1:25565".to_string()
}
fn default_timeout() -> u64 {
	5
}
fn default_compression() -> Option<usize> {
	Some(256)
}

impl Config {
	pub fn load_from_file(path: PathBuf) -> Option<Config> {
		if !fs::exists(&path).unwrap_or_default() {
			let table = Config::default();
			fs::create_dir_all(&path.parent()?).ok()?;
			fs::write(&path, toml::to_string_pretty(&table).ok()?).ok()?;
			return Some(table);
		}
		let content = fs::read_to_string(&path).ok()?;
		let table = toml::from_str::<Config>(&content).ok()?;
		Some(table)
	}
}
