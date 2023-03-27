use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

// Load data.json into serde_json::Value
pub fn open_data() -> Value {
    let f = std::fs::read_to_string("data.json");
    match f {
        Ok(str) => serde_json::from_str(str.as_str()).unwrap(),
        Err(_) => {
            let template_json: String =
                String::from("{\"feature_names\": [], \"paper\": []}");

            // Create file if not found
            let mut f1 = File::create("data.json").expect("failed to create data.json");
            f1.write_all(template_json.as_bytes())
                .expect("failed to write to data.json");

            serde_json::from_str(template_json.as_str()).unwrap()
        }
    }
}

// Get stopword list from stopwords.txt
pub fn get_stop_words<'a>() -> Vec<String> {
    let f = std::fs::read_to_string("stopwords.txt").unwrap();

    let mut words = vec![];
    for item in f.split_whitespace() {
        words.push(String::from(item));
    }

    words
}

// Write data into data.json
pub fn write_data(data: Value) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")
        .expect("failed to open data.json");
    f.write_all(data.to_string().as_bytes()).unwrap();

    Ok(())
}
