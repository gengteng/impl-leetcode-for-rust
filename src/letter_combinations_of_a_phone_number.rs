/// # 17. Letter Combinations of a Phone Number
///
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
///
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
///
/// [http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png]
///
/// Example:
///
/// Input: "23"
/// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
/// Note:
///
/// Although the above answer is in lexicographical order, your answer could be in any order you want.
use std::collections::HashSet;

pub trait LetterCombinationsOfAPhoneNumber {
    fn letter_combinations(digits: &str) -> HashSet<String>;
}

pub struct Solution1;

impl LetterCombinationsOfAPhoneNumber for Solution1 {
    fn letter_combinations(digits: &str) -> HashSet<String> {
        let table = [
            None,
            None, Some("abc"), Some("def"),
            Some("ghi"), Some("jkl"), Some("mno"),
            Some("pqrs"), Some("tuv"), Some("wxyz")
        ];

        let mut result = HashSet::<String>::new();

        for d in digits.as_bytes().iter().map(|b| (b - b'0') as usize).into_iter() {
            match table.get(d) {
                Some(Some(s)) => {
                    if result.len() > 0 {

                        let mut set = HashSet::<String>::new();
                        for c in s.chars() {
                            let mut sub = HashSet::new();
                            for item in result.iter() {
                                let mut i = item.clone();
                                i.push(c);
                                sub.insert(i);
                            }

                            for s in sub.iter() {
                                set.insert(s.clone());
                            }
                        }
                        result.clear();
                        for s in set.iter() {
                            result.insert(s.clone());
                        }
                    } else {
                        for c in s.chars() {
                            let mut f = String::with_capacity(digits.len());
                            f.push(c);
                            result.insert(f);
                        }
                    }
                },
                _ => {

                }
            }
        }

        result
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::letter_combinations("23"), {
        let mut set = HashSet::new();
        ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().for_each(|&s| {
            set.insert(s.to_string());
        });
        set
    });
}