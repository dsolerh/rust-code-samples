use std::collections::HashSet;

fn main() {
    let nums = &[1, 3, 5, 7, 9];
    let lseq = longuest_sequence(nums);
    println!("longuest_sequence({:?}) = {}", nums, lseq);
}

fn longuest_sequence(nums: &[i32]) -> i32 {
    let nums_set = nums.iter().map(|x| *x).collect::<HashSet<_>>();

    let mut max_len = 0;
    for &n in nums {
        // check if the number is a start of a sequence
        if !nums_set.contains(&(n - 1)) {
            let mut len = 1;
            while nums_set.contains(&(n + len)) {
                len += 1;
            }
            if len > max_len {
                max_len = len;
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[2,20,4,10,3,4,5], 4)]
    #[case(&[0,3,2,5,4,6,1,1], 7)]
    fn test_longuest_sequence(#[case] nums: &[i32], #[case] expected: i32) {
        let got = longuest_sequence(nums);
        assert_eq!(expected, got);
    }
}
