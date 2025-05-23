use std::collections::HashMap;

#[derive(Debug)]
pub struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1
    }

    pub fn display(self) {
      for (key, value) in &self.0 {
          println!("{}: {}", key, value)
      }
    }
}
