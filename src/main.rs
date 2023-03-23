use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::json;

mod cut;
mod data;

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

    store["paper"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id": req.id, "text": cut::cut(&req.text)}));
    data::write_data(store).expect("Failed to write to data.json");

    Json(AddData {
        id: req.id,
        text: req.text,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![add])
}
