use askama::Template;

pub struct RenderedMetadata<'a> {
    pub permalink: &'a str,
    pub title: &'a str,
}

pub struct RenderedTag<'a> {
    pub name: &'a str,
    pub slug: &'a str,
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    pub meta: RenderedMetadata<'a>,
    pub current_url: &'a str,
    pub date: &'a str,
    pub content: &'a str,
    pub tags: Vec<&'a RenderedTag<'a>>
}

#[derive(Template)]
#[template(path = "articles.html")]
pub struct ArticleListTemplate<'a> {
    pub meta: RenderedMetadata<'a>,
    pub base_url: &'a str,
    pub items: Vec<&'a ArticleTemplate<'a>>
}

#[derive(Template)]
#[template(path = "tag.html")]
pub struct TagArticleListTemplate<'a> {
    pub meta: RenderedMetadata<'a>,
    pub tag_name: &'a str,
    pub list: &'a ArticleListTemplate<'a>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub meta: RenderedMetadata<'a>,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate<'a> {
    pub meta: RenderedMetadata<'a>,
    pub content: &'a str,
}
