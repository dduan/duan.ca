use askama::Template;
use crate::article::Article;
use crate::page::Page;
use crate::site::Site;
use crate::templates::*;
use std::error::Error;
use slug;

fn build_page(page: &Page, base_url: &str, root_path: &str, output_path: &str) -> Result<(),Box<dyn Error>> {
    let permalink = format!("{}{}", base_url, page.relative_link);
    if let Some(body) = page.read_body(root_path) {
        let template = PageTemplate {
            meta: RenderedMetadata {
                permalink: permalink.to_string(),
                title: page.title.to_string(),
            },
            content: &body,
        };

        let rendered = template.render()?;
        let page_output_path = format!("{}{}", output_path, page.relative_link);
        let page_output = format!("{}/index.html", page_output_path);
        std::fs::create_dir_all(&page_output_path)?;
        std::fs::write(page_output, rendered)?;
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Could not read page {}", page.relative_link))))
    }

    Ok(())
}

fn instantiate_article_template<'a>(article: &'a Article, base_url: &str, root_path: &str) -> Option<ArticleTemplate<'a>> {
    match article.read_body(root_path) {
        None => None,
        Some(body) => {
            instantiate_article_template_with_body(body.to_owned(), article, base_url)
        }
    }
}

fn instantiate_article_template_with_body<'a>(body: String, article: &'a Article, base_url: &str) -> Option<ArticleTemplate<'a>> {
    let permalink = format!("{}{}", base_url, article.relative_link);
    let date_string = format!("{}", article.date.format("%Y-%m-%d"));
    Some(ArticleTemplate {
        meta: RenderedMetadata {
            permalink: permalink.to_string(),
            title: article.title.to_string(),
        },
        current_url: &article.relative_link,
        date: date_string.to_string(),
        content: body.to_owned(),
        tags: article.tags.iter().map(|tag| {
            let slug = slug::slugify(&tag);
            RenderedTag {
                name: tag.clone(),
                slug: slug,
            }
        })
        .collect()
    })
}

fn build_article(article_template: &ArticleTemplate, output_path: &str) -> Result<(), Box<dyn Error>> {
    let rendered = article_template.render()?;
    let page_output_path = format!("{}{}", output_path, article_template.current_url);
    let page_output = format!("{}/index.html", page_output_path);
    std::fs::create_dir_all(&page_output_path)?;
    std::fs::write(page_output, rendered)?;

    Ok(())
}


fn build_article_list(article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let template = ArticleListTemplate {
        meta: RenderedMetadata {
            permalink: "/articles".to_string(),
            title: "Daniel Duan's Articles".to_string()
        },
        base_url: base_url,
        items: article_templates,
    };

    let rendered = template.render()?;
    let page_output_path = format!("{}{}", output_path, "/articles");
    let page_output = format!("{}/index.html", page_output_path);
    std::fs::create_dir_all(&page_output_path)?;
    std::fs::write(page_output, rendered)?;

    Ok(())
}

fn build_global_feed(article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let template = GlobalFeedTemplate {
        base_url: base_url,
        items: article_templates,
    };

    let rendered = template.render()?;
    let page_output = format!("{}/feed.xml", output_path);
    std::fs::write(page_output, rendered)?;

    Ok(())
}

fn build_tag_list(tag: &str, article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagArticleListTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url: base_url,
        tag: RenderedTag {
            name: tag.to_string(),
            slug: slug::slugify(tag),
        },
        items: article_templates,
    };

    let rendered = template.render()?;
    let page_output_path = format!("{}/tag/{}", output_path, slug);
    let page_output = format!("{}/index.html", page_output_path);
    std::fs::create_dir_all(&page_output_path)?;
    std::fs::write(page_output, rendered)?;

    Ok(())
}

fn build_tag_feed(tag: &str, article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagFeedTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}/feed.xml", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url: base_url,
        tag: RenderedTag {
            name: tag.to_string(),
            slug: slug::slugify(tag),
        },
        items: article_templates,
    };

    let rendered = template.render()?;
    let page_output_path = format!("{}/tag/{}", output_path, slug);
    let page_output = format!("{}/feed.xml", page_output_path);
    std::fs::create_dir_all(&page_output_path)?;
    std::fs::write(page_output, rendered)?;

    Ok(())
}

pub fn build_site(site: Site, root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if std::fs::metadata(output_path).is_ok() {
        std::fs::remove_dir_all(output_path)?;
        std::fs::create_dir_all(output_path)?;
    }

    let article_templates = site
        .articles
        .iter()
        .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
        .collect::<Vec<ArticleTemplate>>();

    build_article_list(&article_templates, &site.base_url, output_path)?;
    build_global_feed(&article_templates, &site.base_url, output_path)?;

    for article_template in article_templates {
        build_article(&article_template, output_path)?;
    }

    for page in &site.pages {
        build_page(page, &site.base_url, root_path, output_path)?;
    }

    for (tag, tagged) in &site.tags {
        let articles = tagged
            .iter()
            .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
            .collect::<Vec<ArticleTemplate>>();
        build_tag_list(&tag, &articles, &site.base_url, output_path)?;
        build_tag_feed(&tag, &articles, &site.base_url, output_path)?;
    }

    Ok(())
}
