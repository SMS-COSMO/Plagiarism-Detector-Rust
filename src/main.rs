use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::Value;

mod cut;
mod data;
mod process;

#[derive(Deserialize, Serialize, Debug)]
struct ReqData {
    id: String,
    text: String,
}

#[derive(Serialize, Debug)]
struct ResData {
    similarity: Vec<(f64, String)>,
}

fn similarity(
    req: &ReqData,
    store: &mut Value,
    jieba: &jieba_rs::Jieba,
    stop_words: &std::collections::HashSet<String>,
) -> Vec<(f64, String)> {
    // Remove whitespace
    let trimmed: String = req.text.chars().filter(|c| !c.is_whitespace()).collect();
    // Cut text
    let sep_text = cut::cut(&trimmed, jieba, stop_words);
    // Get tf array of current text
    let tf_array = process::get_tf_array(&sep_text);

    // Add paper
    process::add_paper(&req.id, &tf_array, store);
    // Update df
    process::update_feature_names(&sep_text, store);

    // Get result
    process::global_similarity(&req.id, &tf_array, store)
}

#[rocket::post("/add", format = "json", data = "<data>")]
fn add(data: Json<ReqData>, state: &rocket::State<SharedData>) -> Json<ResData> {
    let mut store = data::open_data();
    let req = data.into_inner();

    let res = similarity(&req, &mut store, &state.jieba, &state.stop_words);

    // Write data
    data::write_data(&store).unwrap();

    Json(ResData { similarity: res })
}

#[rocket::post("/check", format = "json", data = "<data>")]
fn check(data: Json<ReqData>, state: &rocket::State<SharedData>) -> Json<ResData> {
    let mut store = data::open_data();
    let req = data.into_inner();

    Json(ResData {
        similarity: similarity(&req, &mut store, &state.jieba, &state.stop_words),
    })
}

/// Data shared through services as a Rocket state.
struct SharedData {
    jieba: jieba_rs::Jieba,
    stop_words: std::collections::HashSet<String>,
}

#[rocket::launch]
fn rocket() -> _ {
    // Start Rocket server
    rocket::build()
        .mount("/", rocket::routes![add, check])
        .manage(SharedData {
            jieba: jieba_rs::Jieba::new(),
            stop_words: data::stop_words(),
        })
}
