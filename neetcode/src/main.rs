mod arrays_and_hashing;

fn main() {
    let numbers = vec![1, 2, 3, 4, 2];
    let numbers2 = vec![1, 2, 3, 4, 5];
    assert!(arrays_and_hashing::contains_duplicate(numbers));
    assert!(!arrays_and_hashing::contains_duplicate(numbers2));

    let word = "anagram".to_string();
    let word2 = "naamarg".to_string();
    assert!(arrays_and_hashing::is_anagram(word, word2));

    let word = "word".to_string();
    let word2 = "this".to_string();
    assert!(!arrays_and_hashing::is_anagram(word, word2));

    println!("Success!");
    
    let nums = vec![2,7,11,15];
    assert_eq!(arrays_and_hashing::two_sum(nums, 9), [1, 0])
}
