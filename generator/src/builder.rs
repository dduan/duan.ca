use askama::Template;
use crate::article::Article;
use crate::page::Page;
use crate::site::Site;
use crate::templates::*;
use std::error::Error;
use slug;

fn build_page(page: Page, base_url: &str, root_path: &str, output_path: &str) -> Result<(),Box<dyn Error>> {
    let permalink = format!("{}{}", base_url, page.relative_link);
    if let Some(body) = page.read_body(root_path) {
        let template = PageTemplate {
            meta: RenderedMetadata {
                permalink: &permalink,
                title: &page.title,
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

fn build_article(article: Article, base_url: &str, root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {

    let permalink = format!("{}{}", base_url, article.relative_link);
    if let Some(body) = article.read_body(root_path) {
        let date_string = format!("{}", article.date.format("%Y-%m-%d"));
        let template = ArticleTemplate {
            meta: RenderedMetadata {
                permalink: &permalink,
                title: &article.title,
            },
            current_url: &article.relative_link,
            date: &date_string,
            content: &body,
            tags: article.tags.into_iter().map(|tag| {
                let slug = slug::slugify(&tag);
                RenderedTag {
                    name: tag,
                    slug: slug,
                }
            })
            .collect()
        };

        let rendered = template.render()?;
        let page_output_path = format!("{}{}", output_path, article.relative_link);
        let page_output = format!("{}/index.html", page_output_path);
        std::fs::create_dir_all(&page_output_path)?;
        std::fs::write(page_output, rendered)?;
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Could not read page {}", article.relative_link))))
    }

    Ok(())
}

pub fn build_site(site: Site, root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if std::fs::metadata(output_path).is_ok() {
        std::fs::remove_dir_all(output_path)?;
        std::fs::create_dir_all(output_path)?;
    }
    for page in site.pages {
        build_page(page, &site.base_url, root_path, output_path)?;
    }

    for article in site.articles {
        build_article(article, &site.base_url, root_path, output_path)?;
    }

    Ok(())
}
