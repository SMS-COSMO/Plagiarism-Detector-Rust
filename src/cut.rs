use crate::data;
use jieba_rs::Jieba;

pub fn cut<'a>(text: &'a String) -> Vec<&'a str> {
    let jieba = Jieba::new();

    let stop_words = data::get_stop_words();
    let sep_list = jieba.cut(text.as_str(), false).to_vec();
    let mut res_list = vec![];

    for word in sep_list {
        if !stop_words.contains(&word.to_string()) {
            res_list.push(word);
        }
    }

    res_list
}
