pub struct BlogEntry {
    pub meta: BlogMeta,
    pub paragraphs: Vec<BlogParagraph>,
}

pub struct BlogMeta {
    pub id: i32,
    pub title: &'static str,
    pub sub_title: &'static str,
}

pub struct BlogParagraph {
    pub text: &'static str,
    pub is_code: bool,
}
