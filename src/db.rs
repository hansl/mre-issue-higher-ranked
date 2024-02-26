use diesel::Connection;
use rocket_db_pools::diesel::{PgConnection, PgPool};
use rocket_db_pools::{ Database, Initializer};

#[derive(Database)]
#[database("diesel_demo")]
pub struct DbPool(PgPool);

impl DbPool {
    pub fn init() -> Initializer<Self> {
        Database::init()
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type Db = rocket_db_pools::Connection<DbPool>;
