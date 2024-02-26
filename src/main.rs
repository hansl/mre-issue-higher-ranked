use crate::pages::Paginate;
use rocket::{get, launch, routes};
use rocket_db_pools::diesel::QueryDsl;

mod db;
mod models;
mod pages;
mod schema;

use db::Db;

#[get("/")]
async fn index(mut db: Db) -> String {
    let query = schema::posts::table.into_boxed();

    let (items, total): (Vec<models::Post>, i64) = query
        .paginate(Some(0))
        .load_and_count_total(&mut db)
        .await
        .unwrap();

    format!("{items:?} {total}\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
