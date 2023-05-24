mod arrays_and_hashing;
mod stack;

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


    let nums = vec![2,7,11,15];
    assert_eq!(arrays_and_hashing::two_sum(nums, 9), [1, 0]);

    let not_palin = "0P".to_string();
    assert!(!arrays_and_hashing::is_palindrome(not_palin));

    let palin = "dad".to_string();
    assert!(arrays_and_hashing::is_palindrome(palin));

    let nums = vec![1, 1, 1, 2, 2, 3, 4];
    assert_eq!(arrays_and_hashing::top_k_frequent(nums, 2), [1, 2]);

    let nums = vec![1, 2, 3 ,4];
    assert_eq!(arrays_and_hashing::product_except_self(nums), vec![24, 12, 8, 6]);

    // let test = String::from("()[{}]");
    let test2 = String::from("(])");
    // assert!(stack::is_valid(test));
    assert!(!stack::is_valid(test2));
    println!("Success!");
}
