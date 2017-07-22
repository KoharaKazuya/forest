use std::fmt::{Display, Formatter, Result};
use std::io::{self, BufRead, BufReader};
use std::ops::Index;
use std::vec::Vec;

type NodeId = usize;

#[derive(Debug)]
struct Node<T> {
  value: T,
  parent: Option<NodeId>,
  children: Vec<NodeId>,
  has_next_sibling: bool,
  depth: usize,
}

#[derive(Debug)]
struct Tree<T> {
  nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
  fn new() -> Tree<T> {
    Tree {
      nodes: Vec::new(),
    }
  }

  fn new_node(&mut self, value: T) -> NodeId {
    let id = self.nodes.len();
    let node = Node {
      value: value,
      parent: None,
      children: Vec::new(),
      has_next_sibling: false,
      depth: 0,
    };
    self.nodes.push(node);
    id
  }

  fn append_child(&mut self, parent: NodeId, new_child: NodeId) {
    {
      let l = self.nodes[parent].children.len();
      if l > 0 {
        let s = self.nodes[parent].children[l - 1];
        self.nodes[s].has_next_sibling = true;
      }
      self.nodes[parent].children.push(new_child);
    }
    {
      let mut p = &mut self.nodes[parent];
      p.children.push(new_child);
    }
    {
      let mut c = &mut self.nodes[new_child];
      c.parent = Some(parent);
    }
    {
      self.nodes[new_child].depth = self.nodes[parent].depth + 1;
    }
  }
}

impl<T> Index<usize> for Tree<T> {
  type Output = Node<T>;
  fn index(&self, index: usize) -> &Self::Output {
    &self.nodes[index]
  }
}

impl Display for Tree<String> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut output = String::new();

    let mut open_close = Vec::new();
    for node in self.nodes.iter() {
      while open_close.len() <= node.depth {
        open_close.push(true);
      }
      open_close[node.depth] = node.has_next_sibling;
      for d in 0..node.depth {
        let last = d == node.depth - 1;
        output.push_str(if open_close[d + 1] {
          if last { "├ " } else { "│ " }
        } else {
          if last { "└ " } else { "　 " }
        });
      }
      output.push_str(" ");
      output.push_str(&node.value);
      output.push_str("\n");
    }

    write!(f, "{}", output)
  }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = BufReader::new(handle).lines();

    let mut tree = Tree::new();
    let root_node = tree.new_node("".to_string());
    let mut parents = vec![root_node];
    for line in lines {
      match line {
        Ok(l) => {
          let node_id = tree.new_node(l.to_string());
          {
            let mut new_parents = Vec::new();
            for p in parents {
              if tree[node_id].value.starts_with(&tree[p].value) {
                new_parents.push(p);
              }
            }
            parents = new_parents;
          }
          tree.append_child(parents[parents.len() - 1], node_id);
          parents.push(node_id);
        }
        Err(_) => {
          // TODO: output error
        }
      }
    }
    println!("{}", tree);
}

/*

pattern1: root
└ pattern2: last leaf
root_node
├ pattern3: non-last leaf
│ └ pattern4: non-last node's child
│ 　└ pattern4: non-last node's child
└ leaf_node

output = *line
line   = [guide SP] name LF
guide  = ("│" / "　") SP *("├" / "└")
name   = *CHAR

*/
