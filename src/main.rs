use std::io::{self, BufRead};

mod entry;
mod tree_buider;
mod tree_formatter;

fn main() {
  let mut builder = tree_buider::Builder::new();
  let stdin = io::stdin();
  let handle = stdin.lock();
  for r in handle.lines() {
    match r {
      Ok(line) => builder.append(&line),
      _ => ()
    }
  }
  let mut stdout = io::stdout();
  let mut formatter = tree_formatter::Formatter::new(
    &mut stdout,
    "│\u{00a0}\u{00a0}\u{00a0}",
    "\u{00a0}\u{00a0}\u{00a0}\u{00a0}",
    "├\u{00a0}─\u{00a0}",
    "└\u{00a0}─\u{00a0}",
  );
  formatter.print(builder.build());
}
