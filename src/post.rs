use chrono::{NaiveDateTime, DateTime};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tracing::info;
use crate::{schema::posts, notion::{PageObject, BlockObject, RichText}, templates::{Paragraph, H1, H2, H3, Code, UnorderedList, OrderedList, Quote}};

#[derive(Queryable, Debug, AsChangeset)]
pub struct Post {
  pub id: i32,
  pub post_url: String,
  pub posted: NaiveDateTime,
  pub image_url: Option<String>,
  pub title: String,
  pub body: String,
  pub plaintext_body: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct CreatePost {
  pub post_url: String,
  pub posted: NaiveDateTime,
  pub image_url: Option<String>,
  pub title: String,
  pub body: String,
  pub plaintext_body: String,
}

impl CreatePost {
  pub async fn reconcile_against_db(&self, conn: &mut AsyncPgConnection) {
    use crate::schema::posts::dsl::*;

    let existing_post: Result<Post, _> = posts.filter(posted.eq(self.posted)).first::<Post>(conn).await;
    if let Ok(mut existing_post) = existing_post {
      info!("Existing post, updating {}", self.title);
      existing_post.title = self.title.clone();
      existing_post.body = self.body.clone();
      existing_post.image_url = self.image_url.clone();

      diesel::update(posts.filter(id.eq(existing_post.id)))
        .set(existing_post)
        .execute(conn)
        .await
        .unwrap();
    } else {
      info!("Creating new post for {}", self.title);
      self.insert_into(posts).execute(conn).await.unwrap();
    }
  }
}

impl From<(PageObject, Vec<BlockObject>)> for CreatePost {
  fn from((page, blocks): (PageObject, Vec<BlockObject>)) -> Self {
    
    let title = page.properties.name.title[0].plain_text.clone();
    let post_url: String = title.replace(" ", "-").to_ascii_lowercase().chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-').collect();

    Self {
      post_url,
      posted: DateTime::parse_from_rfc3339(&page.created_time).unwrap().naive_local(),
      image_url: None,
      title,
      body: blocks.clone().into_iter().map(String::from).collect::<Vec<_>>().join(""),
      plaintext_body: blocks.into_iter().map(plaintext_block_obj_parse).collect::<Vec<_>>().join("\n")
    }
  }
}

fn rich_text_to_spans(rich_text: &RichText) -> String {
  let annotations = vec![
    (rich_text.annotations.bold, "font-bold"),
    (rich_text.annotations.italic, "italic"),
    (rich_text.annotations.underline, "underline"),
    (rich_text.annotations.code, "font-mono"),
    (rich_text.annotations.strikethrough, "strikethrough"),
  ];

  let to_apply = annotations.into_iter().filter_map(|(apply, class_name)| if apply {
    Some(class_name)
  } else {
    None
  }).collect::<Vec<_>>().join(" ");

  if to_apply.len() > 0 {
    if let Some(href) = &rich_text.href {
      return markup::new! {
        a[class=&to_apply, href=href] { @rich_text.plain_text }
      }.to_string();
    }

    return markup::new! {
      span[class=&to_apply] { @rich_text.plain_text }
    }.to_string();
  }

  rich_text.plain_text.clone()
}

fn plaintext_block_obj_parse(value: BlockObject) -> String {
  if let Some(text) = value.paragraph {
    text.rich_text.into_iter().map(|x| x.plain_text).collect::<Vec<_>>().join("\n")
  } else if let Some(text) = value.heading_1 {
    text.rich_text.into_iter().map(|x| format!("# {}", x.plain_text)).collect::<Vec<_>>().join("\n")

  } else if let Some(text) = value.heading_2 {
    text.rich_text.into_iter().map(|x| format!("## {}", x.plain_text)).collect::<Vec<_>>().join("\n")

  } else if let Some(text) = value.heading_3 {
    text.rich_text.into_iter().map(|x| format!("### {}", x.plain_text)).collect::<Vec<_>>().join("\n")

  } else if let Some(text) = value.code {
    text.rich_text.into_iter().map(|x| format!("```{}```", x.plain_text)).collect::<Vec<_>>().join("\n")

  } else if let Some(text) = value.bulleted_list_item {
    text.rich_text.into_iter().map(|x| format!("- {}", x.plain_text)).collect::<Vec<_>>().join("\n")


  } else if let Some(text) = value.numbered_list_item {
    text.rich_text.into_iter().map(|x| format!("- {}", x.plain_text)).collect::<Vec<_>>().join("\n")


  } else if let Some(text) = value.quote {
    text.rich_text.into_iter().map(|x| format!("\"{}\"", x.plain_text)).collect::<Vec<_>>().join("\n")

  } else {
    panic!("Encountered unhandled block type {:?}", value)
  }
}

impl From<BlockObject> for String {
  fn from(value: BlockObject) -> Self {
    if let Some(text) = value.paragraph {
      text.build_nested_rich_text(&rich_text_to_spans, &|x| Paragraph { text: &x }.to_string())
    } else if let Some(text) = value.heading_1 {
      text.build_nested_rich_text(&rich_text_to_spans, &|x| H1 { text: &x }.to_string())

    } else if let Some(text) = value.heading_2 {
      text.build_nested_rich_text(&rich_text_to_spans, &|x| H2 { text: &x }.to_string())

    } else if let Some(text) = value.heading_3 {
      text.build_nested_rich_text(&rich_text_to_spans, &|x| H3 { text: &x }.to_string())

    } else if let Some(text) = value.code {
      text.build_html(&|x| Code { text: &x }.to_string())

    } else if let Some(text) = value.bulleted_list_item {
      let contents = text.rich_text.iter().map(|x| x.plain_text.as_str()).collect::<Vec<_>>();
      UnorderedList { items: contents }.to_string()

    } else if let Some(text) = value.numbered_list_item {
      let contents = text.rich_text.iter().map(|x| x.plain_text.as_str()).collect::<Vec<_>>();
      OrderedList { items: contents }.to_string()

    } else if let Some(text) = value.quote {
      text.build_html(&|x| Quote { text: &x }.to_string())

    } else {
      panic!("Encountered unhandled block type {:?}", value)
    }
  }
}
