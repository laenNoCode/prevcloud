#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use crate::config::config::get_config;
use crate::database::database::User;
use crate::database::database::UserInsert;
use diesel::{insert_into, SelectableHelper, RunQueryDsl};
use crate::schema::users;
use crate::database::database::establish_connection;
use std::fs::create_dir_all;
use super::common::hash_password;
use super::common::create_salt;

#[derive(Deserialize)]
pub struct Credentials{
	pub username:String,
	pub password:String,

}


pub fn create_folder_for_user(user: &User)-> std::io::Result<()>{
	let config = get_config("config.json".to_string());
	let data_path = format!("{}/data/{}", config.base_path, user.id);
	let metadata_path = format!("{}/metadata/{}", config.base_path, user.id);
	create_dir_all(data_path)?;
	create_dir_all(metadata_path)?;
	return Ok(());
}


pub async fn create_user(
    Json(payload): Json<Credentials>,
) -> (StatusCode, String) {
	let to_return : (StatusCode, String);
	let salt = create_salt();
	let conf = get_config("config.json".to_string());
	let hash = hash_password(payload.password, salt.clone(), conf.pepper);
	match hash{
		Ok(hash) => 
		{
			print!("hash created : {}\n", hash.clone());
			let user:UserInsert = UserInsert{
				username:payload.username,
				password_hash:hash,
				salt,
			};
			let connection = establish_connection();
			match connection{
				Ok(mut connection) => {
					let return_user: Result<User, diesel::result::Error> = insert_into(users::table)
					.values(user)
					.returning(User::as_returning())
					.get_result(  &mut connection);
					match return_user {
						Ok(return_user) =>{
							match create_folder_for_user(&return_user){
								Ok(_) => {
									to_return = (StatusCode::CREATED, format!("user created successfully with username {:?}", &return_user));
								}
								Err(_) => {
									match diesel::delete(&return_user).execute(&mut connection)
									{
										Ok(_) => {
											return (StatusCode::INTERNAL_SERVER_ERROR, format!("the folder could not be created for this user, you can retry later {:?}", &return_user));
										}
										Err(_) => {
											return (StatusCode::INTERNAL_SERVER_ERROR, format!("the folder could not be created for this user but the user was created and not dropped from database please contact the admin"));
										}
									}
								}
							}
						}
						Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) =>{
							to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("user not created because duplicated username"));
						}
						Err(error) =>{
							to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("user not created because reaseons ({:?})", error));
						}
					}
				}
				Err(_) => {
					to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("could not connect to database"));
				}
			}
		}
		Err(E) => {
			to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("could not hash {:?}", E));
		}
	}
	return to_return;
}