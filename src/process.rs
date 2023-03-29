use std::cmp::Ordering;

use crate::data;
use serde_json::{json, Value};

// "n" -> "name"
// "d" -> "df"
pub fn update_feature_names(sep_text: Vec<&str>) -> () {
    let mut store = data::open_data();
    let names = store["feature_names"].as_array_mut().unwrap();

    for word in sep_text {
        let mut find = false;
        for i in 0..names.len() {
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
pub fn get_tf_array(sep_text: &Vec<&str>) -> Vec<Value> {
    let mut sorted_sep_text = sep_text.clone();
    sorted_sep_text.sort();

    let mut res: Vec<Value> = vec![];

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
pub fn get_tf_idf_array(paper: &Vec<Value>, store: &Value) -> Vec<f64> {
    let names = store["feature_names"].as_array().unwrap();

    let mut res: Vec<f64> = vec![];
    let nd = names.len() as f64;

    for name in names {
        let mut found = false;
        for word in paper {
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

pub fn cosine_similarity(v_a: &Vec<f64>, v_b: &Vec<f64>) -> f64 {
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

pub fn get_global_similarity(id: String, tf_array: &Vec<Value>) -> Vec<(f64, String)> {
    let store = data::open_data();
    let papers = store["paper"].as_array().unwrap().to_vec();

    let cur_tf_idf_array = get_tf_idf_array(tf_array, &store);
    let mut res = vec![];

    for paper in papers {
        if paper["i"].as_str().unwrap().to_string() != id {
            let tar_tf_idf_array =
                get_tf_idf_array(&paper["t"].as_array().unwrap().to_vec(), &store);

            res.push((
                cosine_similarity(&cur_tf_idf_array, &tar_tf_idf_array),
                paper["i"].as_str().unwrap().to_string(),
            ));
        }
    }

    // Sort & put NaN to last
    res.sort_by(|a, b| -> Ordering {
        if a.0.is_nan() || a.0 < b.0 {
            Ordering::Greater
        } else if b.0.is_nan() || a.0 > b.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let mut ret = vec![];
    for i in 0..std::cmp::min(5, res.len()) {
        ret.push(res[i].clone());
    }

    ret
}
