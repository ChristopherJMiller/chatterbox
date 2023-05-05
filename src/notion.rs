use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct BlockObject {
  paragraph: Option<TextProperty>,
  heading_1: Option<TextProperty>,
  heading_2: Option<TextProperty>,
  heading_3: Option<TextProperty>,
  code: Option<TextProperty>,
}

#[derive(Deserialize, Debug)]
pub struct RichText {
  plain_text: String,
}

#[derive(Deserialize, Debug)]
pub struct TitleProperty {
  title: Vec<RichText>,
}

#[derive(Deserialize, Debug)]
pub struct TextProperty {
  rich_text: Vec<RichText>
}

#[derive(Deserialize, Debug)]
pub struct FilePayload {
  url: String
}

#[derive(Deserialize, Debug)]
pub struct ImagePayload {
  name: String,
  file: FilePayload,
}

#[derive(Deserialize, Debug)]
pub struct ImageProperty {
  files: Vec<ImagePayload>,
}

#[derive(Deserialize, Debug)]
pub struct UrlProperty {
  pub url: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct TimeProperty {
  pub created_time: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct PageProperties {
  pub created: TimeProperty,
  pub music: UrlProperty,
  pub cover_image: ImageProperty,
  pub cover_caption: TextProperty,
  pub name: TitleProperty,
}

#[derive(Deserialize, Debug)]
pub struct PageObject {
  pub id: String,
  pub created_time: String,
  pub properties: PageProperties,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseQueryResponse<T> {
  pub results: Vec<T>,
}

pub struct Notion {
  client: Client,
  token: String,
}

impl Notion {
  pub fn new(token: String) -> Self {
    Self {
      client: Client::new(),
      token
    }
  }

  pub async fn get_database(&self, id: String) -> Vec<PageObject> {
    let res = self.client.post(format!("https://api.notion.com/v1/databases/{}/query", id))
      .bearer_auth(self.token.clone())
      .header("Notion-Version", "2022-06-28")
      .header("Content-Type", "application/json")
      .json(&serde_json::json!({
        "filter": {
          "and": [
            {
              "property": "Publish",
              "checkbox": {
                  "equals": true
              }
            }
          ]
        }
      }))
      .send()
      .await
      .expect("Failed to get data from Notion Database");

    let body: DatabaseQueryResponse<PageObject> = res.json().await.expect("Failed to get JSON from response");

    return body.results;
  }

  pub async fn get_blocks(&self, page_id: String) -> Vec<BlockObject> {
    let res = self.client.get(format!("https://api.notion.com/v1/blocks/{}/children", page_id))
      .bearer_auth(self.token.clone())
      .header("Notion-Version", "2022-06-28")
      .send()
      .await
      .expect("Failed to get data from Notion Database");

    if !res.status().is_success() {
      panic!("Failed with status {}", res.status());
    }

    let body: DatabaseQueryResponse<BlockObject> = res.json().await.expect("Failed to get JSON from response");

    return body.results;
  }
}