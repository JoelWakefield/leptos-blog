use std::{collections::HashMap, fs};

use leptos::prelude::*;
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
    pub text: String,
    pub is_code: bool,
}

pub fn get_blog(slug: String) -> Result<BlogEntry, String> {
    let Ok(content) = fs::read_to_string("../../data/blogs.json") else {
        return Err("could not open blogs data file".to_string());
    };

    print!("file contents: {:?}", content);
    let Ok(blogs) = serde_json::from_str::<HashMap<String, BlogEntry>>(content.as_str()) else {
        return Err("could not deserialize file data".to_string());
    };
    print!("blogs: {:?}", blogs);

    match blogs.get(&slug) {
        Some(blog) => Ok(blog.clone()),
        None => Err(format!("Cannot find blog: {:?}", slug)),
    }
}

#[component]
pub fn Blog(slug: &'static str) -> impl IntoView {
    let blog = get_blog(slug.to_string()).unwrap();

    view! {
      <article>
        <h3>{blog.meta.title}</h3>
      </article>
    }
}
