use jieba_rs::Jieba;

// Cut paper with jieba
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
