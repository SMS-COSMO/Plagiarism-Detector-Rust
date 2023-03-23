use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn open_data() -> Value {
    let f = std::fs::read_to_string("data.json");
    match f {
        Ok(str) => serde_json::from_str(str.as_str()).unwrap(),
        Err(_) => {
            let template_json: String = String::from("{\"paper\": []}");

            let mut f1 = File::create("data.json").expect("failed to create data.json");
            f1.write_all(template_json.as_bytes())
                .expect("failed to write to data.json");

            serde_json::from_str(template_json.as_str()).unwrap()
        }
    }
}

pub fn write_data(data: Value) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")
        .expect("failed to open data.json");
    f.write_all(data.to_string().as_bytes()).unwrap();

    Ok(())
}
