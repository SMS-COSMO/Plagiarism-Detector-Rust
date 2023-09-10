use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

/// Load data.json into serde_json::Value
pub fn open_data() -> Value {
    if let Ok(str) = std::fs::read_to_string("data.json") {
        serde_json::from_str(str.as_str()).unwrap()
    } else {
        let template_json: String = String::from("{\"feature_names\": [], \"paper\": []}");

        // Create file if not found
        let mut f1 =
            File::create("data.json").expect("[data::open_data] failed to create data.json");

        f1.write_all(template_json.as_bytes())
            .expect("[data::open_data] failed to write to data.json");

        serde_json::from_str(template_json.as_str()).unwrap()
    }
}

/// Get stopword list from stopwords.txt
pub fn stop_words() -> std::collections::HashSet<String> {
    let mut words = std::collections::HashSet::new();
    for item in String::from_utf8_lossy(include_bytes!("../stopwords.txt")).split_whitespace() {
        words.insert(item.to_string());
    }

    words
}

/// Write data into data.json
pub fn write_data(data: &Value) -> std::io::Result<()> {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")?
        .write_all(data.to_string().as_bytes())
}
