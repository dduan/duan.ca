use crate::article::Article;
use crate::page::{self, Page};
use crate::quickie::Quickie;
use std::collections::HashMap;
use crate::markdown_post::posts_from_root_path;

#[derive(Debug, PartialEq)]
pub struct Site {
    pub base_url: String,
    pub articles: Vec<Article>,
    pub quickies: Vec<Quickie>,
    pub pages: Vec<Page>,
    pub tags: Vec<(String, Vec<Article>)>,
}

impl Site {
    pub fn from_root_path(base_url: &str, root_path: &str) -> Site {
        let articles: Vec<Article> = posts_from_root_path(root_path, "articles");
        let quickies: Vec<Quickie> = posts_from_root_path(root_path, "quickies");
        let map = Site::tag_article_map_from(&articles);
        Site {
            base_url: base_url.to_string(),
            articles,
            quickies,
            pages: page::pages_from_root_path(root_path),
            tags: map,
        }
    }

    fn tag_article_map_from(articles: &[Article]) -> Vec<(String, Vec<Article>)> {
        let mut map: HashMap<String, Vec<Article>> = HashMap::new();
        for article in articles.to_owned() {
            for tag in &article.tags {
                if let Some(tagged) = map.get_mut(tag) {
                    tagged.push(article.clone());
                } else {
                    map.insert(tag.clone(), vec![article.clone()]);
                }
            }
        }

        for articles in map.values_mut() {
            articles.sort_by(|a, b| b.date.cmp(&a.date));
        }

        let mut result = map.into_iter().collect::<Vec<(String, Vec<Article>)>>();
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    #[test]
    fn test_tag_article_from() {
        let a1 = Article {
            relative_link: "/a1".to_string(),
            title: "A1".to_string(),
            date: DateTime::parse_from_rfc3339("2020-04-18T14:47:02-07:00").unwrap(),
            tags: vec!["Rust".to_string(), "Swift".to_string()],
        };
        let a2 = Article {
            relative_link: "/a2".to_string(),
            title: "A2".to_string(),
            date: DateTime::parse_from_rfc3339("2020-04-18T14:47:01-07:00").unwrap(),
            tags: vec!["Rust".to_string()],
        };
        let a3 = Article {
            relative_link: "/a3".to_string(),
            title: "A3".to_string(),
            date: DateTime::parse_from_rfc3339("2020-04-18T14:47:03-07:00").unwrap(),
            tags: vec!["Swift".to_string()],
        };
        let articles = vec![a1.clone(), a2.clone(), a3.clone()];

        let map = Site::tag_article_map_from(&articles);
        let expected = vec![
            ("Rust".to_string(), vec![a1.clone(), a2.clone()]),
            ("Swift".to_string(), vec![a3.clone(), a1.clone()]),
        ];

        assert_eq!(map, expected)
    }
}
