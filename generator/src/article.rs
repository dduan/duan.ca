use chrono::{DateTime, FixedOffset};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

#[derive(Debug, PartialEq)]
pub struct Article {
    path: String,
    title: String,
    date: DateTime<FixedOffset>,
    tags: Vec<String>,
}

impl Article {
    pub fn from_path(root_path: &str, path: &str) -> Option<Article> {
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
            .map(|lines| Article::from_path_lines(root_path, path, lines))
            .unwrap_or(None)
    }

    fn from_path_lines(root_path: &str, path: &str, lines: Vec<String>) -> Option<Article> {
        let time = DateTime::parse_from_rfc3339(&lines[1]).ok();
        let parts = lines[2].splitn(2, ": ").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let list = parts[1];
            let tags = list
                .split(", ")
                .map(|x| { x.to_string() })
                .collect();
            Some(
                Article {
                    path:  path[root_path.len()..].to_string(),
                    title: lines[0].to_owned(),
                    date: time.unwrap(),
                    tags: tags,
                }
            )
        } else {
            None
        }
    }

    pub fn read_body(&self, root_path: &str) -> Option<String> {
        File::open(format!("{}{}", root_path, self.path))
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

pub fn articles_from_root_path(root_path: &str) -> Vec<Article> {
    WalkDir::new(root_path)
        .into_iter()
        .flat_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() &&
                e.file_name()
                .to_str()
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)

        })
    .flat_map(|entry| {
        entry
            .path()
            .to_str()
            .map(|path| Article::from_path(root_path, path))
    })
    .flat_map(|a| a)
    .collect()
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
                vec![
                    title.clone(),
                    date_string.clone(),
                    tags_string
                ]
            ),
            Some(
                Article {
                    path: path,
                    title: title,
                    date: DateTime::parse_from_rfc3339(&date_string).unwrap(),
                    tags: vec!["a".to_owned(), "b".to_owned()]
                }
            )
        )
    }
}
