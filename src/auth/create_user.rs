use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser{
	username:String,
	password:String,

}


pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, String) {

    (StatusCode::CREATED, format!("user created successfully with username {}", payload.username))
}