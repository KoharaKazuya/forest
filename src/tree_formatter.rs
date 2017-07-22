use entry::*;
use std::io::Write;

pub struct Formatter<'a> {
  writer: &'a mut Write,
  ancestor_guide_open: String,
  ancestor_guide_close: String,
  name_guide_open: String,
  name_guide_close: String,
}

impl<'a> Formatter<'a> {
  pub fn new(writer: &'a mut Write, ago: &str, agc: &str, ngo: &str, ngc: &str) -> Formatter<'a> {
    Formatter {
      writer: writer,
      ancestor_guide_open: ago.to_string(),
      ancestor_guide_close: agc.to_string(),
      name_guide_open: ngo.to_string(),
      name_guide_close: ngc.to_string(),
    }
  }

  pub fn print(&mut self, tree: Tree) {
    let mut ancestor_shapes = vec![];
    for entry in tree {
      ancestor_shapes.truncate(entry.depth - 1);
      for shape in ancestor_shapes.iter() {
        write!(self.writer, "{}", match *shape {
          Shape::Open => &self.ancestor_guide_open,
          Shape::Close => &self.ancestor_guide_close,
          _ => panic!("Unexpected Internal Value\nPerhaps, internal logic error...\nformatter can treat with only Open/Close Shape"),
        });
      }
      let ng: &str = match entry.shape {
        Shape::Open => &self.name_guide_open,
        Shape::Close => &self.name_guide_close,
        _ => panic!("Unexpected Internal Value\nPerhaps, internal logic error...\nformatter can treat with only Open/Close Shape"),
      };
      write!(self.writer, "{}{}\n", ng, entry.name);
      ancestor_shapes.push(entry.shape);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// データ配列から Tree を生成する
  fn build_tree(data: Vec<(Depth, Shape, &str)>) -> Tree {
    data.into_iter().map(|(depth, shape, name)| {
      Entry::new(depth, shape, name)
    }).collect()
  }

  /// 与えた Tree が出力する文字列のテスト
  fn tree_eq(tree: Tree, output: &str) {
    let mut test_writer = vec![];
    {
      let mut formatter = Formatter::new(&mut test_writer, "| ", "  ", "+ ", "L ");
      formatter.print(tree);
    }
    let result = String::from_utf8(test_writer).unwrap();
    assert_eq!(result, output);
  }

  #[test]
  fn test_print_one_entry() {
    let tree = build_tree(vec![
      (1, Shape::Close, "test"),
    ]);
    tree_eq(tree, "\
L test
");
  }

  #[test]
  fn test_print_simple_parent() {
    let tree = build_tree(vec![
      (1, Shape::Close, "parent"),
      (2, Shape::Close, "child"),
    ]);
    tree_eq(tree, "\
L parent
  L child
");
  }

  #[test]
  fn test_print_simple_sibling() {
    let tree = build_tree(vec![
      (1, Shape::Open,  "prev"),
      (1, Shape::Close, "next"),
    ]);
    tree_eq(tree, "\
+ prev
L next
");
  }

  #[test]
  fn test_print_ancle() {
    let tree = build_tree(vec![
      (1, Shape::Open,  "father"),
      (2, Shape::Close, "me"),
      (1, Shape::Close, "ancle"),
    ]);
    tree_eq(tree, "\
+ father
| L me
L ancle
");
  }
}
