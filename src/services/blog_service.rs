use std::collections::HashMap;

use crate::app::models::blog_models::{BlogMeta, BlogEntry};

pub fn get_blogs() -> HashMap<String, BlogEntry> {
    get_data()
}

pub fn get_blog_meta_list() -> Vec<BlogMeta> {
    let blogs = get_blogs();

    let entries = blogs
        .values()
        .cloned()
        .collect::<Vec<BlogEntry>>();

    let mut metas: Vec<BlogMeta> = vec![];
    for entry in entries {
        metas.push(entry.meta);
    }

    metas
}

pub fn get_blog(slug: String) -> Result<BlogEntry, String> {
    let blogs = get_blogs();

    match blogs.get(&slug) {
        Some(blog) => Ok(blog.clone()),
        None => Err(format!("Cannot find blog: {:?}", slug)),
    }
}

fn get_data() -> HashMap<String, BlogEntry> {
    let json = r#"
    {
        "rust-for-blogs": {
            "meta": {
            "slug": "rust-for-blogs",
            "title": "Rust for Blogs?",
            "sub_title": "Using Leptos for a rust based blog site.",
            "date": "2-11-2025"
            },
            "paragraphs": [
            {
                "section": "frameworks",
                "text": "There are many different frameworks to build a website, and blogs have been around for decades - they're pretty straightforward. Professionally, if I were asked to build a blog, I would go with nextjs since I'm comfortable with React. Next is well made, easy to use, and stable.",
                "is_code": false
            },
            {
                "section": "why-rust",
                "text": "So then, why use rust? Frankly, why not?",
                "is_code": false
            },
            {
                "section": "the-challenge",
                "text": "Ok, honestly? I wanted a challenge. True, I would have a much better time using many other approaches, but the point of The Try Angle is to try new things. Also, Leptos 'feels' like React to me (I know it's not React, there's something to be said for that familiar feeling).",
                "is_code": false
            },
            {
                "section": "how-its-going",
                "text": "So how's it going so far?",
                "is_code": false
            },
            {
                "section": "painful",
                "text": "A bit painful - setting up styling was difficult. If you look at the source code, you might realise that the styling settup is very straightforward - it is... but that approach was taken after trying many other approaches. There was a bit of confusion on which approach to take, and there are a few 'just install this and it works' approaches which do not, in fact, just work. This is nothing new - there are many packages which probably did work at one point but have been abandoned since. I won't name them, but after working in the React space for a few years, it's nothing surprizing.",
                "is_code": false
            },
            {
                "section": "other",
                "text": "Other than that, only a few rust-analyzer issues occurered, but the site does run, and it runs great!",
                "is_code": false
            },
            {
                "section": "next-time",
                "text": "Next time, I'm gonna implement nested routes, more components, and 'services' (they won't be connected to a backend at the moment, just pulling local data). I'm also gonna try a different tailwindcss approach.",
                "is_code": false
            }
            ]
        }
    }"#;

    match serde_json::from_str(json) {
        Ok(data) => data,
        Err(e) => panic!("could not deserialize file data: {}", e),
    }
}