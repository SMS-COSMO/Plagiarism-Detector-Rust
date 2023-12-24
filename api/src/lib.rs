extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::http::Status;
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
use plagiarism_detector_rust_service::Query;

#[rocket::post("/", format = "json", data = "<data>")]
async fn check(
    data: Json<ReqData>,
    state: &rocket::State<SharedData>,
    conn: Connection<'_, Db>,
) -> (Status, Json<ResData>) {
    let req: ReqData = data.into_inner();
    let db = conn.into_inner();

    if req.write {
        match Query::find_paper_by_pid(db, &req.id).await {
            Ok(res) => {
                if res.is_some() {
                    return (
                        Status::BadRequest,
                        Json(ResData {
                            msg: format!("论文'{}'已存在", req.id),
                            similarity: None,
                        }),
                    );
                }
            }
            Err(e) => {
                return (
                    Status::InternalServerError,
                    Json(ResData {
                        msg: e.to_string(),
                        similarity: None,
                    }),
                );
            }
        }
    }

    match process::similarity(&req, req.write, db, &state.jieba, &state.stop_words).await {
        Ok(r) => (
            Status::Ok,
            Json(ResData {
                msg: format!("{}成功", if req.write { "加入" } else { "查询" }),
                similarity: Some(r),
            }),
        ),
        Err(e) => (
            Status::InternalServerError,
            Json(ResData {
                msg: e.to_string(),
                similarity: None,
            }),
        ),
    }
}

#[rocket::catch(404)]
fn not_found(req: &rocket::Request) -> Json<ResData> {
    Json(ResData {
        msg: format!("'{}'不存在", req.uri()),
        similarity: None,
    })
}

#[rocket::catch(400)]
fn bad_req() -> Json<ResData> {
    Json(ResData {
        msg: "调用有误".to_string(),
        similarity: None,
    })
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
        .register("/", rocket::catchers![not_found, bad_req])
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
