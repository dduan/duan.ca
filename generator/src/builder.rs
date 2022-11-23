use crate::article::Article;
use crate::markdown_post;
use crate::page::Page;
use crate::quickie::Quickie;
use crate::site::Site;
use crate::templates::*;
use askama::Template;
use std::error::Error;
use std::fs;
use walkdir::WalkDir;

fn write(text: &str, path: &str, file: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&path)?;
    let p = format!("{}/{}", path, file);
    fs::write(&p, text)?;
    Ok(())
}

fn build_page(
    page: &Page,
    base_url: &str,
    root_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let permalink = format!("{}{}", base_url, page.relative_link);
    if let Some(body) = page.read_body(root_path) {
        let template = PageTemplate {
            meta: RenderedMetadata {
                permalink,
                title: page.title.to_string(),
            },
            content: &body,
        };

        write(
            &template.render()?,
            &format!("{}{}", output_path, page.relative_link),
            "index.html",
        )?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Could not read page {}", page.relative_link),
        )));
    }
    Ok(())
}

fn instantiate_article_template<'a>(
    article: &'a Article,
    base_url: &str,
    root_path: &str,
) -> Option<ArticleTemplate<'a>> {
    match article.read_body(root_path) {
        None => None,
        Some(markdown) => {
            let body = markdown_post::markdown_to_html(&markdown);
            let permalink = format!("{}{}", base_url, article.relative_link);
            let date_string = format!("{}", article.date.format("%Y-%m-%d"));
            Some(ArticleTemplate {
                meta: RenderedMetadata {
                    permalink,
                    title: article.title.to_string(),
                },
                current_url: &article.relative_link,
                date: date_string,
                rfc2822_date: article.date.to_rfc2822(),
                rfc3339_date: article.date.to_rfc3339(),
                content: body,
                tags: article
                    .tags
                    .iter()
                    .map(|tag| RenderedTag::from_name(tag))
                    .collect(),
            })
        }
    }
}

fn build_article_list(
    article_templates: &[ArticleTemplate],
    base_url: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let template = ArticleListTemplate {
        meta: RenderedMetadata {
            permalink: "/articles".to_string(),
            title: "Daniel Duan's Articles".to_string(),
        },
        base_url,
        items: article_templates,
    };

    write(
        &template.render()?,
        &format!("{}{}", output_path, "/articles"),
        "index.html",
    )?;
    Ok(())
}

fn build_global_feed(
    article_templates: &[ArticleTemplate],
    base_url: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let template = GlobalFeedTemplate {
        base_url,
        items: article_templates,
    };

    write(&template.render()?, output_path, "feed.xml")?;
    Ok(())
}

fn build_tag_list(
    tag: &str,
    article_templates: &[ArticleTemplate],
    base_url: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagArticleListTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url,
        tag: RenderedTag::from_name(tag),
        items: article_templates,
    };

    write(
        &template.render()?,
        &format!("{}/tag/{}", output_path, slug),
        "index.html",
    )?;
    Ok(())
}

fn build_tag_feed(
    tag: &str,
    article_templates: &[ArticleTemplate],
    base_url: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagFeedTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}/feed.xml", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url,
        tag: RenderedTag::from_name(tag),
        items: article_templates,
    };

    write(
        &template.render()?,
        &format!("{}/tag/{}", output_path, slug),
        "feed.xml",
    )?;
    Ok(())
}

fn copy_static_assets(root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let asset_path = format!("{}/static", root_path);
    if fs::metadata(&asset_path).is_err() {
        return Ok(());
    }

    for entry in WalkDir::new(&asset_path).into_iter().filter_map(|e| e.ok()) {
        match entry.path().to_str() {
            None => {}
            Some(path) => {
                let target = format!("{}{}", output_path, &path[asset_path.len()..]);
                if entry.file_type().is_dir() {
                    fs::create_dir_all(target)?;
                } else if entry.file_type().is_file() {
                    fs::copy(path, target)?;
                }
            }
        }
    }
    Ok(())
}

