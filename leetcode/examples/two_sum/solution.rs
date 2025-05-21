fn main() {
    let nums = &[3, 4, 5, 6];
    let target = 7;
    println!(
        "two_sum({:?}, {}) = {:?}",
        nums,
        target,
        two_sum(nums, target)
    );
    let nums = &[4, 5, 6];
    let target = 10;
    println!(
        "two_sum({:?}, {}) = {:?}",
        nums,
        target,
        two_sum(nums, target)
    );
    let nums = &[5, 5];
    let target = 10;
    println!(
        "two_sum({:?}, {}) = {:?}",
        nums,
        target,
        two_sum(nums, target)
    );
}

fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    if nums.len() < 2 {
        return None;
    }
    let mut initial = 0;
    let mut current = 1;
    while initial < nums.len() - 1 {
        if nums[initial] + nums[current] == target {
            return Some((initial as i32, current as i32));
        } else {
            current += 1;
            if current == nums.len() {
                initial += 1;
                current = initial + 1;
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[3,4,5,6], 7, Some((0,1)))]
    #[case(&[4,5,6], 10, Some((0,2)))]
    #[case(&[6,1,1], 7, Some((0,1)))]
    #[case(&[5,5], 10, Some((0,1)))]
    #[case(&[5,4], 10, None)]
    #[case(&[5], 10, None)]
    #[case(&[5], 5, None)]
    fn test_two_sum(
        #[case] nums: &[i32],
        #[case] target: i32,
        #[case] expected: Option<(i32, i32)>,
    ) {
        let got = two_sum(nums, target);
        assert_eq!(got, expected);
    }
}
