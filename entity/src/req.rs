use rocket::serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct ReqData {
    pub id: String,
    pub text: String,
    pub write: bool,
}

#[derive(Serialize, Debug)]
pub struct ResData {
    pub msg: String,
    pub similarity: Option<Vec<(f64, String)>>,
}

/// Data shared through services as a Rocket state.
pub struct SharedData {
    pub jieba: jieba_rs::Jieba,
    pub stop_words: std::collections::HashSet<String>,
}
