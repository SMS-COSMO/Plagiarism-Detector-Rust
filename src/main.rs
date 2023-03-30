use jieba_rs::Jieba;
use rocket::serde::{json::Json, Deserialize, Serialize};

mod cut;
mod data;
mod process;

// Lazy initialize jieba
lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

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

#[post("/add", format = "json", data = "<data>")]
fn add(data: Json<AddData>) -> Json<ResData> {
    let req = data.into_inner();
    let mut store = data::open_data();

    // Remove whitespace
    let trimmed = req.text.chars().filter(|c| !c.is_whitespace()).collect();
    // Cut text
    let sep_text = cut::cut(&trimmed, &JIEBA);
    // Get tf array of current text
    let tf_array = process::get_tf_array(&sep_text);

    // Add paper
    process::add_paper(req.id.clone(), &tf_array, &mut store);
    // Update df
    process::update_feature_names(sep_text, &mut store);
    // Get result
    let res = process::get_global_similarity(req.id.clone(), &tf_array, &store);

    // Write data
    data::write_data(&store).unwrap();

    Json(ResData { similarity: res })
}

#[launch]
fn rocket() -> _ {
    // Start Rocket server
    rocket::build().mount("/", routes![add])
}
