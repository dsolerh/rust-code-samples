use async_std::task;

async fn say_hello() {
    let (s, r) = async_channel::unbounded();

    match s.send("Hello").await {
        Ok(_) => {}
        Err(_) => {
            println!("error happened")
        }
    }
    match r.recv().await {
        Ok(data) => println!("{data}"),
        Err(_) => println!("error happend"),
    }
}

fn main() {
    task::block_on(say_hello())
}

// fn main() {
//     // fizzbuzz()

//     // let requests = &[
//     //     "http://example.com".to_string(),
//     //     "https://www.red-bean.com".to_string(),
//     //     "https://en.wikipedia.org/wiki/Main_Page".to_string(),
//     // ];
//     // let results = async_std::task::block_on(many_requests(requests));
//     // for result in results {
//     //     match result {
//     //         Ok(response) => println!("*** {}\n", response),
//     //         Err(err) => eprintln!("error: {}\n", err),
//     //     }
//     // }
//     // math();

//     // let f: f32 = 2.2;
//     // let f = f.powf(2.0);
//     // println!("{}", f);
//     // let mut input = String::new();
//     // io::stdin().read_line(&mut input).unwrap();
//     // let n: i32 = input.parse().unwrap();
//     // println!("difference: {}", difference(5))
//     // println!("{:?}", triple())
//     // let _a = vec![1, 2, 3, 4, 5, 5, 6, 6, 7];
//     // let _a = _a.iter().filter(|&x| *x >= 5).collect::<Vec<_>>();

//     // println!(
//     //     "{}",
//     //     (1..=20)
//     //         .filter(|x| x % 3 == 0 || x % 5 == 0)
//     //         .fold(0, |sum, item| sum + item)
//     // );
//     // println!("{:b}", 1)
// }

// fn intersection(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
//     let v1 = v1.iter().map(|&x| x).collect::<HashSet<_>>();
//     let v2 = v2.iter().map(|&x| x).collect::<HashSet<_>>();
//     v1.intersection(&v2).map(|&x| x).collect::<Vec<_>>()
// }

// fn union(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
//     let v1 = v1.iter().map(|&x| x).collect::<HashSet<_>>();
//     let v2 = v2.iter().map(|&x| x).collect::<HashSet<_>>();
//     v1.union(&v2).map(|&x| x).collect::<Vec<_>>()
// }

// fn multiples_sum(n: i32) -> i32 {
//     let mut sum = 0;
//     for i in 3..=n {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum += i
//         }
//     }
//     sum
// }

// fn total_production(hours: i32, speed: i32) -> i32 {
//     let total = 221 * speed * hours;
//     match speed {
//         1..=4 => total,
//         5..=8 => (0.9 * total as f32) as i32,
//         9..=10 => (0.77 * total as f32) as i32,
//         _ => panic!("unexpected speed value"),
//     }
// }

// fn cars_produced_per_minutes(hours: i32, speed: i32) -> i32 {
//     total_production(hours, speed) / (60 * hours)
// }

// fn palindrome(s: &String) -> bool {
//     let b = s.as_bytes();
//     for i in 0..b.len() / 2 {
//         if b[i] != b[b.len() - 1 - i] {
//             return false;
//         }
//     }
//     true
// }

// fn triple() -> (i32, i32, i32) {
//     for a in 1..=1000 {
//         for b in a + 1..1000 {
//             for c in b + 1..1000 {
//                 if a + b + c == 1000 && a * a + b * b == c * c {
//                     return (a as i32, b as i32, c as i32);
//                 }
//             }
//         }
//     }
//     (0, 0, 0)
// }

// fn can_see_movie(age: i32, permission: bool) -> bool {
//     if age < 0 {
//         return false;
//     }
//     match age {
//         0..=12 => false,
//         13..=17 => permission,
//         _ => true,
//     }
// }

// fn difference(n: i32) -> f32 {
//     let mut square_sum: f32 = 0.0;
//     let mut sum_square: f32 = 0.0;
//     for i in 1..=n {
//         square_sum += i as f32;
//         sum_square += (i as f32).powi(2);
//     }
//     square_sum.powi(2) - sum_square
// }

// fn fizzbuzz() {
//     use std::iter::{once, repeat};

//     let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
//     let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
//     let fizzes_buzzes = fizzes.zip(buzzes);

//     let fizz_buzz = (1..100)
//         .zip(fizzes_buzzes)
//         .inspect(|x| println!("{:?}", x))
//         .map(|tuple| match tuple {
//             (i, ("", "")) => i.to_string(),
//             (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
//         });

//     for line in fizz_buzz {
//         println!("{}", line);
//     }
// }

// fn some_function(mut var: i32) -> i32 {
//     var = 50;
//     var
// }

// fn math() {
//     let mut a = 12;
//     a = some_function(a);
//     println!("{}", a)
// }

// async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
//     let client = surf::Client::new();

//     // urls.iter_mut()
//     //     .map(|url| {
//     //         let req = client.get(&url).recv_string();
//     //         async_std::task::spawn(req)
//     //     })
//     //     .map(|h| async { h.await })
//     //     .collect::<Vec<_>>()

//     let mut handles = vec![];
//     for url in urls {
//         let request = client.get(&url).recv_string();
//         handles.push(async_std::task::spawn(request));
//     }

//     let mut results = vec![];
//     for handle in handles {
//         results.push(handle.await);
//     }
//     results
// }
