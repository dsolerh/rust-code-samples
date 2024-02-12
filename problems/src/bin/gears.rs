use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut buf = BufReader::new(File::open("data.txt").unwrap());

    let mut prev_line = String::new();
    let mut current_line;
    let mut next_line;

    let mut total = 0;

    if let Some(line) = read_line(&mut buf) {
        current_line = line;
        while !current_line.is_empty() {
            next_line = read_line(&mut buf).unwrap_or_default();

            total += calculate_gear_sum((&prev_line, &current_line, &next_line));

            prev_line = current_line;
            current_line = next_line;
        }
    }

    println!("The total is: {total}")
}

fn read_line(buf: &mut BufReader<File>) -> Option<String> {
    let mut data = String::new();
    match buf.read_line(&mut data) {
        Err(_) => None,
        Ok(n) => {
            if n == 0 {
                return None;
            }
            Some(data)
        }
    }
}

#[derive(Debug, PartialEq)]
struct NumInLine(i32, usize, usize);

fn calculate_gear_sum(lines: (&str, &str, &str)) -> i32 {
    let (prev, current, next) = lines;

    current
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == '*' {
                return Some(calculate_gear_ratio((prev, current, next), i));
            }
            None
        })
        .sum()
}

fn process_num_in_line(line: &str) -> Vec<NumInLine> {
    let mut start = 0;
    line.split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| {
            x.parse::<i32>().ok().and_then(|n| {
                let index = find_number_pos(line, x, start).unwrap();
                let end = index + x.len() - 1;
                start = end + 1;
                Some(NumInLine(n, index, end))
            })
        })
        .collect::<Vec<_>>()
}

fn find_number_pos(line: &str, num: &str, start: usize) -> Option<usize> {
    let line_chars = line.chars().collect::<Vec<_>>();
    let num_chars = num.chars().collect::<Vec<_>>();
    let mut start = start;
    // let line_char_len = line_chars.len();
    let num_len = num_chars.len();
    while let Some(sub_str) = line_chars.get(start..start + num_len) {
        if cmp_chars(sub_str, &num_chars) {
            // if the value after the sub_str is a number then keep looking.
            if line_chars
                .get(start + num_len)
                .and_then(|v| Some(v.is_ascii_digit()))
                .unwrap_or_default()
            {
                start += 1;
                continue;
            }

            if start == 0
                || !line_chars
                    .get(start - 1)
                    .and_then(|v| Some(v.is_ascii_digit()))
                    .unwrap_or_default()
            {
                return Some(start);
            }
        }
        start += 1
    }
    None
}

fn cmp_chars(ch1: &[char], ch2: &[char]) -> bool {
    if ch1.len() != ch2.len() {
        return false;
    }

    !ch1.iter().zip(ch2.iter()).any(|(c1, c2)| c1 != c2)
}

fn calculate_gear_ratio(lines: (&str, &str, &str), index: usize) -> i32 {
    let (prev, current, next) = lines;

    let nums = process_num_in_line(prev)
        .into_iter()
        .chain(process_num_in_line(current).into_iter())
        .chain(process_num_in_line(next).into_iter())
        .filter(|n| is_neighbor(n, index))
        .collect::<Vec<_>>();

    if nums.len() == 2 {
        return nums.into_iter().map(|v| v.0).fold(1, |acc, x| acc * x);
    }
    0
}

