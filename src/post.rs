use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind, LinkType};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
  pub title: String,
  pub date: String,
  pub cover_photo: String,
  pub cover_caption: String,
  pub tags: Vec<String>,
  pub song_link: Option<String>,
  pub content: String,
}

#[derive(PartialEq)]
enum ParserState {
  Title,
  CoverCaption,
  CoverPhoto,
  Date,
  Tags,
  Done
}

impl TryFrom<String> for Post {
  type Error = String;

  fn try_from(post: String) -> Result<Self, Self::Error> {
    let mut parser_state = ParserState::Title;
    let mut title: Option<String> = None;
    let mut date: Option<String> = None;
    let mut cover_photo: Option<String> = None;
    let mut cover_caption: Option<String> = None;
    let mut tags: Option<Vec<String>> = None;
    let mut song_link: Option<String> = None;
    let mut content = String::new();

    let mut block_quote = false;
    let mut skip_text = false;
    for event in Parser::new(&post) {
      match event {
        Event::Start(Tag::Heading(level, _, _)) => {
          if parser_state != ParserState::Done {
            continue;
          }

          let mut header_tag = String::new();
          for _ in 0..(level as u8) {
            header_tag.push_str("#");
          }
          content.push_str(format!("\n{} ", &header_tag).as_str());
        },
        Event::SoftBreak => {},
        Event::End(Tag::Heading(_, _, _)) => {},
        Event::Start(Tag::BlockQuote) => {
          block_quote = true;
        },
        Event::End(Tag::BlockQuote) => {
          block_quote = false;
        },
        Event::Start(Tag::Paragraph) => {
          if skip_text {
            skip_text = false;
            continue;
          }
          if parser_state != ParserState::Done {
            continue;
          }
          content.push_str("\n\n");
        },
        Event::Code(text) => {
          content.push_str(format!("`{}`", text).as_str());
        },
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
          content.push_str("\n```\n");
        },
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
          content.push_str("\n```\n");
        },
        Event::End(Tag::Paragraph) => {},
        Event::Start(Tag::Link(LinkType::Inline, link, _)) => {
          if parser_state != ParserState::Done {
            continue;
          }

          if link.to_string().contains("open.spotify.com") {
            song_link = Some(link.to_string());
            skip_text = true;
          } else {
            content.push_str("[");
          }
        },
        Event::End(Tag::Link(LinkType::Inline, link, _)) => {
          if parser_state != ParserState::Done {
            continue;
          }

          if !skip_text {
            content.push_str(format!("]({})", link).as_str());
          }
        },
        Event::Text(text) => {
          if skip_text {
            continue;
          }
          match parser_state {
            ParserState::Title => {
              title = Some(text.to_string());
              parser_state = ParserState::CoverCaption;
              continue;
            },
            ParserState::Date => {
              date = Some(text.replace("Created: ", "").to_string());
              parser_state = ParserState::Tags;
              continue;
            },
            ParserState::CoverPhoto => {
              cover_photo = Some(text.replace("CoverImage: ", "").to_string());
              parser_state = ParserState::Date;
              continue;
            },
            ParserState::CoverCaption => {
              cover_caption = Some(text.replace("CoverCaption: ", "").to_string());
              parser_state = ParserState::CoverPhoto;
              continue;
            },
            ParserState::Tags => {
              tags = Some(text.replace("Tags: ", "").split(",").map(|x| x.to_string()).collect());
              parser_state = ParserState::Done;
              continue;
            }
            ParserState::Done => {},
          }

          if block_quote {
            content.push_str("> ");
          }
          content.push_str(&text);
        },
        ev => {
          println!("Warning, skipping {:?}", ev);
        }
      }
    }
    
    if title.is_none() {
      return Err("No title found".to_string());
    }

    if date.is_none() {
      return Err("No date found".to_string());
    }

    if cover_photo.is_none() {
      return Err("No cover photo found".to_string());
    }

    if cover_caption.is_none() {
      return Err("No cover caption found".to_string());
    }

    if tags.is_none() {
      return Err("No tags found".to_string());
    }

    Ok(Post {
      title: title.unwrap(),
      date: date.unwrap(),
      cover_photo: cover_photo.unwrap(),
      cover_caption: cover_caption.unwrap(),
      tags: tags.unwrap(),
      song_link,
      content,
    })
  }
}
