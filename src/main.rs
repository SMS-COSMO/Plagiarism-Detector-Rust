use process::update_feature_names;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::json;

mod cut;
mod data;
mod process;

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize, Debug)]
struct AddData {
    id: String,
    text: String,
}

#[post("/add", format = "json", data = "<data>")]
fn add(data: Json<AddData>) -> Json<AddData> {
    let req = data.into_inner();
    let mut store = data::open_data();
    let tmp = store.clone();

    let sep_text = cut::cut(&req.text);

    // Add paper
    // "i" -> "id"
    // "t" -> "text"
    store["paper"]
        .as_array_mut()
        .unwrap()
        .push(json!({"i": req.id, "t": process::get_tf_array(sep_text.clone())}));
    data::write_data(store).expect("Failed to write to data.json");

    update_feature_names(sep_text);

    // test
    println!("{:?}", process::cosine_similarity(process::get_tf_idf_array(tmp["paper"][0]["t"].as_array().unwrap().to_vec()), process::get_tf_idf_array(tmp["paper"][1]["t"].as_array().unwrap().to_vec())));

    Json(AddData {
        id: req.id,
        text: req.text,
    })
}

#[launch]
fn rocket() -> _ {
    // Load data before start
    let _store = data::open_data();

    // Start Rocket server
    rocket::build().mount("/", routes![add])
}
