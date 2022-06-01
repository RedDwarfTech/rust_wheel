use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_music_connection() -> PgConnection {
    let database_url = std::env::var("MUSIC_DATABASE_URL").expect("MUSIC_DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_quark_connection() -> PgConnection {
    let database_url = std::env::var("QUARK_DATABASE_URL").expect("QUARK_DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_dict_connection() -> PgConnection {
    let database_url = std::env::var("DICT_DATABASE_URL").expect("DICT_DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn connection(db_env_key: String) -> PgConnection {
    let database_url = std::env::var(db_env_key).expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}