fn main() {
    let nums = vec![1, 2, 4, 6];
    let prod = prod_but_self(&nums);
    println!("nums: {:?} prod: {:?}", nums, prod);
}

fn prod_but_self(nums: &Vec<i32>) -> Vec<i32> {
    let mut res = vec![1i32; nums.len()];

    let mut prefix = 1;
    for i in 0..nums.len() {
        res[i] = prefix;
        prefix *= nums[i];
    }

    let mut postfix = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= postfix;
        postfix *= nums[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,6], vec![48,24,12,8])]
    #[case(vec![-1,0,1,2,3], vec![0,-6,0,0,0])]
    fn test_prod_but_self(#[case] nums: Vec<i32>, #[case] prod: Vec<i32>) {
        let got = prod_but_self(&nums);
        assert_eq!(prod, got);
    }
}
