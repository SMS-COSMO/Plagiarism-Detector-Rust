use std::cmp::Ordering;

use ::entity::req::*;
use jieba_rs::Jieba;
use migration::{sea_orm::DbConn, DbErr};
use plagiarism_detector_rust_service::{Mutation, Query};
use serde_json::{json, Value};

/// Cut paper with jieba
pub fn cut<'a>(
    text: &'a str,
    jieba: &Jieba,
    stop_words: &std::collections::HashSet<String>,
) -> Vec<&'a str> {
    jieba
        .cut(text, false)
        .into_iter()
        .filter(|word| !stop_words.contains(*word))
        .collect()
}

pub async fn similarity(
    req: &ReqData,
    update_db: bool,
    db: &DbConn,
    jieba: &jieba_rs::Jieba,
    stop_words: &std::collections::HashSet<String>,
) -> Result<Vec<(f64, String)>, DbErr> {
    // Remove whitespace
    let trimmed: String = req.text.chars().filter(|c| !c.is_whitespace()).collect();
    // Cut text
    let sep_text = cut(&trimmed, jieba, stop_words);
    // Get tf array of current text
    let tf_array = get_tf_array(&sep_text);

    if update_db {
        // Add paper
        if let Err(e) = Mutation::add_paper(db, &req.id, &tf_array).await {
            return Err(e);
        }
        // Update df
        if let Err(e) = update_feature_names(&sep_text, db).await {
            return Err(e);
        }
    }

    // Get result
    Ok(global_similarity(&req.id, &tf_array, db).await)
}

pub async fn update_feature_names(sep_text: &[&str], db: &DbConn) -> Result<(), DbErr> {
    let mut names = Query::list_names(db).await.unwrap();

    for word in sep_text {
        let mut found = false;
        for name in &mut names {
            if name.name == *word {
                found = true;
                if let Err(e) = Mutation::update_name(db, &name.name, name.df + 1).await {
                    return Err(e);
                }
                name.df += 1;
                break;
            }
        }

        if !found {
            if let Err(e) = Mutation::add_name(db, &word, 1).await {
                return Err(e);
            }
            // dummy indicator to avoid the same word being added repeatedly
            names.push(entity::name::Model {
                id: 0,
                name: String::from(*word),
                df: 1,
            });
        }
    }

    Ok(())
}

pub fn get_tf_array(sep_text: &[&str]) -> Vec<Value> {
    let mut sorted_sep_text = sep_text.to_vec();
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

/// Get the tf-idf(smooth) characteristics
pub async fn get_tf_idf_array(paper: &[Value], db: &DbConn) -> Vec<f64> {
    let names = Query::list_names(db).await.unwrap();

    let mut res: Vec<f64> = vec![];
    let nd = names.len() as f64;

    for name in names {
        let mut found = false;
        for word in paper {
            if word["n"].as_str().unwrap() == name.name.as_str() {
                // tf-idf = tf * idf
                // idf(smooth) = ln((1 + nd) / (1 + df)) + 1
                let tf = word["t"].as_f64().unwrap() / paper.len() as f64;
                let idf = 1.0 + f64::log10(nd / (1.0 + name.df as f64));

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

pub fn cosine_similarity(v_a: &[f64], v_b: &[f64]) -> f64 {
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

pub async fn global_similarity(id: &str, tf_array: &[Value], db: &DbConn) -> Vec<(f64, String)> {
    let papers = Query::list_papers(db).await.unwrap();

    let cur_tf_idf_array = get_tf_idf_array(tf_array, db).await;
    let mut res = vec![];

    for paper in papers {
        if paper.pid != id {
            let j: Value = serde_json::from_str(&paper.text).unwrap();
            let tar_tf_idf_array = get_tf_idf_array(j.as_array().unwrap(), db).await;

            res.push((
                cosine_similarity(&cur_tf_idf_array, &tar_tf_idf_array),
                paper.pid,
            ));
        }
    }

    // Sort and put NaN to last
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
