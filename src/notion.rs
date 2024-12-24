use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BlockObject {
  pub paragraph: Option<TextProperty>,
  pub heading_1: Option<TextProperty>,
  pub heading_2: Option<TextProperty>,
  pub heading_3: Option<TextProperty>,
  pub code: Option<TextProperty>,
  pub numbered_list_item: Option<TextProperty>,
  pub bulleted_list_item: Option<TextProperty>,
  pub quote: Option<TextProperty>,
  pub to_do: Option<TextProperty>,
  pub embed: Option<UrlProperty>,
  pub link_preview: Option<UrlProperty>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RichTextAnnotations {
  pub bold: bool,
  pub italic: bool,
  pub strikethrough: bool,
  pub underline: bool,
  pub code: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct RichText {
  pub plain_text: String,
  pub annotations: RichTextAnnotations,
  pub href: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TitleProperty {
  pub title: Vec<RichText>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextProperty {
  pub rich_text: Vec<RichText>
}

impl TextProperty {
  pub fn build_nested_rich_text(&self, nested: &dyn Fn(&RichText) -> String, html_build: &dyn Fn(&str) -> String) -> String {
    html_build(&self.rich_text.iter().map(|x| nested(x)).collect::<Vec<_>>().join(""))
  }

  pub fn build_html(&self, html_build: &dyn Fn(&str) -> String) -> String {
    self.rich_text.iter().map(|x| x.plain_text.as_str()).map(html_build).collect::<Vec<_>>().join("")
  }
}

#[derive(Deserialize, Debug)]
pub struct FilePayload {
  pub url: String
}

#[derive(Deserialize, Debug)]
pub struct ImagePayload {
  pub name: String,
  pub file: FilePayload,
}

#[derive(Deserialize, Debug)]
pub struct ImageProperty {
  pub files: Vec<ImagePayload>,
}

#[derive(Deserialize, Debug, Clone)]
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

  pub async fn get_database(&self, id: &str) -> Vec<PageObject> {
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

  pub async fn get_blocks(&self, page_id: &str) -> Vec<BlockObject> {
    let res = self.client.get(format!("https://api.notion.com/v1/blocks/{}/children", page_id))
      .bearer_auth(self.token.clone())
      .header("Notion-Version", "2022-06-28")
      .send()
      .await
      .expect("Failed to get data from Notion Blocks");

    if !res.status().is_success() {
      panic!("Failed with status {}", res.status());
    }

    let body: DatabaseQueryResponse<BlockObject> = res.json().await.expect("Failed to get JSON from response");

    return body.results;
  }
}