fn is_neighbor(n: &NumInLine, index: usize) -> bool {
    index >= n.1 && index <= n.2
        || index + 1 >= n.1 && index + 1 <= n.2
        || (index > 0 && index - 1 >= n.1 && index - 1 <= n.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gear_sum() {
        let p = ".......497...........................858...923...128..................227..801........487.....664...........................................";
        let c = "436........765..............140.......+....................859.............*.........+.................960........668.......................";
        let n = "...*982...........=..........=....203......266.263...375*....=...402....691..-....................*..........575....................13......";
        assert_eq!(calculate_gear_sum((p, c, n)), 801 * 691);

        let p = "...464../..................608..............*........*................$.94*........................................486.......172............";
        let c = "...*......121........=.....*.......$......909.......583............128........929...................765.............*............+....24....";
        let n = ".............*.......762....230..422..285.....434......................390.....=.....921.................261....#...949.....382...903.*.....";
        assert_eq!(calculate_gear_sum((p, c, n)), 608 * 230 + 486 * 949);

        let p = ".......*............@319.................$.35..........*.....207...............808*.................933.737...........*.502............367..";
        let c = ".....807....219../.....................436......226@...266........215...................................*...................................";
        let n="..............*...769.....41*....922............................$..*.......271.........350..........677.346......................662....725.";
        assert_eq!(calculate_gear_sum((p, c, n)), 737 * 346);

        let p = ".............*..............893..581..534........376..+..........174.119.........138............670...*....$.834............4...............";
        let c = ".....+....234...904.323..=..............&.894..........439.@838............*.....*........47........555........*............*........932....";
        let n = "...594............*...$.654.....+.........*........................84.*....7...231.$102.......286.......*760....817.......719.........*.....";
        assert_eq!(
            calculate_gear_sum((p, c, n)),
            138 * 231 + 834 * 817 + 719 * 4
        );
    }

    #[test]
    fn test_process_num_in_line() {
        let line = ".......497..............+...........+858...923...128..................227..801........487.....664...........................................";
        let got = process_num_in_line(line);
        let expect = vec![
            NumInLine(497, 7, 9),
            NumInLine(858, 37, 39),
            NumInLine(923, 43, 45),
            NumInLine(128, 49, 51),
            NumInLine(227, 70, 72),
            NumInLine(801, 75, 77),
            NumInLine(487, 86, 88),
            NumInLine(664, 94, 96),
        ];
        assert_eq!(got, expect);

        let line = ".............114...588...............*............*......631........*.......952...463..14.......661..........=...706......*333.........595..";
        let got = process_num_in_line(line);
        let expect = vec![
            NumInLine(114, 13, 15),
            NumInLine(588, 19, 21),
            NumInLine(631, 57, 59),
            NumInLine(952, 76, 78),
            NumInLine(463, 82, 84),
            NumInLine(14, 87, 88),
            NumInLine(661, 96, 98),
            NumInLine(706, 113, 115),
            NumInLine(333, 123, 125),
            NumInLine(595, 135, 137),
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_cmp_chars() {
        assert!(cmp_chars(&['a', 'b'], &['a', 'b']), "expect to be true");
        assert!(!cmp_chars(&['a', 'b'], &['a', 'c']), "expect to be true");
    }

    #[test]
    fn test_find_num_pos() {
        assert_eq!(find_number_pos("23...23", "23", 0), Some(0));
        assert_eq!(find_number_pos("123...23", "23", 0), Some(6));
        assert_eq!(find_number_pos("123...234...23", "23", 0), Some(12));
        assert_eq!(find_number_pos(".23...234...23", "23", 5), Some(12));
        assert_eq!(find_number_pos("12323423", "23", 0), None);
    }

    #[test]
    fn test_is_neighbor() {
        // ...300...
        // .........

        assert!(
            !is_neighbor(&NumInLine(300, 3, 5), 0),
            "NumInLine(300,3,5) is not neighbor in index 0"
        );
        assert!(
            !is_neighbor(&NumInLine(300, 3, 5), 1),
            "NumInLine(300,3,5) is not neighbor in index 1"
        );
        assert!(
            is_neighbor(&NumInLine(300, 3, 5), 2),
            "NumInLine(300,3,5) is neighbor in index 2"
        );
        assert!(
            is_neighbor(&NumInLine(300, 3, 5), 3),
            "NumInLine(300,3,5) is neighbor in index 3"
        );
        assert!(
            is_neighbor(&NumInLine(300, 3, 5), 4),
            "NumInLine(300,3,5) is neighbor in index 4"
        );
        assert!(
            is_neighbor(&NumInLine(300, 3, 5), 5),
            "NumInLine(300,3,5) is neighbor in index 5"
        );
        assert!(
            is_neighbor(&NumInLine(300, 3, 5), 6),
            "NumInLine(300,3,5) is neighbor in index 6"
        );
        assert!(
            !is_neighbor(&NumInLine(300, 3, 5), 7),
            "NumInLine(300,3,5) is not neighbor in index 7"
        );
        assert!(
            !is_neighbor(&NumInLine(300, 3, 5), 8),
            "NumInLine(300,3,5) is not neighbor in index 8"
        );
    }
}
