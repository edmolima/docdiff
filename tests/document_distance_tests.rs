use docdiff::algorithm::document_distance::document_distance;

#[test]
fn identical_texts_have_zero_distance() {
    let a = "rust is awesome";
    let b = "rust is awesome";
    let dist = document_distance(a, b);
    assert!((dist - 0.0).abs() < 1e-8);
}

#[test]
fn completely_different_texts_have_high_distance() {
    let a = "rust";
    let b = "python";
    let dist = document_distance(a, b);
    assert!(dist > 0.99);
}

#[test]
fn similar_texts_have_low_distance() {
    let a = "rust is fast";
    let b = "rust is reliable";
    let dist = document_distance(a, b);
    assert!(dist < 1.0 && dist > 0.0);
}

#[test]
fn empty_texts_have_max_distance() {
    let a = "";
    let b = "";
    let dist = document_distance(a, b);
    assert!((dist - 1.0).abs() < 1e-8);
}