fn build_sitemap(site: &Site, output_path: &str) -> Result<(), Box<dyn Error>> {
    let tag_slugs: Vec<String> = site.tags.iter().map(|t| slug::slugify(&t.0)).collect();
    let template = SitemapTemplate {
        base_url: &site.base_url,
        articles: &site
            .articles
            .iter()
            .map(|a| a.relative_link.as_str())
            .collect(),
        pages: &site
            .pages
            .iter()
            .map(|p| p.relative_link.as_str())
            .collect(),
        tags: &tag_slugs.iter().map(|t| t.as_str()).collect(),
    };

    write(&template.render()?, output_path, "sitemap.xml")?;
    Ok(())
}

fn build_404_page(
    base_url: &str,
    root_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let permalink = format!("{}/404.html", base_url);
    match fs::read_to_string(format!("{}/404.html", root_path)) {
        Err(_) => {}
        Ok(body) => {
            let template = PageTemplate {
                meta: RenderedMetadata {
                    permalink,
                    title: "It's a 404.".to_string(),
                },
                content: &body,
            };
            write(&template.render()?, output_path, "404.html")?;
        }
    };

    Ok(())
}

fn media_link_to_display(link: &str) -> Option<String> {
    let lowered = link.to_ascii_lowercase();
    if lowered.ends_with(".png") ||
        lowered.ends_with(".jpg") ||
        lowered.ends_with(".jpeg") 
    {
        Some(format!(r#"<img src="{}">"#, link))
    } else if lowered.ends_with(".mov") {
        Some(
            format!(
                r#"<video id="video-tag" autoplay="" muted="" loop="" playsinline="" width="100%"><source src="{}" type="video/quicktime"></video>
"#,
                link
            )
        )
    } else if lowered.ends_with(".mp4") {
        Some(
            format!(
                r#"<video id="video-tag" autoplay="" muted="" loop="" playsinline="" width="100%"><source src="{}" type="video/mp4"></video>
"#,
                link
            )
        )
    } else {
        None
    }
}

fn instantiate_quickie_template<'a>(
    quickie: &'a Quickie,
    base_url: &'a str,
) -> QuickieTemplate<'a> {
    let permalink = format!("{}{}", base_url, quickie.relative_link);
    let date_string = quickie.date.to_rfc3339();
    let rfc2822_date = quickie.date.to_rfc2822();
    let media_snippets: Vec<String> = (&quickie.media_links)
        .into_iter()
        .filter_map(|link| media_link_to_display(&link))
        .collect();
    QuickieTemplate {
        meta: RenderedMetadata {
            permalink,
            title: quickie.content[0..usize::min(quickie.content.len(), 80)].to_string(),
        },
        current_url: &quickie.relative_link,
        rfc3339_date: date_string,
        rfc2822_date,
        content: markdown_post::markdown_to_html(&quickie.content),
        related_media: media_snippets,
        syndicate_links: &quickie.syndicate_links,
        related_link: &quickie.related_link,
    }
}

pub fn build_site(site: Site, root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if fs::metadata(output_path).is_ok() {
        fs::remove_dir_all(output_path)?;
        fs::create_dir_all(output_path)?;
    }

    copy_static_assets(root_path, output_path)?;

    let article_templates = site
        .articles
        .iter()
        .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
        .collect::<Vec<ArticleTemplate>>();

    build_article_list(&article_templates, &site.base_url, output_path)?;
    build_global_feed(&article_templates, &site.base_url, output_path)?;

    for article_template in article_templates {
        write(
            &article_template.render()?,
            &format!("{}{}", output_path, article_template.current_url),
            "index.html",
        )?;
    }

    let quickie_templates = site
        .quickies
        .iter()
        .map(|quickie| instantiate_quickie_template(quickie, &site.base_url))
        .collect::<Vec<QuickieTemplate>>();

    for quickie_templete in quickie_templates {
        write(
            &quickie_templete.render()?,
            &format!("{}{}", output_path, quickie_templete.current_url),
            "index.html",
        )?;
    }

    for page in &site.pages {
        build_page(page, &site.base_url, root_path, output_path)?;
    }

    for (tag, tagged) in &site.tags {
        let articles = tagged
            .iter()
            .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
            .collect::<Vec<ArticleTemplate>>();
        build_tag_list(tag, &articles, &site.base_url, output_path)?;
        build_tag_feed(tag, &articles, &site.base_url, output_path)?;
    }

    build_sitemap(&site, output_path)?;
    build_404_page(&site.base_url, root_path, output_path)?;
    Ok(())
}
