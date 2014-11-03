
pub struct NGram {
  n: uint
}

impl NGram {
  pub fn new(n: uint) -> NGram {
    NGram{n: n}
  }

  pub fn parse(&self, text: &str) -> Vec<String> {
    let padded_text = self.pad(text);
    let mut tokens: Vec<String> = Vec::new();
    let length = padded_text.len() - self.n + 1;
    for i in range(0u, length) {
      tokens.push(padded_text.slice(i, i + self.n).to_string());
    }
    tokens
  }

  fn pad(&self, text: &str) -> String {
    let mut padded = text.to_string();
    for _ in range(0u, self.n-1u) {
      padded = "$".to_string() + padded + "$".to_string();
    }
    return padded;
  }
}

#[test]
fn test_parse () {
  let idx = NGram::new(2);
  assert!(idx.parse("ngram").len() == 6u)
}
