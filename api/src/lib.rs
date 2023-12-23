extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use migration::MigratorTrait;
use sea_orm_rocket::{Connection, Database};

mod pool;
use ::entity::req::*;
use pool::Db;

mod data;
mod process;

pub use entity::paper;
pub use entity::paper::Entity as Post;

#[rocket::post("/", format = "json", data = "<data>")]
async fn check(
    data: Json<ReqData>,
    state: &rocket::State<SharedData>,
    conn: Connection<'_, Db>,
) -> Json<ResData> {
    let req: ReqData = data.into_inner();
    let db = conn.into_inner();

    match process::similarity(&req, req.write, db, &state.jieba, &state.stop_words).await {
        Ok(r) => Json(ResData {
            code: 200,
            msg: format!("{}成功", if req.write { "加入" } else { "查询" }),
            similarity: r,
        }),
        Err(e) => Json(ResData {
            code: 500,
            msg: e.to_string(),
            similarity: [].to_vec(),
        }),
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", rocket::routes![check])
        .manage(SharedData {
            jieba: jieba_rs::Jieba::new(),
            stop_words: data::stop_words(),
        })
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
