struct BlogEntry {
    pub meta: BlogMeta,
    pub paragraphs: Vec<BlogParagraph>,
}

struct BlogMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub sub_title: &'static str,
    pub date: &'static str,
}

struct BlogParagraph {
    pub text: &'static str,
    pub is_code: bool,
}
