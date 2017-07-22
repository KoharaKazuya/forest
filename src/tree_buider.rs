use entry::*;

pub struct Builder {
  entries: Tree,
  ancestors: Vec<usize>,
  prev: Option<usize>,
}

impl Builder {
  pub fn new() -> Builder {
    Builder {
      entries: vec![],
      ancestors: vec![],
      prev: None,
    }
  }

  pub fn append(&mut self, line: &str) {
    let mut name = line.to_string();
    while self.ancestors.len() > 0 {
      let ancestors_name = self.ancestors.iter().fold(String::new(), |accu, a| {
        accu + &self.entries[*a].name
      });
      if name.starts_with(&ancestors_name) {
        name = name.split_at(ancestors_name.len()).1.to_string();
        break
      }
      let a = self.ancestors.pop().unwrap();
      self.entries[a].shape = Shape::Close;
      self.prev = Some(a);
    }
    let entry = Entry::new(self.ancestors.len() + 1, Shape::Pending, &name);
    if let Some(p) = self.prev {
      self.prev = None;
      self.entries[p].shape = Shape::Open;
    }
    self.ancestors.push(self.entries.len());
    self.entries.push(entry);
  }

  pub fn build(mut self) -> Tree {
    self.prev = None;
    for a in self.ancestors {
      self.entries[a].shape = Shape::Close;
    }
    self.entries
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::io::{BufRead, BufReader};

  /// データ配列から Tree を生成する
  fn build_tree(data: Vec<(Depth, Shape, &str)>) -> Tree {
    data.into_iter().map(|(depth, shape, name)| {
      Entry::new(depth, shape, name)
    }).collect()
  }

  /// 文字列から Builder で Tree を生成する
  fn build_from_str(text: &str) -> Tree {
    let reader = BufReader::new(text.as_bytes());
    let mut builder = Builder::new();
    for l in reader.lines() {
      builder.append(&l.unwrap());
    }
    builder.build()
  }

  #[test]
  fn test_build_empty_tree() {
    let tree = build_from_str("");
    let expectation = build_tree(vec![]);
    assert_eq!(tree, expectation);
  }

  #[test]
  fn test_build_one_entry() {
    let tree = build_from_str("\
/test
");
    let expectation = build_tree(vec![
      (1, Shape::Close, "/test"),
    ]);
    assert_eq!(tree, expectation);
  }

  #[test]
  fn test_build_simple_parent() {
    let tree = build_from_str("\
/parent
/parent/child
/parent/child/grandchild
");
    let expectation = build_tree(vec![
      (1, Shape::Close, "/parent"),
      (2, Shape::Close, "/child"),
      (3, Shape::Close, "/grandchild"),
    ]);
    assert_eq!(tree, expectation);
  }

  #[test]
  fn test_build_simple_sibling() {
    let tree = build_from_str("\
/prev
/next
");
    let expectation = build_tree(vec![
      (1, Shape::Open, "/prev"),
      (1, Shape::Close, "/next"),
    ]);
    assert_eq!(tree, expectation);
  }

  #[test]
  fn test_build_ancle() {
    let tree = build_from_str("\
/father
/father/me
/ancle
");
    let expectation = build_tree(vec![
      (1, Shape::Open,  "/father"),
      (2, Shape::Close, "/me"),
      (1, Shape::Close, "/ancle"),
    ]);
    assert_eq!(tree, expectation);
  }

  #[test]
  fn test_build_with_space_separator() {
    let tree = build_from_str("\
father
father me
ancle
");
    let expectation = build_tree(vec![
      (1, Shape::Open,  "father"),
      (2, Shape::Close, " me"),
      (1, Shape::Close, "ancle"),
    ]);
    assert_eq!(tree, expectation);
  }
}
