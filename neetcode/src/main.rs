mod solutions;

fn main() {
    let numbers = vec![1, 2, 3, 4, 2];
    let numbers2 = vec![1, 2, 3, 4, 5];
    assert!(solutions::contains_duplicate(numbers));
    assert!(!solutions::contains_duplicate(numbers2));

    let word = "anagram".to_string();
    let word2 = "naamarg".to_string();
    assert!(solutions::is_anagram(word, word2));

    let word = "word".to_string();
    let word2 = "this".to_string();
    assert!(!solutions::is_anagram(word, word2));

    println!("Success!") 
}
