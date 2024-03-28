use std::fmt::Debug;

fn main() {
    let mut data = vec!['-', '-', '-'];
    combinationUtil(&['.', '?', '#'], &mut data, 0, 0)
}

fn generate_combinations(arr: &[char]) {}

fn combinationUtil<T: Debug + Copy>(arr: &[T], data: &mut Vec<T>, start: usize, index: usize) {
    if index == arr.len() - 1 {
        for val in data.iter() {
            print!("{:?}", val);
        }
        println!("");
    }

    let mut i = start;
    while i < arr.len() && arr.len() - i >= arr.len() - 1 - index {
        data[index] = arr[i];
        combinationUtil(arr, data, i + 1, index + 1);
        i += 1;
    }
}

// . . . # # #
// . . # . # #
//  
// . # # . . #