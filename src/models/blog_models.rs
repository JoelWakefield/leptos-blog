use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blogs {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogEntry {
    pub meta: BlogMeta,
    pub paragraphs: Vec<BlogParagraph>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogMeta {
    pub slug: String,
    pub title: String,
    pub sub_title: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogParagraph {
    pub section: String,
    pub text: String,
    pub is_code: bool,
}