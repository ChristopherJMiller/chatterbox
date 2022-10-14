use std::{fs, path::PathBuf};

use post::{PostFile, PostEntry};
use tracing::{info, warn};

use crate::{post::Post, image::PostImage};

mod post;
mod image;

fn prepare_output_dir() {
  info!("Preparing Output Directories");
  fs::remove_dir_all("out").unwrap_or_else(|_| {});
  fs::create_dir_all("out/images").expect("Failed to create output dir");
  fs::create_dir_all("out/posts").expect("Failed to create output dir");
}

fn main() {
  tracing_subscriber::fmt::init();
  info!("Hello Chatterbox!");

  prepare_output_dir();
  let post_files = PostFile::from_glob("posts/*.md");
  info!("Found {} Post Files", post_files.len());
  let post_tuple = post_files.into_iter().map(|pf| (pf.name, Post::try_from(pf.file).unwrap())).collect::<Vec<_>>();
  let image_paths = post_tuple.iter().map(|(_, post)| post.real_cover_photo.clone());
  
  let entries = post_tuple.iter().map(|(name, post)| {
    post.save(name.clone());
    post.post_entry(name.clone())
  }).collect::<Vec<_>>();

  image_paths.map(|path| {
    info!("Loading {}", path.clone());
    PostImage::try_from(PathBuf::from(path.clone()))
  })
    .filter_map(|i| i.map_err(|e| warn!("{}", e)).ok())
    .for_each(|i| {
      info!("Saving {}", i.name);
      i.save();
    });

  info!("Saving Entries File");
  PostEntry::save_entries(entries);
}
