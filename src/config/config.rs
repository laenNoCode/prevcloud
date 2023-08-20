#![allow(non_camel_case_types)]
use serde::{Serialize, Deserialize};
use std::cell::OnceCell;
use std::io::Read;
use std::fs::File;
#[derive(Serialize, Deserialize, Clone)]
pub struct Config{
	pub port : u16,
	pub base_path : String,
	pub db_user_name : String,
	pub db_password : String,
	pub db_name: String,
	pub pepper : String,
	pub session_time_s : u32
}
const CONFIG:OnceCell<Config> = OnceCell::new();

pub fn get_config(filename: String) -> Config
{
	CONFIG.get_or_init( || {
		let file = File::open(filename);
		let mut out_string: String = String::new();
		file.expect("error").read_to_string(&mut out_string).expect("config file error");
		serde_json::from_str(&out_string).expect("this json config is wrong")
	}).clone()
}