
diesel::table! {
  posts (id) {
      id -> Int4,
      post_url -> Text,
      posted -> Timestamp,
      image_url -> Nullable<Text>,
      title -> Text,
      body -> Text,
      plaintext_body -> Text,
  }
}
