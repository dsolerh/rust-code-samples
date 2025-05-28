fn main() {
    let words = &["neet", "code"];
    let encoded = encode(words);
    println!("{encoded}");
    let decoded = decode(encoded.as_str());
    println!("{decoded:?}");
}

fn encode<S: AsRef<str>>(strs: &[S]) -> String {
    let cap = strs.iter().map(|x| x.as_ref().len()).sum::<usize>() + strs.len();
    let mut encoded = String::with_capacity(cap);

    for s in strs {
        let s = s.as_ref();
        encoded.push(s.len() as u8 as char);
        encoded.push_str(s);
    }

    encoded
}

fn decode<'a>(s: &'a str) -> Vec<&'a str> {
    let mut decoded = vec![];
    let bytes = s.as_bytes();

    let mut i = 0;
    while i < s.len() {
        let l = bytes[i] as usize;
        let w = unsafe { std::str::from_utf8_unchecked(&bytes[i + 1..=i + l]) };
        decoded.push(w);
        i += l + 1;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&["neet","code"])]
    #[case(&["neet","code","love","you"])]
    #[case(&["we","say",":","yes"])]
    #[case(&["1","2"])]
    fn test_encode_decode(#[case] words: &[&str]) {
        let encoded = encode(words);
        let decoded = decode(encoded.as_str());
        assert_eq!(words, &decoded);
    }
}
