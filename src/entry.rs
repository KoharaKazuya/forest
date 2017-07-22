pub type Depth = usize;

#[derive(PartialEq, Debug)]
pub enum Shape {
  Open,
  Close,
  Pending,
}

pub type Name = String;

#[derive(PartialEq, Debug)]
pub struct Entry {
  pub depth: Depth,
  pub shape: Shape,
  pub name: Name,
}

impl Entry {
  pub fn new(depth: usize, shape: Shape, name: &str) -> Entry {
    Entry {
      depth: depth,
      shape: shape,
      name: name.to_string(),
    }
  }
}

pub type Tree = Vec<Entry>;
