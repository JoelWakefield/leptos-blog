pub struct BlogEntry {
    pub meta: BlogMeta,
    pub paragraphs: Vec<BlogParagraph>,
}

pub struct BlogMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub sub_title: &'static str,
    pub date: &'static str,
}

pub struct BlogParagraph {
    pub text: &'static str,
    pub is_code: bool,
}
