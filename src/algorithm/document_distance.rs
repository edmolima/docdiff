

use std::collections::HashSet;


pub fn document_distance(a: &str, b: &str) -> f64 {
    // Tokenize input strings into words (as &str)
    let words_a: Vec<&str> = a.split_whitespace().collect();
    let words_b: Vec<&str> = b.split_whitespace().collect();

    // Build vocabulary as a set of unique words
    let vocab: HashSet<&str> = words_a.iter().copied().chain(words_b.iter().copied()).collect();

    // Frequency vector for a document
    let freq = |words: &[&str]| {
        vocab.iter().map(|&w| words.iter().filter(|&&x| x == w).count() as f64).collect::<Vec<f64>>()
    };

    let vec_a = freq(&words_a);
    let vec_b = freq(&words_b);

    // Cosine similarity
    let dot_product = vec_a.iter().zip(vec_b.iter()).map(|(x, y)| x * y).sum::<f64>();
    let norm = |v: &[f64]| v.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_a = norm(&vec_a);
    let norm_b = norm(&vec_b);

    if norm_a == 0.0 || norm_b == 0.0 {
        return 1.0;
    }

    let cosine_similarity = dot_product / (norm_a * norm_b);
    1.0 - cosine_similarity
}
