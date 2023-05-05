use std::{fs, path::PathBuf, env};

use chrono::NaiveDateTime;
use post::{PostFile, PostEntry};
use tracing::{info, warn};

use crate::{post::Post, image::PostImage, notion::Notion};

mod post;
mod image;
mod notion;

fn prepare_output_dir() {
  info!("Preparing Output Directories");
  fs::remove_dir_all("out").unwrap_or_else(|_| {});
  fs::create_dir_all("out/images").expect("Failed to create output dir");
  fs::create_dir_all("out/posts").expect("Failed to create output dir");
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  info!("Hello Chatterbox!");

  let notion_token = env::var("NOTION_TOKEN").expect("Missing NOTION_TOKEN");
  let database_id = env::var("DATABASE_ID").expect("Missing DATABASE_ID");

  let notion = Notion::new(notion_token);
  let pages = notion.get_database(database_id).await;

  for page in pages {
    println!("{:?}", notion.get_blocks(page.id).await);
  }

  return;

  prepare_output_dir();
  let post_files = PostFile::from_glob("posts/*.md");
  info!("Found {} Post Files", post_files.len());
  let mut post_tuple = post_files.into_iter().map(|pf| (pf.name, Post::try_from(pf.file).unwrap())).collect::<Vec<_>>();
  post_tuple.sort_by(|(_, post_a), (_, post_b)| {
    let a = NaiveDateTime::parse_from_str(&post_a.date, "%B %e, %Y %-I:%M %p").unwrap();
    let b = NaiveDateTime::parse_from_str(&post_b.date, "%B %e, %Y %-I:%M %p").unwrap();

    b.cmp(&a)
  });

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
