use std::collections::HashMap;

fn main() {
    let word1 = "racecar";
    let word2 = "carrace";
    println!(
        "is_anagram({}, {}) = {}",
        word1,
        word2,
        is_anagram(word1, word2)
    );
    let word1 = "jar";
    let word2 = "jam";
    println!(
        "is_anagram({}, {}) = {}",
        word1,
        word2,
        is_anagram(word1, word2)
    );
}

fn is_anagram<S: AsRef<str>>(s: S, t: S) -> bool {
    let mut letters = HashMap::new();
    for letter in s.as_ref().bytes() {
        letters.entry(letter).and_modify(|e| *e += 1).or_insert(1);
    }
    for letter in t.as_ref().bytes() {
        letters.entry(letter).and_modify(|e| *e -= 1).or_insert(-1);
    }
    return letters.values().all(|&v| v == 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("racecar", "carrace", true)]
    #[case("jar", "jam", false)]
    #[case("a", "a", true)]
    #[case("aaa", "a", false)]
    #[case("aaabb", "ab", false)]
    fn test_is_anagram(#[case] word1: &str, #[case] word2: &str, #[case] expected: bool) {
        let got = is_anagram(word1, word2);
        assert_eq!(got, expected);
    }
}
