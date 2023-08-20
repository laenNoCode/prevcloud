use axum::{Json, http::StatusCode};
use diesel::result::Error::NotFound;

use crate::auth::create_user::Credentials;
use crate::database::database::User;
use diesel::{ RunQueryDsl, QueryDsl};
use diesel::ExpressionMethods;
use crate::schema::users;
use crate::database::database::establish_connection;
use crate::config::config::get_config;
use super::common::hash_password;
//TODO fill this code (i have to do a storage of logged user sessions and cookie system)


pub async fn login(
    Json(payload): Json<Credentials>,
) -> (StatusCode, String){
	let connection = establish_connection();
	let to_return : (StatusCode, String);
	let conf = get_config("config.json".to_string());
	match connection{
		Ok(mut connection) => {
			let return_user : Result<User, diesel::result::Error> = users::table.filter(users::username.eq(payload.username)).first(&mut connection);
			match return_user {
				Ok(user) => {
					let password_hash = hash_password(payload.password, user.salt, conf.pepper);
					match password_hash {
						Ok(password_hash) => {
							let correct_hash = password_hash.bytes().zip(user.password_hash.bytes()).fold(true, |acc,(a, b)|acc&(a==b));
							if correct_hash 
							{
								to_return = (StatusCode::OK, format!("succefully logged in"));
							}
							else{
								to_return = (StatusCode::UNAUTHORIZED, format!("invalid credentials"));
							}
						}
						Err(_) => {
							to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("could not connect to database"))
						}
						
					}
				}
				Err(NotFound) => {
					to_return = (StatusCode::UNAUTHORIZED, format!("invalid credentials"));
				}
				Err(_) => {
					to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("could not connect to database"));
				}
			}
		}
		Err(_) => {
			to_return = (StatusCode::INTERNAL_SERVER_ERROR, format!("could not connect to database"));
		}
	}
	return to_return;
}