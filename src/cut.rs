use jieba_rs::Jieba;

pub fn cut<'a>(text: &'a String) -> Vec<&'a str>{
    let jieba = Jieba::new();

    jieba.cut(text.as_str(), false).to_vec()
}