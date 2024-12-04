fn main() {
    println!("record: {}", calculate_record(DATA).unwrap());
    println!("second record: {}", calculate_record(DATA_2).unwrap());
}

fn calculate_record(data: &str) -> Result<i64, ParseError> {
    Ok(parse_data(data)?
        .map(|(time, distance)| {
            (1..time)
                .map(|x| x * (time - x))
                .filter(|&x| x > distance)
                .count() as i64
        })
        .reduce(|acc, x| acc * x)
        .ok_or(ParseError("error trying to reduce the data".to_string()))?)
}

#[derive(Debug, Clone, PartialEq)]
struct ParseError(String);

fn parse_data(data: &str) -> Result<impl Iterator<Item = (i64, i64)>, ParseError> {
    let mut lines = data.lines().map(str::trim).filter(|&s| !s.is_empty());

    let time = lines
        .next()
        .ok_or(ParseError("could not parse time line".to_string()))?;
    let distance = lines
        .next()
        .ok_or(ParseError("could not parse distance line".to_string()))?;

    let time = time
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|_| ParseError("error parsing one of the times".to_string()))?;

    let distance = distance
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|_| ParseError("error parsing one of the distances".to_string()))?;

    Ok(time.into_iter().zip(distance.into_iter()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let pairs = parse_data(DATA);

        assert!(pairs.is_ok());
        let pairs = pairs.unwrap().collect::<Vec<_>>();
        assert_eq!(pairs, vec![(60, 601), (80, 1163), (86, 1559), (76, 1300)]);
    }

    #[test]
    fn test_calculate_record() {
        let record = calculate_record(TEST_DATA);

        assert_eq!(record, Ok(288));
    }

    const TEST_DATA: &str = r"
    Time:      7  15   30
    Distance:  9  40  200";
}

const DATA: &str = r"
Time:        60     80     86     76
Distance:   601   1163   1559   1300";

const DATA_2: &str = r"
Time:        60808676
Distance:   601116315591300";
