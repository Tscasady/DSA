mod arrays_and_hashing;
mod stack;
mod binary_search;
mod two_pointer;
mod linked_list;
mod sliding_window;
mod tree;

fn main() {
    let numbers = vec![1, 2, 3, 4, 2];
    let numbers2 = vec![1, 2, 3, 4, 5];
    assert!(arrays_and_hashing::contains_duplicate(&numbers));
    assert!(!arrays_and_hashing::contains_duplicate(&numbers2));

    let word = "anagram";
    let word2 = "naamarg";
    assert!(arrays_and_hashing::is_anagram(word, word2));

    let word = "word";
    let word2 = "this";
    assert!(!arrays_and_hashing::is_anagram(word, word2));


    let nums = vec![2,7,11,15];
    assert_eq!(arrays_and_hashing::two_sum(nums, 9), [1, 0]);

    let not_palin = "0P".to_string();
    assert!(!two_pointer::is_palindrome(not_palin));

    let palin = "dad".to_string();
    assert!(two_pointer::is_palindrome(palin));

    let nums = vec![1, 1, 1, 2, 2, 3, 4];
    assert_eq!(arrays_and_hashing::top_k_frequent(nums, 2), [1, 2]);

    let nums = vec![1, 2, 3 ,4];
    assert_eq!(arrays_and_hashing::product_except_self(nums), vec![24, 12, 8, 6]);

    let test = String::from("()[{}]");
    let test2 = String::from("(])");
    assert!(stack::is_valid(test));
    assert!(!stack::is_valid(test2));

    let tokens = vec!["3".to_string(), "4".to_string(), "+".to_string()];
    assert_eq!(stack::eval_rpn(tokens), 7);
    
    let parens = vec!["()()", "(())"];
    assert_eq!(stack::generate_parenthesis(2), parens);

    let nums = vec![73,74,75,71,69,72,76,73];
    assert_eq!(stack::daily_temperature(nums), vec![1, 1, 4, 2, 1, 1, 0, 0]);

    let board: Vec<Vec<char>> = vec![vec!['5','3','.','.','7','.','.','.','.']
,vec!['6','.','.','1','9','5','.','.','.']
,vec!['.','9','8','.','.','.','.','6','.']
,vec!['8','.','.','.','6','.','.','.','3']
,vec!['4','.','.','8','.','3','.','.','1']
,vec!['7','.','.','.','2','.','.','.','6']
,vec!['.','6','.','.','.','.','2','8','.']
,vec!['.','.','.','4','1','9','.','.','5']
,vec!['.','.','.','.','8','.','.','7','9']];
    assert!(arrays_and_hashing::is_valid_sudoku(board));

    let nums = vec![0,3,7,2,5,8,4,6,0,1];
    // let nums = vec![100, 4, 200, 1, 2, 3];
    assert_eq!(arrays_and_hashing::longest_consecutive(nums), 9);

    let nums = vec![2,3,4];
    assert_eq!(two_pointer::two_sum(nums, 6), vec![1, 3]);
    println!("Success!")
}

// you can sort the array

