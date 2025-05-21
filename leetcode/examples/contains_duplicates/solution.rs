use std::collections::HashSet;

fn main() {
    let nums = vec![1, 2, 3, 3];
    println!(
        "contains_duplicates({:?}) = {}",
        nums,
        contains_duplicates(&nums)
    );
    let nums = vec![1, 2, 3, 4];
    println!(
        "contains_duplicates({:?}) = {}",
        nums,
        contains_duplicates(&nums)
    );
}

fn contains_duplicates(nums: &[i32]) -> bool {
    let mut num_set = HashSet::new();
    for n in nums {
        if num_set.contains(&n) {
            return true;
        }
        num_set.insert(n);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::contains_duplicates;
    use rstest::rstest;

    #[rstest]
    #[case(&[1,2,3,3], true)]
    #[case(&[1,2,3,4], false)]
    #[case(&[1], false)]
    fn test_contains_duplicates(#[case] nums: &[i32], #[case] has_dup: bool) {
        let dup = contains_duplicates(nums);
        assert_eq!(has_dup, dup)
    }
}
