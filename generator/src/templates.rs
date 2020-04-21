use askama::Template;
use slug;

pub struct RenderedMetadata {
    pub permalink: String,
    pub title: String,
}

pub struct RenderedTag {
    pub name: String,
    pub slug: String,
}

impl RenderedTag {
    pub fn from_name(name: &str) -> RenderedTag {
        RenderedTag {
            name: name.to_string(),
            slug: slug::slugify(name),
        }
    }
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    pub meta: RenderedMetadata,
    pub current_url: &'a str,
    pub date: String,
    pub rfc2822_date: String,
    pub content: String,
    pub tags: Vec<RenderedTag>
}

#[derive(Template)]
#[template(path = "articles.html")]
pub struct ArticleListTemplate<'a> {
    pub meta: RenderedMetadata,
    pub base_url: &'a str,
    pub items: &'a Vec<ArticleTemplate<'a>>
}

#[derive(Template)]
#[template(path = "global-feed.xml")]
pub struct GlobalFeedTemplate<'a> {
    pub base_url: &'a str,
    pub items: &'a Vec<ArticleTemplate<'a>>
}

#[derive(Template)]
#[template(path = "tag.html")]
pub struct TagArticleListTemplate<'a> {
    pub meta: RenderedMetadata,
    pub base_url: &'a str,
    pub tag: RenderedTag,
    pub items: &'a Vec<ArticleTemplate<'a>>
}

#[derive(Template)]
#[template(path = "tag-feed.xml")]
pub struct TagFeedTemplate<'a> {
    pub meta: RenderedMetadata,
    pub base_url: &'a str,
    pub tag: RenderedTag,
    pub items: &'a Vec<ArticleTemplate<'a>>
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate<'a> {
    pub meta: RenderedMetadata,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct SitemapTemplate<'a> {
    pub base_url: &'a str,
    pub articles: &'a Vec<&'a str>,
    pub pages: &'a Vec<&'a str>,
    pub tags: &'a Vec<&'a str>,
}
