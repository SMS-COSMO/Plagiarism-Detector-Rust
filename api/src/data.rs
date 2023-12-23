/// Get stopword list from stopwords.txt
pub fn stop_words() -> std::collections::HashSet<String> {
    let mut words = std::collections::HashSet::new();
    for item in String::from_utf8_lossy(include_bytes!("../../stopwords.txt")).split_whitespace() {
        words.insert(item.to_string());
    }

    words
}
