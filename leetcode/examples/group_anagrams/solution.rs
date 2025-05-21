use std::collections::HashMap;

fn main() {
    let words = ["act", "pots", "tops", "cat", "stop", "hat"];
    println!("group_anagrams({:?}) = {:?}", words, group_anagrams(&words))
}

fn group_anagrams<S: AsRef<str>>(words: &[S]) -> Vec<Vec<&str>> {
    let mut ana_map: HashMap<[u8; 26], Vec<&str>> = HashMap::new();

    for word in words {
        ana_map
            .entry(get_anahash(word))
            .and_modify(|e| e.push(word.as_ref()))
            .or_insert(vec![word.as_ref()]);
    }

    ana_map.into_iter().map(|(_, v)| v).collect::<Vec<_>>()
}

const ZERO: u8 = 'a' as u8;

fn get_anahash<S: AsRef<str>>(s: S) -> [u8; 26] {
    let mut h = [0; 26];

    for b in s.as_ref().bytes() {
        let idx = b - ZERO;
        h[idx as usize] += 1;
    }

    h
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use super::*;
    use rstest::rstest;

    fn h(pairs: &[(u8, u8)]) -> [u8; 26] {
        let mut h = [0; 26];

        for &(i, v) in pairs {
            h[i as usize] += v
        }

        h
    }

    fn check_groups_equal<S: AsRef<str> + Eq + Ord + Clone>(a: Vec<Vec<S>>, b: Vec<Vec<S>>) {
        // simple check for the lenghts of a and b
        assert_eq!(a.len(), b.len());

        // Convert each inner Vec to a BTreeSet for order-independent comparison
        let mut amount_per_a_set = vec![];
        let a_sets: BTreeSet<BTreeSet<String>> = a
            .into_iter()
            .map(|inner| {
                amount_per_a_set.push(inner.len());
                inner
                    .into_iter()
                    .map(|s| s.as_ref().to_string())
                    .collect::<BTreeSet<_>>()
            })
            .collect();

        let a_amount_set = amount_per_a_set.into_iter().collect::<HashSet<_>>();

        let mut amount_per_b_set = vec![];
        let b_sets: BTreeSet<BTreeSet<String>> = b
            .into_iter()
            .map(|inner| {
                amount_per_b_set.push(inner.len());
                inner
                    .into_iter()
                    .map(|s| s.as_ref().to_string())
                    .collect::<BTreeSet<_>>()
            })
            .collect();

        let b_amount_set = amount_per_b_set.into_iter().collect::<HashSet<_>>();

        // assert the results
        assert_eq!(a_sets, b_sets);
        assert_eq!(a_amount_set, b_amount_set);
    }

    #[rstest]
    #[case(
        &["x"],
        vec![
            vec!["x"]
        ])]
    #[case(
        &["act","pots","tops","cat","stop","hat"],
        vec![
            vec!["hat"],
            vec!["act", "cat"],
            vec!["stop", "pots", "tops"],
        ])]
    fn test_group_anagrams(#[case] words: &[&str], #[case] expected: Vec<Vec<&str>>) {
        let got = group_anagrams(words);
        check_groups_equal(expected, got);
    }

    #[rstest]
    #[case("", [0;26])]
    #[case("abc", h(&[(0,1),(1,1),(2,1)]))]
    #[case("aaa", h(&[(0,3)]))]
    #[case("zzz", h(&[(25,3)]))]
    fn test_get_anahash(#[case] s: &str, #[case] expected: [u8; 26]) {
        let got = get_anahash(s);
        assert_eq!(got, expected);
    }
}
