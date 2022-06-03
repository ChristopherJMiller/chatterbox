use std::{fs::File, io::Read};

use glob::glob;
use pulldown_cmark::{Parser};

fn get_post_files<'a>() -> Vec<String> {
  glob("posts/*.md").expect("Failed to read glob pattern")
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap())
    .map(|path| {
      let mut file = File::open(path).expect("Failed to open file");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file");
      contents
    }).collect()
}

fn main() {
  for file in get_post_files() {
    let parser = Parser::new(&file);
    println!("{:?}", parser.into_iter().collect::<Vec<_>>());
  }
}
