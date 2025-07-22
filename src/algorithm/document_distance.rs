use std::collections::{HashMap, HashSet};

pub fn document_distance(a: &str, b: &str) -> f64 {
    let freq_a = get_word_frequencies(a);
    let freq_b = get_word_frequencies(b);

    let mut unified_vocab: HashSet<String> = HashSet::new();
    for word in freq_a.keys() {
        unified_vocab.insert(word.clone());
    }
    for word in freq_b.keys() {
        unified_vocab.insert(word.clone());
    }

    if unified_vocab.is_empty() {
        return 1.0;
    }

    let mut vec_a: Vec<f64> = Vec::with_capacity(unified_vocab.len());
    let mut vec_b: Vec<f64> = Vec::with_capacity(unified_vocab.len());

    let mut sorted_vocab: Vec<&String> = unified_vocab.iter().collect();
    sorted_vocab.sort_unstable();

    for word in sorted_vocab {
        let count_a = *freq_a.get(word).unwrap_or(&0.0);
        let count_b = *freq_b.get(word).unwrap_or(&0.0);
        vec_a.push(count_a);
        vec_b.push(count_b);
    }

    let dot_product: f64 = vec_a.iter().zip(vec_b.iter()).map(|(x, y)| x * y).sum();

    let norm = |v: &[f64]| v.iter().map(|x| x * x).sum::<f64>().sqrt();

    let norm_a = norm(&vec_a);
    let norm_b = norm(&vec_b);

    if norm_a == 0.0 || norm_b == 0.0 {
        return 1.0;
    }

    let cosine_similarity = dot_product / (norm_a * norm_b);

    (1.0 - cosine_similarity).max(0.0).min(1.0)
}

fn get_word_frequencies(text: &str) -> HashMap<String, f64> {
    let mut word_counts: HashMap<String, f64> = HashMap::new();

    text.split_whitespace()
        .filter(|s| !s.is_empty())
        .for_each(|word| {
            let cleaned_word: String = word
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();

            if !cleaned_word.is_empty() {
                *word_counts.entry(cleaned_word).or_insert(0.0) += 1.0;
            }
        });

    word_counts
}
