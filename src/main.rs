use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::json;
use jieba_rs::Jieba;

mod cut;
mod data;
mod process;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[derive(Deserialize, Serialize, Debug)]
struct AddData {
    id: String,
    text: String,
}

#[derive(Serialize, Debug)]
struct ResData {
    similarity: Vec<(f64, String)>,
}

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

#[post("/add", format = "json", data = "<data>")]
fn add(data: Json<AddData>) -> Json<ResData> {
    let mut store = data::open_data();

    let req = data.into_inner();
    // Remove is_whitespace
    let trimmed = req.text.chars().filter(|c| !c.is_whitespace()).collect();
    // Cut text
    let sep_text = cut::cut(&trimmed, JIEBA.clone());
    // Get tf array of current text
    let tf_array = process::get_tf_array(sep_text.clone());

    // Add paper
    // "i" -> "id"
    // "t" -> "text"
    store["paper"]
        .as_array_mut()
        .unwrap()
        .push(json!({"i": req.id.clone(), "t": tf_array}));
    data::write_data(store).expect("Failed to write to data.json");

    // Update df
    process::update_feature_names(sep_text);

    let res = process::get_global_similarity(req.id.clone(), tf_array);

    // test
    // println!("{:?}", res);

    Json(ResData { similarity: res })
}

#[launch]
fn rocket() -> _ {
    // Load data before start
    let _store = data::open_data();

    // Start Rocket server
    rocket::build().mount("/", routes![add])
}
