#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use base64::Engine;


use argon2::{
	password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Salt
    },
    Argon2,
	Error
};


#[derive(Debug)]
pub enum hash_password_err{
	Argon(Error),
	Passhash(argon2::password_hash::Error)
}

impl From <Error> for hash_password_err{
	fn from(value: Error) -> Self {
		hash_password_err::Argon(value)
	}
}
impl From <argon2::password_hash::Error> for hash_password_err{
	fn from(value: argon2::password_hash::Error) -> Self {
		hash_password_err::Passhash(value)
	}
}

pub fn hash_password(pass:String, salt: String, pepper : String) -> Result<String, hash_password_err>
{
	let salt_and_pepper : String = format!("{}:{}", &salt, pepper);
	let b64_engine = base64::engine::general_purpose::STANDARD_NO_PAD;
	let salt_and_pepper_b64 : String = b64_engine.encode(&salt_and_pepper);
	let argon2 = Argon2::default();
	let salt_string = SaltString::from_b64(&salt_and_pepper_b64)?;
	let mut pass_hash = argon2.hash_password(pass.as_bytes(), &salt_string)?;
	let salt_b64 = b64_engine.encode(&salt);
	pass_hash.salt = Some(Salt::from_b64(&salt_b64)?);
	return Ok(pass_hash.to_string());
}

pub fn create_salt() -> String
{
	SaltString::generate(&mut OsRng).to_string()
}
