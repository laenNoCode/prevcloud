use diesel::pg::PgConnection;
use diesel::prelude::*;

use time::Date;
use dotenvy::dotenv;
use std::env;
pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}


#[derive(Queryable, Selectable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User  {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub salt: String
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInsert {
    pub username: String,
    pub password_hash: String,
    pub salt: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cookies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cookie{
    pub id : String,
    pub user_id : i32,
    pub expires : Date
}
