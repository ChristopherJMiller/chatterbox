use std::{fs::{File, self}, io::{Read, Write}};

use glob::glob;
use serde::Serialize;

use crate::post::Post;

mod post;

fn get_post_files<'a>() -> Vec<(String, String)> {
  glob("posts/*.md").expect("Failed to read glob pattern")
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap())
    .map(|path| {
      let mut file = File::open(path.clone()).expect("Failed to open file");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file");
      (path.with_extension("").file_name().unwrap().to_string_lossy().to_string(), contents)
    }).collect()
}

fn prepare_output_dir() {
  fs::remove_dir_all("out").unwrap_or_else(|_| {});
  fs::create_dir_all("out/images").expect("Failed to create output dir");
  fs::create_dir_all("out/posts").expect("Failed to create output dir");
}

#[derive(Serialize)]
pub struct PostEntry {
  name: String,
  description: String,
  path: String,
}

fn main() {
  prepare_output_dir();
  let mut entries = Vec::new();
  for (name, file) in get_post_files() {
    if let Ok(post) = Post::try_from(file) {
      if let Ok(post_json) = serde_json::to_string(&post) {
        let mut post_file = File::create(format!("out/posts/{}.json", name)).unwrap();
        post_file.write_all(post_json.replace("’", "'").as_bytes()).unwrap();

        let mut description = post.content
          .replace("#", "")
          .replace("\n", " ")
          .split(" ")
          .take(25)
          .collect::<Vec<_>>()
          .join(" ");

        description.push_str("...");
        entries.push(PostEntry {
          name: post.title,
          description,
          path: name,
        });
      }
    }
  }

  let mut index_file = File::create("out/index.json").unwrap();
  let index_json = serde_json::to_string(&entries).unwrap();
  index_file.write_all(index_json.replace("’", "'").as_bytes()).unwrap();
}
