use crate::markdown_post::{relative_link, MarkdownPost};
use chrono::{DateTime, FixedOffset};
use std::clone::Clone;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub struct Article {
    pub relative_link: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<String>,
}

impl Article {
    fn from_path_lines(root_path: &str, path: &str, lines: Vec<String>) -> Option<Article> {
        let time = DateTime::parse_from_rfc3339(&lines[1]).ok();
        let parts = lines[2].splitn(2, ": ").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let list = parts[1];
            let tags = list.split(", ").map(|x| x.to_string()).collect();
            Some(Article {
                relative_link: relative_link(path, root_path),
                title: lines[0][2..].to_owned(),
                date: time.unwrap(),
                tags,
            })
        } else {
            None
        }
    }

    pub fn read_body(&self, root_path: &str) -> Option<String> {
        File::open(format!("{}/articles/{}.md", root_path, self.relative_link))
            .ok()
            .map(|file| {
                BufReader::new(file)
                    .lines()
                    .skip(3)
                    .flat_map(|x| x.ok())
                    .collect::<Vec<String>>()
                    .join("\n")
            })
    }
}

impl MarkdownPost for Article {
    fn from_path(root_path: &str, path: &str) -> Option<Article> {
        File::open(path)
            .ok()
            .and_then(|file| -> Option<Vec<String>> {
                let lines = BufReader::new(file)
                    .lines()
                    .take(3)
                    .flat_map(|x| x.ok())
                    .collect::<Vec<String>>();
                if lines.len() == 3 {
                    Some(lines)
                } else {
                    None
                }
            })
            .and_then(|lines| Article::from_path_lines(root_path, path, lines))
    }

    fn publish_date(&self) -> DateTime<FixedOffset> {
        self.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn article_from_data() {
        let path = "path".to_owned();
        let title = "A Title".to_owned();
        let date_string = "2020-04-12T21:27:28-07:00".to_owned();
        let tags_string = "tags: a, b".to_owned();
        assert_eq!(
            Article::from_path_lines(
                "",
                "path",
                vec![format!("# {}", title), date_string.clone(), tags_string]
            ),
            Some(Article {
                relative_link: path,
                title: title,
                date: DateTime::parse_from_rfc3339(&date_string).unwrap(),
                tags: vec!["a".to_owned(), "b".to_owned()]
            })
        )
    }
}
