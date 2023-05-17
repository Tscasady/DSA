mod arrays_and_hashing;

fn main() {
    let numbers = vec![1, 2, 3, 4, 2];
    let numbers2 = vec![1, 2, 3, 4, 5];
    assert!(arrays_and_hashing::contains_duplicate(numbers));
    assert!(!arrays_and_hashing::contains_duplicate(numbers2));

    let word = "anagram";
    let word2 = "naamarg";
    assert!(arrays_and_hashing::is_anagram(word, word2));

    let word = "word";
    let word2 = "this";
    assert!(!arrays_and_hashing::is_anagram(word, word2));

    println!("Success!");
    
    let nums = vec![2,7,11,15];
    assert_eq!(arrays_and_hashing::two_sum(nums, 9), [1, 0]);

    let not_palin = "0P".to_string();
    assert!(!arrays_and_hashing::is_palindrome(not_palin));

    let palin = "dad".to_string();
    assert!(arrays_and_hashing::is_palindrome(palin))
}
