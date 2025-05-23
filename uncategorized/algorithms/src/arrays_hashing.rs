use std::collections::{HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort();
    let numset = nums.iter().collect::<HashSet<_>>();
    nums.len() != numset.len()
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s: Vec<_> = s.chars().collect();
    let mut t: Vec<_> = t.chars().collect();

    s.sort();
    t.sort();

    s == t
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut existing_nums = HashMap::new();

    for (index, num) in nums.into_iter().enumerate() {
        let remainder = target - num;

        if let Some(&prev_index) = existing_nums.get(&remainder) {
            return vec![prev_index as i32, index as i32];
        }
        existing_nums.insert(num, index);
    }

    panic!("no two sum solution");
}

// a b c d e f g h i j k l m n o p q r s t u v w x y z
type AnaHash = [u8; 26];

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<AnaHash, Vec<String>> = HashMap::new();

    for word in strs {
        anagrams
            .entry(get_anahash(&word))
            .or_insert_with(Vec::new)
            .push(word);
    }

    anagrams.into_values().collect::<Vec<_>>()
}

fn get_anahash(word: &str) -> AnaHash {
    let mut h: AnaHash = [0; 26];

    for &letter_code in word.as_bytes() {
        h[letter_code as usize - 97] += 1;
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_anahash_letter_are_correct() {
        let word = "abc";
        let expect: AnaHash = {
            let mut ah: AnaHash = [0; 26];
            ah[0] = 1;
            ah[1] = 1;
            ah[2] = 1;
            ah
        };

        assert_eq!(get_anahash(word), expect, "should be equal");
    }

    #[test]
    fn test_get_anahash_letter_do_not_overflow() {
        let word = "xyz";
        let expect: AnaHash = {
            let mut ah: AnaHash = [0; 26];
            ah[23] = 1;
            ah[24] = 1;
            ah[25] = 1;
            ah
        };

        assert_eq!(get_anahash(word), expect, "should be equal");
    }

    #[test]
    fn test_group_anagrams() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected_output = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        assert_eq!(group_anagrams(strs), expected_output);
    }
}
