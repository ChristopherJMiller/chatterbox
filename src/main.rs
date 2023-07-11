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
  let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

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
