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

    store["paper"].as_array_mut().unwrap().append(
        [json!({"id": req.id, "text": cut::cut(&req.text)})]
            .to_vec()
            .as_mut(),
    );
    data::write_data(store).expect("Failed to write to data.json");

    Json(AddData {
        id: req.id,
        text: req.text,
    })
}

#[launch]
fn rocket() -> _ {
    let store = data::open_data();
    println!("{}", store);
    rocket::build().mount("/", routes![add])
}
