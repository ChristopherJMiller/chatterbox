use std::{env, time::Duration};
use diesel_async::{AsyncPgConnection, AsyncConnection};
use post::Post;
use tokio::time::sleep;
use tracing::info;

use crate::{notion::Notion, post::CreatePost};

mod image;
mod notion;
mod post;
mod schema;
mod templates;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  info!("Hello Chatterbox!");

  let notion_token = env::var("NOTION_TOKEN").expect("Missing NOTION_TOKEN");
  let database_id = env::var("DATABASE_ID").expect("Missing DATABASE_ID");
  let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
    let username = env::var("DATABASE_USER").expect("No user, but also no datbase url provided");
    let password = env::var("DATABASE_PASSWORD").expect("No password, but also no datbase url provided");
    let host = env::var("DATABASE_HOST").expect("No host, but also no url provided");
    let db = env::var("DATABASE_DB").expect("No DB, but also no url provided");

    format!("postgresql://{}:{}@{}/{}", username, password, host, db)
  });

  let mut conn = AsyncPgConnection::establish(&db_url).await.expect("Unable to connect to Database");

  let notion = Notion::new(notion_token);

  info!("Getting Published Posts");
  let pages = notion.get_database(&database_id).await;
  info!("Found {} published posts", pages.len());

  for page in pages {
    let blocks = notion.get_blocks(&page.id).await;
    sleep(Duration::from_millis(400)).await;
    let post: CreatePost = (page, blocks).into();
    post.reconcile_against_db(&mut conn).await;
  }
}
