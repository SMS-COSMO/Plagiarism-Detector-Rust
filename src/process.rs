use crate::data;
use serde_json::{json, Value};

// "n" -> "name"
// "d" -> "df"
pub fn update_feature_names(sep_text: Vec<&str>) -> () {
    let mut store = data::open_data();
    let names = store["feature_names"].as_array_mut().unwrap();

    for word in sep_text {
        let mut find = false;
        for i in 0..names.clone().len() {
            let name = &mut names[i];
            if name["n"].as_str().unwrap() == word {
                find = true;

                let df = name["d"].as_i64().unwrap();
                name["d"] = json!(df + 1);
                break;
            }
        }

        if !find {
            names.push(json!({"n": word, "d": 1}));
        }
    }

    data::write_data(store).unwrap();
}

// "n" -> "name"
// "t" -> "tf"
pub fn get_tf_array(sep_text: Vec<&str>) -> Vec<Value> {
    let mut sorted_sep_text = sep_text.clone();
    sorted_sep_text.sort();

    let mut res: Vec<Value> = [].to_vec();

    let mut i = 0;
    while i < sorted_sep_text.len() {
        let mut cnt = sorted_sep_text.len() - i;
        for j in (i + 1)..sorted_sep_text.len() {
            if sorted_sep_text[j] != sorted_sep_text[i] {
                cnt = j - i;
                break;
            }
        }

        res.push(json!({"n": sorted_sep_text[i], "t": cnt}));
        i += cnt;
    }

    res
}

// Get the tf-idf(smooth) characteristics
pub fn get_tf_idf_array(paper: Vec<Value>) -> Vec<f64> {
    let store = data::open_data();
    let names = store["feature_names"].as_array().unwrap();

    let mut res: Vec<f64> = vec![];
    let nd = names.len() as f64;

    for name in names {
        let mut found = false;
        for word in paper.clone() {
            if word["n"].as_str().unwrap() == name["n"].as_str().unwrap() {
                // tf-idf = tf * idf
                // idf(smooth) = ln((1 + nd) / (1 + df)) + 1

                let tf = word["t"].as_f64().unwrap();
                let idf = 1.0 + f64::ln((1.0 + nd) / (1.0 + name["d"].as_f64().unwrap()));

                res.push(tf * idf);
                found = true;
                break;
            }
        }

        if !found {
            res.push(0.0);
        }
    }

    res
}

pub fn cosine_similarity(v_a: Vec<f64>, v_b: Vec<f64>) -> f64 {
    let mut product_sum = 0.0;
    let mut a_square_sum = 0.0;
    let mut b_square_sum = 0.0;

    for i in 0..v_a.len() {
        product_sum += v_a[i] * v_b[i];
        a_square_sum += v_a[i].powi(2);
        b_square_sum += v_b[i].powi(2);
    }

    product_sum / (a_square_sum.sqrt() * b_square_sum.sqrt())
}
