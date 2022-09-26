use chrono::{DateTime, FixedOffset};
use comrak::{self, ComrakOptions};
use regex::Regex;
use std::clone::Clone;
use std::fs::File;
use std::io::{BufRead, BufReader};
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Clone)]
pub struct Article {
    pub relative_link: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<String>,
}

impl Article {
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
            .map(|lines| Article::from_path_lines(root_path, path, lines))
            .unwrap_or(None)
    }

    fn from_path_lines(root_path: &str, path: &str, lines: Vec<String>) -> Option<Article> {
        let time = DateTime::parse_from_rfc3339(&lines[1]).ok();
        let parts = lines[2].splitn(2, ": ").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let list = parts[1];
            let tags = list.split(", ").map(|x| x.to_string()).collect();
            let relative_link = path[root_path.len()..]
                .rsplitn(2, '.')
                .last()
                .unwrap_or("")
                .to_string();
            Some(Article {
                relative_link,
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
pub fn markdown_to_html(syntax_set: &SyntaxSet, markdown: String) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    let html = comrak::markdown_to_html(&markdown, &options);
    let re = Regex::new(r#"(?s)<pre lang="(\w+)"><code>(.+?)</code></pre>"#).unwrap();
    let mut start: usize = 0;
    let mut highlighted = String::new();
    for cap in re.captures_iter(&html) {
        let lang = &cap[1];
        let code = htmlescape::decode_html(&cap[2]).unwrap();
        let highlighted_code = match syntax_set.find_syntax_by_extension(lang) {
            None => code.to_owned(),
            Some(syntax) => {
                let mut code_gen = ClassedHTMLGenerator::new(syntax, syntax_set);
                for line in code.lines() {
                    code_gen.parse_html_for_line(line)
                }
                code_gen.finalize()
            }
        };

        highlighted.push_str(&html[start..cap.get(0).unwrap().start()]);
        highlighted.push_str("<pre>");
        highlighted.push_str(&highlighted_code);
        highlighted.push_str("</pre>");
        start = cap.get(0).unwrap().end();
    }

    highlighted.push_str(&html[start..html.len()]);
    highlighted
}

pub fn articles_from_root_path(root_path: &str) -> Vec<Article> {
    let root_path = format!(
        "{}/articles",
        if root_path.ends_with('/') {
            &root_path[0..root_path.len() - 1]
        } else {
            root_path
        }
    );

    let mut articles: Vec<Article> = WalkDir::new(&root_path)
        .into_iter()
        .flat_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".md"))
                    .unwrap_or(false)
        })
        .flat_map(|entry| {
            entry
                .path()
                .to_str()
                .map(|path| Article::from_path(&root_path, path))
        })
        .flatten()
        .collect();
    articles.sort_by(|a, b| b.date.cmp(&a.date));
    articles
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
