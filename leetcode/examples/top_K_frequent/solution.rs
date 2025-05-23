use std::collections::HashMap;

fn main() {
    let nums = &[1, 2, 2, 3, 3, 3];
    let k = 2 as usize;
    println!(
        "top_k_frequent(nums: {:?}, k: {}) = {:?}",
        nums,
        k,
        top_k_frequent(nums, k)
    );
    let nums = &[7, 7];
    let k = 1 as usize;
    println!(
        "top_k_frequent(nums: {:?}, k: {}) = {:?}",
        nums,
        k,
        top_k_frequent_bucket(nums, k)
    );
}

// this solution has O(n log n) time complexity
// it can be done better
fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut data = nums
        .iter()
        .fold(HashMap::new(), |mut map, num| {
            map.entry(num).and_modify(|count| *count += 1).or_insert(1);
            map
        })
        .into_iter()
        .map(|(&key, val)| (key, val))
        .collect::<Vec<_>>();

    data.sort_unstable_by(|(_, count1), (_, count2)| count2.cmp(count1));

    data.into_iter()
        .take(k)
        .map(|(num, _)| num)
        .collect::<Vec<_>>()
}

fn top_k_frequent_bucket(nums: &[i32], k: usize) -> Vec<i32> {
    let count = nums.iter().fold(HashMap::new(), |mut map, num| {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    let mut freq: Vec<Vec<i32>> = vec![Vec::new(); nums.len()];
    for (&num, count) in count {
        freq[count - 1].push(num);
    }
    let mut out = Vec::with_capacity(k);
    for i in (0..freq.len()).rev() {
        for &num in freq[i].iter() {
            out.push(num);
            if out.len() == k {
                return out;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[1,2,2,3,3,3],2,vec![2,3])]
    #[case(&[7,7],1,vec![7])]
    #[case(&[7,7],2,vec![7])]
    #[case(&[7],2,vec![7])]
    fn test_top_k_frequent(#[case] nums: &[i32], #[case] k: usize, #[case] expected: Vec<i32>) {
        let mut got = top_k_frequent(nums, k);
        got.sort(); // sort to get them in the same order
        assert_eq!(expected, got);
    }

    #[rstest]
    #[case(&[1,2,2,3,3,3],2,vec![2,3])]
    #[case(&[7,7],1,vec![7])]
    #[case(&[7,7],2,vec![7])]
    #[case(&[7],2,vec![7])]
    fn test_top_k_frequent_bucket(
        #[case] nums: &[i32],
        #[case] k: usize,
        #[case] expected: Vec<i32>,
    ) {
        let mut got = top_k_frequent_bucket(nums, k);
        got.sort(); // sort to get them in the same order
        assert_eq!(expected, got);
    }

    #[test]
    fn test_rev_range() {
        let got: Vec<i32> = (1..=3).rev().collect();
        assert_eq!(vec![3, 2, 1], got);
    }
}
